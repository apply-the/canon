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

mod bootstrap;
mod runtime;

/// Summary of a Canon workspace initialization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InitSummary {
    /// Absolute path of the repository root.
    pub repo_root: String,
    /// Absolute path of the `.canon/` directory.
    pub canon_root: String,
    /// Number of method files materialized.
    pub methods_materialized: usize,
    /// Number of policy files materialized.
    pub policies_materialized: usize,
    /// Number of skill files materialized.
    pub skills_materialized: usize,
    /// Whether the `CLAUDE.md` file was created.
    pub claude_md_created: bool,
}

/// The target location for materializing Canon skills.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkillMaterializationTarget {
    /// Materialize skills into `.agents/skills/`.
    Agents,
    /// Materialize skills into `.claude/skills/` and create `CLAUDE.md`.
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

/// Summary of a skill materialization operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SkillsSummary {
    /// Absolute path of the skills directory written to.
    pub skills_dir: String,
    /// Number of skill files written.
    pub skills_materialized: usize,
    /// Number of skill files skipped because they already existed.
    pub skills_skipped: usize,
    /// Whether the `CLAUDE.md` file was created.
    pub claude_md_created: bool,
}

/// A discoverable skill entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SkillEntry {
    /// The skill name (directory name under `.agents/skills/`).
    pub name: String,
    /// Human-readable support state label for the skill.
    pub support_state: String,
}

/// A persisted run artifact, pairing its record with its on-disk contents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersistedArtifact {
    /// The artifact record metadata.
    pub record: ArtifactRecord,
    /// The full text contents of the artifact file.
    pub contents: String,
}

/// A complete bundle of all persisted data for a single governed run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersistedRunBundle {
    /// The run manifest (identity, mode, risk, zone, owner, etc.).
    pub run: RunManifest,
    /// The runtime context (inputs, inline inputs, adapter context).
    pub context: RunContext,
    /// The current lifecycle state of the run.
    pub state: RunStateManifest,
    /// The artifact contract specifying required output files.
    pub artifact_contract: ArtifactContract,
    /// Persisted artifacts (records + contents).
    pub artifacts: Vec<PersistedArtifact>,
    /// The link manifest referencing all associated files.
    pub links: LinkManifest,
    /// Gate evaluations recorded for the run.
    pub gates: Vec<GateEvaluation>,
    /// Approval records for the run.
    pub approvals: Vec<ApprovalRecord>,
    /// Verification records produced by the run.
    pub verification_records: Vec<VerificationRecord>,
    /// The evidence bundle, if captured.
    pub evidence: Option<EvidenceBundle>,
    /// Persisted adapter invocation records.
    pub invocations: Vec<crate::persistence::invocations::PersistedInvocation>,
}

/// Provides unified access to the repository's governance state, policies, and artifacts.
///
/// `WorkspaceStore` is the primary interface for reading and writing persisted Canon data,
/// including the materialization of embedded skills and method definitions.
#[derive(Debug, Clone)]
pub struct WorkspaceStore {
    /// The directory layout for this repository.
    pub layout: ProjectLayout,
    filesystem: FilesystemAdapter,
}

impl WorkspaceStore {
    /// Creates a new `WorkspaceStore` anchored to the given repository root.
    pub fn new(repo_root: impl AsRef<Path>) -> Self {
        Self { layout: ProjectLayout::new(repo_root), filesystem: FilesystemAdapter }
    }

    /// Persists all data from a completed run bundle to the canonical layout.
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

    /// Returns the relative `.canon/`-prefixed paths of all artifact files for a run.
    /// Persists a text payload for an invocation request.
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

    use super::{
        PersistedArtifact, PersistedRunBundle, SkillMaterializationTarget, WorkspaceStore,
    };
    use crate::domain::artifact::{
        ArtifactContract, ArtifactFormat, ArtifactRecord, ArtifactRequirement,
    };
    use crate::domain::execution::PayloadRetentionLevel;
    use crate::domain::execution::{ExecutionPosture, MutationBounds, MutationExpansionPolicy};
    use crate::domain::gate::GateKind;
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

