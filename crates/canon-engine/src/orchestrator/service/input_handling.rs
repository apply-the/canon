use super::EngineService;
use super::*;
use crate::domain::run::{
    ClarificationAnswerKind, ClarificationRecord, ClarificationResolutionState,
    ReadinessDeltaSourceKind,
};

const AUTHORITY_GAP_READINESS_ID: &str = "rd-authority-gap";
const AUTHORITY_GAP_READINESS_SECTION: &str = "Authority";
const AUTHORITY_GAP_READINESS_SUMMARY: &str = "Canon could not identify one authoritative current-mode brief from the supplied inputs; add `brief.md` or reduce the packet to one clear readiness brief.";
const CLARIFICATION_GAP_READINESS_ID: &str = "rd-clarification-gap";
const CLARIFICATION_GAP_READINESS_SECTION: &str = "Clarification";
const DEFERRED_CLARIFICATION_ANSWER: &str = "deferred";
const MISSING_CONTEXT_READINESS_ID_PREFIX: &str = "rd-missing-context";
const MISSING_CONTEXT_READINESS_SECTION: &str = "Missing Context";
const SUPPORTING_INPUT_WARNING_READINESS_ID: &str = "rd-supporting-input-warning";
const SUPPORTING_INPUT_WARNING_READINESS_SECTION: &str = "Supporting Inputs";
const SUPPORTING_INPUT_WARNING_READINESS_SUMMARY: &str = "Supporting inputs are present, but they do not replace the current-mode brief Canon uses for readiness.";

impl EngineService {
    pub(super) fn collect_input_files(&self, input: &str) -> Result<Vec<PathBuf>, EngineError> {
        let resolved = self.resolve_input_path(input);
        if resolved.is_file() {
            return Ok(vec![resolved]);
        }
        if resolved.is_dir() {
            let mut files = Vec::new();
            collect_files_recursively(&resolved, &mut files)?;
            files.sort();
            return Ok(files);
        }

        Ok(Vec::new())
    }

    pub(super) fn collect_content_input_files(
        &self,
        input: &str,
    ) -> Result<Vec<PathBuf>, EngineError> {
        Ok(self
            .collect_input_files(input)?
            .into_iter()
            .filter(|path| !is_known_mutation_payload_file(path))
            .collect())
    }

    pub(super) fn validate_review_authored_input_path(
        &self,
        inputs: &[String],
    ) -> Result<(), EngineError> {
        const REVIEW_INPUT_HINT: &str = "canon-input/review.md or canon-input/review/";

        if inputs.len() != 1 {
            return Err(EngineError::Validation(format!(
                "review requires exactly one authored input at {REVIEW_INPUT_HINT}"
            )));
        }

        let input = &inputs[0];
        let resolved = self.resolve_input_path(input);
        if !resolved.exists() {
            return Err(EngineError::Validation(format!(
                "review input `{input}` was not found from {}; expected {REVIEW_INPUT_HINT}",
                self.repo_root.display()
            )));
        }

        let resolved_canonical = resolved.canonicalize()?;
        let canonical_review_file = self.repo_root.join("canon-input").join("review.md");
        let canonical_review_dir = self.repo_root.join("canon-input").join("review");
        let mut allowed_paths = Vec::new();

        if canonical_review_file.exists() {
            allowed_paths.push(canonical_review_file.canonicalize()?);
        }
        if canonical_review_dir.exists() {
            allowed_paths.push(canonical_review_dir.canonicalize()?);
        }

        if !allowed_paths.iter().any(|path| path == &resolved_canonical) {
            return Err(EngineError::Validation(format!(
                "review accepts only {REVIEW_INPUT_HINT}, not `{input}`"
            )));
        }

        Ok(())
    }

    pub(super) fn validate_authored_input_paths(
        &self,
        mode: Mode,
        inputs: &[String],
    ) -> Result<(), EngineError> {
        if matches!(mode, Mode::PrReview) {
            return Ok(());
        }

        if matches!(mode, Mode::Review) && !inputs.is_empty() {
            self.validate_review_authored_input_path(inputs)?;
        }

        let canon_root = self.canonical_canon_root()?;

        for input in inputs {
            self.validate_single_authored_input_path(mode, input, canon_root.as_deref())?;
        }

        Ok(())
    }

