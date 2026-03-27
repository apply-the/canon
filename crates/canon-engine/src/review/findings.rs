use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FindingCategory {
    BoundaryCheck,
    DuplicationCheck,
    ContractDrift,
    MissingTests,
    DecisionImpact,
}

impl FindingCategory {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BoundaryCheck => "boundary-check",
            Self::DuplicationCheck => "duplication-check",
            Self::ContractDrift => "contract-drift",
            Self::MissingTests => "missing-tests",
            Self::DecisionImpact => "decision-impact",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FindingSeverity {
    Note,
    MustFix,
}

impl FindingSeverity {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Note => "note",
            Self::MustFix => "must-fix",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewFinding {
    pub category: FindingCategory,
    pub severity: FindingSeverity,
    pub title: String,
    pub details: String,
    pub changed_surfaces: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewPacket {
    pub base_ref: String,
    pub head_ref: String,
    pub changed_surfaces: Vec<String>,
    pub inferred_intent: String,
    pub surprising_surface_area: Vec<String>,
    pub findings: Vec<ReviewFinding>,
}

impl ReviewPacket {
    pub fn from_diff(
        base_ref: &str,
        head_ref: &str,
        changed_surfaces: Vec<String>,
        patch: &str,
    ) -> Self {
        let surprising_surface_area = changed_surfaces
            .iter()
            .filter(|surface| is_contract_surface(surface) || is_boundary_surface(surface))
            .cloned()
            .collect::<Vec<_>>();
        let source_changed = changed_surfaces.iter().any(|surface| is_source_surface(surface));
        let tests_changed = changed_surfaces.iter().any(|surface| is_test_surface(surface));

        let inferred_intent = if changed_surfaces.is_empty() {
            "No changed surfaces were detected between the supplied refs.".to_string()
        } else {
            format!(
                "Review the bounded change across {} changed surface(s) and {} diff line(s) between {base_ref} and {head_ref}.",
                changed_surfaces.len(),
                patch.lines().count()
            )
        };

        let mut findings = Vec::new();

        if changed_surfaces.iter().any(|surface| is_boundary_surface(surface)) {
            findings.push(ReviewFinding {
                category: FindingCategory::BoundaryCheck,
                severity: FindingSeverity::MustFix,
                title: "Boundary-marked surfaces changed".to_string(),
                details: "Public or boundary-marked files changed and require explicit reviewer disposition.".to_string(),
                changed_surfaces: changed_surfaces
                    .iter()
                    .filter(|surface| is_boundary_surface(surface))
                    .cloned()
                    .collect(),
            });
        }

        if changed_surfaces.iter().any(|surface| is_contract_surface(surface)) {
            findings.push(ReviewFinding {
                category: FindingCategory::ContractDrift,
                severity: FindingSeverity::MustFix,
                title: "Contract-facing files changed".to_string(),
                details: "Contract or API surfaces drifted and need explicit acceptance before readiness can pass.".to_string(),
                changed_surfaces: changed_surfaces
                    .iter()
                    .filter(|surface| is_contract_surface(surface))
                    .cloned()
                    .collect(),
            });
        }

        if source_changed && !tests_changed {
            findings.push(ReviewFinding {
                category: FindingCategory::MissingTests,
                severity: FindingSeverity::MustFix,
                title: "Source changes lack companion verification updates".to_string(),
                details: "Changed source files do not have adjacent test-surface updates in the reviewed diff.".to_string(),
                changed_surfaces: changed_surfaces
                    .iter()
                    .filter(|surface| is_source_surface(surface))
                    .cloned()
                    .collect(),
            });
        }

        if !surprising_surface_area.is_empty() {
            findings.push(ReviewFinding {
                category: FindingCategory::DecisionImpact,
                severity: FindingSeverity::MustFix,
                title: "High-impact surfaces imply hidden decisions".to_string(),
                details: "Boundary or contract changes imply architectural consequences that need an explicit reviewer disposition.".to_string(),
                changed_surfaces: surprising_surface_area.clone(),
            });
        }

        if findings.is_empty() {
            findings.push(ReviewFinding {
                category: FindingCategory::DuplicationCheck,
                severity: FindingSeverity::Note,
                title: "No material duplication concerns inferred".to_string(),
                details:
                    "The diff remains bounded and changed tests moved with the source surface."
                        .to_string(),
                changed_surfaces: changed_surfaces.clone(),
            });
        }

        Self {
            base_ref: base_ref.to_string(),
            head_ref: head_ref.to_string(),
            changed_surfaces,
            inferred_intent,
            surprising_surface_area,
            findings,
        }
    }

    pub fn findings_for(&self, category: FindingCategory) -> Vec<&ReviewFinding> {
        self.findings.iter().filter(|finding| finding.category == category).collect()
    }

    pub fn must_fix_findings(&self) -> Vec<&ReviewFinding> {
        self.findings
            .iter()
            .filter(|finding| matches!(finding.severity, FindingSeverity::MustFix))
            .collect()
    }

    pub fn note_findings(&self) -> Vec<&ReviewFinding> {
        self.findings
            .iter()
            .filter(|finding| matches!(finding.severity, FindingSeverity::Note))
            .collect()
    }
}

fn is_source_surface(surface: &str) -> bool {
    let normalized = surface.to_ascii_lowercase();
    normalized.starts_with("src/") || normalized.contains("/src/")
}

fn is_test_surface(surface: &str) -> bool {
    let normalized = surface.to_ascii_lowercase();
    normalized.starts_with("tests/")
        || normalized.contains("/tests/")
        || normalized.ends_with("_test.rs")
        || normalized.ends_with("_tests.rs")
        || normalized.ends_with(".snap")
}

fn is_contract_surface(surface: &str) -> bool {
    let normalized = surface.to_ascii_lowercase();
    normalized.contains("contract") || normalized.contains("api") || normalized.contains("schema")
}

fn is_boundary_surface(surface: &str) -> bool {
    let normalized = surface.to_ascii_lowercase();
    normalized.contains("boundary")
        || normalized.contains("public")
        || normalized.contains("interface")
}
