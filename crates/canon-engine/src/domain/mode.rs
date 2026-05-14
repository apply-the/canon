use std::fmt;

use serde::{Deserialize, Serialize};

use canon_adapters::AdapterKind;

use crate::domain::gate::GateKind;

// Modes stay focused on the governed work type; `SystemContext` carries new vs existing state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mode {
    Requirements,
    Discovery,
    SystemShaping,
    Change,
    Backlog,
    Architecture,
    SystemAssessment,
    Implementation,
    Refactor,
    Verification,
    Review,
    PrReview,
    Incident,
    SecurityAssessment,
    Migration,
    SupplyChainAnalysis,
    DomainLanguage,
    DomainModel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum GovernedExpertiseKind {
    DomainLanguage,
    DomainModel,
}

impl GovernedExpertiseKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DomainLanguage => "domain-language",
            Self::DomainModel => "domain-model",
        }
    }

    pub fn all() -> &'static [GovernedExpertiseKind] {
        &[Self::DomainLanguage, Self::DomainModel]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModeEmphasis {
    AnalysisHeavy,
    ExecutionHeavy,
    ReviewHeavy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImplementationDepth {
    Full,
    ContractOnly,
    Skeleton,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModeProfile {
    pub mode: Mode,
    pub purpose: &'static str,
    pub emphasis: ModeEmphasis,
    pub implementation_depth: ImplementationDepth,
    pub gate_profile: Vec<GateKind>,
    pub artifact_families: Vec<&'static str>,
    pub allowed_adapters: Vec<AdapterKind>,
}

impl Mode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Requirements => "requirements",
            Self::Discovery => "discovery",
            Self::SystemShaping => "system-shaping",
            Self::Change => "change",
            Self::Backlog => "backlog",
            Self::Architecture => "architecture",
            Self::SystemAssessment => "system-assessment",
            Self::Implementation => "implementation",
            Self::Refactor => "refactor",
            Self::Verification => "verification",
            Self::Review => "review",
            Self::PrReview => "pr-review",
            Self::Incident => "incident",
            Self::SecurityAssessment => "security-assessment",
            Self::Migration => "migration",
            Self::SupplyChainAnalysis => "supply-chain-analysis",
            Self::DomainLanguage => "domain-language",
            Self::DomainModel => "domain-model",
        }
    }

    pub fn governed_expertise_kind(self) -> Option<GovernedExpertiseKind> {
        match self {
            Self::DomainLanguage => Some(GovernedExpertiseKind::DomainLanguage),
            Self::DomainModel => Some(GovernedExpertiseKind::DomainModel),
            _ => None,
        }
    }

    pub fn all() -> &'static [Mode] {
        &[
            Self::Discovery,
            Self::Requirements,
            Self::SystemShaping,
            Self::Architecture,
            Self::SystemAssessment,
            Self::Change,
            Self::Backlog,
            Self::PrReview,
            Self::Implementation,
            Self::Refactor,
            Self::Verification,
            Self::Review,
            Self::Incident,
            Self::SecurityAssessment,
            Self::Migration,
            Self::SupplyChainAnalysis,
            Self::DomainLanguage,
            Self::DomainModel,
        ]
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl fmt::Display for GovernedExpertiseKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::str::FromStr for Mode {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "requirements" => Ok(Self::Requirements),
            "discovery" => Ok(Self::Discovery),
            "system-shaping" => Ok(Self::SystemShaping),
            "change" => Ok(Self::Change),
            "backlog" => Ok(Self::Backlog),
            "architecture" => Ok(Self::Architecture),
            "system-assessment" => Ok(Self::SystemAssessment),
            "implementation" => Ok(Self::Implementation),
            "refactor" => Ok(Self::Refactor),
            "verification" => Ok(Self::Verification),
            "review" => Ok(Self::Review),
            "pr-review" => Ok(Self::PrReview),
            "incident" => Ok(Self::Incident),
            "security-assessment" => Ok(Self::SecurityAssessment),
            "migration" => Ok(Self::Migration),
            "supply-chain-analysis" => Ok(Self::SupplyChainAnalysis),
            "domain-language" => Ok(Self::DomainLanguage),
            "domain-model" => Ok(Self::DomainModel),
            other => Err(format!("unsupported mode: {other}")),
        }
    }
}