    pub(super) fn validate_authored_inputs(
        &self,
        mode: Mode,
        inputs: &[String],
        inline_inputs: &[String],
    ) -> Result<(), EngineError> {
        Self::validate_inline_input_mode_constraints(mode, inline_inputs)?;
        Self::validate_authored_source_count(mode, inputs.len() + inline_inputs.len())?;

        self.validate_authored_input_paths(mode, inputs)?;
        Self::validate_inline_input_contents(mode, inline_inputs)?;

        Ok(())
    }

    pub(super) fn persisted_input_path(&self, resolved: &Path) -> String {
        resolved
            .strip_prefix(&self.repo_root)
            .map(|path| path.to_string_lossy().into_owned())
            .unwrap_or_else(|_| resolved.display().to_string())
    }

    pub(super) fn build_authoring_lifecycle_summary(
        &self,
        inputs: &[String],
        source_inputs: &[String],
        missing_context: &[String],
        clarification_questions: &[ClarificationQuestionSummary],
        materially_closed: bool,
    ) -> AuthoringLifecycleSummary {
        let resolved_inputs =
            inputs.iter().map(|input| self.resolve_input_path(input)).collect::<Vec<_>>();
        let packet_shape = Self::classify_packet_shape(&resolved_inputs);
        let normalized_source_inputs = Self::normalize_source_inputs(source_inputs);
        let authoritative_inputs = Self::select_authoritative_inputs(&normalized_source_inputs);
        let supporting_inputs =
            Self::select_supporting_inputs(&normalized_source_inputs, &authoritative_inputs);
        let authority_status = Self::classify_authority_status(packet_shape, &authoritative_inputs);
        let readiness_delta = Self::build_readiness_delta(
            authority_status,
            missing_context,
            clarification_questions,
            &supporting_inputs,
        );
        let next_authoring_step = Self::build_next_authoring_step(
            authority_status,
            missing_context,
            clarification_questions,
            materially_closed,
            &supporting_inputs,
        );

        AuthoringLifecycleSummary {
            packet_shape: packet_shape.as_str().to_string(),
            authority_status: authority_status.as_str().to_string(),
            authoritative_inputs,
            supporting_inputs,
            readiness_delta,
            next_authoring_step,
        }
    }

    pub(super) fn refinement_source_inputs(
        &self,
        inputs: &[String],
        inline_inputs: &[String],
    ) -> Result<Vec<String>, EngineError> {
        let mut source_inputs = self.clarity_source_inputs(inputs)?;
        source_inputs
            .extend(inline_inputs.iter().enumerate().map(|(index, _)| inline_input_label(index)));

        Ok(Self::normalize_source_inputs(&source_inputs))
    }

    pub(super) fn build_refinement_clarification_records(
        clarification_questions: &[ClarificationQuestionSummary],
        recorded_at: OffsetDateTime,
    ) -> Vec<ClarificationRecord> {
        clarification_questions
            .iter()
            .map(|question| ClarificationRecord {
                id: question.id.clone(),
                prompt: question.prompt.clone(),
                answer: DEFERRED_CLARIFICATION_ANSWER.to_string(),
                answer_kind: ClarificationAnswerKind::Deferred,
                affected_sections: if question.affects.trim().is_empty() {
                    Vec::new()
                } else {
                    vec![question.affects.clone()]
                },
                resolution_state: ClarificationResolutionState::Deferred,
                recorded_at,
            })
            .collect()
    }

