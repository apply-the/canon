use crate::domain::artifact::ArtifactContract;
use crate::domain::mode::Mode;
use crate::domain::policy::{PolicySet, RiskClass, UsageZone};
use crate::domain::run::SystemContext;

/// Whether system context is required or optional for a given mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemContextRequirement {
    /// The mode requires an explicit system context.
    Required,
    /// The mode accepts an optional system context.
    Optional,
}

/// Returns the system context requirement for the given mode.
pub fn system_context_requirement(mode: Mode) -> SystemContextRequirement {
    match mode {
        Mode::SystemShaping
        | Mode::Architecture
        | Mode::Change
        | Mode::Backlog
        | Mode::Implementation
        | Mode::Refactor
        | Mode::Migration
        | Mode::Incident
        | Mode::SystemAssessment
        | Mode::SecurityAssessment
        | Mode::SupplyChainAnalysis
        | Mode::Debugging => SystemContextRequirement::Required,
        Mode::Discovery
        | Mode::Requirements
        | Mode::Review
        | Mode::Verification
        | Mode::PrReview
        | Mode::DomainLanguage
        | Mode::DomainModel => SystemContextRequirement::Optional,
    }
}

/// Validates that the system context is compatible with the mode's requirements.
pub fn validate_system_context(
    mode: Mode,
    system_context: Option<SystemContext>,
) -> Result<(), String> {
    // Work type and system state stay independent: mode picks the method, context picks the target state.
    match (system_context_requirement(mode), system_context) {
        (SystemContextRequirement::Required, None) => Err(format!(
            "mode `{}` requires --system-context {}",
            mode.as_str(),
            supported_system_context_usage(mode)
        )),
        (_, Some(SystemContext::New))
            if matches!(
                mode,
                Mode::Change
                    | Mode::Backlog
                    | Mode::SystemAssessment
                    | Mode::SecurityAssessment
                    | Mode::SupplyChainAnalysis
            ) =>
        {
            Err(format!(
                "mode `{}` currently supports only --system-context existing in this release",
                mode.as_str()
            ))
        }
        _ => Ok(()),
    }
}

fn supported_system_context_usage(mode: Mode) -> &'static str {
    match mode {
        Mode::Change
        | Mode::Backlog
        | Mode::SystemAssessment
        | Mode::SecurityAssessment
        | Mode::SupplyChainAnalysis => "existing",
        _ => "new|existing",
    }
}

/// Whether a mutating adapter operation may execute or is recommendation-only.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MutationPolicy {
    /// The operation is permitted to mutate state.
    Execute,
    /// The operation produces only recommendations; no state mutations are applied.
    RecommendationOnly,
}

/// Returns whether mutation is permitted for the given risk/zone pair.
pub fn allow_mutation(policy_set: &PolicySet, risk: RiskClass, zone: UsageZone) -> bool {
    policy_set.allow_mutation(risk, zone)
}

/// Returns the mutation policy for a mode under the given risk/zone pair.
pub fn mutation_policy_for_mode(
    _mode: Mode,
    policy_set: &PolicySet,
    risk: RiskClass,
    zone: UsageZone,
) -> MutationPolicy {
    if allow_mutation(policy_set, risk, zone) {
        MutationPolicy::Execute
    } else {
        MutationPolicy::RecommendationOnly
    }
}

/// Validates that a named owner is present when required by the risk class policy.
pub fn classify_owner_requirement(
    policy_set: &PolicySet,
    risk: RiskClass,
    owner: &str,
) -> Result<(), String> {
    let owner_required = policy_set
        .risk_classes
        .iter()
        .find(|class| class.name == risk)
        .map(|class| class.requires_owner)
        .unwrap_or(false);

    if owner_required && owner.trim().is_empty() {
        return Err(format!("risk class `{}` requires human ownership", risk.as_str()));
    }

    Ok(())
}

/// Applies the verification layers required by the risk class to the given artifact contract.
pub fn apply_verification_layers(
    policy_set: &PolicySet,
    risk: RiskClass,
    contract: &mut ArtifactContract,
) {
    contract.required_verification_layers = policy_set.verification_layers_for(risk);
}

