use super::EngineService;
use super::*;
use crate::domain::artifact::artifact_slug;

impl EngineService {
    pub(super) fn run_system_shaping(
        &self,
        store: &WorkspaceStore,
        request: RunRequest,
        policy_set: crate::domain::policy::PolicySet,
    ) -> Result<RunSummary, EngineError> {
        let identity = self.next_unique_run_identity(store)?;
        let now = identity.created_at;
        let run_id = identity.run_id.clone();
        let run_uuid = identity.uuid.as_simple().to_string();
        let run_short_id = identity.short_id.clone();
        let mut artifact_contract = contract_for_mode(request.mode);
        classifier::apply_verification_layers(&policy_set, request.risk, &mut artifact_contract);

        let input_fingerprints =
            self.capture_input_fingerprints(&request.inputs, &request.inline_inputs)?;
        let input_scope = request.merged_input_sources();
        let evidence_path = format!("runs/{run_id}/evidence.toml");
        let context_request = self.governed_request(GovernedRequestSpec {
            run_id: &run_id,
            mode: Mode::SystemShaping,
            risk: request.risk,
            zone: request.zone,
            system_context: request.system_context,
            owner: &request.owner,
            adapter: canon_adapters::AdapterKind::Filesystem,
            capability: CapabilityKind::ReadRepository,
            summary: "capture system-shaping context and bounded intent",
            scope: input_scope.clone(),
        });
        let context_decision =
            invocation_runtime::evaluate_request_policy(&context_request, &policy_set);
        let context_summary =
            self.read_requirements_context(&request.inputs, &request.inline_inputs)?;
        let context_attempt = self.completed_attempt(
            &context_request,
            1,
            "filesystem",
            ToolOutcome {
                kind: ToolOutcomeKind::Succeeded,
                summary: format!(
                    "Captured system-shaping context from {} input(s).",
                    request.authored_input_count()
                ),
                exit_code: Some(0),
                payload_refs: Vec::new(),
                candidate_artifacts: Vec::new(),
                recorded_at: OffsetDateTime::now_utc(),
            },
        );

        let generation_request = self.governed_request(GovernedRequestSpec {
            run_id: &run_id,
            mode: Mode::SystemShaping,
            risk: request.risk,
            zone: request.zone,
            system_context: request.system_context,
            owner: &request.owner,
            adapter: canon_adapters::AdapterKind::CopilotCli,
            capability: CapabilityKind::GenerateContent,
            summary: "generate bounded system-shaping analysis",
            scope: input_scope.clone(),
        });
        let generation_decision =
            invocation_runtime::evaluate_request_policy(&generation_request, &policy_set);
        let copilot = CopilotCliAdapter;
        let generation_output = copilot.generate(&context_summary);
        let generation_attempt = self.completed_attempt(
            &generation_request,
            1,
            &generation_output.executor,
            ToolOutcome {
                kind: ToolOutcomeKind::Succeeded,
                summary: generation_output.summary.clone(),
                exit_code: Some(0),
                payload_refs: Vec::new(),
                candidate_artifacts: artifact_contract
                    .artifact_requirements
                    .iter()
                    .map(|requirement| requirement.file_name.clone())
                    .collect(),
                recorded_at: OffsetDateTime::now_utc(),
            },
        );

        let critique_request = self.governed_request(GovernedRequestSpec {
            run_id: &run_id,
            mode: Mode::SystemShaping,
            risk: request.risk,
            zone: request.zone,
            system_context: request.system_context,
            owner: &request.owner,
            adapter: canon_adapters::AdapterKind::CopilotCli,
            capability: CapabilityKind::CritiqueContent,
            summary: "critique bounded system-shaping analysis",
            scope: input_scope.clone(),
        });
        let critique_decision =
            invocation_runtime::evaluate_request_policy(&critique_request, &policy_set);
        let critique_output = copilot.critique(&generation_output.summary);
        let critique_attempt = self.completed_attempt(
            &critique_request,
            1,
            &critique_output.executor,
            ToolOutcome {
                kind: ToolOutcomeKind::Succeeded,
                summary: critique_output.summary.clone(),
                exit_code: Some(0),
                payload_refs: Vec::new(),
                candidate_artifacts: Vec::new(),
                recorded_at: OffsetDateTime::now_utc(),
            },
        );

        let artifact_paths = artifact_contract
            .artifact_requirements
            .iter()
            .map(|requirement| {
                format!("artifacts/{}/{}/{}", run_id, request.mode.as_str(), requirement.file_name)
            })
            .collect::<Vec<_>>();
        let generation_path = GenerationPath {
            path_id: format!("generation:{}", generation_request.request_id),
            request_ids: vec![generation_request.request_id.clone()],
            lineage_classes: vec![LineageClass::AiVendorFamily],
            derived_artifacts: artifact_paths.clone(),
        };
        let validation_path = ValidationPath {
            path_id: format!("validation:{}", critique_request.request_id),
            request_ids: vec![critique_request.request_id.clone()],
            lineage_classes: vec![LineageClass::AiVendorFamily],
            verification_refs: vec![format!(
                "runs/{run_id}/invocations/{}/attempt-01.toml",
                critique_request.request_id
            )],
            independence: evidence_builder::assess_validation_independence(
                &generation_path,
                &ValidationPath {
                    path_id: format!("validation:{}", critique_request.request_id),
                    request_ids: vec![critique_request.request_id.clone()],
                    lineage_classes: vec![LineageClass::AiVendorFamily],
                    verification_refs: vec![format!(
                        "runs/{run_id}/invocations/{}/attempt-01.toml",
                        critique_request.request_id
                    )],
                    independence: evidence_builder::default_independence(&generation_path.path_id),
                },
            ),
        };

        let packet_metadata_contents = self.build_runtime_packet_metadata(
            &run_id,
            &request,
            &[],
            &artifact_contract.artifact_requirements,
            None,
            None,
        )?;

        let artifacts = artifact_contract
            .artifact_requirements
            .iter()
            .map(|requirement| PersistedArtifact {
                record: ArtifactRecord {
                    file_name: requirement.file_name.clone(),
                    relative_path: format!(
                        "artifacts/{}/{}/{}",
                        run_id,
                        request.mode.as_str(),
                        requirement.file_name
                    ),
                    format: requirement.format,
                    provenance: Some(crate::domain::artifact::ArtifactProvenance {
                        request_ids: vec![
                            context_request.request_id.clone(),
                            generation_request.request_id.clone(),
                            critique_request.request_id.clone(),
                        ],
                        evidence_bundle: Some(evidence_path.clone()),
                        disposition: crate::domain::execution::EvidenceDisposition::Supporting,
                    }),
                },
                contents: match artifact_slug(&requirement.file_name) {
                    "packet-metadata.json" => packet_metadata_contents.clone(),
                    _ => render_system_shaping_artifact(
                        &requirement.file_name,
                        &context_summary,
                        &generation_output.summary,
                        &critique_output.summary,
                    ),
                },
            })
            .collect::<Vec<_>>();

        let approvals = Vec::new();
        let gate_inputs = artifacts
            .iter()
            .map(|artifact| (artifact.record.file_name.clone(), artifact.contents.clone()))
            .collect::<Vec<_>>();
        let gates = gatekeeper::evaluate_system_shaping_gates(
            &artifact_contract,
            &gate_inputs,
            gatekeeper::SystemShapingGateContext {
                owner: &request.owner,
                risk: request.risk,
                zone: request.zone,
                approvals: &approvals,
                evidence_complete: true,
            },
        );
        let state = run_state_from_gates(&gates);

        let mut verification_records = verification_runner::analysis_verification_records(
            "system-shaping",
            &artifact_contract.required_verification_layers,
            &artifact_paths,
        );
        for record in &mut verification_records {
            record.request_ids =
                vec![generation_request.request_id.clone(), critique_request.request_id.clone()];
            record.validation_path_id = Some(validation_path.path_id.clone());
            record.evidence_bundle = Some(evidence_path.clone());
        }

        let evidence = EvidenceBundle {
            run_id: run_id.clone(),
            generation_paths: vec![generation_path],
            validation_paths: vec![validation_path],
            denied_invocations: Vec::new(),
            trace_refs: vec![format!("traces/{run_id}.jsonl")],
            artifact_refs: artifact_paths.clone(),
            decision_refs: vec![
                format!("runs/{run_id}/invocations/{}/decision.toml", context_request.request_id),
                format!(
                    "runs/{run_id}/invocations/{}/decision.toml",
                    generation_request.request_id
                ),
                format!("runs/{run_id}/invocations/{}/decision.toml", critique_request.request_id),
            ],
            approval_refs: Vec::new(),
        };

        let bundle = PersistedRunBundle {
            run: RunManifest {
                run_id: run_id.clone(),
                uuid: Some(run_uuid.clone()),
                short_id: Some(run_short_id.clone()),
                slug: None,
                title: None,
                mode: request.mode,
                risk: request.risk,
                zone: request.zone,
                system_context: request.system_context,
                classification: request.classification.clone(),
                owner: request.owner.clone(),
                lineage: None,
                created_at: now,
            },
            context: self.build_run_context(&request, input_fingerprints, now),
            state: RunStateManifest { state, updated_at: now },
            artifact_contract: artifact_contract.clone(),
            links: LinkManifest {
                artifacts: artifact_paths.clone(),
                decisions: Vec::new(),
                traces: Vec::new(),
                invocations: Vec::new(),
                evidence: Some(evidence_path.clone()),
            },
            verification_records,
            artifacts,
            gates,
            approvals,
            evidence: Some(evidence),
            invocations: vec![
                PersistedInvocation {
                    request: context_request,
                    decision: context_decision,
                    attempts: vec![context_attempt],
                    approvals: Vec::new(),
                },
                PersistedInvocation {
                    request: generation_request,
                    decision: generation_decision,
                    attempts: vec![generation_attempt],
                    approvals: Vec::new(),
                },
                PersistedInvocation {
                    request: critique_request,
                    decision: critique_decision,
                    attempts: vec![critique_attempt],
                    approvals: Vec::new(),
                },
            ],
        };

        store.persist_run_bundle(&bundle)?;
        self.summarize_run(
            store,
            RunSummarySpec {
                run_id: &run_id,
                mode: request.mode,
                risk: request.risk,
                zone: request.zone,
                state,
                artifact_count: bundle.artifacts.len(),
            },
        )
    }