    #[test]
    fn load_persisted_artifacts_skips_missing_optional_artifacts() {
        let workspace = TempDir::new().expect("temp dir");
        let store = WorkspaceStore::new(workspace.path());

        let bundle = PersistedRunBundle {
            run: RunManifest {
                run_id: "optional-artifacts".to_string(),
                uuid: None,
                short_id: None,
                slug: None,
                title: None,
                mode: Mode::Architecture,
                risk: RiskClass::BoundedImpact,
                zone: UsageZone::Yellow,
                system_context: Some(crate::domain::run::SystemContext::Existing),
                classification: ClassificationProvenance::explicit(),
                owner: "principal-architect".to_string(),
                created_at: OffsetDateTime::UNIX_EPOCH,
            },
            context: RunContext {
                repo_root: workspace.path().display().to_string(),
                owner: Some("principal-architect".to_string()),
                inputs: vec!["architecture.md".to_string()],
                excluded_paths: Vec::new(),
                input_fingerprints: Vec::new(),
                system_context: Some(crate::domain::run::SystemContext::Existing),
                upstream_context: None,
                implementation_execution: None,
                refactor_execution: None,
                backlog_planning: None,
                inline_inputs: Vec::new(),
                captured_at: OffsetDateTime::UNIX_EPOCH,
            },
            state: RunStateManifest {
                state: RunState::Completed,
                updated_at: OffsetDateTime::UNIX_EPOCH,
            },
            artifact_contract: ArtifactContract {
                version: 1,
                artifact_requirements: vec![
                    ArtifactRequirement {
                        file_name: "architecture-overview.md".to_string(),
                        format: ArtifactFormat::Markdown,
                        required_sections: vec!["Summary".to_string()],
                        gates: Vec::new(),
                        required: true,
                    },
                    ArtifactRequirement {
                        file_name: "dynamic-view.md".to_string(),
                        format: ArtifactFormat::Markdown,
                        required_sections: vec!["Summary".to_string()],
                        gates: Vec::new(),
                        required: false,
                    },
                ],
                required_verification_layers: Vec::new(),
            },
            artifacts: vec![PersistedArtifact {
                record: ArtifactRecord {
                    file_name: "architecture-overview.md".to_string(),
                    relative_path:
                        "artifacts/optional-artifacts/architecture/architecture-overview.md"
                            .to_string(),
                    format: ArtifactFormat::Markdown,
                    provenance: None,
                },
                contents: "# Architecture Overview\n\n## Summary\n\nPrimary packet.\n".to_string(),
            }],
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

        let loaded = store
            .load_persisted_artifacts(
                "optional-artifacts",
                Mode::Architecture,
                &bundle.artifact_contract,
            )
            .expect("load artifacts");

        assert_eq!(loaded.len(), 1, "optional missing artifacts should be ignored");
        assert_eq!(loaded[0].record.file_name, "architecture-overview.md");
    }

    #[test]
    fn update_skills_for_claude_overwrites_files_without_recreating_claude_md() {
        let workspace = TempDir::new().expect("temp dir");
        let store = WorkspaceStore::new(workspace.path());

        let install =
            store.install_skills(SkillMaterializationTarget::Claude).expect("install skills");
        assert!(install.claude_md_created);

        let skill_path =
            workspace.path().join(".claude").join("skills").join("canon-init").join("SKILL.md");
        fs::write(&skill_path, "customized\n").expect("overwrite installed skill");

        let update =
            store.update_skills(SkillMaterializationTarget::Claude).expect("update skills");

        assert!(update.skills_dir.ends_with(".claude/skills"));
        assert_eq!(update.skills_skipped, 0);
        assert!(!update.claude_md_created);
        assert_ne!(fs::read_to_string(&skill_path).expect("read updated skill"), "customized\n");
    }

    #[test]
    fn load_policy_set_applies_gate_and_verification_overrides_without_adapter_override() {
        let workspace = TempDir::new().expect("temp dir");
        let store = WorkspaceStore::new(workspace.path());
        store.init_runtime_state(None).expect("init runtime state");

        let override_root = workspace.path().join("policy-overrides");
        fs::create_dir_all(&override_root).expect("create override directory");
        fs::write(
            override_root.join("gates.toml"),
            "version = 1\nmandatory_gates = [\"Exploration\"]\n",
        )
        .expect("write gate override");
        fs::write(
            override_root.join("verification.toml"),
            concat!(
                "version = 2\n\n",
                "[layers]\n",
                "low = [\"SelfCritique\"]\n",
                "bounded = [\"SelfCritique\"]\n",
                "systemic = [\"SelfCritique\"]\n\n",
                "[independence]\n",
                "ai_generation_requires_distinct_validation = false\n",
                "human_review_counts_independent = false\n",
            ),
        )
        .expect("write verification override");

        let policy_set =
            store.load_policy_set(Some(&override_root)).expect("load policy set with overrides");

        assert_eq!(policy_set.gate_policy.mandatory_gates, vec![GateKind::Exploration]);
        assert!(!policy_set.validation_independence.ai_generation_requires_distinct_validation);
        assert!(!policy_set.validation_independence.human_review_counts_independent);
    }

    #[test]
    fn load_policy_set_applies_risk_zone_and_adapter_overrides_without_verification_override() {
        let workspace = TempDir::new().expect("temp dir");
        let store = WorkspaceStore::new(workspace.path());
        store.init_runtime_state(None).expect("init runtime state");

        let override_root = workspace.path().join("adapter-overrides");
        fs::create_dir_all(&override_root).expect("create override directory");
        fs::write(
            override_root.join("risk.toml"),
            concat!(
                "version = 1\n\n",
                "[[class]]\n",
                "name = \"LowImpact\"\n",
                "requires_owner = true\n",
                "mutable_execution = false\n",
                "verification_layers = [\"SelfCritique\"]\n",
            ),
        )
        .expect("write risk override");
        fs::write(
            override_root.join("zones.toml"),
            concat!(
                "version = 1\n\n",
                "[[zone]]\n",
                "name = \"Yellow\"\n",
                "mutable_execution = false\n",
            ),
        )
        .expect("write zone override");
        fs::write(
            override_root.join("adapters.toml"),
            concat!(
                "version = 2\n\n",
                "[[adapter]]\n",
                "kind = \"Shell\"\n",
                "capabilities = [\"RunCommand\"]\n\n",
                "[[constraint_profile]]\n",
                "id = \"override-profile\"\n",
                "payload_retention = \"SummaryOnly\"\n",
                "max_payload_bytes = 1024\n",
                "command_profile = \"override-profile\"\n",
                "recommendation_only = true\n",
                "patch_disabled = true\n\n",
                "[rules]\n",
                "block_mutation_for_red_or_systemic = false\n",
                "runtime_disabled_adapters = [\"Shell\"]\n",
            ),
        )
        .expect("write adapter override");

        let policy_set = store
            .load_policy_set(Some(&override_root))
            .expect("load policy set with adapter overrides");

        let low_impact = policy_set
            .risk_classes
            .iter()
            .find(|class| class.name == RiskClass::LowImpact)
            .expect("low-impact class override");
        assert!(low_impact.requires_owner);
        assert!(!low_impact.mutable_execution);

        let yellow = policy_set
            .zones
            .iter()
            .find(|zone| zone.name == UsageZone::Yellow)
            .expect("yellow zone override");
        assert!(!yellow.mutable_execution);

        let shell = policy_set
            .adapter_matrix
            .iter()
            .find(|entry| entry.adapter == canon_adapters::AdapterKind::Shell)
            .expect("shell adapter override");
        assert_eq!(shell.capabilities, vec![canon_adapters::CapabilityKind::RunCommand]);

        let constraint_profile =
            policy_set.constraint_profile("override-profile").expect("constraint profile override");
        assert_eq!(constraint_profile.payload_retention, Some(PayloadRetentionLevel::SummaryOnly));
        assert_eq!(policy_set.runtime_disabled_adapters, vec![canon_adapters::AdapterKind::Shell]);
        assert!(!policy_set.block_mutation_for_red_or_systemic);
    }

    #[test]
    fn runtime_listing_helpers_cover_missing_present_and_invalid_artifact_cases() {
        let workspace = TempDir::new().expect("temp dir");
        let store = WorkspaceStore::new(workspace.path());

        assert_eq!(
            store
                .list_artifact_files("missing-run")
                .expect("missing artifacts should return empty"),
            Vec::<String>::new()
        );

        let run_artifacts_root = store.layout.artifacts_dir().join("run-listing");
        fs::create_dir_all(run_artifacts_root.join("requirements"))
            .expect("create valid mode directory without manifest");
        assert_eq!(
            store
                .list_artifact_files("run-listing")
                .expect("missing manifest directories should be skipped"),
            Vec::<String>::new()
        );

        fs::create_dir_all(run_artifacts_root.join("invalid-mode"))
            .expect("create invalid mode directory");
        let error = store
            .list_artifact_files("run-listing")
            .expect_err("unsupported artifact mode directories should fail");
        assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);
        assert!(error.to_string().contains("artifact manifest directory `invalid-mode`"));
    }

