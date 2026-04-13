use serde::{Deserialize, Serialize};
use strum_macros::{Display, IntoStaticStr};

use canon_adapters::{AdapterKind, CapabilityKind};

use crate::domain::execution::{InvocationConstraintSet, PayloadRetentionLevel};
use crate::domain::gate::GateKind;
use crate::domain::verification::VerificationLayer;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, IntoStaticStr)]
#[strum(serialize_all = "kebab-case")]
pub enum RiskClass {
    LowImpact,
    BoundedImpact,
    SystemicImpact,
}

impl RiskClass {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, IntoStaticStr)]
#[strum(serialize_all = "lowercase")]
pub enum UsageZone {
    Green,
    Yellow,
    Red,
}

impl UsageZone {
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RiskPolicyClass {
    pub name: RiskClass,
    pub requires_owner: bool,
    pub mutable_execution: bool,
    pub verification_layers: Vec<VerificationLayer>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ZonePolicy {
    pub name: UsageZone,
    pub mutable_execution: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GatePolicy {
    pub mandatory_gates: Vec<GateKind>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdapterPolicyMatrix {
    #[serde(alias = "kind")]
    pub adapter: AdapterKind,
    pub capabilities: Vec<CapabilityKind>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InvocationConstraintProfile {
    pub id: String,
    pub payload_retention: PayloadRetentionLevel,
    pub max_payload_bytes: Option<u64>,
    pub command_profile: Option<String>,
    pub recommendation_only: bool,
    pub patch_disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ValidationIndependencePolicy {
    pub ai_generation_requires_distinct_validation: bool,
    pub human_review_counts_independent: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicySet {
    pub risk_classes: Vec<RiskPolicyClass>,
    pub zones: Vec<ZonePolicy>,
    pub gate_policy: GatePolicy,
    pub adapter_matrix: Vec<AdapterPolicyMatrix>,
    pub constraint_profiles: Vec<InvocationConstraintProfile>,
    pub runtime_disabled_adapters: Vec<AdapterKind>,
    pub validation_independence: ValidationIndependencePolicy,
    pub block_mutation_for_red_or_systemic: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PolicySetOverrides {
    pub risk_classes: Vec<RiskPolicyClass>,
    pub zones: Vec<ZonePolicy>,
    pub gate_policy: Option<GatePolicy>,
    pub adapter_matrix: Vec<AdapterPolicyMatrix>,
    pub constraint_profiles: Vec<InvocationConstraintProfile>,
    pub runtime_disabled_adapters: Option<Vec<AdapterKind>>,
    pub validation_independence: Option<ValidationIndependencePolicy>,
    pub block_mutation_for_red_or_systemic: Option<bool>,
}

impl PolicySet {
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

    pub fn verification_layers_for(&self, risk: RiskClass) -> Vec<VerificationLayer> {
        self.risk_classes
            .iter()
            .find(|class| class.name == risk)
            .map(|class| class.verification_layers.clone())
            .unwrap_or_default()
    }

    pub fn runtime_adapter_enabled(&self, adapter: AdapterKind) -> bool {
        !self.runtime_disabled_adapters.contains(&adapter)
    }

    pub fn adapter_supports(&self, adapter: AdapterKind, capability: CapabilityKind) -> bool {
        self.adapter_matrix
            .iter()
            .find(|entry| entry.adapter == adapter)
            .map(|entry| entry.capabilities.contains(&capability))
            .unwrap_or(false)
    }

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
