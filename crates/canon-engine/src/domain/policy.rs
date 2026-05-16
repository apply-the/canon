use serde::{Deserialize, Serialize};
use strum_macros::{Display, IntoStaticStr};

use canon_adapters::{AdapterKind, CapabilityKind};

use crate::domain::execution::{InvocationConstraintSet, PayloadRetentionLevel};
use crate::domain::gate::GateKind;
use crate::domain::verification::VerificationLayer;

/// The risk profile of a Canon run, controlling mutation rights and required oversight.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, IntoStaticStr)]
#[strum(serialize_all = "kebab-case")]
pub enum RiskClass {
    /// Narrow, self-contained changes with no cross-system effects.
    LowImpact,
    /// Changes with clear blast radius that require explicit scope control.
    BoundedImpact,
    /// Cross-cutting changes with potential system-wide effects; require human ownership.
    SystemicImpact,
}

impl RiskClass {
    /// Returns the kebab-case string representation of this risk class.
    pub fn as_str(self) -> &'static str {
        self.into()
    }
}

impl std::str::FromStr for RiskClass {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "low-impact" | "LowImpact" => Ok(Self::LowImpact),
            "bounded-impact" | "BoundedImpact" => Ok(Self::BoundedImpact),
            "systemic-impact" | "SystemicImpact" => Ok(Self::SystemicImpact),
            other => Err(format!("unsupported risk class: {other}")),
        }
    }
}

/// The operational zone of a Canon run, controlling whether mutations are applied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, IntoStaticStr)]
#[strum(serialize_all = "lowercase")]
pub enum UsageZone {
    /// Full mutation is permitted; the run is operating safely.
    Green,
    /// Mutation is constrained; human review is required before changes land.
    Yellow,
    /// No mutation; the run produces recommendations only.
    Red,
}

impl UsageZone {
    /// Returns the lowercase string representation of this usage zone.
    pub fn as_str(self) -> &'static str {
        self.into()
    }
}

impl std::str::FromStr for UsageZone {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "green" | "Green" => Ok(Self::Green),
            "yellow" | "Yellow" => Ok(Self::Yellow),
            "red" | "Red" => Ok(Self::Red),
            other => Err(format!("unsupported usage zone: {other}")),
        }
    }
}

/// Cross-repo authority posture exported through `authority-governance-v1`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, IntoStaticStr)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum AuthorityZone {
    /// Full delivery posture is permitted without an additional human gate.
    Green,
    /// Delivery posture is constrained and downstream systems should elevate review.
    Yellow,
    /// Delivery posture is recommendation-only.
    Red,
    /// Delivery posture requires an unresolved human gate before progressing.
    Restricted,
}

impl AuthorityZone {
    /// Returns the lowercase string representation of this authority zone.
    pub fn as_str(self) -> &'static str {
        self.into()
    }
}

impl std::str::FromStr for AuthorityZone {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "green" | "Green" => Ok(Self::Green),
            "yellow" | "Yellow" => Ok(Self::Yellow),
            "red" | "Red" => Ok(Self::Red),
            "restricted" | "Restricted" => Ok(Self::Restricted),
            other => Err(format!("unsupported authority zone: {other}")),
        }
    }
}

/// Cross-repo change-impact classification exported through `authority-governance-v1`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, IntoStaticStr)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum ChangeClass {
    /// Narrow, self-contained change.
    LowImpact,
    /// Bounded but meaningful delivery change.
    BoundedImpact,
    /// Cross-cutting change with systemic risk.
    SystemicImpact,
    /// Operationally critical change that requires the highest delivery posture.
    CriticalOperations,
}

impl ChangeClass {
    /// Returns the kebab-case string representation of this change class.
    pub fn as_str(self) -> &'static str {
        self.into()
    }

    /// Derives a first-slice change class from existing Canon risk and mode vocabulary.
    pub fn from_risk_and_mode(risk: RiskClass, mode: crate::domain::mode::Mode) -> Self {
        match mode {
            crate::domain::mode::Mode::Incident
            | crate::domain::mode::Mode::Migration
            | crate::domain::mode::Mode::SecurityAssessment => Self::CriticalOperations,
            _ => match risk {
                RiskClass::LowImpact => Self::LowImpact,
                RiskClass::BoundedImpact => Self::BoundedImpact,
                RiskClass::SystemicImpact => Self::SystemicImpact,
            },
        }
    }
}