    pub(super) fn run_architecture(
        &self,
        store: &WorkspaceStore,
        request: RunRequest,
        policy_set: crate::domain::policy::PolicySet,
    ) -> Result<RunSummary, EngineError> {
        let identity = self.next_unique_run_identity(store)?;
        let now = identity.created_at;
        let run_id = identity.run_id.clone();
        let run_uuid = identity.uuid.as_simple().to_string();
        let run_short_id = identity.short_id.clone();
        let mut artifact_contract = contract_for_mode(request.mode);
        classifier::apply_verification_layers(&policy_set, request.risk, &mut artifact_contract);

        let input_fingerprints =
            self.capture_input_fingerprints(&request.inputs, &request.inline_inputs)?;
        let input_scope = request.merged_input_sources();
        let evidence_path = format!("runs/{run_id}/evidence.toml");
        let context_request = self.governed_request(GovernedRequestSpec {
            run_id: &run_id,
            mode: Mode::Architecture,
            risk: request.risk,
            zone: request.zone,
            system_context: request.system_context,
            owner: &request.owner,
            adapter: canon_adapters::AdapterKind::Filesystem,
            capability: CapabilityKind::ReadRepository,
            summary: "capture architecture context and structural dilemma",
            scope: input_scope.clone(),
        });
        let context_decision =
            invocation_runtime::evaluate_request_policy(&context_request, &policy_set);
        let context_summary =
            self.read_requirements_context(&request.inputs, &request.inline_inputs)?;
        artifact_contract = crate::artifacts::contract::architecture_contract_for_context(
            &artifact_contract,
            &context_summary,
        );
        let context_attempt = self.completed_attempt(
            &context_request,
            1,
            "filesystem",
            ToolOutcome {
                kind: ToolOutcomeKind::Succeeded,
                summary: format!(
                    "Captured architecture context from {} input(s).",
                    request.authored_input_count()
                ),
                exit_code: Some(0),
                payload_refs: Vec::new(),
                candidate_artifacts: Vec::new(),
                recorded_at: OffsetDateTime::now_utc(),
            },
        );

        let generation_request = self.governed_request(GovernedRequestSpec {
            run_id: &run_id,
            mode: Mode::Architecture,
            risk: request.risk,
            zone: request.zone,
            system_context: request.system_context,
            owner: &request.owner,
            adapter: canon_adapters::AdapterKind::CopilotCli,
            capability: CapabilityKind::GenerateContent,
            summary: "generate bounded architecture analysis",
            scope: input_scope.clone(),
        });
        let generation_decision =
            invocation_runtime::evaluate_request_policy(&generation_request, &policy_set);
        let copilot = CopilotCliAdapter;
        let generation_output = copilot.generate(&context_summary);

        let critique_request = self.governed_request(GovernedRequestSpec {
            run_id: &run_id,
            mode: Mode::Architecture,
            risk: request.risk,
            zone: request.zone,
            system_context: request.system_context,
            owner: &request.owner,
            adapter: canon_adapters::AdapterKind::CopilotCli,
            capability: CapabilityKind::CritiqueContent,
            summary: "critique bounded architecture decisions and invariants",
            scope: input_scope.clone(),
        });
        let critique_decision =
            invocation_runtime::evaluate_request_policy(&critique_request, &policy_set);
        let critique_output = copilot.critique(&generation_output.summary);
        let critique_attempt = self.completed_attempt(
            &critique_request,
            1,
            &critique_output.executor,
            ToolOutcome {
                kind: ToolOutcomeKind::Succeeded,
                summary: critique_output.summary.clone(),
                exit_code: Some(0),
                payload_refs: Vec::new(),
                candidate_artifacts: Vec::new(),
                recorded_at: OffsetDateTime::now_utc(),
            },
        );

        let selected_artifact_names = artifact_contract
            .artifact_requirements
            .iter()
            .map(|requirement| requirement.file_name.clone())
            .collect::<Vec<_>>();

        let view_manifest_contents =
            build_architecture_view_manifest(&selected_artifact_names, &context_summary)?;
        let packet_metadata_contents = build_architecture_packet_metadata(
            &run_id,
            &selected_artifact_names,
            &context_summary,
        )?;

        let artifacts = artifact_contract
            .artifact_requirements
            .iter()
            .map(|requirement| PersistedArtifact {
                record: ArtifactRecord {
                    file_name: requirement.file_name.clone(),
                    relative_path: format!(
                        "artifacts/{}/{}/{}",
                        run_id,
                        request.mode.as_str(),
                        requirement.file_name
                    ),
                    format: requirement.format,
                    provenance: Some(crate::domain::artifact::ArtifactProvenance {
                        request_ids: vec![
                            context_request.request_id.clone(),
                            generation_request.request_id.clone(),
                            critique_request.request_id.clone(),
                        ],
                        evidence_bundle: Some(evidence_path.clone()),
                        disposition: crate::domain::execution::EvidenceDisposition::Supporting,
                    }),
                },
                contents: match artifact_slug(&requirement.file_name) {
                    "view-manifest.json" => view_manifest_contents.clone(),
                    "packet-metadata.json" => packet_metadata_contents.clone(),
                    _ => render_architecture_artifact(
                        &requirement.file_name,
                        &context_summary,
                        &generation_output.summary,
                        &critique_output.summary,
                    ),
                },
            })
            .collect::<Vec<_>>();

        let artifact_paths = artifacts
            .iter()
            .map(|artifact| artifact.record.relative_path.clone())
            .collect::<Vec<_>>();

        let generation_attempt = self.completed_attempt(
            &generation_request,
            1,
            &generation_output.executor,
            ToolOutcome {
                kind: ToolOutcomeKind::Succeeded,
                summary: generation_output.summary.clone(),
                exit_code: Some(0),
                payload_refs: Vec::new(),
                candidate_artifacts: selected_artifact_names.clone(),
                recorded_at: OffsetDateTime::now_utc(),
            },
        );

        let generation_path = GenerationPath {
            path_id: format!("generation:{}", generation_request.request_id),
            request_ids: vec![generation_request.request_id.clone()],
            lineage_classes: vec![LineageClass::AiVendorFamily],
            derived_artifacts: artifact_paths.clone(),
        };
        let validation_path = ValidationPath {
            path_id: format!("validation:{}", critique_request.request_id),
            request_ids: vec![critique_request.request_id.clone()],
            lineage_classes: vec![LineageClass::AiVendorFamily],
            verification_refs: vec![format!(
                "runs/{run_id}/invocations/{}/attempt-01.toml",
                critique_request.request_id
            )],
            independence: evidence_builder::assess_validation_independence(
                &generation_path,
                &ValidationPath {
                    path_id: format!("validation:{}", critique_request.request_id),
                    request_ids: vec![critique_request.request_id.clone()],
                    lineage_classes: vec![LineageClass::AiVendorFamily],
                    verification_refs: vec![format!(
                        "runs/{run_id}/invocations/{}/attempt-01.toml",
                        critique_request.request_id
                    )],
                    independence: evidence_builder::default_independence(&generation_path.path_id),
                },
            ),
        };

        let approvals = Vec::new();
        let gate_inputs = artifacts
            .iter()
            .map(|artifact| (artifact.record.file_name.clone(), artifact.contents.clone()))
            .collect::<Vec<_>>();
        let gates = gatekeeper::evaluate_architecture_gates(
            &artifact_contract,
            &gate_inputs,
            gatekeeper::ArchitectureGateContext {
                owner: &request.owner,
                risk: request.risk,
                zone: request.zone,
                approvals: &approvals,
                evidence_complete: true,
            },
        );
        let state = run_state_from_gates(&gates);

        let mut verification_records = verification_runner::analysis_verification_records(
            "architecture",
            &artifact_contract.required_verification_layers,
            &artifact_paths,
        );
        for record in &mut verification_records {
            record.request_ids =
                vec![generation_request.request_id.clone(), critique_request.request_id.clone()];
            record.validation_path_id = Some(validation_path.path_id.clone());
            record.evidence_bundle = Some(evidence_path.clone());
        }

        let evidence = EvidenceBundle {
            run_id: run_id.clone(),
            generation_paths: vec![generation_path],
            validation_paths: vec![validation_path],
            denied_invocations: Vec::new(),
            trace_refs: vec![format!("traces/{run_id}.jsonl")],
            artifact_refs: artifact_paths.clone(),
            decision_refs: vec![
                format!("runs/{run_id}/invocations/{}/decision.toml", context_request.request_id),
                format!(
                    "runs/{run_id}/invocations/{}/decision.toml",
                    generation_request.request_id
                ),
                format!("runs/{run_id}/invocations/{}/decision.toml", critique_request.request_id),
            ],
            approval_refs: Vec::new(),
        };

        let bundle = PersistedRunBundle {
            run: RunManifest {
                run_id: run_id.clone(),
                uuid: Some(run_uuid.clone()),
                short_id: Some(run_short_id.clone()),
                slug: None,
                title: None,
                mode: request.mode,
                risk: request.risk,
                zone: request.zone,
                system_context: request.system_context,
                classification: request.classification.clone(),
                owner: request.owner.clone(),
                lineage: None,
                created_at: now,
            },
            context: self.build_run_context(&request, input_fingerprints, now),
            state: RunStateManifest { state, updated_at: now },
            artifact_contract: artifact_contract.clone(),
            links: LinkManifest {
                artifacts: artifact_paths.clone(),
                decisions: Vec::new(),
                traces: Vec::new(),
                invocations: Vec::new(),
                evidence: Some(evidence_path.clone()),
            },
            verification_records,
            artifacts,
            gates,
            approvals,
            evidence: Some(evidence),
            invocations: vec![
                PersistedInvocation {
                    request: context_request,
                    decision: context_decision,
                    attempts: vec![context_attempt],
                    approvals: Vec::new(),
                },
                PersistedInvocation {
                    request: generation_request,
                    decision: generation_decision,
                    attempts: vec![generation_attempt],
                    approvals: Vec::new(),
                },
                PersistedInvocation {
                    request: critique_request,
                    decision: critique_decision,
                    attempts: vec![critique_attempt],
                    approvals: Vec::new(),
                },
            ],
        };

        store.persist_run_bundle(&bundle)?;
        self.summarize_run(
            store,
            RunSummarySpec {
                run_id: &run_id,
                mode: request.mode,
                risk: request.risk,
                zone: request.zone,
                state,
                artifact_count: bundle.artifacts.len(),
            },
        )
    }
}