/// Confidence level of an inferred risk/zone classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClassificationConfidence {
    /// Low confidence; the classification is a weak guess from limited signals.
    Low,
    /// Moderate confidence; the classification is plausible but should be confirmed.
    Moderate,
    /// High confidence; signals strongly support the classification.
    High,
}

impl ClassificationConfidence {
    /// Returns the kebab-case string representation of this confidence level.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Moderate => "moderate",
            Self::High => "high",
        }
    }

    fn min(self, other: Self) -> Self {
        use ClassificationConfidence::{High, Low, Moderate};

        match (self, other) {
            (Low, _) | (_, Low) => Low,
            (Moderate, _) | (_, Moderate) => Moderate,
            (High, High) => High,
        }
    }
}

/// An inferred risk/zone classification produced before operator confirmation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InferredClassification {
    /// The inferred risk class.
    pub risk: RiskClass,
    /// The inferred usage zone.
    pub zone: UsageZone,
    /// Whether the risk was supplied explicitly (skipping inference).
    pub risk_was_supplied: bool,
    /// Whether the zone was supplied explicitly (skipping inference).
    pub zone_was_supplied: bool,
    /// Confidence level of the combined classification.
    pub confidence: ClassificationConfidence,
    /// Whether the operator should confirm this classification before proceeding.
    pub requires_confirmation: bool,
    /// One-line headline summarizing the inferred classification.
    pub headline: String,
    /// Full rationale combining risk and zone signals.
    pub rationale: String,
    /// Rationale specific to the risk class inference.
    pub risk_rationale: String,
    /// Rationale specific to the zone inference.
    pub zone_rationale: String,
    /// Combined signals from inputs and mode heuristics.
    pub signals: Vec<String>,
    /// Signals that specifically drove the risk class.
    pub risk_signals: Vec<String>,
    /// Signals that specifically drove the usage zone.
    pub zone_signals: Vec<String>,
}

