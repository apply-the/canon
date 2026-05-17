use super::EngineService;
use super::*;

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

    fn canonical_canon_root(&self) -> Result<Option<PathBuf>, EngineError> {
        self.repo_root
            .join(".canon")
            .exists()
            .then(|| self.repo_root.join(".canon").canonicalize())
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