fn build_architecture_view_manifest(
    selected_artifact_names: &[String],
    context_summary: &str,
) -> Result<String, EngineError> {
    let primary_artifact = architecture_packet_body_artifacts(selected_artifact_names)
        .into_iter()
        .next()
        .unwrap_or_else(|| "architecture-overview.md".to_string());

    let views = [
        ("system-context", "System Context", true, "system-context.md", "system-context.mmd"),
        ("container", "Container View", true, "container-view.md", "container-view.mmd"),
        ("deployment", "Deployment View", true, "deployment-view.md", "deployment-view.mmd"),
        ("component", "Component View", false, "component-view.md", "component-view.mmd"),
        ("dynamic", "Dynamic View", false, "dynamic-view.md", "dynamic-view.mmd"),
    ]
    .into_iter()
    .map(|(view, title, required, markdown_artifact, mermaid_artifact)| {
        let emitted_markdown = selected_architecture_artifact_name(
            selected_artifact_names,
            markdown_artifact,
        );
        let emitted_mermaid = selected_architecture_artifact_name(
            selected_artifact_names,
            mermaid_artifact,
        );
        let authored = crate::artifacts::markdown::architecture_view_authored(
            markdown_artifact,
            context_summary,
        );
        let included = emitted_markdown.is_some();
        let artifacts = [emitted_markdown, emitted_mermaid]
            .into_iter()
            .flatten()
            .map(|file_name| serde_json::Value::String(file_name.to_string()))
            .collect::<Vec<_>>();

        serde_json::json!({
            "view": view,
            "title": title,
            "required": required,
            "authored": authored,
            "included": included,
            "artifacts": artifacts,
            "reason": if authored {
                serde_json::Value::Null
            } else if required {
                serde_json::Value::String(format!(
                    "No `## {}` section was authored; Canon emitted an explicit omission artifact instead.",
                    architecture_view_heading(markdown_artifact)
                ))
            } else {
                serde_json::Value::String(format!(
                    "No `## {}` section was authored; this optional view was omitted from the packet.",
                    architecture_view_heading(markdown_artifact)
                ))
            }
        })
    })
    .collect::<Vec<_>>();

    serde_json::to_string_pretty(&serde_json::json!({
        "primary_artifact": primary_artifact,
        "render_targets": {
            "mermaid_source": "generated",
            "svg": "unsupported",
            "png": "unsupported"
        },
        "views": views,
    }))
    .map_err(|error| {
        EngineError::Validation(format!("architecture view manifest serialization failed: {error}"))
    })
}