impl std::str::FromStr for ChangeClass {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "low-impact" | "LowImpact" => Ok(Self::LowImpact),
            "bounded-impact" | "BoundedImpact" => Ok(Self::BoundedImpact),
            "systemic-impact" | "SystemicImpact" => Ok(Self::SystemicImpact),
            "critical-operations" | "CriticalOperations" => Ok(Self::CriticalOperations),
            other => Err(format!("unsupported change class: {other}")),
        }
    }
}

/// Policy configuration for a specific risk class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RiskPolicyClass {
    /// The risk class this policy applies to.
    pub name: RiskClass,
    /// Whether a named owner must be declared for runs of this risk class.
    pub requires_owner: bool,
    /// Whether mutating adapter capabilities are permitted.
    pub mutable_execution: bool,
    /// The verification layers that must be completed before a run can close.
    pub verification_layers: Vec<VerificationLayer>,
}

/// Policy configuration for a specific usage zone.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ZonePolicy {
    /// The usage zone this policy applies to.
    pub name: UsageZone,
    /// Whether mutating adapter capabilities are permitted in this zone.
    pub mutable_execution: bool,
}

/// The global gate policy: which gates are mandatory for all runs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GatePolicy {
    /// Gates that must pass before any run can be marked complete.
    pub mandatory_gates: Vec<GateKind>,
}

/// Maps a specific adapter kind to its permitted capability kinds.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdapterPolicyMatrix {
    /// The adapter kind this entry applies to.
    #[serde(alias = "kind")]
    pub adapter: AdapterKind,
    /// The capability kinds permitted for this adapter.
    pub capabilities: Vec<CapabilityKind>,
}

/// A named constraint profile that caps payload retention and mutation rights.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InvocationConstraintProfile {
    /// Unique identifier for this profile.
    pub id: String,
    /// The payload retention level applied to invocations using this profile.
    pub payload_retention: PayloadRetentionLevel,
    /// Maximum payload bytes permitted, if limited.
    pub max_payload_bytes: Option<u64>,
    /// Optional command profile identifier for shell adapter constraints.
    pub command_profile: Option<String>,
    /// Whether invocations under this profile are limited to recommendations.
    pub recommendation_only: bool,
    /// Whether workspace patch application is disabled.
    pub patch_disabled: bool,
}

/// Policy that controls whether AI generation and validation paths must be independent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ValidationIndependencePolicy {
    /// Whether AI-generated content requires a distinct validation step.
    pub ai_generation_requires_distinct_validation: bool,
    /// Whether a human review counts as an independent validation path.
    pub human_review_counts_independent: bool,
}

/// The complete governance policy configuration for a Canon runtime instance.
///
/// Loaded from YAML at startup; the orchestrator consults this set to make
/// all policy decisions during run execution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicySet {
    /// Per-risk-class mutation and verification requirements.
    pub risk_classes: Vec<RiskPolicyClass>,
    /// Per-zone mutation permissions.
    pub zones: Vec<ZonePolicy>,
    /// Global mandatory gate configuration.
    pub gate_policy: GatePolicy,
    /// Per-adapter capability allow-lists.
    pub adapter_matrix: Vec<AdapterPolicyMatrix>,
    /// Named constraint profiles referenced by invocation policy decisions.
    pub constraint_profiles: Vec<InvocationConstraintProfile>,
    /// Adapters that are unconditionally disabled in this runtime instance.
    pub runtime_disabled_adapters: Vec<AdapterKind>,
    /// Validation independence requirements for AI-generated content.
    pub validation_independence: ValidationIndependencePolicy,
    /// When `true`, mutation is blocked for `SystemicImpact` risk or `Red` zone.
    pub block_mutation_for_red_or_systemic: bool,
}

