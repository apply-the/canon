use super::EngineService;
use crate::domain::help_next::{CanonHelpNextRecommendation, CanonHelpNextState};
use crate::orchestrator::service::EngineError;

impl EngineService {
    /// Inspect the current workspace and return a help-next recommendation.
    pub fn help_next(&self) -> Result<CanonHelpNextRecommendation, EngineError> {
        let canon_dir = self.canon_runtime_dir();

        // Not initialized
        if !canon_dir.exists() {
            return Ok(CanonHelpNextRecommendation::from_diagnostics(
                CanonHelpNextState::NotInitialized,
                vec![],
                None,
            ));
        }

        // Check for active runs in .canon/runs/
        let runs_dir = canon_dir.join("runs");
        if !runs_dir.exists() {
            return Ok(CanonHelpNextRecommendation::from_diagnostics(
                CanonHelpNextState::NoActiveRun,
                vec![],
                None,
            ));
        }

        // Attempt to determine the most recent active run
        let entries: Vec<_> = match std::fs::read_dir(&runs_dir) {
            Ok(entries) => entries.filter_map(|e| e.ok()).collect(),
            Err(_) => {
                return Ok(CanonHelpNextRecommendation::from_diagnostics(
                    CanonHelpNextState::NoActiveRun,
                    vec![],
                    None,
                ));
            }
        };

        if entries.is_empty() {
            return Ok(CanonHelpNextRecommendation::from_diagnostics(
                CanonHelpNextState::NoActiveRun,
                vec![],
                None,
            ));
        }

        // For now, return a ready recommendation as a scaffold.
        // Full mode-aware diagnostics (packet inspection, document checks,
        // evidence, approval, lineage) is deferred to follow-on slices.
        Ok(CanonHelpNextRecommendation::ready(None))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::orchestrator::service::EngineService;

    #[test]
    fn uninitialized_workspace_returns_not_initialized() {
        let tmp = std::env::temp_dir().join("canon-help-next-test-nonexistent");
        let svc = EngineService::new(&tmp);
        let rec = svc.help_next().unwrap();
        assert_eq!(rec.state, CanonHelpNextState::NotInitialized);
        assert_eq!(rec.recommended_command.as_deref(), Some("canon init"));
    }

    #[test]
    fn empty_canon_dir_returns_no_active_run() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(tmp.path().join(".canon")).unwrap();
        let svc = EngineService::new(tmp.path());
        let rec = svc.help_next().unwrap();
        assert_eq!(rec.state, CanonHelpNextState::NoActiveRun);
    }

    #[test]
    fn empty_runs_dir_returns_no_active_run() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(tmp.path().join(".canon/runs")).unwrap();
        let svc = EngineService::new(tmp.path());
        let rec = svc.help_next().unwrap();
        assert_eq!(rec.state, CanonHelpNextState::NoActiveRun);
        assert!(rec.recommended_command.is_some());
    }

    #[test]
    fn runs_dir_with_entries_returns_ready() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(tmp.path().join(".canon/runs/some-run")).unwrap();
        let svc = EngineService::new(tmp.path());
        let rec = svc.help_next().unwrap();
        assert_eq!(rec.state, CanonHelpNextState::Ready);
        assert!(!rec.blockers_found);
        assert_eq!(rec.recommended_command.as_deref(), Some("canon publish"));
    }

    #[test]
    fn help_next_is_deterministic() {
        let tmp = tempfile::tempdir().unwrap();
        let svc = EngineService::new(tmp.path());
        let rec1 = svc.help_next().unwrap();
        let rec2 = svc.help_next().unwrap();
        assert_eq!(rec1.state, rec2.state);
        assert_eq!(rec1.recommended_command, rec2.recommended_command);
    }
}