    #[test]
    fn runtime_loading_helpers_cover_evidence_trace_and_invocation_wrappers() {
        let workspace = TempDir::new().expect("temp dir");
        let store = WorkspaceStore::new(workspace.path());

        assert_eq!(
            store
                .list_invocation_ids("missing-run")
                .expect("missing invocation dir should list empty ids"),
            Vec::<String>::new()
        );
        assert_eq!(
            store
                .list_evidence_entries("missing-run")
                .expect("missing evidence should list empty entries"),
            Vec::<String>::new()
        );
        assert!(
            store
                .load_evidence_bundle("missing-run")
                .expect("missing evidence bundle should load")
                .is_none()
        );
        assert_eq!(
            store
                .load_trace_events("missing-run")
                .expect("missing trace file should load empty events"),
            Vec::new()
        );

        let run_id = "runtime-helpers";
        fs::create_dir_all(store.layout.run_dir(run_id).join("invocations"))
            .expect("create invocation root");
        fs::create_dir_all(store.layout.traces_dir()).expect("create traces dir");
        fs::write(
            store.layout.run_evidence_path(run_id),
            concat!(
                "run_id = \"runtime-helpers\"\n",
                "generation_paths = []\n",
                "validation_paths = []\n",
                "denied_invocations = []\n",
                "trace_refs = []\n",
                "artifact_refs = []\n",
                "decision_refs = []\n",
                "approval_refs = []\n",
            ),
        )
        .expect("write evidence bundle");
        fs::write(store.layout.traces_dir().join(format!("{run_id}.jsonl")), "not-json\n")
            .expect("write invalid trace file");

        assert_eq!(
            store.list_evidence_entries(run_id).expect("present evidence should be listed"),
            vec!["evidence.toml".to_string()]
        );
        assert!(
            store
                .load_evidence_bundle(run_id)
                .expect("present evidence bundle should load")
                .is_some()
        );

        let trace_error =
            store.load_trace_events(run_id).expect_err("invalid trace jsonl should fail");
        assert_eq!(trace_error.kind(), std::io::ErrorKind::Other);
    }
}
