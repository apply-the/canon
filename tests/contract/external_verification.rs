//! Contract tests for externally produced semantic verification evidence.

use canon_adapters::reviewer::{
    EvidenceFreshness, ExternalEvidenceValidationContext, SemanticReviewExecutionStatus,
    SemanticReviewerExecutor, TerminalEvidenceState, validate_external_evidence,
};
use canon_contracts::{
    ChallengeTier, Claim, EvidenceReference, ExternalVerificationEvidence, Finding,
};

type TestResult = Result<(), Box<dyn std::error::Error>>;

const CANON_VALIDATOR: &str = "canon-deterministic-validator";
const CURRENT_INVOCATION: &str = "canon-invocation-42";
const EXTERNAL_REVIEWER: &str = "independent-reviewer";
const EXTERNAL_LINEAGE: &str = "provider:anthropic/executor:reviewer-a/invocation:external-7";
const INDEPENDENT_CONTEXT: &str = "context:independent-review-9";

fn require(condition: bool, message: &str) -> TestResult {
    if condition { Ok(()) } else { Err(message.to_string().into()) }
}

fn claim(value: &str) -> Claim {
    Claim::new(value)
}

fn evidence_ref(value: &str) -> EvidenceReference {
    EvidenceReference::new(value)
}

fn valid_evidence() -> ExternalVerificationEvidence {
    ExternalVerificationEvidence {
        reviewer_identity: EXTERNAL_REVIEWER.to_string(),
        lineage: EXTERNAL_LINEAGE.to_string(),
        independent_context_identity: INDEPENDENT_CONTEXT.to_string(),
        claims: vec![claim("claim:packet-structure")],
        findings: vec![Finding::new("finding:packet-is-bounded")],
        evidence_references: vec![evidence_ref("sha256:external-evidence-7")],
        challenge_tier: ChallengeTier::Tier2,
        named_override: None,
    }
}

fn validation_context() -> ExternalEvidenceValidationContext {
    ExternalEvidenceValidationContext {
        canon_validator_identity: CANON_VALIDATOR.to_string(),
        current_invocation_identity: CURRENT_INVOCATION.to_string(),
        required_claims: vec![claim("claim:packet-structure")],
        accepted_evidence_references: vec![evidence_ref("sha256:external-evidence-7")],
        minimum_challenge_tier: ChallengeTier::Tier2,
        freshness: EvidenceFreshness::Fresh,
        terminal_state: TerminalEvidenceState::Terminal,
        forbidden_lineages: vec!["conversation:implementer".to_string()],
    }
}

#[test]
fn valid_external_evidence_is_accepted_structurally_without_semantic_attestation() -> TestResult {
    let accepted = validate_external_evidence(&valid_evidence(), &validation_context())?;

    require(accepted.structure_valid, "external evidence structure was not accepted")?;
    require(
        !accepted.semantic_judgment_asserted,
        "Canon converted structural validation into semantic truth",
    )
}

#[test]
fn missing_self_attested_or_current_invocation_lineage_fails_closed() -> TestResult {
    let context = validation_context();
    for (identity, lineage, independent_context) in [
        (EXTERNAL_REVIEWER, "", INDEPENDENT_CONTEXT),
        (CANON_VALIDATOR, EXTERNAL_LINEAGE, INDEPENDENT_CONTEXT),
        (EXTERNAL_REVIEWER, CURRENT_INVOCATION, INDEPENDENT_CONTEXT),
        (EXTERNAL_REVIEWER, EXTERNAL_LINEAGE, CURRENT_INVOCATION),
        (EXTERNAL_REVIEWER, "conversation:implementer", INDEPENDENT_CONTEXT),
        (EXTERNAL_REVIEWER, "malformed-lineage", INDEPENDENT_CONTEXT),
    ] {
        let mut evidence = valid_evidence();
        evidence.reviewer_identity = identity.to_string();
        evidence.lineage = lineage.to_string();
        evidence.independent_context_identity = independent_context.to_string();
        require(
            validate_external_evidence(&evidence, &context).is_err(),
            "non-independent evidence was accepted",
        )?;
    }
    Ok(())
}