    pub(super) fn build_structured_refinement_readiness_items(
        authoring_lifecycle: &AuthoringLifecycleSummary,
        missing_context: &[String],
        clarification_questions: &[ClarificationQuestionSummary],
    ) -> Vec<ReadinessDeltaItem> {
        let mut readiness_items = Vec::new();
        let authority_gap =
            authoring_lifecycle.authority_status == AuthorityStatus::AmbiguousCurrentBrief.as_str();

        if authority_gap {
            readiness_items.push(ReadinessDeltaItem {
                id: AUTHORITY_GAP_READINESS_ID.to_string(),
                section: AUTHORITY_GAP_READINESS_SECTION.to_string(),
                summary: AUTHORITY_GAP_READINESS_SUMMARY.to_string(),
                blocking: true,
                source_kind: ReadinessDeltaSourceKind::AuthorityGap,
                default_available: false,
                resolved: false,
            });
        }

        readiness_items.extend(missing_context.iter().enumerate().map(|(index, summary)| {
            ReadinessDeltaItem {
                id: format!("{MISSING_CONTEXT_READINESS_ID_PREFIX}-{index:02}"),
                section: MISSING_CONTEXT_READINESS_SECTION.to_string(),
                summary: summary.clone(),
                blocking: true,
                source_kind: ReadinessDeltaSourceKind::MissingContext,
                default_available: false,
                resolved: false,
            }
        }));

        if !clarification_questions.is_empty() {
            readiness_items.push(ReadinessDeltaItem {
                id: CLARIFICATION_GAP_READINESS_ID.to_string(),
                section: CLARIFICATION_GAP_READINESS_SECTION.to_string(),
                summary: format!(
                    "{} clarification question(s) still remain before this packet is unambiguously ready.",
                    clarification_questions.len()
                ),
                blocking: true,
                source_kind: ReadinessDeltaSourceKind::ClarificationGap,
                default_available: clarification_questions
                    .iter()
                    .any(|question| !question.default_if_skipped.trim().is_empty()),
                resolved: false,
            });
        }

        if !authoring_lifecycle.supporting_inputs.is_empty()
            && (authority_gap || !missing_context.is_empty())
        {
            readiness_items.push(ReadinessDeltaItem {
                id: SUPPORTING_INPUT_WARNING_READINESS_ID.to_string(),
                section: SUPPORTING_INPUT_WARNING_READINESS_SECTION.to_string(),
                summary: SUPPORTING_INPUT_WARNING_READINESS_SUMMARY.to_string(),
                blocking: false,
                source_kind: ReadinessDeltaSourceKind::SupportingInputWarning,
                default_available: false,
                resolved: false,
            });
        }

        readiness_items
    }

    fn canonical_canon_root(&self) -> Result<Option<PathBuf>, EngineError> {
        self.canon_runtime_dir()
            .exists()
            .then(|| self.canon_runtime_dir().canonicalize())
            .transpose()
            .map_err(Into::into)
    }

    fn validate_single_authored_input_path(
        &self,
        mode: Mode,
        input: &str,
        canon_root: Option<&Path>,
    ) -> Result<(), EngineError> {
        let resolved = self.resolve_input_path(input);
        if !resolved.exists() {
            return Err(EngineError::Validation(format!(
                "input `{input}` was not found from {}",
                self.repo_root.display()
            )));
        }

        let canonical = resolved.canonicalize()?;
        if canon_root.is_some_and(|root| canonical.starts_with(root)) {
            return Err(EngineError::Validation(format!(
                "input `{input}` points inside .canon/ and cannot be used as authored input for {}",
                mode.as_str()
            )));
        }

        let files = self.collect_content_input_files(input)?;
        Self::validate_authored_input_content(input, &resolved, &files)
    }

    fn validate_authored_input_content(
        input: &str,
        resolved: &Path,
        files: &[PathBuf],
    ) -> Result<(), EngineError> {
        if resolved.is_dir() && files.is_empty() {
            return Err(EngineError::Validation(format!(
                "input `{input}` is an empty directory and does not contain authored content"
            )));
        }

        if Self::files_have_usable_content(files)? {
            return Ok(());
        }

        let message = if resolved.is_dir() {
            format!("input `{input}` expands to files with no usable authored content")
        } else {
            format!("input `{input}` is empty or whitespace-only")
        };
        Err(EngineError::Validation(message))
    }

    fn files_have_usable_content(files: &[PathBuf]) -> Result<bool, EngineError> {
        for file in files {
            let contents = std::fs::read_to_string(file)?;
            if !contents.trim().is_empty() {
                return Ok(true);
            }
        }

        Ok(false)
    }

    fn validate_inline_input_mode_constraints(
        mode: Mode,
        inline_inputs: &[String],
    ) -> Result<(), EngineError> {
        if matches!(mode, Mode::PrReview) && !inline_inputs.is_empty() {
            return Err(EngineError::Validation(
                "pr-review does not support --input-text; pass two refs via --input".to_string(),
            ));
        }

        Ok(())
    }

