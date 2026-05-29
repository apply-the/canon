use super::EngineService;
use super::*;

impl EngineService {
    pub(super) fn load_input_summary(
        &self,
        inputs: &[String],
        inline_inputs: &[String],
    ) -> Result<String, EngineError> {
        let mut fragments = Vec::new();

        for input in inputs {
            let files = self.collect_content_input_files(input)?;
            if !files.is_empty() {
                for resolved in files {
                    let contents = std::fs::read_to_string(&resolved)?;
                    fragments.push(contents);
                }
            } else {
                fragments.push(input.clone());
            }
        }

        fragments.extend(inline_inputs.iter().cloned());

        let combined = fragments.join("\n");
        let normalized = preserve_multiline_summary(&combined);
        if normalized.is_empty() {
            Ok("Capture the bounded engineering need before implementation accelerates drift."
                .to_string())
        } else {
            Ok(normalized)
        }
    }

    pub(super) fn build_run_context(
        &self,
        request: &RunRequest,
        input_fingerprints: Vec<InputFingerprint>,
        captured_at: OffsetDateTime,
    ) -> RunContext {
        let (implementation_execution, refactor_execution) =
            self.scaffold_mode_execution_context(request);
        let upstream_context = self.scaffold_upstream_context(request);

        RunContext {
            repo_root: self.repo_root.display().to_string(),
            owner: Some(request.owner.clone()),
            inputs: request.merged_input_sources(),
            excluded_paths: request.excluded_paths.clone(),
            input_fingerprints,
            system_context: request.system_context,
            upstream_context,
            implementation_execution,
            refactor_execution,
            backlog_planning: None,
            clarification_refinement: None,
            inline_inputs: request.transient_inline_inputs(),
            captured_at,
        }
    }

    pub(super) fn build_run_context_with_refinement(
        &self,
        store: &WorkspaceStore,
        run_id: &str,
        request: &RunRequest,
        input_fingerprints: Vec<InputFingerprint>,
        captured_at: OffsetDateTime,
    ) -> Result<RunContext, EngineError> {
        let suggested_candidate =
            self.find_refinement_candidate(store, request.mode, &input_fingerprints)?;
        let mut context = self.build_run_context(request, input_fingerprints, captured_at);
        context.clarification_refinement = Some(self.build_refinement_context(
            run_id,
            request.mode,
            request,
            suggested_candidate,
            captured_at,
        )?);
        Ok(context)
    }

    pub(super) fn scaffold_upstream_context(
        &self,
        request: &RunRequest,
    ) -> Option<UpstreamContext> {
        if !matches!(request.mode, Mode::Implementation | Mode::Refactor) {
            return None;
        }

        let summary = self.load_input_summary(&request.inputs, &request.inline_inputs).ok()?;
        let normalized = summary.to_lowercase();
        let feature_slice = extract_marker(&summary, &normalized, "feature slice");
        let primary_upstream_mode = extract_marker(&summary, &normalized, "primary upstream mode");
        let source_refs = extract_first_marker_entries(&summary, &["upstream sources"]);
        let carried_forward_items = extract_first_marker_entries(
            &summary,
            &["carried-forward decisions", "carried-forward invariants"],
        );
        let excluded_upstream_scope =
            extract_marker(&summary, &normalized, "excluded upstream scope");

        if feature_slice.is_none()
            && primary_upstream_mode.is_none()
            && source_refs.is_empty()
            && carried_forward_items.is_empty()
            && excluded_upstream_scope.is_none()
        {
            None
        } else {
            Some(UpstreamContext {
                feature_slice,
                primary_upstream_mode,
                source_refs,
                carried_forward_items,
                excluded_upstream_scope,
            })
        }
    }

