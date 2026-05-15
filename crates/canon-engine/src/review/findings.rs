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

/// Scope at which a [`ReviewFinding`] applies within the reviewed diff.
///
/// Scope is derived deterministically from the finding's `changed_surfaces`:
/// - [`Pr`](ConventionalCommentScope::Pr): no surfaces, applies at the PR level.
/// - [`Surface`](ConventionalCommentScope::Surface): all surfaces belong to one functional group.
/// - [`File`](ConventionalCommentScope::File): surfaces span multiple functional groups.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, IntoStaticStr)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum ConventionalCommentScope {
    Pr,
    File,
    Surface,
}

impl ConventionalCommentScope {
    /// Returns the kebab-case string representation of the scope.
    pub fn as_str(self) -> &'static str {
        self.into()
    }

    /// Returns all variants in declaration order.
    pub fn all() -> &'static [Self] {
        &[Self::Pr, Self::File, Self::Surface]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewFinding {
    pub category: FindingCategory,
    pub severity: FindingSeverity,
    pub title: String,
    pub details: String,
    /// Scope of this finding, derived deterministically from `changed_surfaces`.
    pub scope: ConventionalCommentScope,
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
            let boundary_surfaces: Vec<String> = changed_surfaces
                .iter()
                .filter(|surface| is_boundary_surface(surface))
                .cloned()
                .collect();
            let scope = derive_scope(&boundary_surfaces);
            findings.push(ReviewFinding {
                category: FindingCategory::BoundaryCheck,
                severity: FindingSeverity::MustFix,
                title: "Boundary-marked surfaces changed".to_string(),
                details: "Public or boundary-marked files changed and require explicit reviewer disposition.".to_string(),
                scope,
                changed_surfaces: boundary_surfaces,
            });
        }

        if changed_surfaces.iter().any(|surface| is_contract_surface(surface)) {
            let contract_surfaces: Vec<String> = changed_surfaces
                .iter()
                .filter(|surface| is_contract_surface(surface))
                .cloned()
                .collect();
            let scope = derive_scope(&contract_surfaces);
            findings.push(ReviewFinding {
                category: FindingCategory::ContractDrift,
                severity: FindingSeverity::MustFix,
                title: "Contract-facing files changed".to_string(),
                details: "Contract or API surfaces drifted and need explicit acceptance before readiness can pass.".to_string(),
                scope,
                changed_surfaces: contract_surfaces,
            });
        }

        if source_changed && !tests_changed {
            let source_surfaces: Vec<String> = changed_surfaces
                .iter()
                .filter(|surface| is_source_surface(surface))
                .cloned()
                .collect();
            let scope = derive_scope(&source_surfaces);
            findings.push(ReviewFinding {
                category: FindingCategory::MissingTests,
                severity: FindingSeverity::MustFix,
                title: "Source changes lack companion verification updates".to_string(),
                details: "Changed source files do not have adjacent test-surface updates in the reviewed diff.".to_string(),
                scope,
                changed_surfaces: source_surfaces,
            });
        }

        if !surprising_surface_area.is_empty() {
            let scope = derive_scope(&surprising_surface_area);
            findings.push(ReviewFinding {
                category: FindingCategory::DecisionImpact,
                severity: FindingSeverity::MustFix,
                title: "High-impact surfaces imply hidden decisions".to_string(),
                details: "Boundary or contract changes imply architectural consequences that need an explicit reviewer disposition.".to_string(),
                scope,
                changed_surfaces: surprising_surface_area.clone(),
            });
        }

        if findings.is_empty() {
            let scope = derive_scope(&changed_surfaces);
            findings.push(ReviewFinding {
                category: FindingCategory::DuplicationCheck,
                severity: FindingSeverity::Note,
                title: "No material duplication concerns inferred".to_string(),
                details:
                    "The diff remains bounded and changed tests moved with the source surface."
                        .to_string(),
                scope,
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

/// Derives a [`ConventionalCommentScope`] from a set of changed surfaces.
///
/// - Returns [`Pr`](ConventionalCommentScope::Pr) when no surfaces are present.
/// - Returns [`Surface`](ConventionalCommentScope::Surface) when every surface
///   belongs to the same functional group (test, source, contract, or boundary).
/// - Returns [`File`](ConventionalCommentScope::File) in all other cases.
fn derive_scope(changed_surfaces: &[String]) -> ConventionalCommentScope {
    if changed_surfaces.is_empty() {
        return ConventionalCommentScope::Pr;
    }
    let all_test = changed_surfaces.iter().all(|s| is_test_surface(s));
    let all_source = changed_surfaces.iter().all(|s| is_source_surface(s));
    let all_contract = changed_surfaces.iter().all(|s| is_contract_surface(s));
    let all_boundary = changed_surfaces.iter().all(|s| is_boundary_surface(s));
    if all_test || all_source || all_contract || all_boundary {
        ConventionalCommentScope::Surface
    } else {
        ConventionalCommentScope::File
    }
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

    #[test]
    fn conventional_comment_scope_serde_round_trip() {
        use super::ConventionalCommentScope;
        let cases = [
            (ConventionalCommentScope::Pr, "\"pr\""),
            (ConventionalCommentScope::File, "\"file\""),
            (ConventionalCommentScope::Surface, "\"surface\""),
        ];
        for (variant, expected_json) in cases {
            let serialized = serde_json::to_string(&variant).expect("serialized");
            assert_eq!(serialized, expected_json, "variant {variant:?}");
            let deserialized: ConventionalCommentScope =
                serde_json::from_str(&serialized).expect("deserialized");
            assert_eq!(deserialized, variant);
        }
    }

    #[test]
    fn conventional_comment_scope_as_str() {
        use super::ConventionalCommentScope;
        assert_eq!(ConventionalCommentScope::Pr.as_str(), "pr");
        assert_eq!(ConventionalCommentScope::File.as_str(), "file");
        assert_eq!(ConventionalCommentScope::Surface.as_str(), "surface");
    }

    #[test]
    fn scope_pr_when_no_surfaces() {
        // derive_scope returns Pr when no surfaces are present.
        use super::{ConventionalCommentScope, derive_scope};
        assert_eq!(derive_scope(&[]), ConventionalCommentScope::Pr);
        // An empty-surface diff still produces the DuplicationCheck note finding
        // (from the `if findings.is_empty()` fallback), with File scope because
        // changed_surfaces is empty and derive_scope yields Pr.
        let packet = ReviewPacket::from_diff("main", "HEAD", vec![], "");
        // The note finding uses derive_scope on the empty changed_surfaces clone,
        // yielding Pr scope.
        if let Some(note) = packet.findings.first() {
            assert_eq!(note.scope, ConventionalCommentScope::Pr);
        }
    }

    #[test]
    fn scope_surface_when_all_test_surfaces() {
        use super::{ConventionalCommentScope, derive_scope};
        let surfaces = vec!["tests/foo_test.rs".to_string(), "tests/bar_test.rs".to_string()];
        assert_eq!(derive_scope(&surfaces), ConventionalCommentScope::Surface);
    }

    #[test]
    fn scope_surface_when_all_contract_surfaces() {
        use super::{ConventionalCommentScope, derive_scope};
        let surfaces = vec!["contracts/api.json".to_string(), "contracts/schema.json".to_string()];
        assert_eq!(derive_scope(&surfaces), ConventionalCommentScope::Surface);
    }

    #[test]
    fn scope_surface_when_all_source_surfaces() {
        use super::{ConventionalCommentScope, derive_scope};
        let surfaces = vec!["src/lib.rs".to_string(), "src/main.rs".to_string()];
        assert_eq!(derive_scope(&surfaces), ConventionalCommentScope::Surface);
    }

    #[test]
    fn scope_file_when_mixed_surfaces() {
        use super::{ConventionalCommentScope, derive_scope};
        let surfaces = vec!["src/lib.rs".to_string(), "tests/lib_test.rs".to_string()];
        assert_eq!(derive_scope(&surfaces), ConventionalCommentScope::File);
    }

    #[test]
    fn scope_file_when_non_empty_but_no_single_group() {
        use super::{ConventionalCommentScope, derive_scope};
        let surfaces = vec!["src/lib.rs".to_string(), "contracts/api.json".to_string()];
        assert_eq!(derive_scope(&surfaces), ConventionalCommentScope::File);
    }

    #[test]
    fn from_diff_boundary_finding_has_surface_scope() {
        use super::ConventionalCommentScope;
        let packet = ReviewPacket::from_diff(
            "main",
            "HEAD",
            vec!["src/public/api.rs".to_string()],
            "@@ -1 +1 @@\n-a\n+b\n",
        );
        let finding = packet
            .findings
            .iter()
            .find(|f| f.category == FindingCategory::BoundaryCheck)
            .expect("boundary finding");
        assert_eq!(finding.scope, ConventionalCommentScope::Surface);
    }

    #[test]
    fn from_diff_note_finding_scope_derived_from_surfaces() {
        use super::ConventionalCommentScope;
        let packet = ReviewPacket::from_diff(
            "main",
            "HEAD",
            vec!["src/lib.rs".to_string(), "tests/lib_test.rs".to_string()],
            "@@ -1 +1 @@\n-a\n+b\n",
        );
        let note = packet.note_findings().pop().expect("note finding");
        // Mixed src + tests → File scope
        assert_eq!(note.scope, ConventionalCommentScope::File);
    }
}
