use serde::{Deserialize, Serialize};
use strum_macros::{Display, IntoStaticStr};
use time::OffsetDateTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, IntoStaticStr)]
#[strum(serialize_all = "kebab-case")]
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
        self.into()
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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::GateKind;

    #[test]
    fn gate_kind_round_trips_supported_labels() {
        let cases = [
            (GateKind::Exploration, "exploration", "Exploration"),
            (GateKind::BrownfieldPreservation, "brownfield-preservation", "BrownfieldPreservation"),
            (GateKind::Architecture, "architecture", "Architecture"),
            (GateKind::Risk, "risk", "Risk"),
            (GateKind::ReviewDisposition, "review-disposition", "ReviewDisposition"),
            (GateKind::ReleaseReadiness, "release-readiness", "ReleaseReadiness"),
            (
                GateKind::ImplementationReadiness,
                "implementation-readiness",
                "ImplementationReadiness",
            ),
            (GateKind::IncidentContainment, "incident-containment", "IncidentContainment"),
            (GateKind::MigrationSafety, "migration-safety", "MigrationSafety"),
        ];

        for (gate, kebab, pascal) in cases {
            assert_eq!(gate.as_str(), kebab);
            assert_eq!(GateKind::from_str(kebab).expect("kebab-case should parse"), gate);
            assert_eq!(GateKind::from_str(pascal).expect("PascalCase should parse"), gate);
        }
    }

    #[test]
    fn gate_kind_rejects_unknown_values() {
        let error = GateKind::from_str("not-a-real-gate").expect_err("unknown gate should fail");

        assert_eq!(error, "unsupported gate kind: not-a-real-gate");
    }
}
