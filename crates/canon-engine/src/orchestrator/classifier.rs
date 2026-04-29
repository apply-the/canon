use crate::domain::artifact::ArtifactContract;
use crate::domain::mode::Mode;
use crate::domain::policy::{PolicySet, RiskClass, UsageZone};
use crate::domain::run::SystemContext;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemContextRequirement {
    Required,
    Optional,
}

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
        | Mode::SecurityAssessment
        | Mode::SupplyChainAnalysis => SystemContextRequirement::Required,
        Mode::Discovery
        | Mode::Requirements
        | Mode::Review
        | Mode::Verification
        | Mode::PrReview => SystemContextRequirement::Optional,
    }
}

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
                Mode::Change | Mode::Backlog | Mode::SecurityAssessment | Mode::SupplyChainAnalysis
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
        Mode::Change | Mode::Backlog | Mode::SecurityAssessment | Mode::SupplyChainAnalysis => {
            "existing"
        }
        _ => "new|existing",
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MutationPolicy {
    Execute,
    RecommendationOnly,
}

pub fn allow_mutation(policy_set: &PolicySet, risk: RiskClass, zone: UsageZone) -> bool {
    policy_set.allow_mutation(risk, zone)
}

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

pub fn apply_verification_layers(
    policy_set: &PolicySet,
    risk: RiskClass,
    contract: &mut ArtifactContract,
) {
    contract.required_verification_layers = policy_set.verification_layers_for(risk);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClassificationConfidence {
    Low,
    Moderate,
    High,
}

impl ClassificationConfidence {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InferredClassification {
    pub risk: RiskClass,
    pub zone: UsageZone,
    pub risk_was_supplied: bool,
    pub zone_was_supplied: bool,
    pub confidence: ClassificationConfidence,
    pub requires_confirmation: bool,
    pub headline: String,
    pub rationale: String,
    pub risk_rationale: String,
    pub zone_rationale: String,
    pub signals: Vec<String>,
    pub risk_signals: Vec<String>,
    pub zone_signals: Vec<String>,
}

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
            if normalized == *marker {
                for candidate in lines.iter().skip(index + 1) {
                    let value = candidate.trim();
                    if value.is_empty() {
                        continue;
                    }
                    if value.starts_with('#') {
                        break;
                    }
                    return Some(value.to_string());
                }
            }

            let inline = format!("{marker}:");
            if normalized.starts_with(&inline) {
                let value = trimmed[trimmed.find(':')? + 1..].trim();
                if !value.is_empty() {
                    return Some(value.to_string());
                }
            }
        }
    }

    None
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
}
