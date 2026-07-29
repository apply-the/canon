//! Typed fixtures for exercising the stable one-shot governance boundary.

use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};

use canon_contracts::{ChallengeTier, Profile, VerificationKind};
use canon_engine::decision_memory::{
    ApprovalContent, ArtifactContent, EvidenceContent, GovernanceBundleDraft,
    GovernancePacketDraft, NodeId, RiskAcceptanceContent, SubjectArtifactBinding,
    VerificationRequirementContent,
};
use serde_json::{Value, json};
use tempfile::TempDir;

type FixtureResult<T> = Result<T, Box<dyn std::error::Error>>;

/// Isolated Canon runtime used by sequential one-shot invocations.
pub(super) struct RpcFixture {
    workspace: TempDir,
}

impl RpcFixture {
    /// Creates a clean isolated repository-shaped runtime root.
    pub(super) fn new() -> FixtureResult<Self> {
        Ok(Self { workspace: tempfile::tempdir()? })
    }

    /// Invokes one operation and requires process success.
    pub(super) fn invoke(
        &self,
        request_id: &str,
        operation: &str,
        payload: Value,
    ) -> FixtureResult<Value> {
        let (status, response) = self.invoke_raw(json!({
            "contract_version": "1.0",
            "request_id": request_id,
            "operation": operation,
            "payload": payload
        }))?;
        if status {
            Ok(response)
        } else {
            Err(format!("RPC `{operation}` unexpectedly failed: {response}").into())
        }
    }

    /// Invokes one operation and requires a typed non-success response.
    pub(super) fn invoke_rejected(
        &self,
        request_id: &str,
        operation: &str,
        payload: Value,
    ) -> FixtureResult<Value> {
        self.invoke_raw_rejected(json!({
            "contract_version": "1.0",
            "request_id": request_id,
            "operation": operation,
            "payload": payload
        }))
    }

    /// Invokes an arbitrary envelope and requires a typed rejection.
    pub(super) fn invoke_raw_rejected(&self, request: Value) -> FixtureResult<Value> {
        let (status, response) = self.invoke_raw(request)?;
        if status {
            Err(format!("RPC unexpectedly succeeded: {response}").into())
        } else {
            Ok(response)
        }
    }

    /// Invokes an envelope and exposes both process status and typed response.
    pub(super) fn invoke_with_status(
        &self,
        request_id: &str,
        operation: &str,
        payload: Value,
    ) -> FixtureResult<(bool, Value)> {
        self.invoke_raw(json!({
            "contract_version": "1.0",
            "request_id": request_id,
            "operation": operation,
            "payload": payload
        }))
    }

    /// Reads the exact durable snapshot for mutation checks.
    pub(super) fn snapshot_bytes(&self) -> FixtureResult<Vec<u8>> {
        Ok(fs::read(self.workspace.path().join(".canon/decision-memory/state.json"))?)
    }

    /// Admits a draft through the stable human CLI and returns its JSON projection.
    pub(super) fn cli_run(&self, draft: &GovernanceBundleDraft) -> FixtureResult<Value> {
        let input = self.workspace.path().join("bundle.json");
        fs::write(&input, serde_json::to_vec(draft)?)?;
        self.cli(&[
            "run",
            "--profile",
            profile_wire(draft.profile),
            "--bundle",
            "bundle.json",
            "--output",
            "json",
        ])
    }

    /// Reads decision memory through the stable human CLI.
    pub(super) fn cli_inspect(&self) -> FixtureResult<Value> {
        self.cli(&["inspect", "decision-memory", "--output", "json"])
    }

    /// Makes the runtime root unwritable as a directory for persistence-failure tests.
    pub(super) fn obstruct_state_root(&self) -> FixtureResult<()> {
        fs::write(self.workspace.path().join(".canon"), b"not-a-directory")?;
        Ok(())
    }

    fn cli(&self, args: &[&str]) -> FixtureResult<Value> {
        let output = Command::new("cargo")
            .args([
                "run",
                "--quiet",
                "--manifest-path",
                concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"),
                "-p",
                "canon-cli",
                "--bin",
                "canon",
                "--",
                "--canon-root",
            ])
            .arg(self.workspace.path())
            .args(["--repo-root"])
            .arg(self.workspace.path())
            .args(args)
            .current_dir(self.workspace.path())
            .output()?;
        if !output.status.success() {
            return Err(format!("CLI failed: {}", String::from_utf8_lossy(&output.stderr)).into());
        }
        Ok(serde_json::from_slice(&output.stdout)?)
    }

    fn invoke_raw(&self, request: Value) -> FixtureResult<(bool, Value)> {
        let mut child = Command::new("cargo")
            .args([
                "run",
                "--quiet",
                "--manifest-path",
                concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"),
                "-p",
                "canon-cli",
                "--bin",
                "canon",
                "--",
                "--canon-root",
            ])
            .arg(self.workspace.path())
            .args(["--repo-root"])
            .arg(self.workspace.path())
            .args(["rpc", "--stdio"])
            .current_dir(self.workspace.path())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        let mut stdin =
            child.stdin.take().ok_or_else(|| "Canon child stdin was unavailable".to_string())?;
        serde_json::to_writer(&mut stdin, &request)?;
        stdin.flush()?;
        drop(stdin);
        let output = child.wait_with_output()?;
        if !output.stderr.is_empty() {
            return Err(format!(
                "RPC diagnostics leaked to stderr: {}",
                String::from_utf8_lossy(&output.stderr)
            )
            .into());
        }
        let response = serde_json::from_slice(&output.stdout)?;
        Ok((output.status.success(), response))
    }
}

