use std::fs;
use std::io::Error;
use std::path::{Path, PathBuf};

use canon_adapters::{AdapterInvocation, AdapterKind, filesystem::FilesystemAdapter};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::artifacts::manifest::ArtifactManifest;
use crate::domain::approval::ApprovalRecord;
use crate::domain::artifact::{ArtifactContract, ArtifactRecord};
use crate::domain::execution::{EvidenceBundle, PayloadReference};
use crate::domain::gate::GateEvaluation;
use crate::domain::mode::Mode;
use crate::domain::policy::{
    AdapterPolicyMatrix, GatePolicy, InvocationConstraintProfile, PolicySet, PolicySetOverrides,
    RiskPolicyClass, ValidationIndependencePolicy, ZonePolicy,
};
use crate::domain::run::RunContext;
use crate::domain::verification::VerificationRecord;
use crate::persistence::atomic::write_text_file;
use crate::persistence::invocations::{
    PersistedInvocation, attempt_path, decision_path, invocation_dir, list_invocation_ids,
    payload_dir, payload_path, payload_reference, request_path,
};
use crate::persistence::layout::ProjectLayout;
use crate::persistence::manifests::{LinkManifest, RunManifest, RunStateManifest};
use crate::persistence::traces::{TraceEvent, TraceEventKind, parse_trace_events};

const METHOD_FILES: &[(&str, &str)] = &[
    ("requirements.toml", include_str!("../../../../defaults/methods/requirements.toml")),
    ("discovery.toml", include_str!("../../../../defaults/methods/discovery.toml")),
    ("greenfield.toml", include_str!("../../../../defaults/methods/greenfield.toml")),
    ("brownfield-change.toml", include_str!("../../../../defaults/methods/brownfield-change.toml")),
    ("architecture.toml", include_str!("../../../../defaults/methods/architecture.toml")),
    ("implementation.toml", include_str!("../../../../defaults/methods/implementation.toml")),
    ("refactor.toml", include_str!("../../../../defaults/methods/refactor.toml")),
    ("verification.toml", include_str!("../../../../defaults/methods/verification.toml")),
    ("review.toml", include_str!("../../../../defaults/methods/review.toml")),
    ("pr-review.toml", include_str!("../../../../defaults/methods/pr-review.toml")),
    ("incident.toml", include_str!("../../../../defaults/methods/incident.toml")),
    ("migration.toml", include_str!("../../../../defaults/methods/migration.toml")),
];