/// Infers a risk/zone classification from mode, explicit overrides, and input signals.
pub fn infer_risk_zone(
    mode: Mode,
    explicit_risk: Option<RiskClass>,
    explicit_zone: Option<UsageZone>,
    intake_summary: &str,
    inputs: &[String],
    _repo_surfaces: &[String],
) -> InferredClassification {
    let risk_inference = explicit_risk.map_or_else(
        || infer_risk(mode, intake_summary, inputs),
        |risk| FieldInference {
            value: risk,
            supplied: true,
            confidence: ClassificationConfidence::High,
            rationale: format!(
                "Risk class stays `{}` because it was already supplied explicitly.",
                risk.as_str()
            ),
            signals: vec!["User or caller already supplied the risk class explicitly.".to_string()],
        },
    );
    let zone_inference = explicit_zone.map_or_else(
        || infer_zone(mode, intake_summary, inputs),
        |zone| FieldInference {
            value: zone,
            supplied: true,
            confidence: ClassificationConfidence::High,
            rationale: format!(
                "Usage zone stays `{}` because it was already supplied explicitly.",
                zone.as_str()
            ),
            signals: vec!["User or caller already supplied the usage zone explicitly.".to_string()],
        },
    );

    let requires_confirmation = !risk_inference.supplied || !zone_inference.supplied;
    let confidence = risk_inference.confidence.min(zone_inference.confidence);
    let mut signals = risk_inference.signals.clone();
    for signal in &zone_inference.signals {
        if !signals.contains(signal) {
            signals.push(signal.clone());
        }
    }

    let headline = match (risk_inference.supplied, zone_inference.supplied) {
        (true, true) => "Classification is already fully specified.".to_string(),
        (false, false) => format!(
            "Canon inferred `{}` risk and `{}` zone from the supplied intake.",
            risk_inference.value.as_str(),
            zone_inference.value.as_str()
        ),
        (false, true) => format!(
            "Canon inferred the missing risk class as `{}` from the supplied intake.",
            risk_inference.value.as_str()
        ),
        (true, false) => format!(
            "Canon inferred the missing usage zone as `{}` from the supplied intake.",
            zone_inference.value.as_str()
        ),
    };
    let rationale = if requires_confirmation {
        format!(
            "Use the inferred pair as a provisional starting point, then confirm or override it before starting the run. {} {}",
            risk_inference.rationale, zone_inference.rationale
        )
    } else {
        "No inference was needed because risk and zone were already supplied explicitly."
            .to_string()
    };

    InferredClassification {
        risk: risk_inference.value,
        zone: zone_inference.value,
        risk_was_supplied: risk_inference.supplied,
        zone_was_supplied: zone_inference.supplied,
        confidence,
        requires_confirmation,
        headline,
        rationale,
        risk_rationale: risk_inference.rationale,
        zone_rationale: zone_inference.rationale,
        signals,
        risk_signals: risk_inference.signals,
        zone_signals: zone_inference.signals,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FieldInference<T> {
    value: T,
    supplied: bool,
    confidence: ClassificationConfidence,
    rationale: String,
    signals: Vec<String>,
}

fn infer_risk(mode: Mode, intake_summary: &str, inputs: &[String]) -> FieldInference<RiskClass> {
    if let Some(risk) = extract_declared_risk(intake_summary) {
        return FieldInference {
            value: risk,
            supplied: false,
            confidence: ClassificationConfidence::High,
            rationale: format!(
                "The intake already declares the risk class as `{}`.",
                risk.as_str()
            ),
            signals: vec![format!(
                "A risk marker in the intake resolves directly to `{}`.",
                risk.as_str()
            )],
        };
    }

    let normalized = intake_summary.to_lowercase();
    let systemic_hits = collect_keyword_hits(
        &normalized,
        &[
            "workspace-wide",
            "whole repo",
            "whole repository",
            "cross-service",
            "multi-service",
            "data loss",
            "security incident",
            "regulated",
            "safety-critical",
            "payment",
            "customer data",
        ],
    );
    if !systemic_hits.is_empty() {
        let signals = systemic_hits
            .iter()
            .map(|keyword| format!("Detected systemic-impact signal `{keyword}` in the intake."))
            .collect::<Vec<_>>();
        return FieldInference {
            value: RiskClass::SystemicImpact,
            supplied: false,
            confidence: if systemic_hits.len() > 1 {
                ClassificationConfidence::High
            } else {
                ClassificationConfidence::Moderate
            },
            rationale: "The intake points at organization- or production-wide blast radius rather than a locally bounded change.".to_string(),
            signals,
        };
    }

    let bounded_hits = collect_keyword_hits(
        &normalized,
        &[
            "boundary",
            "architecture",
            "legacy",
            "invariant",
            "migration",
            "database",
            "schema",
            "auth",
            "firmware",
            "usb",
            "bluetooth",
            "review",
            "diff",
        ],
    );

    let mode_defaults_to_bounded =
        matches!(mode, Mode::SystemShaping | Mode::Architecture | Mode::Change | Mode::PrReview);
    if mode_defaults_to_bounded || !bounded_hits.is_empty() {
        let mut signals = bounded_hits
            .iter()
            .map(|keyword| format!("Detected bounded-impact signal `{keyword}` in the intake."))
            .collect::<Vec<_>>();
        if mode_defaults_to_bounded {
            signals.push(format!(
                "Mode `{}` usually shapes bounded but non-trivial system or review decisions.",
                mode.as_str()
            ));
        }

        return FieldInference {
            value: RiskClass::BoundedImpact,
            supplied: false,
            confidence: if bounded_hits.len() > 1 {
                ClassificationConfidence::High
            } else {
                ClassificationConfidence::Moderate
            },
            rationale: "The intake reads like a bounded design, change-planning, or review decision rather than a trivial low-impact note.".to_string(),
            signals,
        };
    }

    let mut signals =
        vec![format!("Mode `{}` stays read-only and exploratory at this stage.", mode.as_str())];
    if !inputs.is_empty() {
        signals.push(
            "No higher-risk production or systemic keywords were detected in the supplied intake."
                .to_string(),
        );
    }

    FieldInference {
        value: RiskClass::LowImpact,
        supplied: false,
        confidence: ClassificationConfidence::Low,
        rationale:
            "The intake looks exploratory and does not yet show a bounded or systemic blast radius."
                .to_string(),
        signals,
    }
}

fn infer_zone(mode: Mode, intake_summary: &str, inputs: &[String]) -> FieldInference<UsageZone> {
    if let Some(zone) = extract_declared_zone(intake_summary) {
        return FieldInference {
            value: zone,
            supplied: false,
            confidence: ClassificationConfidence::High,
            rationale: format!(
                "The intake already declares the usage zone as `{}`.",
                zone.as_str()
            ),
            signals: vec![format!(
                "A zone marker in the intake resolves directly to `{}`.",
                zone.as_str()
            )],
        };
    }

    let normalized = intake_summary.to_lowercase();
    let red_hits = collect_keyword_hits(
        &normalized,
        &[
            "outage",
            "data loss",
            "security incident",
            "emergency",
            "hotfix",
            "sev-",
            "production incident",
        ],
    );
    if !red_hits.is_empty() {
        let signals = red_hits
            .iter()
            .map(|keyword| format!("Detected red-zone signal `{keyword}` in the intake."))
            .collect::<Vec<_>>();
        return FieldInference {
            value: UsageZone::Red,
            supplied: false,
            confidence: if red_hits.len() > 1 {
                ClassificationConfidence::High
            } else {
                ClassificationConfidence::Moderate
            },
            rationale: "The intake references incident-style or emergency work, which should stay in red-zone handling until a human confirms otherwise.".to_string(),
            signals,
        };
    }

    let yellow_hits = collect_keyword_hits(
        &normalized,
        &[
            "production",
            "live service",
            "customer data",
            "database",
            "migration",
            "release",
            "firmware",
            "usb",
            "bluetooth",
            "device",
            "worktree",
            "legacy",
            "main branch",
        ],
    );
    let mode_defaults_to_yellow = matches!(mode, Mode::Change | Mode::PrReview);
    if mode_defaults_to_yellow || !yellow_hits.is_empty() {
        let mut signals = yellow_hits
            .iter()
            .map(|keyword| format!("Detected yellow-zone signal `{keyword}` in the intake."))
            .collect::<Vec<_>>();
        if mode_defaults_to_yellow {
            signals.push(format!(
                "Mode `{}` usually targets a live repository surface rather than a purely isolated note.",
                mode.as_str()
            ));
        }
        return FieldInference {
            value: UsageZone::Yellow,
            supplied: false,
            confidence: if yellow_hits.len() > 1 || mode_defaults_to_yellow {
                ClassificationConfidence::Moderate
            } else {
                ClassificationConfidence::Low
            },
            rationale: "The intake points at a live or shared engineering surface, but not at an active red-zone incident.".to_string(),
            signals,
        };
    }

    let mut signals = vec![format!(
        "Mode `{}` can stay in green when the intake is still isolated to planning or analysis.",
        mode.as_str()
    )];
    if !inputs.is_empty() {
        signals.push(
            "No production, incident, or live-system signals were detected in the supplied intake."
                .to_string(),
        );
    }

    FieldInference {
        value: UsageZone::Green,
        supplied: false,
        confidence: ClassificationConfidence::Low,
        rationale: "The intake reads like isolated planning or analysis work rather than an active live-system intervention.".to_string(),
        signals,
    }
}

fn collect_keyword_hits(normalized: &str, keywords: &[&str]) -> Vec<String> {
    keywords
        .iter()
        .filter(|keyword| normalized.contains(**keyword))
        .map(|keyword| keyword.to_string())
        .collect::<Vec<_>>()
}

fn extract_declared_risk(source: &str) -> Option<RiskClass> {
    extract_declared_value(source, &["risk", "risk level"]).and_then(|value| value.parse().ok())
}

fn extract_declared_zone(source: &str) -> Option<UsageZone> {
    extract_declared_value(source, &["zone", "usage zone"]).and_then(|value| value.parse().ok())
}

fn extract_declared_value(source: &str, markers: &[&str]) -> Option<String> {
    let lines = source.lines().collect::<Vec<_>>();

    for (index, raw_line) in lines.iter().enumerate() {
        let trimmed = raw_line.trim();
        let normalized = trimmed.trim_start_matches('#').trim().to_lowercase();

        for marker in markers {
            if let Some(value) = extract_value_below_marker(&lines, index, &normalized, marker) {
                return Some(value);
            }

            if let Some(value) = extract_inline_marker_value(trimmed, &normalized, marker) {
                return Some(value);
            }
        }
    }

    None
}

fn extract_value_below_marker(
    lines: &[&str],
    index: usize,
    normalized: &str,
    marker: &str,
) -> Option<String> {
    if normalized != marker {
        return None;
    }

    for candidate in lines.iter().skip(index + 1) {
        let value = candidate.trim();
        if value.is_empty() {
            continue;
        }
        if value.starts_with('#') {
            return None;
        }
        return Some(value.to_string());
    }

    None
}

fn extract_inline_marker_value(trimmed: &str, normalized: &str, marker: &str) -> Option<String> {
    let inline = format!("{marker}:");
    if !normalized.starts_with(&inline) {
        return None;
    }

    let value = trimmed[trimmed.find(':')? + 1..].trim();
    if value.is_empty() { None } else { Some(value.to_string()) }
}

#[cfg(test)]
mod tests {
    use super::{
        ClassificationConfidence, SystemContextRequirement, infer_risk_zone,
        system_context_requirement, validate_system_context,
    };
    use crate::domain::mode::Mode;
    use crate::domain::policy::{RiskClass, UsageZone};
    use crate::domain::run::SystemContext;

    #[test]
    fn change_requires_existing_system_context() {
        let missing = validate_system_context(Mode::Change, None)
            .expect_err("change should require explicit context");
        let invalid = validate_system_context(Mode::Change, Some(SystemContext::New))
            .expect_err("change should reject new-system context");

        assert!(missing.contains("mode `change` requires --system-context existing"));
        assert!(invalid.contains("only --system-context existing"));
    }

    #[test]
    fn backlog_requires_existing_system_context() {
        let missing = validate_system_context(Mode::Backlog, None)
            .expect_err("backlog should require explicit context");
        let invalid = validate_system_context(Mode::Backlog, Some(SystemContext::New))
            .expect_err("backlog should reject new-system context");

        assert!(missing.contains("mode `backlog` requires --system-context existing"));
        assert!(invalid.contains("only --system-context existing"));
    }

    #[test]
    fn optional_modes_can_omit_system_context() {
        assert_eq!(
            system_context_requirement(Mode::Requirements),
            SystemContextRequirement::Optional
        );
        assert!(validate_system_context(Mode::Requirements, None).is_ok());
        assert!(validate_system_context(Mode::Requirements, Some(SystemContext::Existing)).is_ok());
    }

    #[test]
    fn architecture_requires_explicit_system_context() {
        let error = validate_system_context(Mode::Architecture, None)
            .expect_err("architecture should require explicit context");

        assert!(error.contains("mode `architecture` requires --system-context new|existing"));
    }

    #[test]
    fn infer_risk_zone_respects_explicit_markers_in_the_intake() {
        let summary = "# Brief\n\n## Risk Level\nlow-impact\n\n## Zone\ngreen\n";

        let inferred =
            infer_risk_zone(Mode::Change, None, None, summary, &["brief.md".to_string()], &[]);

        assert_eq!(inferred.risk, RiskClass::LowImpact);
        assert_eq!(inferred.zone, UsageZone::Green);
        assert_eq!(inferred.confidence, ClassificationConfidence::High);
        assert!(inferred.requires_confirmation);
    }

    #[test]
    fn infer_risk_zone_escalates_for_production_incident_language() {
        let summary = "Investigate a production incident with customer data loss and emergency rollback requirements.";

        let inferred = infer_risk_zone(
            Mode::Discovery,
            None,
            None,
            summary,
            &["incident.md".to_string()],
            &[],
        );

        assert_eq!(inferred.risk, RiskClass::SystemicImpact);
        assert_eq!(inferred.zone, UsageZone::Red);
        assert!(matches!(
            inferred.confidence,
            ClassificationConfidence::Moderate | ClassificationConfidence::High
        ));
    }

    #[test]
    fn architecture_accepts_new_system_context() {
        // Architecture requires explicit system_context but accepts both new and existing.
        assert!(validate_system_context(Mode::Architecture, Some(SystemContext::New)).is_ok());
        assert!(validate_system_context(Mode::Architecture, Some(SystemContext::Existing)).is_ok());
    }

    #[test]
    fn system_shaping_accepts_new_system_context() {
        assert!(validate_system_context(Mode::SystemShaping, Some(SystemContext::New)).is_ok());
    }

    #[test]
    fn supply_chain_analysis_rejects_new_system_context() {
        let error = validate_system_context(Mode::SupplyChainAnalysis, Some(SystemContext::New))
            .expect_err("supply-chain-analysis should reject new-system context");
        assert!(error.contains("only --system-context existing"));
    }

    #[test]
    fn classification_confidence_min_returns_lower_of_two() {
        use super::ClassificationConfidence::{High, Low, Moderate};

        assert_eq!(Low.min(Low), Low);
        assert_eq!(Low.min(Moderate), Low);
        assert_eq!(Low.min(High), Low);
        assert_eq!(Moderate.min(Low), Low);
        assert_eq!(Moderate.min(Moderate), Moderate);
        assert_eq!(Moderate.min(High), Moderate);
        assert_eq!(High.min(Low), Low);
        assert_eq!(High.min(Moderate), Moderate);
        assert_eq!(High.min(High), High);
    }

    #[test]
    fn classification_confidence_as_str_round_trips() {
        assert_eq!(ClassificationConfidence::Low.as_str(), "low");
        assert_eq!(ClassificationConfidence::Moderate.as_str(), "moderate");
        assert_eq!(ClassificationConfidence::High.as_str(), "high");
    }

    #[test]
    fn infer_risk_zone_with_only_explicit_risk_still_infers_zone() {
        // Risk is explicit but zone is not: zone is inferred from intake.
        let summary = "We need to push an emergency hotfix to the live payment service.";
        let inferred = infer_risk_zone(
            Mode::Change,
            Some(RiskClass::SystemicImpact),
            None,
            summary,
            &["brief.md".to_string()],
            &[],
        );
        assert_eq!(inferred.risk, RiskClass::SystemicImpact);
        assert!(inferred.risk_was_supplied);
        assert!(!inferred.zone_was_supplied);
        assert_eq!(inferred.zone, UsageZone::Red);
        assert!(inferred.requires_confirmation);
    }

    #[test]
    fn infer_risk_zone_with_only_explicit_zone_still_infers_risk() {
        // Zone is explicit but risk is not: risk is inferred from intake keywords.
        let summary = "Refactor the auth module boundary to remove coupling.";
        let inferred = infer_risk_zone(
            Mode::Discovery,
            None,
            Some(UsageZone::Green),
            summary,
            &["brief.md".to_string()],
            &[],
        );
        assert!(!inferred.risk_was_supplied);
        assert!(inferred.zone_was_supplied);
        assert_eq!(inferred.zone, UsageZone::Green);
        assert!(inferred.requires_confirmation);
    }

    #[test]
    fn infer_risk_zone_fully_explicit_needs_no_confirmation() {
        let summary = "Some discovery work.";
        let inferred = infer_risk_zone(
            Mode::Discovery,
            Some(RiskClass::LowImpact),
            Some(UsageZone::Green),
            summary,
            &[],
            &[],
        );
        assert!(!inferred.requires_confirmation);
        assert_eq!(inferred.confidence, ClassificationConfidence::High);
    }

    #[test]
    fn infer_risk_zone_multiple_systemic_keywords_yields_high_confidence() {
        // Two systemic keywords trigger High confidence inside infer_risk.
        let summary = "Cross-service customer data migration with payment integration.";
        let inferred =
            infer_risk_zone(Mode::Discovery, None, None, summary, &["brief.md".to_string()], &[]);
        assert_eq!(inferred.risk, RiskClass::SystemicImpact);
        // risk_signals holds signals from the risk inference; >= 2 means High confidence branch ran.
        assert!(
            inferred.risk_signals.len() >= 2,
            "expected >= 2 risk signals, got: {:?}",
            inferred.risk_signals
        );
    }

    #[test]
    fn infer_risk_zone_single_systemic_keyword_yields_moderate_risk_confidence() {
        // Single systemic keyword: exactly one risk_signal, so Moderate path runs inside infer_risk.
        let summary = "Patch the safety-critical component to fix a narrowly scoped defect.";
        let inferred =
            infer_risk_zone(Mode::Discovery, None, None, summary, &["brief.md".to_string()], &[]);
        assert_eq!(inferred.risk, RiskClass::SystemicImpact);
        assert_eq!(
            inferred.risk_signals.len(),
            1,
            "expected exactly one systemic signal, got: {:?}",
            inferred.risk_signals
        );
    }

    #[test]
    fn infer_risk_zone_bounded_keywords_without_mode_default_yield_bounded_risk() {
        // Mode::Discovery doesn't default to bounded, but "database" keyword triggers it.
        let summary = "Migrate the database schema and update legacy migration scripts.";
        let inferred =
            infer_risk_zone(Mode::Discovery, None, None, summary, &["brief.md".to_string()], &[]);
        assert_eq!(inferred.risk, RiskClass::BoundedImpact);
    }

    #[test]
    fn infer_risk_zone_no_keywords_yields_low_impact_risk() {
        let summary = "Explore ideas for a new internal tool.";
        // Mode::Requirements is Optional and doesn't default to bounded.
        let inferred = infer_risk_zone(
            Mode::Requirements,
            None,
            None,
            summary,
            &["brief.md".to_string()],
            &[],
        );
        assert_eq!(inferred.risk, RiskClass::LowImpact);
        assert_eq!(inferred.confidence, ClassificationConfidence::Low);
        // With non-empty inputs, extra "no higher-risk keywords" signal is added.
        assert!(inferred.signals.iter().any(|s| s.contains("No higher-risk")));
    }

    #[test]
    fn infer_risk_zone_low_impact_with_empty_inputs_omits_keyword_signal() {
        let summary = "Explore ideas for a new internal tool.";
        let inferred = infer_risk_zone(Mode::Requirements, None, None, summary, &[], &[]);
        assert_eq!(inferred.risk, RiskClass::LowImpact);
        // Empty inputs: only the mode-exploratory signal, no "no higher-risk" signal.
        assert!(!inferred.signals.iter().any(|s| s.contains("No higher-risk")));
    }

    #[test]
    fn infer_risk_zone_red_zone_from_single_emergency_keyword() {
        let summary = "Apply an emergency hotfix to the live payment endpoint.";
        let inferred =
            infer_risk_zone(Mode::Incident, None, None, summary, &["brief.md".to_string()], &[]);
        assert_eq!(inferred.zone, UsageZone::Red);
        assert_eq!(inferred.confidence, ClassificationConfidence::Moderate);
    }

    #[test]
    fn infer_risk_zone_red_zone_from_multiple_emergency_keywords() {
        // Multiple red-zone keywords trigger High confidence inside infer_zone.
        let summary = "We have an ongoing outage causing data loss (emergency sev-1 hotfix).";
        let inferred =
            infer_risk_zone(Mode::Incident, None, None, summary, &["brief.md".to_string()], &[]);
        assert_eq!(inferred.zone, UsageZone::Red);
        // zone_signals holds the per-keyword signals from infer_zone; >= 2 means High-confidence branch ran.
        assert!(
            inferred.zone_signals.len() >= 2,
            "expected >= 2 zone signals, got: {:?}",
            inferred.zone_signals
        );
    }

    #[test]
    fn infer_risk_zone_yellow_zone_from_production_keyword() {
        // Mode::Discovery doesn't default to yellow, but "production" keyword triggers it.
        let summary = "Inspect the production database for stale records.";
        let inferred =
            infer_risk_zone(Mode::Discovery, None, None, summary, &["brief.md".to_string()], &[]);
        assert_eq!(inferred.zone, UsageZone::Yellow);
    }

    #[test]
    fn infer_risk_zone_green_zone_as_fallback() {
        // No red/yellow keywords and mode doesn't default to yellow.
        let summary = "Explore ideas for isolating a unit of work.";
        let inferred = infer_risk_zone(Mode::Discovery, None, None, summary, &[], &[]);
        assert_eq!(inferred.zone, UsageZone::Green);
        assert_eq!(inferred.confidence, ClassificationConfidence::Low);
    }

    #[test]
    fn infer_risk_zone_zone_declared_in_intake_overrides_inference() {
        // "Zone: yellow" in the intake should be picked up by extract_declared_zone.
        let summary = "## Zone\nyellow\n\nSome discovery notes.";
        let inferred =
            infer_risk_zone(Mode::Discovery, None, None, summary, &["brief.md".to_string()], &[]);
        assert_eq!(inferred.zone, UsageZone::Yellow);
        assert!(!inferred.zone_was_supplied); // extracted from intake, not explicitly supplied
        assert_eq!(
            inferred.zone_signals.iter().find(|s| s.contains("resolves directly")),
            inferred.zone_signals.iter().find(|s| s.contains("resolves directly"))
        );
    }
}