    fn validate_authored_source_count(mode: Mode, source_count: usize) -> Result<(), EngineError> {
        if matches!(mode, Mode::PrReview) {
            return Ok(());
        }

        if matches!(mode, Mode::Review) {
            if source_count != 1 {
                return Err(EngineError::Validation(
                    "review requires exactly one authored input at canon-input/review.md or canon-input/review/, or exactly one --input-text value"
                        .to_string(),
                ));
            }
            return Ok(());
        }

        if source_count == 0 {
            return Err(EngineError::Validation(format!(
                "{} requires at least one authored input via --input or --input-text",
                mode.as_str()
            )));
        }

        Ok(())
    }

    fn validate_inline_input_contents(
        mode: Mode,
        inline_inputs: &[String],
    ) -> Result<(), EngineError> {
        for (index, inline_input) in inline_inputs.iter().enumerate() {
            if inline_input.trim().is_empty() {
                return Err(EngineError::Validation(format!(
                    "inline input {} for {} is empty or whitespace-only",
                    index + 1,
                    mode.as_str()
                )));
            }
        }

        Ok(())
    }

    fn classify_packet_shape(resolved_inputs: &[PathBuf]) -> PacketShape {
        let directory_roots =
            resolved_inputs.iter().filter(|path| path.is_dir()).cloned().collect::<Vec<_>>();
        if directory_roots.len() == 1
            && resolved_inputs
                .iter()
                .all(|path| path == &directory_roots[0] || path.starts_with(&directory_roots[0]))
        {
            PacketShape::DirectoryBacked
        } else if resolved_inputs.len() == 1 && resolved_inputs[0].is_file() {
            PacketShape::SingleFile
        } else {
            PacketShape::MultiInput
        }
    }

    fn normalize_source_inputs(source_inputs: &[String]) -> Vec<String> {
        source_inputs.iter().cloned().collect::<BTreeSet<_>>().into_iter().collect::<Vec<_>>()
    }

    fn select_authoritative_inputs(normalized_source_inputs: &[String]) -> Vec<String> {
        if normalized_source_inputs
            .iter()
            .any(|path| Self::authored_input_name(path) == Some("brief.md"))
        {
            return normalized_source_inputs
                .iter()
                .filter(|path| Self::authored_input_name(path) == Some("brief.md"))
                .cloned()
                .collect::<Vec<_>>();
        }

        if normalized_source_inputs.len() == 1 {
            return normalized_source_inputs.to_vec();
        }

        Vec::new()
    }

    fn select_supporting_inputs(
        normalized_source_inputs: &[String],
        authoritative_inputs: &[String],
    ) -> Vec<String> {
        let authoritative_lookup = authoritative_inputs.iter().cloned().collect::<BTreeSet<_>>();
        normalized_source_inputs
            .iter()
            .filter(|path| !authoritative_lookup.contains(*path))
            .cloned()
            .collect::<Vec<_>>()
    }

    fn classify_authority_status(
        packet_shape: PacketShape,
        authoritative_inputs: &[String],
    ) -> AuthorityStatus {
        if authoritative_inputs
            .iter()
            .any(|path| Self::authored_input_name(path) == Some("brief.md"))
        {
            AuthorityStatus::ExplicitAuthoritativeBrief
        } else if packet_shape == PacketShape::SingleFile && !authoritative_inputs.is_empty() {
            AuthorityStatus::SingleInputAuthoritativeBrief
        } else if !authoritative_inputs.is_empty() {
            AuthorityStatus::DerivedAuthoritativeInput
        } else {
            AuthorityStatus::AmbiguousCurrentBrief
        }
    }

