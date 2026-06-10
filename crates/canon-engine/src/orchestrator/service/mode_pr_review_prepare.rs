//! Orchestrator for the `pr-review prepare` phase.
//!
//! Collects the diff, builds the context index, and writes all output files
//! under `.canon/runs/<run-id>/pr-review/`.

use std::fs;
use std::path::Path;

use super::EngineService;

impl EngineService {
    /// Runs the prepare phase: collects diff, runs early signal pass,
    /// generates layer directories, builds context, and writes output files.
    pub fn run_pr_review_prepare(
        &self,
        run_id: &str,
        base_ref: &str,
        head_ref: &str,
        skip_early_signal: bool,
        skip_reason: Option<&str>,
    ) -> Result<(), String> {
        let run_dir = self.canon_runtime_dir().join("runs").join(run_id).join("pr-review");
        fs::create_dir_all(&run_dir).map_err(|e| format!("create run dir: {e}"))?;

        let shell = canon_adapters::shell::ShellAdapter;
        let diff_output = shell
            .git_diff(base_ref, head_ref, &self.repo_root)
            .map_err(|e| format!("collect diff: {e}"))?;

        // ── Early signal pass ───────────────────────────────────────────────
        let removed_files: std::collections::HashSet<&str> = std::collections::HashSet::new();
        let findings = if skip_early_signal {
            // Write skip metadata
            let es_dir = run_dir.join("early-signal");
            fs::create_dir_all(&es_dir).map_err(|e| e.to_string())?;
            let skip_json = serde_json::json!({
                "early_signal_status": "skipped_with_reason",
                "skip_reason": skip_reason.unwrap_or("unspecified"),
                "source": "operator",
                "confidence_impact": "medium",
            });
            fs::write(
                es_dir.join("skip-metadata.json"),
                serde_json::to_string_pretty(&skip_json).unwrap_or_default(),
            )
            .map_err(|e| e.to_string())?;
            Vec::new()
        } else {
            let findings = crate::orchestrator::service::early_signal::execute_early_signal_pass(
                run_id,
                &diff_output.changed_files,
                &removed_files,
            );
            // Persist artifacts
            persist_early_signal_artifacts(&run_dir, &findings)?;
            findings
        };
        let _findings = findings;

        // ── 7-layer directory structure ─────────────────────────────────────
        generate_layer_directories(&run_dir)?;
        write_review_plan_md(&run_dir)?;

        // ── Existing onion-layer logic ──────────────────────────────────────
        let index = crate::review::context::ContextIndex::build(
            &diff_output.changed_files,
            &diff_output.patch,
        );

        let high_risk: Vec<&String> =
            diff_output.changed_files.iter().filter(|f| is_high_risk(f)).collect();

        write_review_brief(&run_dir, base_ref, head_ref, &diff_output.changed_files)?;
        write_review_plan(&run_dir)?;
        write_context_index(&run_dir, &index)?;
        write_changed_files(&run_dir, &diff_output.changed_files)?;
        write_high_risk_files(&run_dir, &high_risk)?;
        write_relation_hints(&run_dir, &diff_output.changed_files)?;
        write_diff_patch(&run_dir, &diff_output.patch)?;
        write_layer_files(&run_dir, &index)?;
        write_run_state(&run_dir, "awaiting_diff_review")?;
        write_reviewer_schema(&run_dir)?;

        Ok(())
    }
}

fn is_high_risk(path: &str) -> bool {
    let lower = path.to_ascii_lowercase();
    lower.contains("boundary")
        || lower.contains("public")
        || lower.contains("interface")
        || lower.contains("contract")
        || lower.contains("api")
        || lower.contains("schema")
}

// ── Output file writers ─────────────────────────────────────────────────

fn write_review_brief(
    dir: &Path,
    base: &str,
    head: &str,
    changed: &[String],
) -> Result<(), String> {
    let content = format!(
        "# Review Brief\n\n\
         **Base**: `{base}`\n\
         **Head**: `{head}`\n\
         **Mode**: pr-review (onion-layer orchestration)\n\
         **Changed files**: {count}\n\
         **Expected outcome**: Actionable review findings across 5 onion layers \
         (diff → whole-file → related-context → logical-stress → tests).\n",
        count = changed.len(),
    );
    fs::write(dir.join("review-brief.md"), content).map_err(|e| e.to_string())
}

