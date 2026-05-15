use serde::{Deserialize, Serialize};
use strum_macros::{Display, IntoStaticStr};
use time::OffsetDateTime;

/// The various types of governance gates that can block or permit a run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, IntoStaticStr)]
#[strum(serialize_all = "kebab-case")]
pub enum GateKind {
    /// Basic problem space exploration.
    Exploration,
    /// Preserving existing behavior during change.
    ChangePreservation,
    /// Architectural review and design consistency.
    Architecture,
    /// Risk classification and zone enforcement.
    Risk,
    /// Mutation or execution safety.
    Execution,
    /// Final review and sign-off.
    ReviewDisposition,
    /// Readiness for distribution or release.
    ReleaseReadiness,
    /// Readiness for implementation work.
    ImplementationReadiness,
    /// Containment of a live incident.
    IncidentContainment,
    /// Safety of data or system migration.
    MigrationSafety,
}

impl GateKind {
    /// Returns the kebab-case string representation of this gate kind.
    pub fn as_str(self) -> &'static str {
        self.into()
    }
}

impl std::str::FromStr for GateKind {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "exploration" | "Exploration" => Ok(Self::Exploration),
            "change-preservation" | "ChangePreservation" => Ok(Self::ChangePreservation),
            "architecture" | "Architecture" => Ok(Self::Architecture),
            "risk" | "Risk" => Ok(Self::Risk),
            "execution" | "Execution" => Ok(Self::Execution),
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

/// The current status of a specific governance gate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GateStatus {
    /// Not yet evaluated.
    Pending,
    /// The gate criteria were met.
    Passed,
    /// The gate criteria failed, blocking progress.
    Blocked,
    /// The gate is waiting for a human to provide approval.
    NeedsApproval,
    /// The gate failure was acknowledged and bypass was permitted.
    Overridden,
}

/// A persisted result of evaluating a single governance gate during a run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GateEvaluation {
    /// The gate that was evaluated.
    pub gate: GateKind,
    /// The outcome of the evaluation.
    pub status: GateStatus,
    /// Human-readable descriptions of any conditions that are blocking the gate.
    pub blockers: Vec<String>,
    /// When this evaluation was recorded.
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
            (GateKind::ChangePreservation, "change-preservation", "ChangePreservation"),
            (GateKind::Architecture, "architecture", "Architecture"),
            (GateKind::Risk, "risk", "Risk"),
            (GateKind::Execution, "execution", "Execution"),
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