pub fn all_mode_profiles() -> Vec<ModeProfile> {
    use AdapterKind::{CopilotCli, Filesystem, McpStdio, Shell};
    use GateKind::{
        Architecture, ChangePreservation, Exploration, ImplementationReadiness,
        IncidentContainment, MigrationSafety, ReleaseReadiness, ReviewDisposition, Risk,
    };
    use ImplementationDepth::Full;
    use Mode::{
        Architecture as ArchitectureMode, Backlog, Change, Discovery, DomainLanguage, DomainModel,
        Implementation, Incident, Migration, PrReview, Refactor, Requirements, Review,
        SecurityAssessment, SupplyChainAnalysis, SystemAssessment, SystemShaping, Verification,
    };
    use ModeEmphasis::{AnalysisHeavy, ExecutionHeavy, ReviewHeavy};

    vec![
        ModeProfile {
            mode: Discovery,
            purpose: "Explore unknowns without turning exploration into solution drift.",
            emphasis: AnalysisHeavy,
            implementation_depth: Full,
            gate_profile: vec![Exploration, Risk, ReleaseReadiness],
            artifact_families: vec![
                "problem map",
                "unknowns and assumptions",
                "context boundary",
                "exploration options",
                "decision pressure points",
            ],
            allowed_adapters: vec![Filesystem, Shell, CopilotCli, McpStdio],
        },
        ModeProfile {
            mode: Requirements,
            purpose: "Bound an initiative before generation expands it into platform sprawl.",
            emphasis: AnalysisHeavy,
            implementation_depth: Full,
            gate_profile: vec![Exploration, Risk, ReleaseReadiness],
            artifact_families: vec![
                "problem framing",
                "constraints",
                "options",
                "tradeoffs",
                "scope cuts",
                "decision checklist",
            ],
            allowed_adapters: vec![Filesystem, Shell, CopilotCli],
        },
        ModeProfile {
            mode: SystemShaping,
            purpose: "Shape a new capability from bounded intent through early delivery structure.",
            emphasis: AnalysisHeavy,
            implementation_depth: Full,
            gate_profile: vec![Exploration, Architecture, Risk, ReleaseReadiness],
            artifact_families: vec![
                "system shape",
                "architecture outline",
                "capability map",
                "delivery options",
                "risk hotspots",
            ],
            allowed_adapters: vec![Filesystem, Shell, CopilotCli],
        },
        ModeProfile {
            mode: ArchitectureMode,
            purpose: "Evaluate boundaries, invariants, and structural decisions.",
            emphasis: AnalysisHeavy,
            implementation_depth: Full,
            gate_profile: vec![Exploration, Architecture, Risk, ReleaseReadiness],
            artifact_families: vec![
                "architecture decisions",
                "invariants",
                "tradeoff matrix",
                "boundary map",
                "readiness assessment",
            ],
            allowed_adapters: vec![Filesystem, Shell, CopilotCli],
        },
        ModeProfile {
            mode: SystemAssessment,
            purpose: "Assess a bounded existing system as-is with explicit evidence, coverage, and gaps before downstream design work.",
            emphasis: AnalysisHeavy,
            implementation_depth: Full,
            gate_profile: vec![Risk, Architecture, ReleaseReadiness],
            artifact_families: vec![
                "assessment overview",
                "coverage map",
                "asset inventory",
                "functional view",
                "component view",
                "deployment view",
                "technology view",
                "integration view",
                "risk register",
                "assessment evidence",
            ],
            allowed_adapters: vec![Filesystem, Shell, CopilotCli],
        },
        ModeProfile {
            mode: Change,
            purpose: "Constrain change in an existing system before implementation begins.",
            emphasis: AnalysisHeavy,
            implementation_depth: Full,
            gate_profile: vec![
                Exploration,
                ChangePreservation,
                Architecture,
                Risk,
                ReleaseReadiness,
            ],
            artifact_families: vec![
                "system slice",
                "legacy invariants",
                "change surface",
                "implementation plan",
                "validation strategy",
                "decision record",
            ],
            allowed_adapters: vec![Filesystem, Shell, CopilotCli],
        },
        ModeProfile {
            mode: Backlog,
            purpose: "Decompose bounded upstream decisions into credible delivery epics, slices, and sequencing.",
            emphasis: AnalysisHeavy,
            implementation_depth: Full,
            gate_profile: vec![Exploration, Architecture, Risk, ReleaseReadiness],
            artifact_families: vec![
                "backlog overview",
                "epic tree",
                "capability map",
                "dependency map",
                "delivery slices",
                "sequencing plan",
                "acceptance anchors",
                "planning risks",
            ],
            allowed_adapters: vec![Filesystem, Shell, CopilotCli],
        },
        ModeProfile {
            mode: Implementation,
            purpose: "Turn an approved bounded plan into controlled execution.",
            emphasis: ExecutionHeavy,
            implementation_depth: Full,
            gate_profile: vec![ImplementationReadiness, Risk, ReleaseReadiness],
            artifact_families: vec![
                "task mapping",
                "mutation bounds",
                "implementation notes",
                "completion evidence",
                "validation hooks",
                "rollback notes",
            ],
            allowed_adapters: vec![Filesystem, Shell, CopilotCli],
        },
        ModeProfile {
            mode: Refactor,
            purpose: "Improve structure while preserving externally meaningful behavior.",
            emphasis: ExecutionHeavy,
            implementation_depth: Full,
            gate_profile: vec![ChangePreservation, Architecture, Risk, ReleaseReadiness],
            artifact_families: vec![
                "preserved behavior",
                "refactor scope",
                "structural rationale",
                "regression evidence",
                "contract drift check",
                "no feature addition",
            ],
            allowed_adapters: vec![Filesystem, Shell, CopilotCli],
        },
        ModeProfile {
            mode: Verification,
            purpose: "Challenge claims, invariants, contracts, and evidence directly.",
            emphasis: ReviewHeavy,
            implementation_depth: Full,
            gate_profile: vec![Risk, ReleaseReadiness],
            artifact_families: vec![
                "invariants checklist",
                "contract matrix",
                "adversarial review",
                "verification report",
                "unresolved findings",
            ],
            allowed_adapters: vec![Filesystem, Shell, CopilotCli, McpStdio],
        },
        ModeProfile {
            mode: Review,
            purpose: "Review a change package or artifact bundle outside pull request semantics.",
            emphasis: ReviewHeavy,
            implementation_depth: Full,
            gate_profile: vec![Risk, Architecture, ReviewDisposition, ReleaseReadiness],
            artifact_families: vec![
                "review brief",
                "boundary assessment",
                "missing evidence",
                "decision impact",
                "review disposition",
            ],
            allowed_adapters: vec![Filesystem, Shell, CopilotCli],
        },
        ModeProfile {
            mode: PrReview,
            purpose: "Produce structured review artifacts for a branch or pull request diff.",
            emphasis: ReviewHeavy,
            implementation_depth: Full,
            gate_profile: vec![Risk, Architecture, ReviewDisposition, ReleaseReadiness],
            artifact_families: vec![
                "PR analysis",
                "boundary check",
                "duplication check",
                "contract drift",
                "missing tests",
                "decision impact",
                "review summary",
            ],
            allowed_adapters: vec![Filesystem, Shell, CopilotCli],
        },
        ModeProfile {
            mode: Incident,
            purpose: "Bound investigation and containment work during failures.",
            emphasis: AnalysisHeavy,
            implementation_depth: Full,
            gate_profile: vec![Risk, IncidentContainment, Architecture, ReleaseReadiness],
            artifact_families: vec![
                "incident frame",
                "hypothesis log",
                "blast radius map",
                "containment plan",
                "incident decision record",
                "follow-up verification",
            ],
            allowed_adapters: vec![Filesystem, Shell, CopilotCli, McpStdio],
        },
        ModeProfile {
            mode: SecurityAssessment,
            purpose: "Assess a bounded existing system for threats, risks, and recommendation-only mitigations.",
            emphasis: AnalysisHeavy,
            implementation_depth: Full,
            gate_profile: vec![Risk, Architecture, ReleaseReadiness],
            artifact_families: vec![
                "assessment overview",
                "threat model",
                "risk register",
                "mitigations",
                "assumptions and gaps",
                "assessment evidence",
            ],
            allowed_adapters: vec![Filesystem, Shell, CopilotCli],
        },
        ModeProfile {
            mode: Migration,
            purpose: "Manage movement between systems or contracts with explicit compatibility control.",
            emphasis: AnalysisHeavy,
            implementation_depth: Full,
            gate_profile: vec![Exploration, Architecture, MigrationSafety, Risk, ReleaseReadiness],
            artifact_families: vec![
                "source-target map",
                "compatibility matrix",
                "sequencing plan",
                "fallback plan",
                "migration verification report",
                "decision record",
            ],
            allowed_adapters: vec![Filesystem, Shell, CopilotCli],
        },
        ModeProfile {
            mode: SupplyChainAnalysis,
            purpose: "Assess a bounded existing repository for SBOM, vulnerability, license, and legacy posture with explicit coverage gaps.",
            emphasis: AnalysisHeavy,
            implementation_depth: Full,
            gate_profile: vec![Risk, ReleaseReadiness],
            artifact_families: vec![
                "analysis overview",
                "sbom bundle",
                "vulnerability triage",
                "license compliance",
                "legacy posture",
                "policy decisions",
                "analysis evidence",
            ],
            allowed_adapters: vec![Filesystem, Shell, CopilotCli],
        },
        ModeProfile {
            mode: DomainLanguage,
            purpose: "Stabilize the shared vocabulary of a product area before downstream requirements, architecture, or change work.",
            emphasis: AnalysisHeavy,
            implementation_depth: Full,
            gate_profile: vec![Risk, Architecture, ReleaseReadiness],
            artifact_families: vec![
                "language overview",
                "domain glossary",
                "preferred language",
                "language conflicts",
                "contextual meanings",
                "business language rules",
                "code and api vocabulary",
                "downstream language guidance",
                "language decision record",
                "ai provenance",
            ],
            allowed_adapters: vec![Filesystem, Shell, CopilotCli],
        },
        ModeProfile {
            mode: DomainModel,
            purpose: "Formalize domain concepts, relationships, invariants, and feature-impact rules before architecture or backlog decomposition.",
            emphasis: AnalysisHeavy,
            implementation_depth: Full,
            gate_profile: vec![Risk, Architecture, ReleaseReadiness],
            artifact_families: vec![
                "model overview",
                "concept catalog",
                "relationship map",
                "bounded context map",
                "lifecycle and state model",
                "domain invariants",
                "policy and constraint rules",
                "feature impact rules",
                "code data alignment",
                "model gaps and risks",
                "downstream model guidance",
                "domain model json",
                "ai provenance",
            ],
            allowed_adapters: vec![Filesystem, Shell, CopilotCli],
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::{GovernedExpertiseKind, Mode};

    #[test]
    fn mode_string_inventory_round_trips_and_display_is_stable() {
        let cases = [
            (Mode::Requirements, "requirements", None),
            (Mode::Discovery, "discovery", None),
            (Mode::SystemShaping, "system-shaping", None),
            (Mode::Change, "change", None),
            (Mode::Backlog, "backlog", None),
            (Mode::Architecture, "architecture", None),
            (Mode::SystemAssessment, "system-assessment", None),
            (Mode::Implementation, "implementation", None),
            (Mode::Refactor, "refactor", None),
            (Mode::Verification, "verification", None),
            (Mode::Review, "review", None),
            (Mode::PrReview, "pr-review", None),
            (Mode::Incident, "incident", None),
            (Mode::SecurityAssessment, "security-assessment", None),
            (Mode::Migration, "migration", None),
            (Mode::SupplyChainAnalysis, "supply-chain-analysis", None),
            (Mode::DomainLanguage, "domain-language", Some(GovernedExpertiseKind::DomainLanguage)),
            (Mode::DomainModel, "domain-model", Some(GovernedExpertiseKind::DomainModel)),
        ];

        assert_eq!(
            Mode::all(),
            &[
                Mode::Discovery,
                Mode::Requirements,
                Mode::SystemShaping,
                Mode::Architecture,
                Mode::SystemAssessment,
                Mode::Change,
                Mode::Backlog,
                Mode::PrReview,
                Mode::Implementation,
                Mode::Refactor,
                Mode::Verification,
                Mode::Review,
                Mode::Incident,
                Mode::SecurityAssessment,
                Mode::Migration,
                Mode::SupplyChainAnalysis,
                Mode::DomainLanguage,
                Mode::DomainModel,
            ]
        );

        for (mode, expected, expertise_kind) in cases {
            assert_eq!(mode.as_str(), expected);
            assert_eq!(mode.to_string(), expected);
            assert_eq!(expected.parse::<Mode>().unwrap(), mode);
            assert_eq!(mode.governed_expertise_kind(), expertise_kind);
        }

        assert_eq!("unknown-mode".parse::<Mode>().unwrap_err(), "unsupported mode: unknown-mode");
    }

    #[test]
    fn governed_expertise_kind_classifies_supported_modes() {
        assert_eq!(
            Mode::DomainLanguage.governed_expertise_kind(),
            Some(GovernedExpertiseKind::DomainLanguage)
        );
        assert_eq!(
            Mode::DomainModel.governed_expertise_kind(),
            Some(GovernedExpertiseKind::DomainModel)
        );
        assert_eq!(Mode::Requirements.governed_expertise_kind(), None);
        assert_eq!(Mode::Review.governed_expertise_kind(), None);
    }

    #[test]
    fn governed_expertise_kind_inventory_is_stable() {
        assert_eq!(
            GovernedExpertiseKind::all(),
            &[GovernedExpertiseKind::DomainLanguage, GovernedExpertiseKind::DomainModel,]
        );

        for (kind, expected) in [
            (GovernedExpertiseKind::DomainLanguage, "domain-language"),
            (GovernedExpertiseKind::DomainModel, "domain-model"),
        ] {
            assert_eq!(kind.as_str(), expected);
            assert_eq!(kind.to_string(), expected);
        }
    }
}
