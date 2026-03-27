use crate::domain::artifact::ArtifactContract;
use crate::domain::mode::Mode;
use crate::domain::policy::{PolicySet, RiskClass, UsageZone};

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
