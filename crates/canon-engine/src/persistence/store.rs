use std::fs;
use std::io::{Error, ErrorKind};
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
use crate::domain::run::{InputSourceKind, RunContext};
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
    ("system-shaping.toml", include_str!("../../../../defaults/methods/system-shaping.toml")),
    ("change.toml", include_str!("../../../../defaults/methods/change.toml")),
    ("backlog.toml", include_str!("../../../../defaults/methods/backlog.toml")),
    ("architecture.toml", include_str!("../../../../defaults/methods/architecture.toml")),
    ("implementation.toml", include_str!("../../../../defaults/methods/implementation.toml")),
    ("refactor.toml", include_str!("../../../../defaults/methods/refactor.toml")),
    ("verification.toml", include_str!("../../../../defaults/methods/verification.toml")),
    ("review.toml", include_str!("../../../../defaults/methods/review.toml")),
    ("pr-review.toml", include_str!("../../../../defaults/methods/pr-review.toml")),
    ("incident.toml", include_str!("../../../../defaults/methods/incident.toml")),
    (
        "security-assessment.toml",
        include_str!("../../../../defaults/methods/security-assessment.toml"),
    ),
    ("system-assessment.toml", include_str!("../../../../defaults/methods/system-assessment.toml")),
    ("migration.toml", include_str!("../../../../defaults/methods/migration.toml")),
    (
        "supply-chain-analysis.toml",
        include_str!("../../../../defaults/methods/supply-chain-analysis.toml"),
    ),
];

const POLICY_FILES: &[(&str, &str)] = &[
    ("risk.toml", include_str!("../../../../defaults/policies/risk.toml")),
    ("zones.toml", include_str!("../../../../defaults/policies/zones.toml")),
    ("gates.toml", include_str!("../../../../defaults/policies/gates.toml")),
    ("verification.toml", include_str!("../../../../defaults/policies/verification.toml")),
    ("adapters.toml", include_str!("../../../../defaults/policies/adapters.toml")),
];

/// Skill SKILL.md files embedded at compile time for materialization into
/// `.agents/skills/<name>/SKILL.md` during `canon init` or `canon skills install`.
const SKILL_FILES: &[(&str, &str)] = &[
    (
        "canon-init/SKILL.md",
        include_str!("../../../../defaults/embedded-skills/canon-init/skill-source.md"),
    ),
    (
        "canon-requirements/SKILL.md",
        include_str!("../../../../defaults/embedded-skills/canon-requirements/skill-source.md"),
    ),
    (
        "canon-change/SKILL.md",
        include_str!("../../../../defaults/embedded-skills/canon-change/skill-source.md"),
    ),
    (
        "canon-backlog/SKILL.md",
        include_str!("../../../../defaults/embedded-skills/canon-backlog/skill-source.md"),
    ),
    (
        "canon-pr-review/SKILL.md",
        include_str!("../../../../defaults/embedded-skills/canon-pr-review/skill-source.md"),
    ),
    (
        "canon-status/SKILL.md",
        include_str!("../../../../defaults/embedded-skills/canon-status/skill-source.md"),
    ),
    (
        "canon-inspect-invocations/SKILL.md",
        include_str!(
            "../../../../defaults/embedded-skills/canon-inspect-invocations/skill-source.md"
        ),
    ),
    (
        "canon-inspect-evidence/SKILL.md",
        include_str!("../../../../defaults/embedded-skills/canon-inspect-evidence/skill-source.md"),
    ),
    (
        "canon-inspect-artifacts/SKILL.md",
        include_str!(
            "../../../../defaults/embedded-skills/canon-inspect-artifacts/skill-source.md"
        ),
    ),
    (
        "canon-inspect-clarity/SKILL.md",
        include_str!("../../../../defaults/embedded-skills/canon-inspect-clarity/skill-source.md"),
    ),
    (
        "canon-approve/SKILL.md",
        include_str!("../../../../defaults/embedded-skills/canon-approve/skill-source.md"),
    ),
    (
        "canon-resume/SKILL.md",
        include_str!("../../../../defaults/embedded-skills/canon-resume/skill-source.md"),
    ),
    (
        "canon-discovery/SKILL.md",
        include_str!("../../../../defaults/embedded-skills/canon-discovery/skill-source.md"),
    ),
    (
        "canon-system-shaping/SKILL.md",
        include_str!("../../../../defaults/embedded-skills/canon-system-shaping/skill-source.md"),
    ),
    (
        "canon-architecture/SKILL.md",
        include_str!("../../../../defaults/embedded-skills/canon-architecture/skill-source.md"),
    ),
    (
        "canon-implementation/SKILL.md",
        include_str!("../../../../defaults/embedded-skills/canon-implementation/skill-source.md"),
    ),
    (
        "canon-refactor/SKILL.md",
        include_str!("../../../../defaults/embedded-skills/canon-refactor/skill-source.md"),
    ),
    (
        "canon-review/SKILL.md",
        include_str!("../../../../defaults/embedded-skills/canon-review/skill-source.md"),
    ),
    (
        "canon-incident/SKILL.md",
        include_str!("../../../../defaults/embedded-skills/canon-incident/skill-source.md"),
    ),
    (
        "canon-security-assessment/SKILL.md",
        include_str!(
            "../../../../defaults/embedded-skills/canon-security-assessment/skill-source.md"
        ),
    ),
    (
        "canon-system-assessment/SKILL.md",
        include_str!(
            "../../../../defaults/embedded-skills/canon-system-assessment/skill-source.md"
        ),
    ),
    (
        "canon-migration/SKILL.md",
        include_str!("../../../../defaults/embedded-skills/canon-migration/skill-source.md"),
    ),
    (
        "canon-supply-chain-analysis/SKILL.md",
        include_str!(
            "../../../../defaults/embedded-skills/canon-supply-chain-analysis/skill-source.md"
        ),
    ),
    (
        "canon-verification/SKILL.md",
        include_str!("../../../../defaults/embedded-skills/canon-verification/skill-source.md"),
    ),
];

