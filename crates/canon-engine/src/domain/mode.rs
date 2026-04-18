use std::fmt;

use serde::{Deserialize, Serialize};

use canon_adapters::AdapterKind;

use crate::domain::gate::GateKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mode {
    Requirements,
    Discovery,
    Greenfield,
    BrownfieldChange,
    Architecture,
    Implementation,
    Refactor,
    Verification,
    Review,
    PrReview,
    Incident,
    Migration,
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
            Self::Greenfield => "system-shaping",
            Self::BrownfieldChange => "brownfield-change",
            Self::Architecture => "architecture",
            Self::Implementation => "implementation",
            Self::Refactor => "refactor",
            Self::Verification => "verification",
            Self::Review => "review",
            Self::PrReview => "pr-review",
            Self::Incident => "incident",
            Self::Migration => "migration",
        }
    }

    pub fn all() -> &'static [Mode] {
        &[
            Self::Discovery,
            Self::Requirements,
            Self::Greenfield,
            Self::Architecture,
            Self::BrownfieldChange,
            Self::PrReview,
            Self::Implementation,
            Self::Refactor,
            Self::Verification,
            Self::Review,
            Self::Incident,
            Self::Migration,
        ]
    }
}

impl fmt::Display for Mode {
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
            "system-shaping" => Ok(Self::Greenfield),
            "brownfield-change" => Ok(Self::BrownfieldChange),
            "architecture" => Ok(Self::Architecture),
            "implementation" => Ok(Self::Implementation),
            "refactor" => Ok(Self::Refactor),
            "verification" => Ok(Self::Verification),
            "review" => Ok(Self::Review),
            "pr-review" => Ok(Self::PrReview),
            "incident" => Ok(Self::Incident),
            "migration" => Ok(Self::Migration),
            other => Err(format!("unsupported mode: {other}")),
        }
    }
}

pub fn all_mode_profiles() -> Vec<ModeProfile> {
    use AdapterKind::{CopilotCli, Filesystem, McpStdio, Shell};
    use GateKind::{
        Architecture, BrownfieldPreservation, Exploration, ImplementationReadiness,
        IncidentContainment, MigrationSafety, ReleaseReadiness, ReviewDisposition, Risk,
    };
    use ImplementationDepth::{ContractOnly, Full, Skeleton};
    use Mode::{
        Architecture as ArchitectureMode, BrownfieldChange, Discovery, Greenfield, Implementation,
        Incident, Migration, PrReview, Refactor, Requirements, Review, Verification,
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
            mode: Greenfield,
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
            mode: BrownfieldChange,
            purpose: "Constrain change in an existing system before implementation begins.",
            emphasis: AnalysisHeavy,
            implementation_depth: Full,
            gate_profile: vec![
                Exploration,
                BrownfieldPreservation,
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
            mode: Implementation,
            purpose: "Turn an approved bounded plan into controlled execution.",
            emphasis: ExecutionHeavy,
            implementation_depth: Skeleton,
            gate_profile: vec![Risk, ImplementationReadiness, ReleaseReadiness],
            artifact_families: vec![
                "execution brief",
                "task bundle",
                "contract checklist",
                "change log",
                "verification hooks",
                "completion record",
            ],
            allowed_adapters: vec![Filesystem, Shell, CopilotCli],
        },
        ModeProfile {
            mode: Refactor,
            purpose: "Improve structure while preserving externally meaningful behavior.",
            emphasis: ExecutionHeavy,
            implementation_depth: ContractOnly,
            gate_profile: vec![Exploration, BrownfieldPreservation, Risk, ReleaseReadiness],
            artifact_families: vec![
                "equivalence criteria",
                "preserved surface",
                "untangling plan",
                "rollback notes",
                "validation strategy",
            ],
            allowed_adapters: vec![Filesystem, Shell, CopilotCli],
        },
        ModeProfile {
            mode: Verification,
            purpose: "Challenge claims, invariants, contracts, and evidence directly.",
            emphasis: ReviewHeavy,
            implementation_depth: ContractOnly,
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
            implementation_depth: ContractOnly,
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
            implementation_depth: Skeleton,
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
            mode: Migration,
            purpose: "Manage movement between systems or contracts with explicit compatibility control.",
            emphasis: AnalysisHeavy,
            implementation_depth: Skeleton,
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
    ]
}