fn build_architecture_packet_metadata(
    run_id: &str,
    selected_artifact_names: &[String],
    context_summary: &str,
) -> Result<String, EngineError> {
    let artifact_order = architecture_packet_body_artifacts(selected_artifact_names);
    let primary_artifact =
        artifact_order.first().cloned().unwrap_or_else(|| "architecture-overview.md".to_string());

    let included_views = [
        "system-context.md",
        "container-view.md",
        "deployment-view.md",
        "component-view.md",
        "dynamic-view.md",
    ]
    .into_iter()
    .filter(|file_name| {
        selected_artifact_names.iter().any(|selected| artifact_slug(selected) == *file_name)
    })
    .map(|file_name| serde_json::Value::String(architecture_view_heading(file_name).to_string()))
    .collect::<Vec<_>>();

    serde_json::to_string_pretty(&serde_json::json!({
        "packet_kind": "architecture-visual-packet",
        "run_id": run_id,
        "mode": "architecture",
        "primary_artifact": primary_artifact,
        "artifact_order": artifact_order,
        "artifact_count": selected_artifact_names.len(),
        "included_views": included_views,
        "render_targets": {
            "mermaid_source": "generated",
            "svg": "unsupported",
            "png": "unsupported"
        },
        "source_context": {
            "system_context_authored": crate::artifacts::markdown::architecture_view_authored(
                "system-context.md",
                context_summary,
            ),
            "container_view_authored": crate::artifacts::markdown::architecture_view_authored(
                "container-view.md",
                context_summary,
            ),
            "deployment_view_authored": crate::artifacts::markdown::architecture_view_authored(
                "deployment-view.md",
                context_summary,
            ),
        },
    }))
    .map_err(|error| {
        EngineError::Validation(format!(
            "architecture packet metadata serialization failed: {error}"
        ))
    })
}