fn write_review_plan(dir: &Path) -> Result<(), String> {
    let content = "\
# Review Plan\n\n\
## Onion-Layer Review Sequence\n\n\
1. **Diff analysis**: Inspect the diff hunks and changed files.\n\
2. **Whole-file review**: Read full modified files; check invariants, \
   error handling, performance.\n\
3. **Related-context review**: Inspect callers, tests, docs, contracts, exports.\n\
4. **Logical stress test**: Edge cases, malformed inputs, async, timeouts, security.\n\
5. **Test review**: Check test coverage and identify missing tests.\n\n\
## Progressive Context Discovery\n\n\
- Start from `context-index.tsv` — a compact map of all review context.\n\
- Expand only the files needed for the current layer.\n\
- Record evidence using stable context IDs (`C001`, `C002`, ...).\n\
- Per-layer instructions are in `layers/<NN>-<layer>/instructions.md`.\n";
    fs::write(dir.join("review-plan.md"), content).map_err(|e| e.to_string())
}

fn write_context_index(
    dir: &Path,
    index: &crate::review::context::ContextIndex,
) -> Result<(), String> {
    fs::write(dir.join("context-index.tsv"), index.to_tsv()).map_err(|e| e.to_string())?;
    fs::write(dir.join("context-index.json"), index.to_json()).map_err(|e| e.to_string())?;
    Ok(())
}

fn write_changed_files(dir: &Path, files: &[String]) -> Result<(), String> {
    let tsv = std::iter::once("path".to_string())
        .chain(files.iter().cloned())
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(dir.join("changed-files.tsv"), tsv).map_err(|e| e.to_string())
}

fn write_high_risk_files(dir: &Path, files: &[&String]) -> Result<(), String> {
    let tsv = if files.is_empty() {
        "path\treason\n".to_string()
    } else {
        let header = "path\treason".to_string();
        let rows: Vec<String> =
            files.iter().map(|f| format!("{f}\thigh-risk surface changed")).collect();
        format!("{header}\n{}", rows.join("\n"))
    };
    fs::write(dir.join("high-risk-files.tsv"), tsv).map_err(|e| e.to_string())
}

fn write_relation_hints(dir: &Path, files: &[String]) -> Result<(), String> {
    let mut hints = Vec::new();
    for f in files {
        if f.starts_with("src/") {
            let test = f.replace("src/", "tests/").replace(".rs", "_test.rs");
            hints.push(format!("{f}\ttest\t{test}"));
        }
    }
    let tsv = if hints.is_empty() {
        "source\ttype\thint\n".to_string()
    } else {
        format!("source\ttype\thint\n{}", hints.join("\n"))
    };
    fs::write(dir.join("relation-hints.tsv"), tsv).map_err(|e| e.to_string())
}

fn write_diff_patch(dir: &Path, patch: &str) -> Result<(), String> {
    fs::write(dir.join("diff.patch"), patch).map_err(|e| e.to_string())
}

