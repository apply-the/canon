use super::*;

/// Contract line string for the first authority-governance metadata slice.
pub const AUTHORITY_GOVERNANCE_V1_CONTRACT_LINE: &str = "authority-governance-v1";
/// Contract line string for the first adaptive-governance companion slice.
pub const ADAPTIVE_GOVERNANCE_V1_CONTRACT_LINE: &str = "adaptive-governance-v1";

/// Approval state exported through `authority-governance-v1`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityApprovalState {
    NotNeeded,
    Requested,
    Granted,
    Rejected,
    Expired,
}

impl AuthorityApprovalState {
    /// Returns true when the authority posture still requires a human gate.
    pub const fn requires_gate(self) -> bool {
        matches!(self, Self::Requested)
    }
}

/// Packet readiness exported through `authority-governance-v1`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityPacketReadiness {
    Pending,
    Incomplete,
    Reusable,
    Rejected,
}

/// Risk vocabulary exported through `authority-governance-v1`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AuthorityRiskClass {
    LowImpact,
    BoundedImpact,
    SystemicImpact,
}

impl AuthorityRiskClass {
    /// Maps the runtime risk class into the stable contract vocabulary.
    pub const fn from_risk_class(risk: RiskClass) -> Self {
        match risk {
            RiskClass::LowImpact => Self::LowImpact,
            RiskClass::BoundedImpact => Self::BoundedImpact,
            RiskClass::SystemicImpact => Self::SystemicImpact,
        }
    }
}

/// Governance-state vocabulary exported through `adaptive-governance-v1`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AdaptiveGovernanceState {
    Advisory,
    Catch,
    Rule,
    Hook,
}

impl AdaptiveGovernanceState {
    /// Returns the kebab-case string representation of this governance state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Advisory => "advisory",
            Self::Catch => "catch",
            Self::Rule => "rule",
            Self::Hook => "hook",
        }
    }
}

impl std::fmt::Display for AdaptiveGovernanceState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Rollout-profile vocabulary exported through `adaptive-governance-v1`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AdaptiveRolloutProfile {
    Minimal,
    Guided,
    Governed,
    Strict,
}

impl AdaptiveRolloutProfile {
    /// Returns the kebab-case string representation of this rollout profile.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Minimal => "minimal",
            Self::Guided => "guided",
            Self::Governed => "governed",
            Self::Strict => "strict",
        }
    }
}

impl std::fmt::Display for AdaptiveRolloutProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Typed `adaptive-governance-v1` envelope published with governed packet metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdaptiveGovernanceV1Envelope {
    pub contract_line: String,
    pub governance_state: AdaptiveGovernanceState,
    pub rollout_profile: AdaptiveRolloutProfile,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state_rationale: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_rationale: Option<String>,
}

/// Typed runtime inputs used to build an `adaptive-governance-v1` envelope.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdaptiveGovernanceV1RuntimeInputs {
    pub risk: RiskClass,
    pub zone: UsageZone,
    pub approval_state: AuthorityApprovalState,
    pub packet_readiness: AuthorityPacketReadiness,
}

impl AdaptiveGovernanceV1Envelope {
    /// Builds the first-slice adaptive-governance companion from existing Canon runtime inputs.
    pub fn from_runtime_inputs(inputs: AdaptiveGovernanceV1RuntimeInputs) -> Self {
        let AdaptiveGovernanceV1RuntimeInputs { risk, zone, approval_state, packet_readiness } =
            inputs;

        let governance_state = if matches!(zone, UsageZone::Red) {
            AdaptiveGovernanceState::Hook
        } else if approval_state.requires_gate() {
            AdaptiveGovernanceState::Rule
        } else if matches!(
            packet_readiness,
            AuthorityPacketReadiness::Pending
                | AuthorityPacketReadiness::Incomplete
                | AuthorityPacketReadiness::Rejected
        ) {
            AdaptiveGovernanceState::Catch
        } else {
            AdaptiveGovernanceState::Advisory
        };

        let rollout_profile = if matches!(zone, UsageZone::Red) {
            AdaptiveRolloutProfile::Strict
        } else if approval_state.requires_gate() || matches!(risk, RiskClass::SystemicImpact) {
            AdaptiveRolloutProfile::Governed
        } else if matches!(risk, RiskClass::BoundedImpact | RiskClass::LowImpact)
            && matches!(zone, UsageZone::Yellow)
            || matches!(risk, RiskClass::BoundedImpact)
        {
            AdaptiveRolloutProfile::Guided
        } else {
            AdaptiveRolloutProfile::Minimal
        };

        Self {
            contract_line: ADAPTIVE_GOVERNANCE_V1_CONTRACT_LINE.to_string(),
            governance_state,
            rollout_profile,
            state_rationale: None,
            profile_rationale: None,
        }
    }
}

/// Typed `authority-governance-v1` envelope published with governed packet metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorityGovernanceV1Envelope {
    pub contract_line: String,
    pub authority_zone: AuthorityZone,
    pub change_class: ChangeClass,
    pub intended_persona: IntendedPersona,
    pub approval_state: AuthorityApprovalState,
    pub packet_readiness: AuthorityPacketReadiness,
    pub risk: AuthorityRiskClass,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub persona_anti_behaviors: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_artifact: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub artifact_order: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub promotion_refs: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stage_role_hints: Vec<StageRoleHint>,
}

/// Typed runtime inputs used to build an `authority-governance-v1` envelope.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorityGovernanceV1RuntimeInputs {
    pub mode: Mode,
    pub risk: RiskClass,
    pub zone: UsageZone,
    pub approval_state: AuthorityApprovalState,
    pub packet_readiness: AuthorityPacketReadiness,
    pub primary_artifact: Option<String>,
    pub artifact_order: Vec<String>,
    pub promotion_refs: Vec<String>,
}

impl AuthorityGovernanceV1Envelope {
    /// Builds the first-slice authority-governance envelope from existing Canon runtime inputs.
    pub fn from_runtime_inputs(inputs: AuthorityGovernanceV1RuntimeInputs) -> Self {
        let AuthorityGovernanceV1RuntimeInputs {
            mode,
            risk,
            zone,
            approval_state,
            packet_readiness,
            primary_artifact,
            artifact_order,
            promotion_refs,
        } = inputs;
        let persona = mode.intended_persona_profile();
        let authority_zone = if approval_state.requires_gate() {
            AuthorityZone::Restricted
        } else {
            match zone {
                UsageZone::Green => AuthorityZone::Green,
                UsageZone::Yellow => AuthorityZone::Yellow,
                UsageZone::Red => AuthorityZone::Red,
            }
        };

        Self {
            contract_line: AUTHORITY_GOVERNANCE_V1_CONTRACT_LINE.to_string(),
            authority_zone,
            change_class: ChangeClass::from_risk_and_mode(risk, mode),
            intended_persona: persona.intended_persona,
            approval_state,
            packet_readiness,
            risk: AuthorityRiskClass::from_risk_class(risk),
            persona_anti_behaviors: persona.persona_anti_behaviors,
            primary_artifact,
            artifact_order,
            promotion_refs,
            stage_role_hints: mode.stage_role_hints(),
        }
    }
}
