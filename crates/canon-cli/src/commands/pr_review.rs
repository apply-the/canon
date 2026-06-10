//! CLI handlers for onion-layer PR review: prepare, accept, finalize.
//!
//! Delegates to the EngineService orchestrators for each phase.

use canon_engine::EngineService;

use crate::app::PrReviewCommand;
use crate::error::{CliError, CliResult};

/// Dispatches a [`PrReviewCommand`] to the appropriate engine orchestrator.
pub fn execute(service: &EngineService, command: PrReviewCommand) -> CliResult<i32> {
    match command {
        PrReviewCommand::Prepare { run, base, head, skip_early_signal, skip_reason, output } => {
            // Validate skip-reason requirement (FR-015, T021)
            if skip_early_signal && skip_reason.as_deref().is_none_or(|r| r.trim().is_empty()) {
                return Err(CliError::InvalidInput(
                    "--skip-early-signal requires a non-empty --skip-reason".to_string(),
                ));
            }
            let run_id = run.unwrap_or_else(generate_run_id);
            let skip_reason_ref = skip_reason.as_deref();
            service
                .run_pr_review_prepare(&run_id, &base, &head, skip_early_signal, skip_reason_ref)
                .map_err(CliError::InvalidInput)?;
            if output == crate::app::OutputFormat::Json {
                println!(r#"{{"event":"prepare.completed","run_id":"{run_id}","status":"ready"}}"#);
            } else {
                println!("Prepare complete. Run ID: {run_id}");
                println!(
                    "Next: submit reviewer-output.md and run `canon pr-review accept --run {run_id}`"
                );
            }
            Ok(0)
        }
        PrReviewCommand::Accept { run } => {
            service.run_pr_review_accept(&run).map_err(CliError::InvalidInput)?;
            println!("Accept complete. Run ID: {run}");
            println!("Next: run `canon pr-review finalize --run {run}`");
            Ok(0)
        }
        PrReviewCommand::Finalize { run } => {
            service.run_pr_review_finalize(&run).map_err(CliError::InvalidInput)?;
            println!("Finalize complete. Run ID: {run}");
            println!("Artifacts written to .canon/runs/{run}/pr-review/");
            Ok(0)
        }
    }
}

/// Generates a short run identifier from the current timestamp.
fn generate_run_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
    format!("pr-review-{ts}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_generate_run_id_produces_non_empty_string() {
        let id = generate_run_id();
        assert!(id.starts_with("pr-review-"));
        assert!(id.len() > "pr-review-".len());
    }

    #[test]
    fn test_generate_run_id_is_deterministic_within_same_second() {
        let id1 = generate_run_id();
        let id2 = generate_run_id();
        assert_eq!(id1, id2);
    }

    #[test]
    fn execute_rejects_skip_early_signal_without_reason() {
        let workspace = tempfile::TempDir::new().unwrap();
        let service = EngineService::new(workspace.path());
        let cmd = PrReviewCommand::Prepare {
            run: None,
            base: "HEAD~1".to_string(),
            head: "HEAD".to_string(),
            skip_early_signal: true,
            skip_reason: None,
            output: crate::app::OutputFormat::Text,
        };
        let result = execute(&service, cmd);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("skip-early-signal"));
    }

    #[test]
    fn execute_accepts_skip_early_signal_with_valid_reason() {
        let workspace = tempfile::TempDir::new().unwrap();
        // Init a minimal git repo for the prepare flow
        std::process::Command::new("git")
            .args(["init"])
            .current_dir(workspace.path())
            .output()
            .ok();
        std::process::Command::new("git")
            .args(["config", "user.name", "Canon Test"])
            .current_dir(workspace.path())
            .output()
            .ok();
        std::process::Command::new("git")
            .args(["config", "user.email", "canon@example.com"])
            .current_dir(workspace.path())
            .output()
            .ok();
        std::process::Command::new("git")
            .args(["-c", "commit.gpgsign=false", "commit", "--allow-empty", "-m", "init"])
            .current_dir(workspace.path())
            .output()
            .ok();
        let service = EngineService::new(workspace.path());
        let cmd = PrReviewCommand::Prepare {
            run: Some("test-accept-skip".to_string()),
            base: "HEAD".to_string(),
            head: "HEAD".to_string(),
            skip_early_signal: true,
            skip_reason: Some("debugging accept flow".to_string()),
            output: crate::app::OutputFormat::Text,
        };
        let result = execute(&service, cmd);
        assert!(result.is_ok(), "expected ok, got {:?}", result.err());
    }

    // ── Accept dispatch tests ───────────────────────────────────────────

    /// Sets up a minimal `.canon/runs/{run_id}/pr-review/` directory with all
    /// files needed by `run_pr_review_accept`.
    fn setup_accept_fixture(workspace: &std::path::Path, run_id: &str) {
        let run_dir = workspace.join(".canon").join("runs").join(run_id).join("pr-review");
        fs::create_dir_all(&run_dir).unwrap();

        // Valid reviewer output JSON
        let reviewer_output = serde_json::json!({
            "schema_version": "1.0",
            "findings": [
                {
                    "id": "f1",
                    "severity": "minor",
                    "path": "src/a.rs",
                    "line": 10,
                    "comment_id": "c1",
                    "layer": "early-signal"
                }
            ],
            "recommendation": "Approve"
        });
        fs::write(run_dir.join("reviewer-output.md"), reviewer_output.to_string()).unwrap();

        // Changed files
        fs::write(run_dir.join("changed-files.tsv"), "src/a.rs\n").unwrap();

        // Run state with layers
        let run_state = serde_json::json!({
            "state": "prepared",
            "layers": {
                "early-signal": {"status": "completed"},
                "application-source": {"status": "completed"},
                "high-risk-surfaces": {"status": "completed"},
                "related-context": {"status": "skipped"},
                "logical-stress": {"status": "completed"},
                "tests": {"status": "completed"},
                "coverage-accounting": {"status": "completed"}
            }
        });
        fs::write(run_dir.join("run-state.json"), run_state.to_string()).unwrap();

        // 7 layer directories with output.md
        let layer_slugs = &[
            "early-signal",
            "application-source",
            "high-risk-surfaces",
            "related-context",
            "logical-stress",
            "tests",
            "coverage-accounting",
        ];
        for (idx, slug) in layer_slugs.iter().enumerate() {
            let ordinal = idx + 1;
            let layer_dir = run_dir.join("layers").join(format!("{:02}-{}", ordinal, slug));
            fs::create_dir_all(&layer_dir).unwrap();
            let output = format!(
                "# {slug} Review\n\nFindings for layer {ordinal}. All checks passed.\n\nDetailed analysis complete.\n"
            );
            fs::write(layer_dir.join("output.md"), output).unwrap();
        }
    }

    #[test]
    fn execute_accept_dispatches_to_engine_with_full_fixture() {
        let workspace = tempfile::TempDir::new().unwrap();
        setup_accept_fixture(workspace.path(), "test-accept-run");
        let service = EngineService::new(workspace.path());
        let cmd = PrReviewCommand::Accept { run: "test-accept-run".to_string() };
        let result = execute(&service, cmd);
        assert!(result.is_ok(), "expected ok, got {:?}", result.err());
    }

    // ── Finalize dispatch tests ─────────────────────────────────────────

    /// Sets up a run directory with state `ReviewerOutputAccepted` and all
    /// files needed by `run_pr_review_finalize`.
    fn setup_finalize_fixture(workspace: &std::path::Path, run_id: &str) {
        let run_dir = workspace.join(".canon").join("runs").join(run_id).join("pr-review");
        fs::create_dir_all(&run_dir).unwrap();

        // Run state: ReviewerOutputAccepted
        let run_state = serde_json::json!({
            "state": "reviewer_output_accepted"
        });
        fs::write(run_dir.join("run-state.json"), run_state.to_string()).unwrap();

        // Early signal findings (for verify_early_signal_finalize_gate)
        let es_dir = run_dir.join("early-signal");
        fs::create_dir_all(&es_dir).unwrap();
        fs::write(
            es_dir.join("findings.tsv"),
            "rule-id\tseverity\tpath\tline\tmessage\nRULE_TEST_MISSING\tmajor\tsrc/a.rs\t10\tadd tests\n",
        ).unwrap();
        fs::write(
            es_dir.join("findings.json"),
            serde_json::json!([{
                "rule_id": "RULE_TEST_MISSING",
                "severity": "major",
                "path": "src/a.rs",
                "line": 10,
                "message": "add tests"
            }])
            .to_string(),
        )
        .unwrap();

        // Canonical review output
        let canonical = serde_json::json!({
            "valid": true,
            "errors": [],
            "downgrades": []
        });
        fs::write(run_dir.join("canonical-review-output.json"), canonical.to_string()).unwrap();

        // Review packet (minimal)
        let packet = serde_json::json!({
            "slug": "pr-review",
            "recommendation": "Approve",
            "summary": "All good",
            "reviewed_by": "test"
        });
        fs::write(run_dir.join("review-packet.json"), packet.to_string()).unwrap();

        // Changed files
        fs::write(run_dir.join("changed-files.tsv"), "src/a.rs\n").unwrap();

        // Files inspected
        fs::write(run_dir.join("files-inspected.tsv"), "src/a.rs\tfull\n").unwrap();

        // Layer directories with output.md (for coverage accounting)
        let layer_slugs = &[
            "early-signal",
            "application-source",
            "high-risk-surfaces",
            "related-context",
            "logical-stress",
            "tests",
            "coverage-accounting",
        ];
        for (idx, slug) in layer_slugs.iter().enumerate() {
            let ordinal = idx + 1;
            let layer_dir = run_dir.join("layers").join(format!("{:02}-{}", ordinal, slug));
            fs::create_dir_all(&layer_dir).unwrap();
            let output =
                format!("# {slug} Review\n\nFindings for layer {ordinal}.\n\nAll checks passed.\n");
            fs::write(layer_dir.join("output.md"), output).unwrap();
        }
    }

    #[test]
    fn execute_finalize_dispatches_to_engine_with_full_fixture() {
        let workspace = tempfile::TempDir::new().unwrap();
        setup_finalize_fixture(workspace.path(), "test-finalize-run");
        let service = EngineService::new(workspace.path());
        let cmd = PrReviewCommand::Finalize { run: "test-finalize-run".to_string() };
        let result = execute(&service, cmd);
        assert!(result.is_ok(), "expected ok, got {:?}", result.err());
    }

    #[test]
    fn execute_prepare_produces_json_output() {
        let workspace = tempfile::TempDir::new().unwrap();
        // Init a minimal git repo
        std::process::Command::new("git")
            .args(["init"])
            .current_dir(workspace.path())
            .output()
            .ok();
        std::process::Command::new("git")
            .args(["config", "user.name", "Canon Test"])
            .current_dir(workspace.path())
            .output()
            .ok();
        std::process::Command::new("git")
            .args(["config", "user.email", "canon@example.com"])
            .current_dir(workspace.path())
            .output()
            .ok();
        std::process::Command::new("git")
            .args(["-c", "commit.gpgsign=false", "commit", "--allow-empty", "-m", "init"])
            .current_dir(workspace.path())
            .output()
            .ok();
        let service = EngineService::new(workspace.path());
        let cmd = PrReviewCommand::Prepare {
            run: Some("test-json-output".to_string()),
            base: "HEAD".to_string(),
            head: "HEAD".to_string(),
            skip_early_signal: true,
            skip_reason: Some("json output test".to_string()),
            output: crate::app::OutputFormat::Json,
        };
        let result = execute(&service, cmd);
        assert!(result.is_ok(), "expected ok, got {:?}", result.err());
    }
}
