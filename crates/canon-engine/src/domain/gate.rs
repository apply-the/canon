use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GateKind {
    Exploration,
    BrownfieldPreservation,
    Architecture,
    Risk,
    ReviewDisposition,
    ReleaseReadiness,
    ImplementationReadiness,
    IncidentContainment,
    MigrationSafety,
}

impl GateKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exploration => "exploration",
            Self::BrownfieldPreservation => "brownfield-preservation",
            Self::Architecture => "architecture",
            Self::Risk => "risk",
            Self::ReviewDisposition => "review-disposition",
            Self::ReleaseReadiness => "release-readiness",
            Self::ImplementationReadiness => "implementation-readiness",
            Self::IncidentContainment => "incident-containment",
            Self::MigrationSafety => "migration-safety",
        }
    }
}

impl std::str::FromStr for GateKind {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "exploration" | "Exploration" => Ok(Self::Exploration),
            "brownfield-preservation" | "BrownfieldPreservation" => {
                Ok(Self::BrownfieldPreservation)
            }
            "architecture" | "Architecture" => Ok(Self::Architecture),
            "risk" | "Risk" => Ok(Self::Risk),
            "review-disposition" | "ReviewDisposition" => Ok(Self::ReviewDisposition),
            "release-readiness" | "ReleaseReadiness" => Ok(Self::ReleaseReadiness),
            "implementation-readiness" | "ImplementationReadiness" => {
                Ok(Self::ImplementationReadiness)
            }
            "incident-containment" | "IncidentContainment" => Ok(Self::IncidentContainment),
            "migration-safety" | "MigrationSafety" => Ok(Self::MigrationSafety),
            other => Err(format!("unsupported gate kind: {other}")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GateStatus {
    Pending,
    Passed,
    Blocked,
    NeedsApproval,
    Overridden,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GateEvaluation {
    pub gate: GateKind,
    pub status: GateStatus,
    pub blockers: Vec<String>,
    pub evaluated_at: OffsetDateTime,
}