fn architecture_packet_body_artifacts(selected_artifact_names: &[String]) -> Vec<String> {
    selected_artifact_names
        .iter()
        .filter(|file_name| {
            !matches!(artifact_slug(file_name), "view-manifest.json" | "packet-metadata.json")
        })
        .cloned()
        .collect()
}

fn selected_architecture_artifact_name<'a>(
    selected_artifact_names: &'a [String],
    bare_name: &str,
) -> Option<&'a str> {
    selected_artifact_names
        .iter()
        .find(|file_name| artifact_slug(file_name) == bare_name)
        .map(String::as_str)
}

fn architecture_view_heading(file_name: &str) -> &'static str {
    match artifact_slug(file_name) {
        "system-context.md" => "System Context",
        "container-view.md" => "Containers",
        "deployment-view.md" => "Deployment",
        "component-view.md" => "Components",
        "dynamic-view.md" => "Dynamic View",
        _ => "System Context",
    }
}

#[cfg(test)]
mod tests {
    use super::{
        architecture_view_heading, build_architecture_packet_metadata,
        build_architecture_view_manifest,
    };

    #[test]
    fn architecture_view_heading_returns_system_context_for_known_views() {
        assert_eq!(architecture_view_heading("system-context.md"), "System Context");
        assert_eq!(architecture_view_heading("container-view.md"), "Containers");
        assert_eq!(architecture_view_heading("deployment-view.md"), "Deployment");
        assert_eq!(architecture_view_heading("component-view.md"), "Components");
        assert_eq!(architecture_view_heading("dynamic-view.md"), "Dynamic View");
    }