const POLICY_FILES: &[(&str, &str)] = &[
    ("risk.toml", include_str!("../../../../defaults/policies/risk.toml")),
    ("zones.toml", include_str!("../../../../defaults/policies/zones.toml")),
    ("gates.toml", include_str!("../../../../defaults/policies/gates.toml")),
    ("verification.toml", include_str!("../../../../defaults/policies/verification.toml")),
    ("adapters.toml", include_str!("../../../../defaults/policies/adapters.toml")),
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InitSummary {
    pub repo_root: String,
    pub canon_root: String,
    pub methods_materialized: usize,
    pub policies_materialized: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersistedArtifact {
    pub record: ArtifactRecord,
    pub contents: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersistedRunBundle {
    pub run: RunManifest,
    pub context: RunContext,
    pub state: RunStateManifest,
    pub artifact_contract: ArtifactContract,
    pub artifacts: Vec<PersistedArtifact>,
    pub links: LinkManifest,
    pub gates: Vec<GateEvaluation>,
    pub approvals: Vec<ApprovalRecord>,
    pub verification_records: Vec<VerificationRecord>,
    pub evidence: Option<EvidenceBundle>,
    pub invocations: Vec<crate::persistence::invocations::PersistedInvocation>,
}

#[derive(Debug, Clone)]
pub struct WorkspaceStore {
    pub layout: ProjectLayout,
    filesystem: FilesystemAdapter,
}

impl WorkspaceStore {
    pub fn new(repo_root: impl AsRef<Path>) -> Self {
        Self { layout: ProjectLayout::new(repo_root), filesystem: FilesystemAdapter }
    }

    pub fn init_runtime_state(&self) -> Result<InitSummary, Error> {
        self.ensure_layout()?;
        let methods_materialized =
            self.materialize_defaults(self.layout.methods_dir(), METHOD_FILES)?;
        let policies_materialized =
            self.materialize_defaults(self.layout.policies_dir(), POLICY_FILES)?;

        Ok(InitSummary {
            repo_root: self.layout.repo_root.display().to_string(),
            canon_root: self.layout.canon_root.display().to_string(),
            methods_materialized,
            policies_materialized,
        })
    }

    pub fn list_method_files(&self) -> Result<Vec<String>, Error> {
        list_file_names(self.layout.methods_dir())
    }

    pub fn list_policy_files(&self) -> Result<Vec<String>, Error> {
        list_file_names(self.layout.policies_dir())
    }

    pub fn load_policy_set(&self, override_root: Option<&Path>) -> Result<PolicySet, Error> {
        let risk_file: RiskPolicyFile =
            read_toml_file(self.layout.policies_dir().join("risk.toml"))?;
        let zone_file: ZonePolicyFile =
            read_toml_file(self.layout.policies_dir().join("zones.toml"))?;
        let gate_file: GatePolicyFile =
            read_toml_file(self.layout.policies_dir().join("gates.toml"))?;
        let verification_file: VerificationPolicyFile =
            read_toml_file(self.layout.policies_dir().join("verification.toml"))?;
        let _ = risk_file.version;
        let _ = zone_file.version;
        let _ = gate_file.version;
        let adapter_file: AdapterPolicyFile =
            read_toml_file(self.layout.policies_dir().join("adapters.toml"))?;
        let _ = adapter_file.version;
        let _ = adapter_file.adapter.len();
        let _ = verification_file.version;
        let _ = verification_file.layers.low.len();
        let _ = verification_file.layers.bounded.len();
        let _ = verification_file.layers.systemic.len();

        let mut policy_set = PolicySet {
            risk_classes: risk_file.class,
            zones: zone_file.zone,
            gate_policy: GatePolicy { mandatory_gates: gate_file.mandatory_gates },
            adapter_matrix: adapter_file.adapter,
            constraint_profiles: adapter_file.constraint_profile,
            runtime_disabled_adapters: adapter_file.rules.runtime_disabled_adapters,
            validation_independence: verification_file.independence,
            block_mutation_for_red_or_systemic: adapter_file
                .rules
                .block_mutation_for_red_or_systemic,
        };

        if let Some(override_root) = override_root {
            let overrides = self.load_policy_overrides(override_root)?;
            policy_set.apply_overrides(overrides);
        }

        Ok(policy_set)
    }

    pub fn persist_run_bundle(&self, bundle: &PersistedRunBundle) -> Result<(), Error> {
        self.ensure_layout()?;

        let run_dir = self.layout.run_dir(&bundle.run.run_id);
        let gates_dir = self.layout.run_gates_dir(&bundle.run.run_id);
        let approvals_dir = self.layout.run_approvals_dir(&bundle.run.run_id);
        let verification_dir = self.layout.run_verification_dir(&bundle.run.run_id);
        let invocations_dir = self.layout.run_invocations_dir(&bundle.run.run_id);
        let artifact_dir = self.layout.run_artifact_dir(&bundle.run.run_id, bundle.run.mode);
        let mut trace_invocations = Vec::new();
        let mut trace_events = Vec::new();

        for directory in [
            run_dir.clone(),
            gates_dir.clone(),
            approvals_dir,
            verification_dir.clone(),
            invocations_dir.clone(),
            artifact_dir.clone(),
        ] {
            trace_invocations.push(
                self.filesystem
                    .create_dir_all_traced(
                        &directory,
                        &format!("materialize {}", directory.display()),
                    )
                    .map_err(adapter_error_to_io)?,
            );
        }

        let trace_relative_path = format!("traces/{}.jsonl", bundle.run.run_id);
        let mut links = bundle.links.clone();
        if !links.traces.iter().any(|entry| entry == &trace_relative_path) {
            links.traces.push(trace_relative_path.clone());
        }
        if bundle.evidence.is_some() {
            links.evidence = Some(format!("runs/{}/evidence.toml", bundle.run.run_id));
        }
        links.invocations = bundle
            .invocations
            .iter()
            .map(|invocation| {
                format!(
                    "runs/{}/invocations/{}/request.toml",
                    bundle.run.run_id, invocation.request.request_id
                )
            })
            .collect();

        let run_toml = run_dir.join("run.toml");
        write_toml_file(run_toml.clone(), &bundle.run)?;
        trace_invocations.push(self.filesystem.trace_write(&run_toml, "persist run manifest"));
        let context_toml = run_dir.join("context.toml");
        write_toml_file(context_toml.clone(), &bundle.context)?;
        trace_invocations.push(self.filesystem.trace_write(&context_toml, "persist run context"));
        let contract_toml = run_dir.join("artifact-contract.toml");
        write_toml_file(contract_toml.clone(), &bundle.artifact_contract)?;
        trace_invocations
            .push(self.filesystem.trace_write(&contract_toml, "persist artifact contract"));
        let state_toml = run_dir.join("state.toml");
        write_toml_file(state_toml.clone(), &bundle.state)?;
        trace_invocations.push(self.filesystem.trace_write(&state_toml, "persist run state"));
        let links_toml = run_dir.join("links.toml");
        write_toml_file(links_toml.clone(), &links)?;
        trace_invocations.push(self.filesystem.trace_write(&links_toml, "persist run links"));

        if let Some(evidence) = &bundle.evidence {
            let evidence_path = self.layout.run_evidence_path(&bundle.run.run_id);
            write_toml_file(evidence_path.clone(), evidence)?;
            trace_invocations
                .push(self.filesystem.trace_write(&evidence_path, "persist evidence bundle"));
            trace_events.push(TraceEvent {
                run_id: bundle.run.run_id.clone(),
                request_id: None,
                adapter: None,
                capability: None,
                event: TraceEventKind::EvidenceBundleUpdated,
                summary: "persisted run-level evidence bundle".to_string(),
                policy_decision: None,
                outcome: None,
                recorded_at: OffsetDateTime::now_utc(),
            });
        }

        let manifest = ArtifactManifest {
            records: bundle.artifacts.iter().map(|artifact| artifact.record.clone()).collect(),
        };
        let artifact_manifest_toml = artifact_dir.join("manifest.toml");
        write_toml_file(artifact_manifest_toml.clone(), &manifest)?;
        trace_invocations.push(
            self.filesystem.trace_write(&artifact_manifest_toml, "persist artifact manifest"),
        );

        for artifact in &bundle.artifacts {
            let path = self.layout.canon_root.join(&artifact.record.relative_path);
            write_text_file(&path, &artifact.contents)?;
            trace_invocations.push(
                self.filesystem
                    .trace_write(&path, &format!("write artifact {}", artifact.record.file_name)),
            );
        }

        for gate in &bundle.gates {
            let gate_path = gates_dir.join(format!("{}.toml", gate.gate.as_str()));
            write_toml_file(gate_path, gate)?;
            trace_invocations.push(self.filesystem.trace_write(
                &gates_dir.join(format!("{}.toml", gate.gate.as_str())),
                "persist gate evaluation",
            ));
        }

        for (index, approval) in bundle.approvals.iter().enumerate() {
            let path = run_dir.join("approvals").join(format!("approval-{index:02}.toml"));
            write_toml_file(path, approval)?;
            trace_invocations.push(self.filesystem.trace_write(
                &run_dir.join("approvals").join(format!("approval-{index:02}.toml")),
                "persist approval record",
            ));
        }

        for (index, record) in bundle.verification_records.iter().enumerate() {
            let path = verification_dir.join(format!("verification-{index:02}.toml"));
            write_toml_file(path, record)?;
            trace_invocations.push(self.filesystem.trace_write(
                &verification_dir.join(format!("verification-{index:02}.toml")),
                "persist verification record",
            ));
        }

        for invocation in &bundle.invocations {
            let invocation_root = invocation_dir(&run_dir, &invocation.request.request_id);
            let request_toml = request_path(&run_dir, &invocation.request.request_id);
            let decision_toml = decision_path(&run_dir, &invocation.request.request_id);
            let payload_root = payload_dir(&run_dir, &invocation.request.request_id);
            self.filesystem.create_dir_all(&invocation_root).map_err(adapter_error_to_io)?;
            self.filesystem.create_dir_all(&payload_root).map_err(adapter_error_to_io)?;
            write_toml_file(request_toml.clone(), &invocation.request)?;
            write_toml_file(decision_toml.clone(), &invocation.decision)?;
            trace_events.push(TraceEvent {
                run_id: bundle.run.run_id.clone(),
                request_id: Some(invocation.request.request_id.clone()),
                adapter: Some(invocation.request.adapter),
                capability: Some(invocation.request.capability),
                event: TraceEventKind::RequestPersisted,
                summary: invocation.request.summary.clone(),
                policy_decision: None,
                outcome: None,
                recorded_at: OffsetDateTime::now_utc(),
            });
            trace_events.push(TraceEvent {
                run_id: bundle.run.run_id.clone(),
                request_id: Some(invocation.request.request_id.clone()),
                adapter: Some(invocation.request.adapter),
                capability: Some(invocation.request.capability),
                event: if invocation.decision.requires_approval {
                    TraceEventKind::ApprovalRequired
                } else {
                    TraceEventKind::DecisionPersisted
                },
                summary: invocation.decision.rationale.clone(),
                policy_decision: Some(invocation.decision.kind),
                outcome: None,
                recorded_at: invocation.decision.decided_at,
            });

            for attempt in &invocation.attempts {
                let attempt_toml =
                    attempt_path(&run_dir, &invocation.request.request_id, attempt.attempt_number);
                write_toml_file(attempt_toml, attempt)?;
                trace_events.push(TraceEvent {
                    run_id: bundle.run.run_id.clone(),
                    request_id: Some(invocation.request.request_id.clone()),
                    adapter: Some(invocation.request.adapter),
                    capability: Some(invocation.request.capability),
                    event: TraceEventKind::OutcomeRecorded,
                    summary: attempt.outcome.summary.clone(),
                    policy_decision: Some(invocation.decision.kind),
                    outcome: Some(attempt.outcome.kind),
                    recorded_at: attempt.finished_at,
                });
            }
        }

        self.append_trace_stream(&bundle.run.run_id, &trace_invocations)?;
        self.append_trace_events(&bundle.run.run_id, &trace_events)?;

        Ok(())
    }

    pub fn list_artifact_files(&self, run_id: &str) -> Result<Vec<String>, Error> {
        let run_artifacts_root = self.layout.artifacts_dir().join(run_id);
        if !run_artifacts_root.exists() {
            return Ok(Vec::new());
        }

        let mut entries = Vec::new();
        for mode_dir in fs::read_dir(run_artifacts_root)? {
            let mode_dir = mode_dir?;
            let manifest_path = mode_dir.path().join("manifest.toml");
            if !manifest_path.exists() {
                continue;
            }

            let manifest: ArtifactManifest = read_toml_file(manifest_path)?;
            entries.extend(manifest.records.into_iter().map(|record| record.file_name));
        }

        entries.sort();
        Ok(entries)
    }

    pub fn list_invocation_ids(&self, run_id: &str) -> Result<Vec<String>, Error> {
        list_invocation_ids(&self.layout.run_dir(run_id))
    }

    pub fn list_evidence_entries(&self, run_id: &str) -> Result<Vec<String>, Error> {
        let evidence_path = self.layout.run_dir(run_id).join("evidence.toml");
        if evidence_path.exists() { Ok(vec!["evidence.toml".to_string()]) } else { Ok(Vec::new()) }
    }

    pub fn load_evidence_bundle(&self, run_id: &str) -> Result<Option<EvidenceBundle>, Error> {
        let path = self.layout.run_evidence_path(run_id);
        if path.exists() { read_toml_file(path).map(Some) } else { Ok(None) }
    }

    pub fn load_persisted_invocations(
        &self,
        run_id: &str,
    ) -> Result<Vec<PersistedInvocation>, Error> {
        let run_dir = self.layout.run_dir(run_id);
        let mut invocations = Vec::new();

        for request_id in list_invocation_ids(&run_dir)? {
            let request = read_toml_file(request_path(&run_dir, &request_id))?;
            let decision = read_toml_file(decision_path(&run_dir, &request_id))?;
            let invocation_root = invocation_dir(&run_dir, &request_id);
            let mut attempts = fs::read_dir(&invocation_root)?
                .filter_map(|entry| match entry {
                    Ok(entry) if entry.file_name().to_string_lossy().starts_with("attempt-") => {
                        Some(read_toml_file(entry.path()))
                    }
                    Ok(_) => None,
                    Err(error) => Some(Err(error)),
                })
                .collect::<Result<Vec<_>, _>>()?;
            attempts.sort_by_key(|attempt: &crate::domain::execution::InvocationAttempt| {
                attempt.attempt_number
            });

            let approvals = self
                .load_approval_records(run_id)?
                .into_iter()
                .filter(|approval| approval.matches_invocation(&request_id))
                .collect();

            invocations.push(PersistedInvocation { request, decision, attempts, approvals });
        }

        invocations.sort_by(|left, right| left.request.request_id.cmp(&right.request.request_id));
        Ok(invocations)
    }

    pub fn load_trace_events(&self, run_id: &str) -> Result<Vec<TraceEvent>, Error> {
        let trace_path = self.layout.traces_dir().join(format!("{run_id}.jsonl"));
        if !trace_path.exists() {
            return Ok(Vec::new());
        }

        let contents = fs::read_to_string(trace_path)?;
        parse_trace_events(&contents).map_err(|error| Error::other(error.to_string()))
    }

    pub fn load_run_state(&self, run_id: &str) -> Result<RunStateManifest, Error> {
        read_toml_file(self.layout.run_dir(run_id).join("state.toml"))
    }

    pub fn load_run_manifest(&self, run_id: &str) -> Result<RunManifest, Error> {
        read_toml_file(self.layout.run_dir(run_id).join("run.toml"))
    }

    pub fn load_run_context(&self, run_id: &str) -> Result<RunContext, Error> {
        read_toml_file(self.layout.run_dir(run_id).join("context.toml"))
    }

    pub fn load_artifact_contract(&self, run_id: &str) -> Result<ArtifactContract, Error> {
        read_toml_file(self.layout.run_dir(run_id).join("artifact-contract.toml"))
    }

    pub fn load_gate_evaluations(&self, run_id: &str) -> Result<Vec<GateEvaluation>, Error> {
        let mut gates = fs::read_dir(self.layout.run_gates_dir(run_id))?
            .map(|entry| {
                let entry = entry?;
                read_toml_file(entry.path())
            })
            .collect::<Result<Vec<_>, _>>()?;
        gates.sort_by_key(|gate: &GateEvaluation| gate.gate.as_str().to_string());
        Ok(gates)
    }

    pub fn load_approval_records(&self, run_id: &str) -> Result<Vec<ApprovalRecord>, Error> {
        let approvals_dir = self.layout.run_approvals_dir(run_id);
        if !approvals_dir.exists() {
            return Ok(Vec::new());
        }

        let mut approvals = fs::read_dir(approvals_dir)?
            .map(|entry| {
                let entry = entry?;
                read_toml_file(entry.path())
            })
            .collect::<Result<Vec<_>, _>>()?;
        approvals.sort_by_key(|approval: &ApprovalRecord| approval.recorded_at);
        Ok(approvals)
    }

    pub fn load_persisted_artifacts(
        &self,
        run_id: &str,
        mode: Mode,
        contract: &ArtifactContract,
    ) -> Result<Vec<PersistedArtifact>, Error> {
        let artifact_root = self.layout.run_artifact_dir(run_id, mode);
        let manifest: ArtifactManifest = read_toml_file(artifact_root.join("manifest.toml"))?;

        contract
            .artifact_requirements
            .iter()
            .map(|requirement| {
                let record = manifest
                    .records
                    .iter()
                    .find(|record| record.file_name == requirement.file_name)
                    .cloned()
                    .ok_or_else(|| {
                        Error::other(format!(
                            "artifact `{}` missing from persisted manifest",
                            requirement.file_name
                        ))
                    })?;
                let path = self.layout.canon_root.join(&record.relative_path);
                let contents = fs::read_to_string(path)?;
                Ok(PersistedArtifact { record, contents })
            })
            .collect()
    }

    pub fn persist_gate_evaluations(
        &self,
        run_id: &str,
        gates: &[GateEvaluation],
    ) -> Result<(), Error> {
        let gates_dir = self.layout.run_gates_dir(run_id);
        self.filesystem.create_dir_all(&gates_dir).map_err(adapter_error_to_io)?;
        let mut invocations = Vec::new();

        for gate in gates {
            let gate_path = gates_dir.join(format!("{}.toml", gate.gate.as_str()));
            write_toml_file(gate_path.clone(), gate)?;
            invocations.push(self.filesystem.trace_write(&gate_path, "persist gate evaluation"));
        }

        self.append_trace_stream(run_id, &invocations)?;

        Ok(())
    }

    pub fn persist_run_state(&self, run_id: &str, state: &RunStateManifest) -> Result<(), Error> {
        let path = self.layout.run_dir(run_id).join("state.toml");
        write_toml_file(path.clone(), state)?;
        self.append_trace_stream(run_id, &[self.filesystem.trace_write(&path, "persist run state")])
    }

    pub fn persist_approval_record(
        &self,
        run_id: &str,
        approval: &ApprovalRecord,
    ) -> Result<(), Error> {
        let approvals_dir = self.layout.run_approvals_dir(run_id);
        self.filesystem.create_dir_all(&approvals_dir).map_err(adapter_error_to_io)?;
        let next_index = fs::read_dir(&approvals_dir)?.count();
        let path = approvals_dir.join(format!("approval-{next_index:02}.toml"));
        write_toml_file(path.clone(), approval)?;
        self.append_trace_stream(
            run_id,
            &[self.filesystem.trace_write(&path, "persist approval record")],
        )
    }

    pub fn persist_adapter_invocations(
        &self,
        run_id: &str,
        invocations: &[AdapterInvocation],
    ) -> Result<(), Error> {
        self.append_trace_stream(run_id, invocations)
    }

    pub fn persist_invocation_payload_text(
        &self,
        run_id: &str,
        request_id: &str,
        file_name: &str,
        contents: &str,
    ) -> Result<PayloadReference, Error> {
        self.ensure_layout()?;
        let run_dir = self.layout.run_dir(run_id);
        let payload_root = payload_dir(&run_dir, request_id);
        self.filesystem.create_dir_all(&payload_root).map_err(adapter_error_to_io)?;

        let path = payload_path(&run_dir, request_id, file_name);
        write_text_file(&path, contents)?;
        self.append_trace_stream(
            run_id,
            &[self.filesystem.trace_write(&path, "persist invocation payload")],
        )?;

        Ok(payload_reference(run_id, request_id, file_name))
    }

    fn ensure_layout(&self) -> Result<(), Error> {
        for directory in [
            self.layout.canon_root.clone(),
            self.layout.sessions_dir(),
            self.layout.artifacts_dir(),
            self.layout.decisions_dir(),
            self.layout.traces_dir(),
            self.layout.methods_dir(),
            self.layout.policies_dir(),
            self.layout.runs_dir(),
        ] {
            self.filesystem.create_dir_all(&directory).map_err(adapter_error_to_io)?;
        }

        Ok(())
    }

    fn materialize_defaults(
        &self,
        directory: PathBuf,
        files: &[(&str, &str)],
    ) -> Result<usize, Error> {
        let mut written = 0;

        for (name, contents) in files {
            let path = directory.join(name);
            if !path.exists() {
                write_text_file(&path, contents)?;
                written += 1;
            }
        }

        Ok(written)
    }

    fn load_policy_overrides(&self, override_root: &Path) -> Result<PolicySetOverrides, Error> {
        let risk_overrides = if override_root.join("risk.toml").exists() {
            let risk_file = read_toml_file::<RiskPolicyFile>(override_root.join("risk.toml"))?;
            let _ = risk_file.version;
            risk_file.class
        } else {
            Vec::new()
        };
        let zone_overrides = if override_root.join("zones.toml").exists() {
            let zone_file = read_toml_file::<ZonePolicyFile>(override_root.join("zones.toml"))?;
            let _ = zone_file.version;
            zone_file.zone
        } else {
            Vec::new()
        };
        let gate_override = if override_root.join("gates.toml").exists() {
            let gate_file = read_toml_file::<GatePolicyFile>(override_root.join("gates.toml"))?;
            let _ = gate_file.version;
            Some(GatePolicy { mandatory_gates: gate_file.mandatory_gates })
        } else {
            None
        };
        let adapter_override = if override_root.join("adapters.toml").exists() {
            {
                let adapter_policy =
                    read_toml_file::<AdapterPolicyFile>(override_root.join("adapters.toml"))?;
                let _ = adapter_policy.version;
                Some((
                    adapter_policy.adapter,
                    adapter_policy.constraint_profile,
                    adapter_policy.rules.runtime_disabled_adapters,
                    adapter_policy.rules.block_mutation_for_red_or_systemic,
                ))
            }
        } else {
            None
        };
        let verification_override = if override_root.join("verification.toml").exists() {
            let verification_file =
                read_toml_file::<VerificationPolicyFile>(override_root.join("verification.toml"))?;
            let _ = verification_file.version;
            Some(verification_file.independence)
        } else {
            None
        };

        let adapter_matrix_overrides = adapter_override
            .as_ref()
            .map(|override_tuple| override_tuple.0.clone())
            .unwrap_or_default();
        let constraint_profile_overrides = adapter_override
            .as_ref()
            .map(|override_tuple| override_tuple.1.clone())
            .unwrap_or_default();
        let runtime_disabled_adapters =
            adapter_override.as_ref().map(|override_tuple| override_tuple.2.clone());
        let block_mutation_override =
            adapter_override.as_ref().map(|override_tuple| override_tuple.3);

        Ok(PolicySetOverrides {
            risk_classes: risk_overrides,
            zones: zone_overrides,
            gate_policy: gate_override,
            adapter_matrix: adapter_matrix_overrides,
            constraint_profiles: constraint_profile_overrides,
            runtime_disabled_adapters,
            validation_independence: verification_override,
            block_mutation_for_red_or_systemic: block_mutation_override,
        })
    }

    fn append_trace_stream(
        &self,
        run_id: &str,
        invocations: &[AdapterInvocation],
    ) -> Result<(), Error> {
        let events = invocations
            .iter()
            .map(|invocation| TraceEvent::from_adapter_invocation(run_id, invocation))
            .collect::<Vec<_>>();
        self.append_trace_events(run_id, &events)
    }

    fn append_trace_events(&self, run_id: &str, events: &[TraceEvent]) -> Result<(), Error> {
        if events.is_empty() {
            return Ok(());
        }

        let trace_path = self.layout.traces_dir().join(format!("{run_id}.jsonl"));
        let mut buffer = String::new();
        for event in events {
            let line =
                serde_json::to_string(event).map_err(|error| Error::other(error.to_string()))?;
            buffer.push_str(&line);
            buffer.push('\n');
        }

        use std::io::Write;
        let mut file = fs::OpenOptions::new().create(true).append(true).open(trace_path)?;
        file.write_all(buffer.as_bytes())?;
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RiskPolicyFile {
    version: u32,
    class: Vec<RiskPolicyClass>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ZonePolicyFile {
    version: u32,
    zone: Vec<ZonePolicy>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct GatePolicyFile {
    version: u32,
    mandatory_gates: Vec<crate::domain::gate::GateKind>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AdapterPolicyFile {
    version: u32,
    adapter: Vec<AdapterPolicyMatrix>,
    #[serde(default)]
    constraint_profile: Vec<InvocationConstraintProfile>,
    rules: AdapterRuleFile,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AdapterRuleFile {
    block_mutation_for_red_or_systemic: bool,
    #[serde(default)]
    runtime_disabled_adapters: Vec<AdapterKind>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
#[serde(deny_unknown_fields)]
struct VerificationPolicyFile {
    version: u32,
    layers: VerificationLayerMatrix,
    independence: ValidationIndependencePolicy,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
#[serde(deny_unknown_fields)]
struct VerificationLayerMatrix {
    low: Vec<crate::domain::verification::VerificationLayer>,
    bounded: Vec<crate::domain::verification::VerificationLayer>,
    systemic: Vec<crate::domain::verification::VerificationLayer>,
}

fn adapter_error_to_io(error: canon_adapters::AdapterError) -> Error {
    match error {
        canon_adapters::AdapterError::Filesystem(inner) => inner,
        canon_adapters::AdapterError::Process(inner) => Error::other(inner.to_string()),
        canon_adapters::AdapterError::MutationBlocked => {
            Error::other("filesystem adapter unexpectedly blocked")
        }
    }
}

fn list_file_names(directory: PathBuf) -> Result<Vec<String>, Error> {
    if !directory.exists() {
        return Ok(Vec::new());
    }

    let mut files = fs::read_dir(directory)?
        .map(|entry| entry.map(|entry| entry.file_name().to_string_lossy().to_string()))
        .collect::<Result<Vec<_>, _>>()?;
    files.sort();
    Ok(files)
}

fn read_toml_file<T>(path: PathBuf) -> Result<T, Error>
where
    T: for<'de> Deserialize<'de>,
{
    let contents = fs::read_to_string(path)?;
    toml::from_str(&contents).map_err(|error| Error::other(error.to_string()))
}

fn write_toml_file<T>(path: PathBuf, value: &T) -> Result<(), Error>
where
    T: Serialize,
{
    let contents =
        toml::to_string_pretty(value).map_err(|error| Error::other(error.to_string()))?;
    write_text_file(&path, &contents)
}