fn write_layer_files(
    dir: &Path,
    index: &crate::review::context::ContextIndex,
) -> Result<(), String> {
    let layers = [
        (
            "01-diff",
            "diff",
            "Diff Analysis",
            "Inspect the diff hunks. Identify obvious issues, changed intent, incompatible logic, and public API/contract changes.",
        ),
        (
            "02-whole-file",
            "whole_file",
            "Whole-File Review",
            "Read the full modified files. Check local invariants, state consistency, error handling, concurrency, and performance.",
        ),
        (
            "03-related-context",
            "related_context",
            "Related-Context Review",
            "Inspect callers, tests, docs, examples, and contracts. Check caller compatibility, serialization, and drift.",
        ),
        (
            "04-logical-stress",
            "logical_stress",
            "Logical Stress Test",
            "Act as QA/security reviewer. Check edge cases, null/empty inputs, timeouts, race conditions, error propagation.",
        ),
        (
            "05-tests",
            "tests",
            "Test Review",
            "Check test coverage for the behavioral change. Identify missing tests, weak assertions, and untested failure paths.",
        ),
    ];

    for (dir_name, layer, title, instructions) in &layers {
        let layer_dir = dir.join("layers").join(dir_name);
        fs::create_dir_all(&layer_dir).map_err(|e| e.to_string())?;

        let inst = format!(
            "# {title}\n\n## Instructions\n\n{instructions}\n\n\
             ## Required Context\n\nSee `required-context.tsv` for the list of \
             context IDs relevant to this layer.\n\n\
             ## Output\n\nWrite your findings to `output.md` in this directory.\n\
             Use stable context IDs when citing evidence.\n"
        );
        fs::write(layer_dir.join("instructions.md"), inst).map_err(|e| e.to_string())?;

        let ctx_entries: Vec<_> =
            index.entries.iter().filter(|e| e.layer == *layer || e.layer == "diff").collect();
        let tsv = if ctx_entries.is_empty() {
            "id\ttype\tpath\n".to_string()
        } else {
            let header = "id\ttype\tpath".to_string();
            let rows: Vec<String> = ctx_entries
                .iter()
                .map(|e| format!("{}\t{}\t{}", e.id, e.entry_type, e.path))
                .collect();
            format!("{header}\n{}", rows.join("\n"))
        };
        fs::write(layer_dir.join("required-context.tsv"), tsv).map_err(|e| e.to_string())?;
    }
    Ok(())
}

fn write_run_state(dir: &Path, state: &str) -> Result<(), String> {
    let json = serde_json::json!({
        "run_state": state,
        "layer_states": {},
        "actionable_review_status": "governance_only",
    });
    fs::write(dir.join("run-state.json"), json.to_string()).map_err(|e| e.to_string())
}

fn write_reviewer_schema(dir: &Path) -> Result<(), String> {
    let schema = serde_json::json!({
        "schema_version": "1.0",
        "review_status": "actionable_review_executed",
        "coverage": {
            "files_changed": 0,
            "files_inspected_deeply": [],
            "files_skipped": [],
            "limitations": []
        },
        "findings": [],
        "missing_tests": [],
        "recommendation": "Comment",
        "layer_coverage": {}
    });
    fs::write(
        dir.join("reviewer-output.schema.json"),
        serde_json::to_string_pretty(&schema).unwrap_or_default(),
    )
    .map_err(|e| e.to_string())
}

// ── Early signal pass helpers (075-pr-review-early-signal-pass) ───────────────

/// The seven review layers with their display names.
const EARLY_SIGNAL_LAYERS: &[(&str, &str, bool)] = &[
    ("early-signal", "Early Signal Pass", true),
    ("application-source", "Application-Source Review", false),
    ("high-risk-surfaces", "High-Risk Surfaces Review", false),
    ("related-context", "Related-Context Review", false),
    ("logical-stress", "Logical Stress Review", false),
    ("tests", "Tests Review", false),
    ("coverage-accounting", "Coverage Accounting & Final Recommendation", false),
];