/// Partial overrides applied on top of a base [`PolicySet`] during testing or context loading.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PolicySetOverrides {
    /// Risk-class overrides; empty means no override.
    pub risk_classes: Vec<RiskPolicyClass>,
    /// Zone overrides; empty means no override.
    pub zones: Vec<ZonePolicy>,
    /// Gate policy override; `None` means no override.
    pub gate_policy: Option<GatePolicy>,
    /// Adapter matrix overrides; empty means no override.
    pub adapter_matrix: Vec<AdapterPolicyMatrix>,
    /// Constraint profile overrides; empty means no override.
    pub constraint_profiles: Vec<InvocationConstraintProfile>,
    /// Runtime disabled adapters override; `None` means no override.
    pub runtime_disabled_adapters: Option<Vec<AdapterKind>>,
    /// Validation independence override; `None` means no override.
    pub validation_independence: Option<ValidationIndependencePolicy>,
    /// Override for the `block_mutation_for_red_or_systemic` flag.
    pub block_mutation_for_red_or_systemic: Option<bool>,
}

impl PolicySet {
    /// Returns `true` if mutation is permitted for the given risk class and usage zone.
    pub fn allow_mutation(&self, risk: RiskClass, zone: UsageZone) -> bool {
        if self.block_mutation_for_red_or_systemic
            && (matches!(risk, RiskClass::SystemicImpact) || matches!(zone, UsageZone::Red))
        {
            return false;
        }

        let risk_allowed = self
            .risk_classes
            .iter()
            .find(|class| class.name == risk)
            .map(|class| class.mutable_execution)
            .unwrap_or(true);

        let zone_allowed = self
            .zones
            .iter()
            .find(|entry| entry.name == zone)
            .map(|entry| entry.mutable_execution)
            .unwrap_or(true);

        risk_allowed && zone_allowed
    }

    /// Returns the required verification layers for the given risk class.
    pub fn verification_layers_for(&self, risk: RiskClass) -> Vec<VerificationLayer> {
        self.risk_classes
            .iter()
            .find(|class| class.name == risk)
            .map(|class| class.verification_layers.clone())
            .unwrap_or_default()
    }

    /// Returns `true` if the given adapter is not in the runtime-disabled list.
    pub fn runtime_adapter_enabled(&self, adapter: AdapterKind) -> bool {
        !self.runtime_disabled_adapters.contains(&adapter)
    }

    /// Returns `true` if the given adapter is permitted to exercise the given capability.
    pub fn adapter_supports(&self, adapter: AdapterKind, capability: CapabilityKind) -> bool {
        self.adapter_matrix
            .iter()
            .find(|entry| entry.adapter == adapter)
            .map(|entry| entry.capabilities.contains(&capability))
            .unwrap_or(false)
    }

    /// Looks up a constraint profile by ID and returns it as an [`InvocationConstraintSet`], or `None`.
    pub fn constraint_profile(&self, id: &str) -> Option<InvocationConstraintSet> {
        self.constraint_profiles.iter().find(|profile| profile.id == id).map(|profile| {
            InvocationConstraintSet {
                allowed_paths: Vec::new(),
                command_profile: profile.command_profile.clone(),
                max_payload_bytes: profile.max_payload_bytes,
                recommendation_only: profile.recommendation_only,
                patch_disabled: profile.patch_disabled,
                payload_retention: Some(profile.payload_retention),
            }
        })
    }

    /// Like [`constraint_profile`](Self::constraint_profile) but also sets `allowed_paths`.
    pub fn constraint_profile_with_allowed_paths(
        &self,
        id: &str,
        allowed_paths: Vec<String>,
    ) -> Option<InvocationConstraintSet> {
        self.constraint_profile(id).map(|mut constraints| {
            constraints.allowed_paths = allowed_paths;
            constraints
        })
    }

    /// Applies partial overrides from a [`PolicySetOverrides`] onto this policy set.
    pub fn apply_overrides(&mut self, overrides: PolicySetOverrides) {
        for override_class in overrides.risk_classes {
            if let Some(existing) =
                self.risk_classes.iter_mut().find(|class| class.name == override_class.name)
            {
                *existing = override_class;
            } else {
                self.risk_classes.push(override_class);
            }
        }

        for override_zone in overrides.zones {
            if let Some(existing) =
                self.zones.iter_mut().find(|zone| zone.name == override_zone.name)
            {
                *existing = override_zone;
            } else {
                self.zones.push(override_zone);
            }
        }

        if let Some(gate_policy) = overrides.gate_policy {
            self.gate_policy = gate_policy;
        }

        for override_adapter in overrides.adapter_matrix {
            if let Some(existing) = self
                .adapter_matrix
                .iter_mut()
                .find(|entry| entry.adapter == override_adapter.adapter)
            {
                *existing = override_adapter;
            } else {
                self.adapter_matrix.push(override_adapter);
            }
        }

        for profile in overrides.constraint_profiles {
            if let Some(existing) =
                self.constraint_profiles.iter_mut().find(|entry| entry.id == profile.id)
            {
                *existing = profile;
            } else {
                self.constraint_profiles.push(profile);
            }
        }

        if let Some(runtime_disabled_adapters) = overrides.runtime_disabled_adapters {
            self.runtime_disabled_adapters = runtime_disabled_adapters;
        }

        if let Some(validation_independence) = overrides.validation_independence {
            self.validation_independence = validation_independence;
        }

        if let Some(block_mutation_for_red_or_systemic) =
            overrides.block_mutation_for_red_or_systemic
        {
            self.block_mutation_for_red_or_systemic = block_mutation_for_red_or_systemic;
        }
    }
}