    fn build_readiness_delta(
        authority_status: AuthorityStatus,
        missing_context: &[String],
        clarification_questions: &[ClarificationQuestionSummary],
        supporting_inputs: &[String],
    ) -> Vec<String> {
        let mut readiness_delta = Vec::new();
        if authority_status == AuthorityStatus::AmbiguousCurrentBrief {
            readiness_delta.push(
                "Canon could not identify one authoritative current-mode brief from the supplied inputs; add `brief.md` or reduce the packet to one clear readiness brief."
                    .to_string(),
            );
        }
        readiness_delta.extend(missing_context.iter().cloned());
        if !clarification_questions.is_empty() {
            readiness_delta.push(format!(
                "{} clarification question(s) still remain before this packet is unambiguously ready.",
                clarification_questions.len()
            ));
        }
        if !supporting_inputs.is_empty()
            && (authority_status == AuthorityStatus::AmbiguousCurrentBrief
                || !missing_context.is_empty())
        {
            readiness_delta.push(
                "Supporting inputs are present, but they do not replace the current-mode brief Canon uses for readiness."
                    .to_string(),
            );
        }

        readiness_delta
    }

    pub(super) fn build_refinement_readiness_delta(
        readiness_items: &[ReadinessDeltaItem],
    ) -> Vec<String> {
        readiness_items
            .iter()
            .filter(|item| !item.resolved)
            .map(|item| item.summary.clone())
            .collect()
    }

    fn build_next_authoring_step(
        authority_status: AuthorityStatus,
        missing_context: &[String],
        clarification_questions: &[ClarificationQuestionSummary],
        materially_closed: bool,
        supporting_inputs: &[String],
    ) -> String {
        if authority_status == AuthorityStatus::AmbiguousCurrentBrief {
            "Tighten the packet so one current-mode brief is authoritative before relying on the supporting files.".to_string()
        } else if !missing_context.is_empty() {
            "Strengthen the authoritative brief by resolving the named missing-context items before starting the governed run.".to_string()
        } else if !clarification_questions.is_empty() {
            "Answer the remaining clarification questions in the authoritative brief or supporting notes before treating the packet as fully ready.".to_string()
        } else if materially_closed {
            "Packet authority is explicit and the brief already materially closes the decision; preserve that closure when you start the governed run.".to_string()
        } else if !supporting_inputs.is_empty() {
            "Packet authority is explicit; keep the supporting inputs as provenance and move to the matching governed run when ready.".to_string()
        } else {
            "Packet authority is explicit and the brief is ready for the matching governed run."
                .to_string()
        }
    }

    pub(super) fn resolve_input_path(&self, input: &str) -> PathBuf {
        let path = PathBuf::from(input);
        if path.is_absolute() { path } else { self.repo_root.join(path) }
    }

    pub(super) fn auto_bind_canonical_mode_inputs(
        &self,
        mode: Mode,
        inputs: &[String],
        inline_inputs: &[String],
    ) -> Vec<String> {
        if !inputs.is_empty() || !inline_inputs.is_empty() {
            return inputs.to_vec();
        }

        let Some((file_name, dir_name)) = canonical_mode_input_binding(mode) else {
            return Vec::new();
        };

        let canonical_root = self.repo_root.join("canon-input");
        let canonical_dir = canonical_root.join(dir_name);
        if canonical_dir.exists() {
            return vec![format!("canon-input/{dir_name}")];
        }

        let canonical_file = canonical_root.join(file_name);
        if canonical_file.exists() {
            return vec![format!("canon-input/{file_name}")];
        }

        Vec::new()
    }