#[test]
fn label_only_unbound_mismatched_stale_or_nonterminal_evidence_fails_closed() -> TestResult {
    let mut evidence = valid_evidence();
    evidence.claims.clear();
    require(
        validate_external_evidence(&evidence, &validation_context()).is_err(),
        "claim-free evidence was accepted",
    )?;

    let mut evidence = valid_evidence();
    evidence.evidence_references.clear();
    require(
        validate_external_evidence(&evidence, &validation_context()).is_err(),
        "reference-free evidence was accepted",
    )?;

    let mut context = validation_context();
    context.required_claims = vec![claim("claim:different")];
    require(
        validate_external_evidence(&valid_evidence(), &context).is_err(),
        "claim-mismatched evidence was accepted",
    )?;

    let mut context = validation_context();
    context.accepted_evidence_references = vec![evidence_ref("sha256:different")];
    require(
        validate_external_evidence(&valid_evidence(), &context).is_err(),
        "digest/reference-mismatched evidence was accepted",
    )?;

    let mut context = validation_context();
    context.freshness = EvidenceFreshness::Stale;
    require(
        validate_external_evidence(&valid_evidence(), &context).is_err(),
        "stale evidence was accepted",
    )?;

    let mut context = validation_context();
    context.terminal_state = TerminalEvidenceState::Incomplete;
    require(
        validate_external_evidence(&valid_evidence(), &context).is_err(),
        "incomplete evidence was accepted",
    )
}

#[test]
fn duplicate_references_and_unknown_authority_values_fail_closed() -> TestResult {
    let mut evidence = valid_evidence();
    evidence.evidence_references.push(evidence_ref("sha256:external-evidence-7"));
    require(
        validate_external_evidence(&evidence, &validation_context()).is_err(),
        "duplicate evidence identity was accepted",
    )?;

    let malformed = serde_json::json!({
        "reviewer_identity": EXTERNAL_REVIEWER,
        "lineage": EXTERNAL_LINEAGE,
        "independent_context_identity": INDEPENDENT_CONTEXT,
        "claims": ["claim:packet-structure"],
        "findings": [],
        "evidence_references": ["sha256:external-evidence-7"],
        "challenge_tier": "tier_99",
        "named_override": null
    });
    require(
        serde_json::from_value::<ExternalVerificationEvidence>(malformed).is_err(),
        "unknown authority-expanding tier was accepted",
    )?;

    let mut evidence = valid_evidence();
    evidence.challenge_tier = ChallengeTier::Tier1;
    require(
        validate_external_evidence(&evidence, &validation_context()).is_err(),
        "insufficient challenge tier was accepted",
    )?;

    let mut evidence = valid_evidence();
    evidence.named_override = Some("  ".to_string());
    require(
        validate_external_evidence(&evidence, &validation_context()).is_err(),
        "blank named override was accepted",
    )?;

    let evidence = valid_evidence();
    let mut context = validation_context();
    context.accepted_evidence_references.push(evidence_ref("sha256:required-packet-binding"));
    require(
        validate_external_evidence(&evidence, &context).is_err(),
        "a strict subset of the required evidence binding was accepted",
    )
}

#[test]
fn semantic_reviewer_execution_is_explicitly_unsupported_and_side_effect_free() -> TestResult {
    let executor = SemanticReviewerExecutor;
    let outcome = executor.request_execution();

    require(
        matches!(outcome.status, SemanticReviewExecutionStatus::Unsupported),
        "Canon represented semantic-review execution as successful",
    )?;
    require(outcome.process_invocations == 0, "reviewer process was invoked")?;
    require(outcome.network_invocations == 0, "reviewer network was invoked")?;
    require(outcome.provider_credential_reads == 0, "provider credentials were read")?;
    require(outcome.created_evidence == 0, "Canon manufactured semantic evidence")
}