#[cfg(test)]
mod authority_contract_tests {
    use std::str::FromStr;

    use super::{AuthorityZone, ChangeClass, RiskClass, UsageZone};
    use crate::domain::mode::Mode;

    #[test]
    fn authority_zone_round_trips_supported_values() {
        assert_eq!(AuthorityZone::Green.as_str(), "green");
        assert_eq!(AuthorityZone::Restricted.as_str(), "restricted");
        assert_eq!(AuthorityZone::from_str("yellow").unwrap(), AuthorityZone::Yellow);
        assert_eq!(AuthorityZone::from_str("Restricted").unwrap(), AuthorityZone::Restricted);
    }

    #[test]
    fn change_class_derives_operational_modes_to_critical_operations() {
        assert_eq!(
            ChangeClass::from_risk_and_mode(RiskClass::SystemicImpact, Mode::Incident),
            ChangeClass::CriticalOperations
        );
        assert_eq!(
            ChangeClass::from_risk_and_mode(RiskClass::BoundedImpact, Mode::Migration),
            ChangeClass::CriticalOperations
        );
    }

    #[test]
    fn change_class_preserves_risk_for_non_operational_modes() {
        assert_eq!(
            ChangeClass::from_risk_and_mode(RiskClass::LowImpact, Mode::Requirements),
            ChangeClass::LowImpact
        );
        assert_eq!(
            ChangeClass::from_risk_and_mode(RiskClass::SystemicImpact, Mode::Architecture),
            ChangeClass::SystemicImpact
        );
        assert_eq!(UsageZone::from_str("red").unwrap(), UsageZone::Red);
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AdapterPolicyMatrix, GatePolicy, InvocationConstraintProfile, PolicySet,
        PolicySetOverrides, RiskClass, RiskPolicyClass, UsageZone, ValidationIndependencePolicy,
        ZonePolicy,
    };
    use crate::domain::execution::PayloadRetentionLevel;
    use crate::domain::gate::GateKind;
    use crate::domain::verification::VerificationLayer;
    use canon_adapters::{AdapterKind, CapabilityKind};

    fn minimal_policy_set() -> PolicySet {
        PolicySet {
            risk_classes: vec![RiskPolicyClass {
                name: RiskClass::LowImpact,
                requires_owner: false,
                mutable_execution: true,
                verification_layers: vec![],
            }],
            zones: vec![ZonePolicy { name: UsageZone::Green, mutable_execution: true }],
            gate_policy: GatePolicy { mandatory_gates: vec![] },
            adapter_matrix: vec![AdapterPolicyMatrix {
                adapter: AdapterKind::Filesystem,
                capabilities: vec![CapabilityKind::EmitArtifact],
            }],
            constraint_profiles: vec![],
            runtime_disabled_adapters: vec![],
            validation_independence: ValidationIndependencePolicy {
                ai_generation_requires_distinct_validation: false,
                human_review_counts_independent: true,
            },
            block_mutation_for_red_or_systemic: false,
        }
    }

    #[test]
    fn apply_overrides_adds_new_risk_class_when_name_not_present() {
        let mut policy = minimal_policy_set();
        let overrides = PolicySetOverrides {
            risk_classes: vec![RiskPolicyClass {
                name: RiskClass::SystemicImpact,
                requires_owner: true,
                mutable_execution: false,
                verification_layers: vec![VerificationLayer::SelfCritique],
            }],
            ..Default::default()
        };
        policy.apply_overrides(overrides);
        assert_eq!(policy.risk_classes.len(), 2);
        let systemic = policy.risk_classes.iter().find(|c| c.name == RiskClass::SystemicImpact);
        assert!(systemic.is_some_and(|c| c.requires_owner));
    }