    pub(super) fn scaffold_mode_execution_context(
        &self,
        request: &RunRequest,
    ) -> (Option<ImplementationExecutionContext>, Option<RefactorExecutionContext>) {
        let source_refs = request.merged_input_sources();
        let owners =
            if !request.owner.trim().is_empty() { vec![request.owner.clone()] } else { Vec::new() };

        match request.mode {
            Mode::Implementation => (
                Some(ImplementationExecutionContext {
                    plan_sources: source_refs.clone(),
                    mutation_bounds: MutationBounds {
                        declared_paths: Vec::new(),
                        owners,
                        source_refs,
                        expansion_policy: MutationExpansionPolicy::DenyWithoutApproval,
                    },
                    task_targets: Vec::new(),
                    safety_net: Vec::new(),
                    execution_posture: ExecutionPosture::RecommendationOnly,
                    rollback_expectations: Vec::new(),
                    post_approval_execution_consumed: false,
                }),
                None,
            ),
            Mode::Refactor => (
                None,
                Some(RefactorExecutionContext {
                    preserved_behavior: Vec::new(),
                    structural_rationale: None,
                    refactor_scope: MutationBounds {
                        declared_paths: Vec::new(),
                        owners,
                        source_refs,
                        expansion_policy: MutationExpansionPolicy::DenyWithoutApproval,
                    },
                    safety_net: Vec::new(),
                    no_feature_addition_target: None,
                    allowed_exceptions: Vec::new(),
                    execution_posture: ExecutionPosture::RecommendationOnly,
                    post_approval_execution_consumed: false,
                }),
            ),
            _ => (None, None),
        }
    }

    pub(super) fn resume_inputs(&self, context: &RunContext) -> Vec<String> {
        context
            .inputs
            .iter()
            .map(|input| {
                context
                    .input_fingerprints
                    .iter()
                    .find(|fingerprint| {
                        fingerprint.source_kind == InputSourceKind::Inline
                            && fingerprint.path == *input
                    })
                    .and_then(|fingerprint| fingerprint.snapshot_ref.as_ref())
                    .map(|snapshot_ref| format!(".canon/{snapshot_ref}"))
                    .unwrap_or_else(|| input.clone())
            })
            .collect()
    }

    pub(super) fn capture_input_fingerprints(
        &self,
        inputs: &[String],
        inline_inputs: &[String],
    ) -> Result<Vec<InputFingerprint>, EngineError> {
        let mut fingerprints = Vec::new();

        for input in inputs {
            for resolved in self.collect_input_files(input)? {
                let metadata = std::fs::metadata(&resolved)?;
                let modified = metadata
                    .modified()
                    .ok()
                    .and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok())
                    .map(|duration| duration.as_secs() as i64)
                    .unwrap_or_default();

                fingerprints.push(InputFingerprint {
                    path: self.persisted_input_path(&resolved),
                    source_kind: InputSourceKind::Path,
                    size_bytes: metadata.len(),
                    modified_unix_seconds: modified,
                    content_digest_sha256: Some(sha256_hex(&std::fs::read(&resolved)?)),
                    snapshot_ref: None,
                });
            }
        }

        let captured_at = OffsetDateTime::now_utc().unix_timestamp();
        for (index, inline_input) in inline_inputs.iter().enumerate() {
            fingerprints.push(InputFingerprint {
                path: inline_input_label(index),
                source_kind: InputSourceKind::Inline,
                size_bytes: inline_input.len() as u64,
                modified_unix_seconds: captured_at,
                content_digest_sha256: Some(sha256_hex(inline_input.as_bytes())),
                snapshot_ref: None,
            });
        }

        Ok(fingerprints)
    }

    pub(super) fn clarity_source_inputs(
        &self,
        inputs: &[String],
    ) -> Result<Vec<String>, EngineError> {
        let mut source_inputs = Vec::new();

        for input in inputs {
            let files = self.collect_input_files(input)?;
            if files.is_empty() {
                if !source_inputs.iter().any(|existing| existing == input) {
                    source_inputs.push(input.clone());
                }
                continue;
            }

            for path in files {
                let persisted = self.persisted_input_path(&path);
                if !source_inputs.iter().any(|existing| existing == &persisted) {
                    source_inputs.push(persisted);
                }
            }
        }

        Ok(source_inputs)
    }
}
