//! Orchestrator for the `pr-review prepare` phase.
//!
//! Collects the diff, builds the context index, and writes all output files
//! under `.canon/runs/<run-id>/pr-review/`.

use std::fs;
use std::path::Path;

use super::EngineService;

impl EngineService {
    /// Runs the prepare phase: collects diff, builds context, writes output files.
    pub fn run_pr_review_prepare(
        &self,
        run_id: &str,
        base_ref: &str,
        head_ref: &str,
    ) -> Result<(), String> {
        let run_dir = self.repo_root.join(".canon").join("runs").join(run_id).join("pr-review");
        fs::create_dir_all(&run_dir).map_err(|e| format!("create run dir: {e}"))?;

        let shell = canon_adapters::shell::ShellAdapter;
        let diff_output = shell
            .git_diff(base_ref, head_ref, &self.repo_root)
            .map_err(|e| format!("collect diff: {e}"))?;

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
}