    #[test]
    fn architecture_view_heading_falls_back_to_system_context_for_unknown_file_names() {
        assert_eq!(architecture_view_heading("unknown-view.md"), "System Context");
        assert_eq!(architecture_view_heading(""), "System Context");
        assert_eq!(architecture_view_heading("architecture-decisions.md"), "System Context");
    }

    #[test]
    fn build_architecture_view_manifest_renders_json() {
        let selected_artifact_names = vec![
            "01-architecture-overview.md".to_string(),
            "02-system-context.md".to_string(),
            "03-system-context.mmd".to_string(),
            "view-manifest.json".to_string(),
            "packet-metadata.json".to_string(),
        ];

        let rendered = build_architecture_view_manifest(&selected_artifact_names, "")
            .expect("view manifest should render");
        let value: serde_json::Value = serde_json::from_str(&rendered).expect("view manifest json");

        assert_eq!(value["primary_artifact"], "01-architecture-overview.md");
        assert_eq!(value["views"][0]["view"], "system-context");
        assert_eq!(value["views"][0]["included"], true);
        assert_eq!(value["views"][0]["artifacts"][0], "02-system-context.md");
        assert_eq!(value["views"][1]["required"], true);
        assert_eq!(
            value["views"][1]["reason"],
            "No `## Containers` section was authored; Canon emitted an explicit omission artifact instead."
        );
        assert_eq!(value["views"][3]["required"], false);
        assert_eq!(
            value["views"][3]["reason"],
            "No `## Components` section was authored; this optional view was omitted from the packet."
        );
    }

    #[test]
    fn build_architecture_packet_metadata_renders_json() {
        let selected_artifact_names = vec![
            "01-architecture-overview.md".to_string(),
            "02-system-context.md".to_string(),
            "03-system-context.mmd".to_string(),
            "view-manifest.json".to_string(),
            "packet-metadata.json".to_string(),
        ];

        let rendered = build_architecture_packet_metadata(
            "R-architecture-test",
            &selected_artifact_names,
            "## System Context\n\nBounded architecture context.",
        )
        .expect("packet metadata should render");
        let value: serde_json::Value =
            serde_json::from_str(&rendered).expect("packet metadata json");

        assert_eq!(value["packet_kind"], "architecture-visual-packet");
        assert_eq!(value["run_id"], "R-architecture-test");
        assert_eq!(value["primary_artifact"], "01-architecture-overview.md");
        assert_eq!(value["included_views"][0], "System Context");
        assert_eq!(value["source_context"]["system_context_authored"], true);
        assert_eq!(value["source_context"]["container_view_authored"], false);
        assert_eq!(value["source_context"]["deployment_view_authored"], false);
    }
}