/// Generates the 7-layer directory structure under `layers/`.
///
/// Each layer directory receives `instructions.md`, `required-context.tsv`,
/// and an empty `output.md` placeholder.
#[allow(dead_code)]
pub(crate) fn generate_layer_directories(run_dir: &Path) -> Result<(), String> {
    let layers_dir = run_dir.join("layers");
    for (idx, (slug, display_name, canonical)) in EARLY_SIGNAL_LAYERS.iter().enumerate() {
        let ordinal = idx + 1;
        let layer_dir = layers_dir.join(format!("{:02}-{}", ordinal, slug));
        fs::create_dir_all(&layer_dir).map_err(|e| e.to_string())?;

        let instructions = format!(
            "# {display_name} (Layer {ordinal})\n\n\
             ## Instructions\n\n\
             Review the changed files according to the {display_name} methodology. \
             See `required-context.tsv` for the list of files and context entries \
             relevant to this layer.\n\n\
             ## Executed By\n\n\
             {}\n\n\
             ## Output\n\n\
             Write your findings to `output.md` in this directory.\n",
            if *canonical { "Canon (deterministic)" } else { "LLM Agent (semantic review)" }
        );
        fs::write(layer_dir.join("instructions.md"), instructions).map_err(|e| e.to_string())?;

        // Write a minimal required-context.tsv — populated by the full prepare flow
        let tsv = "id\ttype\tpath\n";
        fs::write(layer_dir.join("required-context.tsv"), tsv).map_err(|e| e.to_string())?;

        // Write an empty output.md placeholder
        fs::write(
            layer_dir.join("output.md"),
            format!("# {display_name} Output\n\n*No output yet.*\n"),
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Writes `review-plan.md` listing all seven layers in order with their status.
#[allow(dead_code)]
pub(crate) fn write_review_plan_md(run_dir: &Path) -> Result<(), String> {
    let mut content = String::from("# Review Plan\n\n");
    content.push_str("**Workflow**: prepare → accept → finalize\n\n");
    content.push_str("## Layer Order\n\n");

    for (idx, (slug, display_name, canonical)) in EARLY_SIGNAL_LAYERS.iter().enumerate() {
        let ordinal = idx + 1;
        let status = if *canonical { "Executed by Canon" } else { "Pending (LLM Agent)" };
        content.push_str(&format!(
            "### {ordinal}. {display_name}\n- **Slug**: `{slug}`\n- **Status**: {status}\n\n"
        ));
    }

    fs::write(run_dir.join("review-plan.md"), content).map_err(|e| e.to_string())
}

/// Persists early signal findings as `findings.tsv`, `findings.json`, and `summary.md`.
#[allow(dead_code)]
pub(crate) fn persist_early_signal_artifacts(
    run_dir: &Path,
    findings: &[crate::domain::review::EarlySignalFinding],
) -> Result<(), String> {
    let es_dir = run_dir.join("early-signal");
    fs::create_dir_all(&es_dir).map_err(|e| e.to_string())?;

    // findings.tsv (tab-separated, LLM-scannable)
    let mut tsv = String::from("finding_id\trule_id\tseverity\tcategory\tpath\tsummary\n");
    for f in findings {
        let sev = format!("{:?}", f.severity).to_lowercase();
        tsv.push_str(&format!(
            "{}\t{}\t{}\t{}\t{}\t{}\n",
            f.finding_id, f.rule_id, sev, f.category, f.path, f.summary
        ));
    }
    fs::write(es_dir.join("findings.tsv"), &tsv).map_err(|e| e.to_string())?;

    // findings.json (Canon-validatable)
    let json = serde_json::to_string_pretty(findings).map_err(|e| e.to_string())?;
    fs::write(es_dir.join("findings.json"), &json).map_err(|e| e.to_string())?;

    // summary.md (human-readable)
    let mut sev_counts = std::collections::BTreeMap::new();
    for f in findings {
        let key = format!("{:?}", f.severity).to_lowercase();
        *sev_counts.entry(key).or_insert(0u32) += 1;
    }
    let total = findings.len();
    let summary = format!(
        "# Early Signal Pass Summary\n\n\
         **Total findings**: {total}\n\n\
         **By severity**: \n\n{}\n",
        sev_counts.iter().map(|(k, v)| format!("- {k}: {v}")).collect::<Vec<_>>().join("\n")
    );
    fs::write(es_dir.join("summary.md"), summary).map_err(|e| e.to_string())?;

    Ok(())
}

/// Writes the run state as `AwaitingReviewerOutput`.
#[allow(dead_code)]
pub(crate) fn write_awaiting_reviewer_output(dir: &Path) -> Result<(), String> {
    let json = serde_json::json!({
        "run_state": "AwaitingReviewerOutput",
        "early_signal_status": "completed",
        "layer_states": {},
        "actionable_review_status": "governance_only",
    });
    fs::write(dir.join("run-state.json"), json.to_string()).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_write_review_brief_contains_base_head() {
        let dir = TempDir::new().unwrap();
        write_review_brief(dir.path(), "main", "HEAD", &["src/a.rs".to_string()]).unwrap();
        let content = fs::read_to_string(dir.path().join("review-brief.md")).unwrap();
        assert!(content.contains("`main`"));
        assert!(content.contains("`HEAD`"));
        assert!(content.contains("Changed files"));
    }

    #[test]
    fn test_write_review_plan_contains_onion_layers() {
        let dir = TempDir::new().unwrap();
        write_review_plan(dir.path()).unwrap();
        let content = fs::read_to_string(dir.path().join("review-plan.md")).unwrap();
        assert!(content.contains("Diff analysis"));
        assert!(content.contains("Whole-file review"));
        assert!(content.contains("Logical stress test"));
        assert!(content.contains("Test review"));
    }

    #[test]
    fn test_write_run_state_defaults_to_awaiting_diff_review() {
        let dir = TempDir::new().unwrap();
        write_run_state(dir.path(), "awaiting_diff_review").unwrap();
        let content = fs::read_to_string(dir.path().join("run-state.json")).unwrap();
        assert!(content.contains("awaiting_diff_review"));
        assert!(content.contains("governance_only"));
    }

    #[test]
    fn test_write_changed_files_tsv() {
        let dir = TempDir::new().unwrap();
        write_changed_files(dir.path(), &["src/a.rs".to_string(), "src/b.rs".to_string()]).unwrap();
        let content = fs::read_to_string(dir.path().join("changed-files.tsv")).unwrap();
        assert!(content.contains("src/a.rs"));
        assert!(content.contains("src/b.rs"));
    }

    #[test]
    fn test_write_high_risk_empty() {
        let dir = TempDir::new().unwrap();
        write_high_risk_files(dir.path(), &[]).unwrap();
        let content = fs::read_to_string(dir.path().join("high-risk-files.tsv")).unwrap();
        assert!(content.contains("path\treason"));
    }

    #[test]
    fn test_is_high_risk_detects_boundary_and_contract() {
        assert!(is_high_risk("src/boundary/router.rs"));
        assert!(is_high_risk("contracts/api.json"));
        assert!(!is_high_risk("src/lib.rs"));
    }

    #[test]
    fn generate_layer_directories_creates_seven_layers() {
        let dir = TempDir::new().unwrap();
        generate_layer_directories(dir.path()).unwrap();
        let layers = dir.path().join("layers");
        assert!(layers.is_dir());
        for (idx, (slug, _, _)) in EARLY_SIGNAL_LAYERS.iter().enumerate() {
            let ordinal = idx + 1;
            let layer_dir = layers.join(format!("{:02}-{}", ordinal, slug));
            assert!(layer_dir.is_dir(), "missing {:02}-{}", ordinal, slug);
            assert!(layer_dir.join("instructions.md").exists());
            assert!(layer_dir.join("required-context.tsv").exists());
            assert!(layer_dir.join("output.md").exists());
        }
    }

    #[test]
    fn write_review_plan_md_lists_all_seven_layers() {
        let dir = TempDir::new().unwrap();
        write_review_plan_md(dir.path()).unwrap();
        let content = fs::read_to_string(dir.path().join("review-plan.md")).unwrap();
        assert!(content.contains("Early Signal Pass"));
        assert!(content.contains("Application-Source Review"));
        assert!(content.contains("High-Risk Surfaces Review"));
        assert!(content.contains("Related-Context Review"));
        assert!(content.contains("Logical Stress Review"));
        assert!(content.contains("Tests Review"));
        assert!(content.contains("Coverage Accounting"));
        assert!(content.contains("Executed by Canon"));
        assert!(content.contains("Pending (LLM Agent)"));
    }

    #[test]
    fn persist_early_signal_artifacts_writes_tsv_json_summary() {
        use crate::domain::review::{EarlySignalFinding, EarlySignalSeverity};

        let dir = TempDir::new().unwrap();
        let findings = vec![EarlySignalFinding {
            finding_id: "ES001".to_string(),
            run_id: "prr-1".to_string(),
            rule_id: "reference.dangling_import".to_string(),
            severity: EarlySignalSeverity::Blocking,
            category: "reference".to_string(),
            path: "src/main.rs".to_string(),
            start_line: None,
            end_line: None,
            summary: "dangling import".to_string(),
            evidence_context_ids: vec![],
            suggested_layer: "app-source".to_string(),
            actionable_comment_candidate: true,
        }];
        persist_early_signal_artifacts(dir.path(), &findings).unwrap();

        let es_dir = dir.path().join("early-signal");
        assert!(es_dir.join("findings.tsv").exists());
        assert!(es_dir.join("findings.json").exists());
        assert!(es_dir.join("summary.md").exists());

        let tsv = fs::read_to_string(es_dir.join("findings.tsv")).unwrap();
        assert!(tsv.contains("ES001"));
        assert!(tsv.contains("reference.dangling_import"));

        let json = fs::read_to_string(es_dir.join("findings.json")).unwrap();
        assert!(json.contains("ES001"));

        let summary = fs::read_to_string(es_dir.join("summary.md")).unwrap();
        assert!(summary.contains("Total findings"));
    }

    #[test]
    fn write_awaiting_reviewer_output_sets_correct_state() {
        let dir = TempDir::new().unwrap();
        write_awaiting_reviewer_output(dir.path()).unwrap();
        let content = fs::read_to_string(dir.path().join("run-state.json")).unwrap();
        assert!(content.contains("AwaitingReviewerOutput"));
        assert!(content.contains("completed"));
    }

    // ── run_pr_review_prepare integration tests ──────────────────────────

    #[test]
    fn run_pr_review_prepare_with_skip_early_signal_writes_skip_metadata() {
        let workspace = TempDir::new().unwrap();
        // Set up a minimal git repo
        let status = std::process::Command::new("git")
            .args(["init"])
            .current_dir(workspace.path())
            .output()
            .expect("git init");
        assert!(status.status.success(), "git init failed: {:?}", status);

        // Create an initial commit so there's a valid HEAD
        std::process::Command::new("git")
            .args(["-c", "commit.gpgsign=false", "commit", "--allow-empty", "-m", "init"])
            .current_dir(workspace.path())
            .output()
            .expect("git commit");

        let service = EngineService::new(workspace.path());
        // diff HEAD HEAD is empty — prepare should still succeed
        let result = service.run_pr_review_prepare(
            "test-skip-flow",
            "HEAD",
            "HEAD",
            true,
            Some("integration test"),
        );
        assert!(result.is_ok(), "expected ok, got {:?}", result.err());

        // Verify skip metadata was written
        let skip_path = workspace
            .path()
            .join(".canon")
            .join("runs")
            .join("test-skip-flow")
            .join("pr-review")
            .join("early-signal")
            .join("skip-metadata.json");
        assert!(skip_path.exists(), "skip-metadata.json not written");
        let meta = fs::read_to_string(&skip_path).unwrap();
        assert!(meta.contains("skipped_with_reason"));
    }

    #[test]
    fn run_pr_review_prepare_no_skip_runs_early_signal_pass() {
        let workspace = TempDir::new().unwrap();
        // Set up a minimal git repo with two commits so we have a real diff
        std::process::Command::new("git")
            .args(["init"])
            .current_dir(workspace.path())
            .output()
            .expect("git init");

        // First commit
        let test_file = workspace.path().join("src");
        fs::create_dir_all(&test_file).unwrap();
        fs::write(test_file.join("lib.rs"), "// initial\n").unwrap();
        std::process::Command::new("git")
            .args(["add", "src/lib.rs"])
            .current_dir(workspace.path())
            .output()
            .expect("git add");
        std::process::Command::new("git")
            .args(["-c", "commit.gpgsign=false", "commit", "-m", "first"])
            .current_dir(workspace.path())
            .output()
            .expect("git commit 1");

        // Second commit (modify the file)
        fs::write(test_file.join("lib.rs"), "// modified\n").unwrap();
        std::process::Command::new("git")
            .args(["add", "src/lib.rs"])
            .current_dir(workspace.path())
            .output()
            .expect("git add 2");
        std::process::Command::new("git")
            .args(["-c", "commit.gpgsign=false", "commit", "-m", "second"])
            .current_dir(workspace.path())
            .output()
            .expect("git commit 2");

        let service = EngineService::new(workspace.path());
        let result =
            service.run_pr_review_prepare("test-no-skip-flow", "HEAD~1", "HEAD", false, None);
        assert!(result.is_ok(), "expected ok, got {:?}", result.err());

        // Verify early signal findings were written (not skip metadata)
        let es_dir = workspace
            .path()
            .join(".canon")
            .join("runs")
            .join("test-no-skip-flow")
            .join("pr-review")
            .join("early-signal");
        assert!(es_dir.join("findings.tsv").exists(), "findings.tsv not written");
        assert!(es_dir.join("summary.md").exists(), "summary.md not written");
    }
}
