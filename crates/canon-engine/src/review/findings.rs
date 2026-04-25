use serde::{Deserialize, Serialize};
use strum_macros::{Display, IntoStaticStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, IntoStaticStr)]
#[strum(serialize_all = "kebab-case")]
pub enum FindingCategory {
    BoundaryCheck,
    DuplicationCheck,
    ContractDrift,
    MissingTests,
    DecisionImpact,
}

impl FindingCategory {
    pub fn as_str(self) -> &'static str {
        self.into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, IntoStaticStr)]
#[strum(serialize_all = "kebab-case")]
pub enum FindingSeverity {
    Note,
    MustFix,
}

impl FindingSeverity {
    pub fn as_str(self) -> &'static str {
        self.into()
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

impl ReviewFinding {
    pub fn conventional_comment_kind(&self) -> &'static str {
        match (self.severity, self.category) {
            (FindingSeverity::MustFix, FindingCategory::BoundaryCheck) => "issue",
            (FindingSeverity::MustFix, FindingCategory::ContractDrift) => "issue",
            (FindingSeverity::MustFix, FindingCategory::MissingTests) => "todo",
            (FindingSeverity::MustFix, FindingCategory::DecisionImpact) => "question",
            (FindingSeverity::Note, FindingCategory::DuplicationCheck) => "praise",
            (FindingSeverity::Note, _) => "thought",
            (FindingSeverity::MustFix, _) => "issue",
        }
    }
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

    pub fn from_evidence(
        base_ref: &str,
        head_ref: &str,
        changed_surfaces: Vec<String>,
        patch: &str,
        critique_summary: &str,
    ) -> Self {
        let mut packet = Self::from_diff(base_ref, head_ref, changed_surfaces, patch);

        if !critique_summary.trim().is_empty() {
            packet.inferred_intent = format!(
                "{}\n\nGoverned critique evidence: {}",
                packet.inferred_intent, critique_summary
            );
        }

        packet
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

#[cfg(test)]
mod tests {
    use super::{FindingCategory, FindingSeverity, ReviewPacket};

    #[test]
    fn from_diff_emits_expected_must_fix_findings_for_boundary_and_contract_changes() {
        let packet = ReviewPacket::from_diff(
            "origin/main",
            "HEAD",
            vec![
                "src/boundary/router.rs".to_string(),
                "contracts/schema.json".to_string(),
                "src/service.rs".to_string(),
            ],
            "@@ -1,2 +1,3 @@\n-old\n+new\n",
        );

        assert_eq!(packet.must_fix_findings().len(), 4);
        assert!(packet.note_findings().is_empty());
        assert_eq!(packet.findings_for(FindingCategory::BoundaryCheck).len(), 1);
        assert_eq!(packet.findings_for(FindingCategory::ContractDrift).len(), 1);
        assert_eq!(packet.findings_for(FindingCategory::MissingTests).len(), 1);
        assert_eq!(packet.findings_for(FindingCategory::DecisionImpact).len(), 1);
        assert!(packet.surprising_surface_area.contains(&"src/boundary/router.rs".to_string()));
        assert!(packet.surprising_surface_area.contains(&"contracts/schema.json".to_string()));
    }

    #[test]
    fn from_diff_falls_back_to_note_when_tests_move_with_source() {
        let packet = ReviewPacket::from_diff(
            "origin/main",
            "HEAD",
            vec!["src/lib.rs".to_string(), "tests/lib_test.rs".to_string()],
            "@@ -1 +1 @@\n-old\n+new\n",
        );

        assert!(packet.must_fix_findings().is_empty());
        let notes = packet.note_findings();
        assert_eq!(notes.len(), 1);
        assert_eq!(notes[0].category, FindingCategory::DuplicationCheck);
        assert_eq!(notes[0].severity, FindingSeverity::Note);
    }

    #[test]
    fn from_evidence_appends_critique_summary_to_intent() {
        let packet = ReviewPacket::from_evidence(
            "origin/main",
            "HEAD",
            vec!["tests/lib_test.rs".to_string()],
            "@@ -0,0 +1 @@\n+test\n",
            "Independent critique highlighted missing rollback notes.",
        );

        assert!(packet.inferred_intent.contains(
            "Governed critique evidence: Independent critique highlighted missing rollback notes."
        ));
    }

    #[test]
    fn pr_review_conventional_comment_mapping_matches_first_slice_table() {
        let packet = ReviewPacket::from_diff(
            "origin/main",
            "HEAD",
            vec![
                "src/boundary/router.rs".to_string(),
                "contracts/schema.json".to_string(),
                "src/service.rs".to_string(),
            ],
            "@@ -1,2 +1,3 @@\n-old\n+new\n",
        );

        let rendered_expectations = [
            (FindingCategory::BoundaryCheck, FindingSeverity::MustFix, "issue"),
            (FindingCategory::ContractDrift, FindingSeverity::MustFix, "issue"),
            (FindingCategory::MissingTests, FindingSeverity::MustFix, "todo"),
            (FindingCategory::DecisionImpact, FindingSeverity::MustFix, "question"),
        ];

        for (category, severity, expected_kind) in rendered_expectations {
            let finding = packet
                .findings
                .iter()
                .find(|finding| finding.category == category && finding.severity == severity)
                .expect("expected review finding");
            assert_eq!(
                finding.conventional_comment_kind(),
                expected_kind,
                "unexpected comment kind for {category:?}"
            );
        }

        let note_packet = ReviewPacket::from_diff(
            "origin/main",
            "HEAD",
            vec!["src/lib.rs".to_string(), "tests/lib_test.rs".to_string()],
            "@@ -1 +1 @@\n-old\n+new\n",
        );
        let note = note_packet.note_findings().pop().expect("note finding");
        assert_eq!(note.conventional_comment_kind(), "praise");
    }
}