    #[test]
    fn apply_overrides_replaces_existing_risk_class_by_name() {
        let mut policy = minimal_policy_set();
        let overrides = PolicySetOverrides {
            risk_classes: vec![RiskPolicyClass {
                name: RiskClass::LowImpact,
                requires_owner: true,
                mutable_execution: false,
                verification_layers: vec![],
            }],
            ..Default::default()
        };
        policy.apply_overrides(overrides);
        assert_eq!(policy.risk_classes.len(), 1);
        assert!(policy.risk_classes[0].requires_owner);
    }

    #[test]
    fn apply_overrides_adds_new_zone_when_name_not_present() {
        let mut policy = minimal_policy_set();
        let overrides = PolicySetOverrides {
            zones: vec![ZonePolicy { name: UsageZone::Red, mutable_execution: false }],
            ..Default::default()
        };
        policy.apply_overrides(overrides);
        assert_eq!(policy.zones.len(), 2);
    }

    #[test]
    fn apply_overrides_replaces_gate_policy_when_provided() {
        let mut policy = minimal_policy_set();
        let overrides = PolicySetOverrides {
            gate_policy: Some(GatePolicy { mandatory_gates: vec![GateKind::Risk] }),
            ..Default::default()
        };
        policy.apply_overrides(overrides);
        assert_eq!(policy.gate_policy.mandatory_gates, vec![GateKind::Risk]);
    }

    #[test]
    fn apply_overrides_adds_new_adapter_matrix_entry_when_adapter_not_present() {
        let mut policy = minimal_policy_set();
        let overrides = PolicySetOverrides {
            adapter_matrix: vec![AdapterPolicyMatrix {
                adapter: AdapterKind::CopilotCli,
                capabilities: vec![CapabilityKind::GenerateContent],
            }],
            ..Default::default()
        };
        policy.apply_overrides(overrides);
        assert_eq!(policy.adapter_matrix.len(), 2);
    }

    #[test]
    fn apply_overrides_adds_new_constraint_profile_when_id_not_present() {
        let mut policy = minimal_policy_set();
        let overrides = PolicySetOverrides {
            constraint_profiles: vec![InvocationConstraintProfile {
                id: "strict".to_string(),
                payload_retention: PayloadRetentionLevel::SummaryOnly,
                max_payload_bytes: Some(512),
                command_profile: None,
                recommendation_only: true,
                patch_disabled: true,
            }],
            ..Default::default()
        };
        policy.apply_overrides(overrides);
        assert_eq!(policy.constraint_profiles.len(), 1);
        assert_eq!(policy.constraint_profiles[0].id, "strict");
    }

    #[test]
    fn apply_overrides_sets_runtime_disabled_adapters_when_provided() {
        let mut policy = minimal_policy_set();
        let overrides = PolicySetOverrides {
            runtime_disabled_adapters: Some(vec![AdapterKind::McpStdio]),
            ..Default::default()
        };
        policy.apply_overrides(overrides);
        assert_eq!(policy.runtime_disabled_adapters, vec![AdapterKind::McpStdio]);
    }

    #[test]
    fn apply_overrides_sets_validation_independence_when_provided() {
        let mut policy = minimal_policy_set();
        let overrides = PolicySetOverrides {
            validation_independence: Some(ValidationIndependencePolicy {
                ai_generation_requires_distinct_validation: true,
                human_review_counts_independent: false,
            }),
            ..Default::default()
        };
        policy.apply_overrides(overrides);
        assert!(policy.validation_independence.ai_generation_requires_distinct_validation);
    }

    #[test]
    fn apply_overrides_sets_block_mutation_flag_when_provided() {
        let mut policy = minimal_policy_set();
        assert!(!policy.block_mutation_for_red_or_systemic);
        let overrides = PolicySetOverrides {
            block_mutation_for_red_or_systemic: Some(true),
            ..Default::default()
        };
        policy.apply_overrides(overrides);
        assert!(policy.block_mutation_for_red_or_systemic);
    }
}
