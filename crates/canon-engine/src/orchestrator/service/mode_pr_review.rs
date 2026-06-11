use super::EngineService;
use super::*;

impl EngineService {
    /// `canon run --mode pr-review` is no longer supported.
    ///
    /// The legacy governed-run path has been removed in favor of the
    /// onion-layer actionable review workflow:
    ///
    /// ```text
    /// canon pr-review prepare --base <BASE> --head <HEAD>
    /// canon pr-review accept --run <RUN_ID>
    /// canon pr-review finalize --run <RUN_ID>
    /// ```
    pub(super) fn run_pr_review(
        &self,
        _store: &WorkspaceStore,
        _request: RunRequest,
        _policy_set: crate::domain::policy::PolicySet,
    ) -> Result<RunSummary, EngineError> {
        Err(EngineError::Validation(
            "canon run --mode pr-review has been removed. \
             Use the onion-layer workflow instead:\n  \
             canon pr-review prepare --base <BASE> --head <HEAD>\n  \
             canon pr-review accept --run <RUN_ID>\n  \
             canon pr-review finalize --run <RUN_ID>\n  \
             See `canon pr-review --help` for details."
                .to_string(),
        ))
    }
}