    /// Extracts base and head refs from pr-review inputs.
    #[allow(dead_code)]
    pub(super) fn load_pr_review_refs(
        &self,
        inputs: &[String],
    ) -> Result<(String, String), EngineError> {
        if inputs.len() < 2 {
            return Err(EngineError::Validation(
                "pr-review requires two inputs: <base-ref> <head-ref>".to_string(),
            ));
        }

        Ok((inputs[0].clone(), inputs[1].clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use time::OffsetDateTime;

    #[test]
    fn structured_refinement_readiness_items_include_authority_gap_and_supporting_warning() {
        let authoring_lifecycle = AuthoringLifecycleSummary {
            packet_shape: "multi-input".to_string(),
            authority_status: AuthorityStatus::AmbiguousCurrentBrief.as_str().to_string(),
            authoritative_inputs: vec![],
            supporting_inputs: vec!["context-links.md".to_string()],
            readiness_delta: vec![],
            next_authoring_step: "Add brief.md".to_string(),
        };
        let questions = vec![ClarificationQuestionSummary {
            id: "clarify-problem".to_string(),
            prompt: "What is the bounded problem?".to_string(),
            rationale: "Need a stable problem statement.".to_string(),
            evidence: "No problem heading found.".to_string(),
            affects: "problem statement".to_string(),
            default_if_skipped: "Carry forward unresolved.".to_string(),
            status: "required".to_string(),
        }];

        let items = EngineService::build_structured_refinement_readiness_items(
            &authoring_lifecycle,
            &["Outcome is missing.".to_string()],
            &questions,
        );

        assert_eq!(items[0].id, "rd-authority-gap");
        assert_eq!(items[0].source_kind, ReadinessDeltaSourceKind::AuthorityGap);
        assert_eq!(items[1].id, "rd-missing-context-00");
        assert_eq!(items[1].source_kind, ReadinessDeltaSourceKind::MissingContext);
        assert_eq!(items[2].id, "rd-clarification-gap");
        assert!(items[2].default_available);
        assert_eq!(items[3].id, "rd-supporting-input-warning");
        assert_eq!(items[3].source_kind, ReadinessDeltaSourceKind::SupportingInputWarning);
        assert!(!items[3].blocking);
    }

    #[test]
    fn structured_refinement_readiness_items_skip_supporting_warning_without_gap() {
        let authoring_lifecycle = AuthoringLifecycleSummary {
            packet_shape: "single-file".to_string(),
            authority_status: AuthorityStatus::SingleInputAuthoritativeBrief.as_str().to_string(),
            authoritative_inputs: vec!["brief.md".to_string()],
            supporting_inputs: vec!["context-links.md".to_string()],
            readiness_delta: vec![],
            next_authoring_step: "Proceed".to_string(),
        };

        let items = EngineService::build_structured_refinement_readiness_items(
            &authoring_lifecycle,
            &[],
            &[],
        );

        assert!(items.is_empty());
    }

    #[test]
    fn build_refinement_clarification_records_tracks_affected_sections() {
        let recorded_at = OffsetDateTime::now_utc();
        let questions = vec![
            ClarificationQuestionSummary {
                id: "q-1".to_string(),
                prompt: "Clarify the problem".to_string(),
                rationale: "Need the bounded problem".to_string(),
                evidence: "No explicit problem heading".to_string(),
                affects: "Problem".to_string(),
                default_if_skipped: "Carry unresolved".to_string(),
                status: "required".to_string(),
            },
            ClarificationQuestionSummary {
                id: "q-2".to_string(),
                prompt: "Clarify the scope".to_string(),
                rationale: "Need scope boundaries".to_string(),
                evidence: "Scope is implicit".to_string(),
                affects: "   ".to_string(),
                default_if_skipped: "Carry unresolved".to_string(),
                status: "required".to_string(),
            },
        ];

        let records =
            EngineService::build_refinement_clarification_records(&questions, recorded_at);

        assert_eq!(records.len(), 2);
        assert_eq!(records[0].answer, "deferred");
        assert_eq!(records[0].answer_kind, ClarificationAnswerKind::Deferred);
        assert_eq!(records[0].affected_sections, vec!["Problem".to_string()]);
        assert_eq!(records[0].resolution_state, ClarificationResolutionState::Deferred);
        assert!(records[1].affected_sections.is_empty());
    }

    #[test]
    fn validate_review_authored_input_path_accepts_only_canonical_review_locations() {
        let workspace = tempdir().expect("tempdir");
        let canon_input = workspace.path().join("canon-input");
        std::fs::create_dir_all(&canon_input).expect("create canon-input");
        std::fs::write(canon_input.join("review.md"), "# Review\n").expect("write review.md");
        std::fs::write(canon_input.join("notes.md"), "# Notes\n").expect("write notes.md");

        let service = EngineService::new(workspace.path());

        service
            .validate_review_authored_input_path(&["canon-input/review.md".to_string()])
            .expect("canonical review input should validate");

        let invalid = service
            .validate_review_authored_input_path(&["canon-input/notes.md".to_string()])
            .expect_err("non-canonical review path should be rejected");
        assert!(
            invalid
                .to_string()
                .contains("review accepts only canon-input/review.md or canon-input/review/")
        );

        let missing = service
            .validate_review_authored_input_path(&["canon-input/missing.md".to_string()])
            .expect_err("missing review path should be rejected");
        assert!(missing.to_string().contains("was not found"));
    }

    #[test]
    fn authored_input_validation_covers_source_count_inline_and_empty_directory_cases() {
        let workspace = tempdir().expect("tempdir");
        let service = EngineService::new(workspace.path());

        let missing_source = EngineService::validate_authored_source_count(Mode::Requirements, 0)
            .expect_err("requirements without sources should fail");
        assert!(missing_source.to_string().contains("requires at least one authored input"));

        let review_count = EngineService::validate_authored_source_count(Mode::Review, 2)
            .expect_err("review should require exactly one source");
        assert!(review_count.to_string().contains("review requires exactly one authored input"));

        let inline_mode = EngineService::validate_inline_input_mode_constraints(
            Mode::PrReview,
            &["inline review".to_string()],
        )
        .expect_err("pr-review inline input should fail");
        assert!(inline_mode.to_string().contains("does not support --input-text"));

        let inline_contents =
            EngineService::validate_inline_input_contents(Mode::Requirements, &["   ".to_string()])
                .expect_err("whitespace-only inline input should fail");
        assert!(inline_contents.to_string().contains("empty or whitespace-only"));

        let empty_dir = workspace.path().join("canon-input").join("requirements");
        std::fs::create_dir_all(&empty_dir).expect("create empty dir");
        let empty_dir_error = service
            .validate_authored_input_paths(
                Mode::Requirements,
                &["canon-input/requirements".to_string()],
            )
            .expect_err("empty authored directory should fail");
        assert!(
            empty_dir_error
                .to_string()
                .contains("is an empty directory and does not contain authored content")
        );
    }

    #[test]
    fn auto_bind_canonical_mode_inputs_prefers_directory_file_and_explicit_inputs() {
        let workspace = tempdir().expect("tempdir");
        let service = EngineService::new(workspace.path());
        let canon_input = workspace.path().join("canon-input");
        std::fs::create_dir_all(canon_input.join("backlog")).expect("create backlog dir");
        std::fs::write(canon_input.join("backlog").join("brief.md"), "# Backlog\n")
            .expect("write backlog brief");

        assert_eq!(
            service.auto_bind_canonical_mode_inputs(Mode::Backlog, &[], &[]),
            vec!["canon-input/backlog".to_string()]
        );

        let explicit =
            service.auto_bind_canonical_mode_inputs(Mode::Backlog, &["custom.md".to_string()], &[]);
        assert_eq!(explicit, vec!["custom.md".to_string()]);

        std::fs::remove_dir_all(canon_input.join("backlog")).expect("remove backlog dir");
        std::fs::write(canon_input.join("backlog.md"), "# Backlog\n").expect("write backlog file");
        assert_eq!(
            service.auto_bind_canonical_mode_inputs(Mode::Backlog, &[], &[]),
            vec!["canon-input/backlog.md".to_string()]
        );

        assert!(service.auto_bind_canonical_mode_inputs(Mode::Requirements, &[], &[]).is_empty());
    }

    #[test]
    fn persisted_input_paths_and_pr_review_refs_cover_remaining_helpers() {
        let workspace = tempdir().expect("tempdir");
        let service = EngineService::new(workspace.path());
        let inside = workspace.path().join("canon-input").join("requirements.md");
        std::fs::create_dir_all(inside.parent().expect("parent")).expect("create parent dir");
        std::fs::write(&inside, "# Requirements\n").expect("write inside file");

        assert_eq!(
            service.persisted_input_path(&inside),
            "canon-input/requirements.md".to_string()
        );

        let outside = tempdir().expect("outside tempdir");
        let outside_file = outside.path().join("external.md");
        std::fs::write(&outside_file, "# External\n").expect("write outside file");
        assert_eq!(service.persisted_input_path(&outside_file), outside_file.display().to_string());

        let refs = service
            .load_pr_review_refs(&["main".to_string(), "feature".to_string()])
            .expect("two refs should validate");
        assert_eq!(refs, ("main".to_string(), "feature".to_string()));

        let missing =
            service.load_pr_review_refs(&["main".to_string()]).expect_err("single ref should fail");
        assert!(missing.to_string().contains("pr-review requires two inputs"));
    }
}
