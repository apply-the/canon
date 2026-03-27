use serde::{Deserialize, Serialize};

use crate::domain::gate::GateKind;
use crate::domain::verification::VerificationLayer;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskClass {
    LowImpact,
    BoundedImpact,
    SystemicImpact,
}

impl RiskClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::LowImpact => "low-impact",
            Self::BoundedImpact => "bounded-impact",
            Self::SystemicImpact => "systemic-impact",
        }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UsageZone {
    Green,
    Yellow,
    Red,
}

impl UsageZone {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::Yellow => "yellow",
            Self::Red => "red",
        }
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
pub struct PolicySet {
    pub risk_classes: Vec<RiskPolicyClass>,
    pub zones: Vec<ZonePolicy>,
    pub gate_policy: GatePolicy,
    pub block_mutation_for_red_or_systemic: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PolicySetOverrides {
    pub risk_classes: Vec<RiskPolicyClass>,
    pub zones: Vec<ZonePolicy>,
    pub gate_policy: Option<GatePolicy>,
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

        if let Some(block_mutation_for_red_or_systemic) =
            overrides.block_mutation_for_red_or_systemic
        {
            self.block_mutation_for_red_or_systemic = block_mutation_for_red_or_systemic;
        }
    }
}