fn profile_wire(profile: Profile) -> &'static str {
    match profile {
        Profile::Discovery => "discovery",
        Profile::Requirements => "requirements",
        Profile::Architecture => "architecture",
        Profile::Backlog => "backlog",
        Profile::Change => "change",
        Profile::Refactor => "refactor",
        Profile::Verification => "verification",
        Profile::PrReview => "pr-review",
        Profile::Incident => "incident",
    }
}

/// Complete Tier 2 bundle accepted by the deterministic governance matrix.
pub(super) fn governance_draft(bundle_id: &str, revision: u64) -> GovernanceBundleDraft {
    let packet_id = NodeId::new(format!("packet-{bundle_id}"));
    let artifact_id = NodeId::new(format!("artifact-{bundle_id}"));
    let claim_id = NodeId::new(format!("claim-{bundle_id}-main"));
    let requirement_id = NodeId::new(format!("requirement-{bundle_id}"));
    let evidence_id = NodeId::new(format!("evidence-{bundle_id}"));
    let approval_id = NodeId::new(format!("approval-{bundle_id}"));
    GovernanceBundleDraft {
        bundle_id: bundle_id.to_string(),
        profile: Profile::Discovery,
        decision_memory_revision: revision,
        packets: vec![GovernancePacketDraft {
            packet_id: packet_id.to_string(),
            profile: Profile::Discovery,
            revision,
            change_intent: "preserve deterministic governance".to_string(),
            scope: vec!["workspace".to_string()],
            risks: vec!["incorrect authorization".to_string()],
            invariants: vec!["Canon executes no semantic reviewer".to_string()],
            acceptance_criteria: vec!["exact bindings validate".to_string()],
            cross_packet_references: Vec::new(),
        }],
        subject_artifacts: vec![SubjectArtifactBinding {
            artifact_id: artifact_id.clone(),
            packet_id: packet_id.clone(),
            content: ArtifactContent {
                artifact_identity: "workspace".to_string(),
                revision: format!("git:fixture-{revision}"),
                content_digest: if revision == 1 { "a".repeat(64) } else { "b".repeat(64) },
            },
        }],
        required_approvers: vec!["release-owner".to_string()],
        authority_zone: "governance-release".to_string(),
        risk_tier: 2,
        change_class: "governance-kernel".to_string(),
        context_class: "repository-local".to_string(),
        owners: vec!["release-owner".to_string()],
        claims: vec!["claim-exact-binding".to_string()],
        required_evidence: vec![VerificationRequirementContent {
            requirement_id: requirement_id.clone(),
            packet_id: packet_id.clone(),
            claim_ids: vec![claim_id.clone()],
            artifact_ids: vec![artifact_id.clone()],
            kind: VerificationKind::ExternalSemanticReview,
            minimum_challenge_tier: ChallengeTier::Tier2,
            accepted_evidence_references: vec![format!("sha256:{}", "e".repeat(64))],
        }],
        provided_evidence: vec![EvidenceContent {
            evidence_id: evidence_id.clone(),
            packet_id: packet_id.clone(),
            claim_ids: vec![claim_id.clone()],
            artifact_ids: vec![artifact_id.clone()],
            requirement_ids: vec![requirement_id.clone()],
            references: vec![format!("sha256:{}", "e".repeat(64))],
            lineage: "provider:challenger/executor:review/invocation:m2b-b".to_string(),
            independent_context_identity: "context-independent-m2b-b".to_string(),
            challenge_tier: ChallengeTier::Tier2,
            external_semantic: true,
            fresh: true,
            named_override: None,
        }],
        forbidden_lineages: vec!["implementer-lineage".to_string()],
        approvals: vec![ApprovalContent {
            approval_id: approval_id.clone(),
            packet_id: packet_id.clone(),
            claim_ids: vec![claim_id],
            artifact_ids: vec![artifact_id],
            evidence_ids: vec![evidence_id],
            requirement_ids: vec![requirement_id],
            approver: "release-owner".to_string(),
            authority_zone: "governance-release".to_string(),
            approved: true,
            decision_memory_revision: revision,
            valid_through_revision: Some(revision),
            fresh: true,
        }],
        assumptions: vec!["published contract remains immutable".to_string()],
        alternatives: vec!["defer the bounded operation".to_string()],
        rationale: "exact bindings fail closed".to_string(),
        risk_acceptances: vec![RiskAcceptanceContent {
            acceptance_id: NodeId::new(format!("risk-{bundle_id}")),
            packet_id,
            owner: "release-owner".to_string(),
            risk: "same-lineage:implementer-lineage".to_string(),
            justification: "fixture exercises a named acceptance".to_string(),
            challenge_tier: ChallengeTier::Tier2,
            lineage: "implementer-lineage".to_string(),
            approval_id,
            fresh: true,
        }],
        triggers: vec!["packet content changes".to_string()],
        no_change: false,
    }
}