/// Shared scripts and references embedded at compile time.
const SHARED_SKILL_FILES: &[(&str, &str)] = &[
    (
        "canon-shared/scripts/check-runtime.sh",
        include_str!("../../../../defaults/embedded-skills/canon-shared/scripts/check-runtime.sh"),
    ),
    (
        "canon-shared/scripts/check-runtime.ps1",
        include_str!("../../../../defaults/embedded-skills/canon-shared/scripts/check-runtime.ps1"),
    ),
    (
        "canon-shared/scripts/render-next-steps.sh",
        include_str!(
            "../../../../defaults/embedded-skills/canon-shared/scripts/render-next-steps.sh"
        ),
    ),
    (
        "canon-shared/scripts/render-next-steps.ps1",
        include_str!(
            "../../../../defaults/embedded-skills/canon-shared/scripts/render-next-steps.ps1"
        ),
    ),
    (
        "canon-shared/scripts/render-support-state.sh",
        include_str!(
            "../../../../defaults/embedded-skills/canon-shared/scripts/render-support-state.sh"
        ),
    ),
    (
        "canon-shared/scripts/render-support-state.ps1",
        include_str!(
            "../../../../defaults/embedded-skills/canon-shared/scripts/render-support-state.ps1"
        ),
    ),
    (
        "canon-shared/references/runtime-compatibility.toml",
        include_str!(
            "../../../../defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml"
        ),
    ),
    (
        "canon-shared/references/skill-index.md",
        include_str!("../../../../defaults/embedded-skills/canon-shared/references/skill-index.md"),
    ),
    (
        "canon-shared/references/skill-template.md",
        include_str!(
            "../../../../defaults/embedded-skills/canon-shared/references/skill-template.md"
        ),
    ),
    (
        "canon-shared/references/output-shapes.md",
        include_str!(
            "../../../../defaults/embedded-skills/canon-shared/references/output-shapes.md"
        ),
    ),
    (
        "canon-shared/references/support-states.md",
        include_str!(
            "../../../../defaults/embedded-skills/canon-shared/references/support-states.md"
        ),
    ),
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InitSummary {
    pub repo_root: String,
    pub canon_root: String,
    pub methods_materialized: usize,
    pub policies_materialized: usize,
    pub skills_materialized: usize,
    pub claude_md_created: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkillMaterializationTarget {
    Agents,
    Claude,
}

impl SkillMaterializationTarget {
    fn skills_dir(self, layout: &ProjectLayout) -> PathBuf {
        match self {
            Self::Agents => layout.skills_dir(),
            Self::Claude => layout.claude_skills_dir(),
        }
    }

    fn creates_claude_md(self) -> bool {
        matches!(self, Self::Claude)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SkillsSummary {
    pub skills_dir: String,
    pub skills_materialized: usize,
    pub skills_skipped: usize,
    pub claude_md_created: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SkillEntry {
    pub name: String,
    pub support_state: String,
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

    pub fn init_runtime_state(
        &self,
        skill_target: Option<SkillMaterializationTarget>,
    ) -> Result<InitSummary, Error> {
        self.ensure_layout()?;
        let methods_materialized =
            self.materialize_defaults(self.layout.methods_dir(), METHOD_FILES)?;
        let policies_materialized =
            self.materialize_defaults(self.layout.policies_dir(), POLICY_FILES)?;
        let skills_materialized = skill_target
            .map(|target| self.materialize_skills(target, false))
            .transpose()?
            .unwrap_or(0);
        let claude_md_created = skill_target
            .filter(|target| target.creates_claude_md())
            .map(|_| self.materialize_claude_md())
            .transpose()?
            .unwrap_or(false);

        Ok(InitSummary {
            repo_root: self.layout.repo_root.display().to_string(),
            canon_root: self.layout.canon_root.display().to_string(),
            methods_materialized,
            policies_materialized,
            skills_materialized,
            claude_md_created,
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
        let validated_artifact_paths = bundle
            .artifacts
            .iter()
            .map(|artifact| {
                artifact_storage_path(
                    &self.layout,
                    &artifact.record,
                    &bundle.run.run_id,
                    bundle.run.mode,
                )
            })
            .collect::<Result<Vec<_>, _>>()?;

        self.ensure_layout()?;

        let run_dir = self.layout.run_dir(&bundle.run.run_id);
        let gates_dir = self.layout.run_gates_dir(&bundle.run.run_id);
        let approvals_dir = self.layout.run_approvals_dir(&bundle.run.run_id);
        let verification_dir = self.layout.run_verification_dir(&bundle.run.run_id);
        let invocations_dir = self.layout.run_invocations_dir(&bundle.run.run_id);
        let inputs_dir = self.layout.run_inputs_dir(&bundle.run.run_id);
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

        let persisted_context = self.persist_run_inputs(
            &bundle.run.run_id,
            &inputs_dir,
            &bundle.context,
            &mut trace_invocations,
        )?;

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
        write_toml_file(context_toml.clone(), &persisted_context)?;
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

        for (artifact, path) in bundle.artifacts.iter().zip(validated_artifact_paths.iter()) {
            write_text_file(path, &artifact.contents)?;
            trace_invocations.push(
                self.filesystem
                    .trace_write(path, &format!("write artifact {}", artifact.record.file_name)),
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

    fn persist_run_inputs(
        &self,
        run_id: &str,
        inputs_dir: &Path,
        context: &RunContext,
        trace_invocations: &mut Vec<AdapterInvocation>,
    ) -> Result<RunContext, Error> {
        if context.input_fingerprints.is_empty() {
            return Ok(context.clone());
        }

        trace_invocations.push(
            self.filesystem
                .create_dir_all_traced(inputs_dir, &format!("materialize {}", inputs_dir.display()))
                .map_err(adapter_error_to_io)?,
        );

        let mut persisted_context = context.clone();
        for (index, fingerprint) in persisted_context.input_fingerprints.iter_mut().enumerate() {
            let snapshot_source = PathBuf::from(&fingerprint.path);
            let snapshot_name = snapshot_file_name(index, &snapshot_source);
            let snapshot_path = inputs_dir.join(&snapshot_name);

            match fingerprint.source_kind {
                InputSourceKind::Path => {
                    let source =
                        resolve_context_input_path(&self.layout.repo_root, &fingerprint.path);
                    fs::write(&snapshot_path, fs::read(&source)?)?;
                }
                InputSourceKind::Inline => {
                    let inline_input = persisted_context
                        .inline_inputs
                        .iter()
                        .find(|input| input.label == fingerprint.path)
                        .ok_or_else(|| {
                            Error::new(
                                ErrorKind::InvalidData,
                                format!(
                                    "missing transient inline input contents for `{}` during snapshot persistence",
                                    fingerprint.path
                                ),
                            )
                        })?;
                    fs::write(&snapshot_path, inline_input.contents.as_bytes())?;
                }
            }

            trace_invocations.push(self.filesystem.trace_write(
                &snapshot_path,
                &format!("persist input snapshot {}", fingerprint.path),
            ));
            fingerprint.snapshot_ref = Some(format!("runs/{run_id}/inputs/{snapshot_name}"));
        }

        persisted_context.inline_inputs.clear();

        Ok(persisted_context)
    }

    pub fn list_artifact_files(&self, run_id: &str) -> Result<Vec<String>, Error> {
        let run_artifacts_root = self.layout.artifacts_dir().join(run_id);
        if !run_artifacts_root.exists() {
            return Ok(Vec::new());
        }

        let mut entries = Vec::new();
        for mode_dir in fs::read_dir(run_artifacts_root)? {
            let mode_dir = mode_dir?;
            let mode_name = mode_dir.file_name().to_string_lossy().into_owned();
            let mode = mode_name.parse::<Mode>().map_err(|error| {
                Error::new(
                    ErrorKind::InvalidData,
                    format!(
                        "artifact manifest directory `{mode_name}` is not a supported mode: {error}"
                    ),
                )
            })?;
            let manifest_path = mode_dir.path().join("manifest.toml");
            if !manifest_path.exists() {
                continue;
            }

            let manifest: ArtifactManifest = read_toml_file(manifest_path)?;
            for record in manifest.records {
                validate_run_artifact_record(&record, run_id, mode)?;
                entries.push(format!(".canon/{}", record.relative_path));
            }
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
        let manifest: RunManifest = read_toml_file(self.layout.run_dir(run_id).join("run.toml"))?;
        Ok(manifest.canonicalize())
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
                let path = artifact_storage_path(&self.layout, &record, run_id, mode)?;
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

    /// Materialize skill files into the requested AI-tool skill directory.
    /// When `force` is false, existing files are skipped (idempotent).
    /// When `force` is true, all files are overwritten (update mode).
    /// Returns the number of files written.
    fn materialize_skills(
        &self,
        target: SkillMaterializationTarget,
        force: bool,
    ) -> Result<usize, Error> {
        let skills_dir = target.skills_dir(&self.layout);
        let mut written = 0;

        for (relative_path, contents) in SKILL_FILES.iter().chain(SHARED_SKILL_FILES.iter()) {
            let path = skills_dir.join(relative_path);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            if force || !path.exists() {
                write_text_file(&path, contents)?;
                #[cfg(unix)]
                if relative_path.ends_with(".sh") {
                    use std::os::unix::fs::PermissionsExt;
                    fs::set_permissions(&path, fs::Permissions::from_mode(0o755))?;
                }
                written += 1;
            }
        }

        Ok(written)
    }

    /// Materialize a minimal CLAUDE.md that imports AGENTS.md.
    /// Skips if the file already exists to avoid overwriting user customizations.
    fn materialize_claude_md(&self) -> Result<bool, Error> {
        let path = self.layout.claude_md_path();
        if path.exists() {
            return Ok(false);
        }
        write_text_file(&path, "@AGENTS.md\n")?;
        Ok(true)
    }

    /// Install skills without requiring `.canon/` to exist.
    pub fn install_skills(
        &self,
        target: SkillMaterializationTarget,
    ) -> Result<SkillsSummary, Error> {
        let total = SKILL_FILES.len() + SHARED_SKILL_FILES.len();
        let written = self.materialize_skills(target, false)?;
        let claude_md_created =
            if target.creates_claude_md() { self.materialize_claude_md()? } else { false };

        Ok(SkillsSummary {
            skills_dir: target.skills_dir(&self.layout).display().to_string(),
            skills_materialized: written,
            skills_skipped: total - written,
            claude_md_created,
        })
    }

    /// Force-update all skills, overwriting existing files.
    pub fn update_skills(
        &self,
        target: SkillMaterializationTarget,
    ) -> Result<SkillsSummary, Error> {
        let total = SKILL_FILES.len() + SHARED_SKILL_FILES.len();
        let written = self.materialize_skills(target, true)?;
        let claude_md_created =
            if target.creates_claude_md() { self.materialize_claude_md()? } else { false };

        Ok(SkillsSummary {
            skills_dir: target.skills_dir(&self.layout).display().to_string(),
            skills_materialized: written,
            skills_skipped: total - written,
            claude_md_created,
        })
    }

    /// List all embedded skill names and their support state.
    pub fn list_skills(&self) -> Vec<SkillEntry> {
        SKILL_FILES
            .iter()
            .filter_map(|(relative_path, contents)| {
                let name = relative_path.split('/').next()?;
                let support_state = if contents.contains("`available-now`") {
                    "available-now"
                } else {
                    "discoverable"
                };
                Some(SkillEntry {
                    name: name.to_string(),
                    support_state: support_state.to_string(),
                })
            })
            .collect()
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

fn resolve_context_input_path(repo_root: &Path, input: &str) -> PathBuf {
    let path = PathBuf::from(input);
    if path.is_absolute() { path } else { repo_root.join(path) }
}

fn snapshot_file_name(index: usize, source: &Path) -> String {
    let file_name = source.file_name().and_then(|value| value.to_str()).unwrap_or("input.txt");
    format!("input-{index:02}-{file_name}")
}

fn validate_run_artifact_record(
    record: &ArtifactRecord,
    run_id: &str,
    mode: Mode,
) -> Result<(), Error> {
    record
        .validate_relative_path(run_id, mode)
        .map_err(|message| Error::new(ErrorKind::InvalidData, message))
}

fn artifact_storage_path(
    layout: &ProjectLayout,
    record: &ArtifactRecord,
    run_id: &str,
    mode: Mode,
) -> Result<PathBuf, Error> {
    validate_run_artifact_record(record, run_id, mode)?;
    Ok(layout.canon_root.join(&record.relative_path))
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

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::TempDir;
    use time::OffsetDateTime;

    use super::{PersistedRunBundle, WorkspaceStore};
    use crate::domain::artifact::ArtifactContract;
    use crate::domain::execution::{ExecutionPosture, MutationBounds, MutationExpansionPolicy};
    use crate::domain::mode::Mode;
    use crate::domain::policy::{RiskClass, UsageZone};
    use crate::domain::run::{
        ClassificationProvenance, ImplementationExecutionContext, RefactorExecutionContext,
        RunContext, RunState, UpstreamContext,
    };
    use crate::persistence::manifests::{LinkManifest, RunManifest, RunStateManifest};

    #[test]
    fn materialized_methods_keep_promoted_execution_artifact_lists() {
        let workspace = TempDir::new().expect("temp dir");
        let store = WorkspaceStore::new(workspace.path());

        store.init_runtime_state(None).expect("init runtime state");

        let implementation = fs::read_to_string(
            workspace.path().join(".canon").join("methods").join("implementation.toml"),
        )
        .expect("implementation method");
        assert!(implementation.contains("task-mapping.md"));
        assert!(implementation.contains("validation-hooks.md"));
        assert!(implementation.contains("rollback-notes.md"));
        assert!(!implementation.contains("execution-brief.md"));
        assert!(!implementation.contains("verification-hooks.md"));

        let refactor = fs::read_to_string(
            workspace.path().join(".canon").join("methods").join("refactor.toml"),
        )
        .expect("refactor method");
        assert!(refactor.contains("preserved-behavior.md"));
        assert!(refactor.contains("no-feature-addition.md"));
        assert!(!refactor.contains("equivalence-criteria.md"));
    }

    #[test]
    fn persist_and_load_run_context_round_trip_mode_specific_execution_metadata() {
        let workspace = TempDir::new().expect("temp dir");
        let store = WorkspaceStore::new(workspace.path());

        let context = RunContext {
            repo_root: workspace.path().display().to_string(),
            owner: Some("staff-engineer".to_string()),
            inputs: vec!["canon-input/implementation.md".to_string()],
            excluded_paths: Vec::new(),
            input_fingerprints: Vec::new(),
            system_context: Some(crate::domain::run::SystemContext::Existing),
            upstream_context: Some(UpstreamContext {
                feature_slice: Some("auth session revocation".to_string()),
                primary_upstream_mode: Some("change".to_string()),
                source_refs: vec![
                    "docs/changes/R-20260422-AUTHREVOC/change-surface.md".to_string(),
                ],
                carried_forward_items: vec![
                    "Revocation output formatting remains stable.".to_string(),
                ],
                excluded_upstream_scope: Some("login UI flow".to_string()),
            }),
            implementation_execution: Some(ImplementationExecutionContext {
                plan_sources: vec!["canon-input/implementation.md".to_string()],
                mutation_bounds: MutationBounds {
                    declared_paths: vec!["src/auth".to_string()],
                    owners: vec!["staff-engineer".to_string()],
                    source_refs: vec!["canon-input/implementation.md".to_string()],
                    expansion_policy: MutationExpansionPolicy::DenyWithoutApproval,
                },
                task_targets: vec!["auth-storage".to_string()],
                safety_net: Vec::new(),
                execution_posture: ExecutionPosture::RecommendationOnly,
                rollback_expectations: vec!["rollback on auth regression".to_string()],
                post_approval_execution_consumed: true,
            }),
            refactor_execution: Some(RefactorExecutionContext {
                preserved_behavior: vec!["public API remains stable".to_string()],
                structural_rationale: Some("untangle service boundaries".to_string()),
                refactor_scope: MutationBounds {
                    declared_paths: vec!["src/reviewer".to_string()],
                    owners: vec!["staff-engineer".to_string()],
                    source_refs: vec!["canon-input/refactor.md".to_string()],
                    expansion_policy: MutationExpansionPolicy::DenyWithoutApproval,
                },
                safety_net: Vec::new(),
                no_feature_addition_target: Some("no new CLI surface".to_string()),
                allowed_exceptions: Vec::new(),
                execution_posture: ExecutionPosture::RecommendationOnly,
                post_approval_execution_consumed: true,
            }),
            backlog_planning: None,
            inline_inputs: Vec::new(),
            captured_at: OffsetDateTime::UNIX_EPOCH,
        };

        let bundle = PersistedRunBundle {
            run: RunManifest {
                run_id: "run-context-roundtrip".to_string(),
                uuid: None,
                short_id: None,
                slug: None,
                title: None,
                mode: Mode::Implementation,
                risk: RiskClass::BoundedImpact,
                zone: UsageZone::Yellow,
                system_context: Some(crate::domain::run::SystemContext::Existing),
                classification: ClassificationProvenance::explicit(),
                owner: "staff-engineer".to_string(),
                created_at: OffsetDateTime::UNIX_EPOCH,
            },
            context: context.clone(),
            state: RunStateManifest {
                state: RunState::ContextCaptured,
                updated_at: OffsetDateTime::UNIX_EPOCH,
            },
            artifact_contract: ArtifactContract {
                version: 1,
                artifact_requirements: Vec::new(),
                required_verification_layers: Vec::new(),
            },
            artifacts: Vec::new(),
            links: LinkManifest {
                artifacts: Vec::new(),
                decisions: Vec::new(),
                traces: Vec::new(),
                invocations: Vec::new(),
                evidence: None,
            },
            gates: Vec::new(),
            approvals: Vec::new(),
            verification_records: Vec::new(),
            evidence: None,
            invocations: Vec::new(),
        };

        store.persist_run_bundle(&bundle).expect("persist run bundle");
        let loaded = store.load_run_context("run-context-roundtrip").expect("load run context");

        assert_eq!(loaded.implementation_execution, context.implementation_execution);
        assert_eq!(loaded.refactor_execution, context.refactor_execution);
        assert_eq!(loaded.upstream_context, context.upstream_context);
        assert!(loaded.inline_inputs.is_empty());
    }
}
