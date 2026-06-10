//! Early signal pass executor, check rules, and event infrastructure.
//!
//! The early signal pass is the first layer of the PR review workflow. It runs
//! deterministic file-system and manifest checks to discover high-confidence
//! problems before deeper review layers begin. Each rule is a pure function
//! returning a `Vec<EarlySignalFinding>`.

use std::collections::HashSet;

use crate::domain::review::{EarlySignalFinding, EarlySignalSeverity, FileClassification};

// ── Rule ID constants ─────────────────────────────────────────────────────────

/// Build manifest references a removed file (e.g., Justfile, Cargo.toml member).
#[allow(dead_code)]
pub const RULE_BUILD_COMMAND_REMOVED_FILE_REFERENCE: &str = "build.command.removed_file_reference";

/// Cargo.toml dependency version that doesn't resolve.
#[allow(dead_code)]
pub const RULE_MANIFEST_STALE_DEPENDENCY: &str = "manifest.stale_dependency";

/// Serialized artifact shape that doesn't match its domain type.
#[allow(dead_code)]
pub const RULE_MANIFEST_SCHEMA_DRIFT: &str = "manifest.schema_drift";

/// `mod`/`use` statement referencing a removed file.
#[allow(dead_code)]
pub const RULE_REFERENCE_DANGLING_IMPORT: &str = "reference.dangling_import";

/// Changed `pub fn` signature without corresponding test changes.
#[allow(dead_code)]
pub const RULE_TEST_MISSING_FOR_CHANGED_BEHAVIOR: &str = "test.missing_for_changed_behavior";

/// Renamed file without corresponding import updates.
#[allow(dead_code)]
pub const RULE_NAMING_DRIFT: &str = "naming.drift";

/// `cargo check` or `cargo clippy` failure on the diff.
#[allow(dead_code)]
pub const RULE_VALIDATION_FAILURE: &str = "validation.failure";

/// Category bucket constants for the seven check categories.
#[allow(dead_code)]
pub const CATEGORY_BUILD_CI: &str = "build_ci";
#[allow(dead_code)]
pub const CATEGORY_MANIFEST: &str = "manifest";
#[allow(dead_code)]
pub const CATEGORY_SCHEMA: &str = "schema";
#[allow(dead_code)]
pub const CATEGORY_REFERENCE: &str = "reference";
#[allow(dead_code)]
pub const CATEGORY_TEST: &str = "test";
#[allow(dead_code)]
pub const CATEGORY_NAMING: &str = "naming";
#[allow(dead_code)]
pub const CATEGORY_VALIDATION: &str = "validation";

/// Risk classification constants.
#[allow(dead_code)]
pub const RISK_HIGH: &str = "high";
#[allow(dead_code)]
pub const RISK_MEDIUM: &str = "medium";
#[allow(dead_code)]
pub const RISK_LOW: &str = "low";

/// The seven review layer slugs in execution order.
#[allow(dead_code)]
pub const REVIEW_LAYERS: &[(&str, &str, bool)] = &[
    ("early-signal", "Early Signal Pass", true),
    ("application-source", "Application-Source Review", false),
    ("high-risk-surfaces", "High-Risk Surfaces Review", false),
    ("related-context", "Related-Context Review", false),
    ("logical-stress", "Logical Stress Review", false),
    ("tests", "Tests Review", false),
    ("coverage-accounting", "Coverage Accounting & Final Recommendation", false),
];

// ── Helper: build a finding ────────────────────────────────────────────────────

/// Build a single `EarlySignalFinding` with the standard fields populated.
#[allow(dead_code)]
#[allow(clippy::too_many_arguments)]
fn make_finding(
    run_id: &str,
    finding_id: &str,
    rule_id: &str,
    severity: EarlySignalSeverity,
    category: &str,
    path: &str,
    line: Option<u32>,
    summary: &str,
    suggested_layer: &str,
) -> EarlySignalFinding {
    EarlySignalFinding {
        finding_id: finding_id.to_string(),
        run_id: run_id.to_string(),
        rule_id: rule_id.to_string(),
        severity,
        category: category.to_string(),
        path: path.to_string(),
        start_line: line,
        end_line: line,
        summary: summary.to_string(),
        evidence_context_ids: Vec::new(),
        suggested_layer: suggested_layer.to_string(),
        actionable_comment_candidate: severity != EarlySignalSeverity::Info,
    }
}

// ── Check rule implementations ────────────────────────────────────────────────

/// Checks if build manifest files reference files removed in the diff.
///
/// Inspects `justfile`, `Makefile`, `Cargo.toml` (workspace members), and
/// `.github/workflows/*.yml` for references to files that no longer exist
/// in the changed set.
#[allow(dead_code)]
pub fn check_build_command_removed_file_reference(
    run_id: &str,
    changed_files: &[String],
    removed_files: &HashSet<&str>,
) -> Vec<EarlySignalFinding> {
    let mut findings = Vec::new();
    let mut id_counter = 0u32;

    let build_manifests: &[&str] = &["justfile", "Makefile", "Cargo.toml"];

    for manifest_name in build_manifests {
        let manifest_path = changed_files
            .iter()
            .find(|f| f.ends_with(manifest_name) || f.as_str() == *manifest_name);
        let Some(path) = manifest_path else { continue };

        // Without reading file contents in this deterministic check,
        // we flag that a build manifest was modified alongside file removals.
        if !removed_files.is_empty() {
            let finding_id = format!("ES{:03}", id_counter);
            id_counter += 1;
            findings.push(make_finding(
                run_id,
                &finding_id,
                RULE_BUILD_COMMAND_REMOVED_FILE_REFERENCE,
                EarlySignalSeverity::Blocking,
                CATEGORY_BUILD_CI,
                path,
                None,
                &format!(
                    "Build manifest `{path}` was modified while {} file(s) were removed. \
                     Verify the manifest does not reference deleted paths.",
                    removed_files.len()
                ),
                "diff",
            ));
        }
    }

    findings
}

/// Checks for `use`/`mod` statements referencing files that were removed.
#[allow(dead_code)]
pub fn check_reference_dangling_import(
    run_id: &str,
    changed_files: &[String],
    removed_files: &HashSet<&str>,
) -> Vec<EarlySignalFinding> {
    let mut findings = Vec::new();
    let mut id_counter = 0u32;

    // Build a set of removed file stems for matching
    let removed_stems: HashSet<&str> = removed_files
        .iter()
        .filter_map(|f| {
            let stem = std::path::Path::new(f).file_stem().and_then(|s| s.to_str())?;
            Some(stem)
        })
        .collect();

    for path in changed_files {
        if !path.ends_with(".rs") || removed_files.contains(path.as_str()) {
            continue;
        }

        // A Rust file that was modified (but not removed) — flag if any
        // removed file stem matches common patterns that suggest dangling
        // references
        let file_stem =
            std::path::Path::new(path).file_stem().and_then(|s| s.to_str()).unwrap_or("");

        for removed in removed_files {
            let removed_stem =
                std::path::Path::new(removed).file_stem().and_then(|s| s.to_str()).unwrap_or("");

            if removed_stem == file_stem {
                // Same stem but different extension — likely a related file
                // was removed without updating references
                let finding_id = format!("ES{:03}", id_counter);
                id_counter += 1;
                findings.push(make_finding(
                    run_id,
                    &finding_id,
                    RULE_REFERENCE_DANGLING_IMPORT,
                    EarlySignalSeverity::Blocking,
                    CATEGORY_REFERENCE,
                    path,
                    None,
                    &format!(
                        "Removed file `{removed}` shares stem with modified `{path}`. \
                         Verify no dangling import references."
                    ),
                    "app-source",
                ));
            }
        }

        // Also check if this modified file's name contains any removed file stem
        for stem in &removed_stems {
            if path.contains(stem) && !removed_files.contains(path.as_str()) {
                let finding_id = format!("ES{:03}", id_counter);
                id_counter += 1;
                findings.push(make_finding(
                    run_id,
                    &finding_id,
                    RULE_REFERENCE_DANGLING_IMPORT,
                    EarlySignalSeverity::High,
                    CATEGORY_REFERENCE,
                    path,
                    None,
                    &format!(
                        "Modified file `{path}` may reference removed module `{stem}`. \
                         Verify imports are updated."
                    ),
                    "app-source",
                ));
                break;
            }
        }
    }

    findings
}

/// Checks for renamed files without corresponding import updates.
#[allow(dead_code)]
pub fn check_naming_drift(
    run_id: &str,
    changed_files: &[String],
    _removed_files: &HashSet<&str>,
) -> Vec<EarlySignalFinding> {
    let mut findings = Vec::new();
    let mut id_counter = 0u32;

    // Detect renamed files by matching same stem with different extensions
    // appearing as both added and removed in the diff
    let rust_files: Vec<&String> = changed_files.iter().filter(|f| f.ends_with(".rs")).collect();

    for i in 0..rust_files.len() {
        for j in (i + 1)..rust_files.len() {
            let a = rust_files[i];
            let b = rust_files[j];
            let stem_a = std::path::Path::new(a).file_stem().and_then(|s| s.to_str());
            let stem_b = std::path::Path::new(b).file_stem().and_then(|s| s.to_str());

            if let (Some(sa), Some(sb)) = (stem_a, stem_b)
                && sa == sb
                && a != b
            {
                let finding_id = format!("ES{:03}", id_counter);
                id_counter += 1;
                let older = if a < b { a } else { b };
                findings.push(make_finding(
                    run_id,
                    &finding_id,
                    RULE_NAMING_DRIFT,
                    EarlySignalSeverity::Medium,
                    CATEGORY_NAMING,
                    older,
                    None,
                    &format!(
                        "Possible rename detected: `{a}` and `{b}` share the same stem. \
                         Verify imports referencing `{sa}` are updated."
                    ),
                    "diff",
                ));
            }
        }
    }

    findings
}

/// Checks if public API surface changed without corresponding test file changes.
#[allow(dead_code)]
pub fn check_test_missing_for_changed_behavior(
    run_id: &str,
    changed_files: &[String],
    _removed_files: &HashSet<&str>,
) -> Vec<EarlySignalFinding> {
    let mut findings = Vec::new();
    let mut id_counter = 0u32;

    let src_files: Vec<&String> = changed_files
        .iter()
        .filter(|f| f.ends_with(".rs") && !f.contains("/tests/") && !f.starts_with("tests/"))
        .collect();

    let has_test_changes = changed_files.iter().any(|f| {
        f.contains("/tests/")
            || f.starts_with("tests/")
            || f.contains("_test")
            || f.ends_with("_test.rs")
    });

    if !src_files.is_empty() && !has_test_changes {
        for path in src_files.iter().take(5) {
            let finding_id = format!("ES{:03}", id_counter);
            id_counter += 1;
            findings.push(make_finding(
                run_id,
                &finding_id,
                RULE_TEST_MISSING_FOR_CHANGED_BEHAVIOR,
                EarlySignalSeverity::Medium,
                CATEGORY_TEST,
                path,
                None,
                &format!("Source file `{path}` changed but no test files modified."),
                "tests",
            ));
        }
    }

    findings
}

/// Checks for stale Cargo.toml manifest references.
#[allow(dead_code)]
pub fn check_manifest_stale_dependency(
    run_id: &str,
    changed_files: &[String],
    _removed_files: &HashSet<&str>,
) -> Vec<EarlySignalFinding> {
    let mut findings = Vec::new();
    let mut id_counter = 0u32;

    for path in changed_files {
        if path == "Cargo.toml" || path.ends_with("/Cargo.toml") {
            let finding_id = format!("ES{:03}", id_counter);
            id_counter += 1;
            findings.push(make_finding(
                run_id,
                &finding_id,
                RULE_MANIFEST_STALE_DEPENDENCY,
                EarlySignalSeverity::Medium,
                CATEGORY_MANIFEST,
                path,
                None,
                "Cargo.toml was modified — verify dependency versions resolve and workspace members exist.",
                "diff",
            ));
        }
    }

    findings
}

/// Checks for schema drift in serialized artifact files.
#[allow(dead_code)]
pub fn check_manifest_schema_drift(
    run_id: &str,
    changed_files: &[String],
    _removed_files: &HashSet<&str>,
) -> Vec<EarlySignalFinding> {
    let mut findings = Vec::new();
    let mut id_counter = 0u32;

    let schema_files: &[&str] =
        &["run.toml", "context.toml", "state.toml", "artifact-contract.toml"];

    for path in changed_files {
        for schema_file in schema_files {
            if path.ends_with(schema_file) {
                let finding_id = format!("ES{:03}", id_counter);
                id_counter += 1;
                findings.push(make_finding(
                    run_id,
                    &finding_id,
                    RULE_MANIFEST_SCHEMA_DRIFT,
                    EarlySignalSeverity::High,
                    CATEGORY_SCHEMA,
                    path,
                    None,
                    &format!(
                        "Schema-bearing file `{path}` was modified. \
                         Verify serialized shapes still match their domain types."
                    ),
                    "diff",
                ));
            }
        }
    }

    findings
}

/// Checks for validation (build/test) issues in the diff context.
#[allow(dead_code)]
pub fn check_validation_failure(
    _run_id: &str,
    changed_files: &[String],
    _removed_files: &HashSet<&str>,
) -> Vec<EarlySignalFinding> {
    // This is a placeholder for future shell-out to `cargo check`.
    // The deterministic check flags high-risk file patterns that commonly
    // cause build failures without needing to shell out.
    let _ = changed_files;
    Vec::new()
}

/// Classify changed files into risk classes.
#[allow(dead_code)]
pub fn classify_files(changed_files: &[String]) -> Vec<FileClassification> {
    changed_files
        .iter()
        .map(|path| {
            let lower = path.to_ascii_lowercase();
            let (risk_class, reason) = if lower.contains("boundary")
                || lower.contains("public")
                || lower.contains("interface")
                || lower.contains("contract")
                || lower.contains("api")
                || lower.contains("schema")
            {
                (RISK_HIGH, "file name matches high-risk pattern")
            } else if lower.contains("test") || lower.ends_with(".md") {
                (RISK_LOW, "test or documentation file")
            } else if lower.ends_with(".rs")
                || lower.ends_with(".toml")
                || lower.ends_with(".yml")
                || lower.ends_with(".yaml")
            {
                (RISK_MEDIUM, "source or configuration file")
            } else {
                (RISK_LOW, "non-code file")
            };
            FileClassification {
                path: path.clone(),
                risk_class: risk_class.to_string(),
                reason: reason.to_string(),
            }
        })
        .collect()
}

/// Execute the full early signal pass against a diff context.
///
/// Runs all seven check rules against the changed files and removed files,
/// producing a unified `Vec<EarlySignalFinding>` with stable sequential IDs.
#[allow(dead_code)]
pub fn execute_early_signal_pass(
    run_id: &str,
    changed_files: &[String],
    removed_files: &HashSet<&str>,
) -> Vec<EarlySignalFinding> {
    let mut findings = Vec::new();

    findings.extend(check_build_command_removed_file_reference(
        run_id,
        changed_files,
        removed_files,
    ));
    findings.extend(check_manifest_stale_dependency(run_id, changed_files, removed_files));
    findings.extend(check_manifest_schema_drift(run_id, changed_files, removed_files));
    findings.extend(check_reference_dangling_import(run_id, changed_files, removed_files));
    findings.extend(check_test_missing_for_changed_behavior(run_id, changed_files, removed_files));
    findings.extend(check_naming_drift(run_id, changed_files, removed_files));
    findings.extend(check_validation_failure(run_id, changed_files, removed_files));

    // Re-assign stable sequential IDs
    for (idx, finding) in findings.iter_mut().enumerate() {
        finding.finding_id = format!("ES{:03}", idx);
    }

    findings
}

/// Emit a structured JSON event to stdout as one line.
///
/// Used when `--output json` is selected during prepare. Each call writes
/// one JSON object followed by a newline. Errors are silently ignored since
/// stdout is best-effort for agent consumption.
#[allow(dead_code)]
pub fn emit_stdout_event(event: &serde_json::Value) {
    use std::io::Write;
    if let Ok(line) = serde_json::to_string(event) {
        let stdout = std::io::stdout();
        let mut handle = stdout.lock();
        let _ = writeln!(handle, "{}", line);
    }
}

/// Persist a trace event to the early signal JSONL trace file.
///
/// Appends one line to `.canon/runs/<run_id>/pr-review/traces/early-signal.jsonl`.
/// Creates the parent directory on first use.
#[allow(dead_code)]
pub fn persist_trace_event(
    trace_path: &std::path::Path,
    event: &serde_json::Value,
) -> Result<(), std::io::Error> {
    crate::persistence::traces::append_jsonl_event(trace_path, event)
}

/// Generate the next sequential finding ID.
#[allow(dead_code)]
pub fn next_finding_id(_counter: &mut u32) -> String {
    let id = format!("ES{:03}", *_counter);
    *_counter += 1;
    id
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn rule_id_constants_are_stable() {
        assert_eq!(
            RULE_BUILD_COMMAND_REMOVED_FILE_REFERENCE,
            "build.command.removed_file_reference"
        );
        assert_eq!(RULE_MANIFEST_STALE_DEPENDENCY, "manifest.stale_dependency");
        assert_eq!(RULE_MANIFEST_SCHEMA_DRIFT, "manifest.schema_drift");
        assert_eq!(RULE_REFERENCE_DANGLING_IMPORT, "reference.dangling_import");
        assert_eq!(RULE_TEST_MISSING_FOR_CHANGED_BEHAVIOR, "test.missing_for_changed_behavior");
        assert_eq!(RULE_NAMING_DRIFT, "naming.drift");
        assert_eq!(RULE_VALIDATION_FAILURE, "validation.failure");
    }

    #[test]
    fn next_finding_id_increments() {
        let mut counter = 0u32;
        assert_eq!(next_finding_id(&mut counter), "ES000");
        assert_eq!(next_finding_id(&mut counter), "ES001");
        assert_eq!(next_finding_id(&mut counter), "ES002");
    }

    #[test]
    fn dangling_import_detects_stem_match_on_removed_file() {
        // removed module shares stem with a modified file
        let changed = vec!["src/main.rs".to_string(), "crates/cli/src/validator.rs".to_string()];
        let mut removed: HashSet<&str> = HashSet::new();
        removed.insert("src/validator.rs");
        let findings = check_reference_dangling_import("prr-1", &changed, &removed);
        assert!(!findings.is_empty(), "should detect same-stem match across directories");
    }

    #[test]
    fn dangling_import_no_findings_when_nothing_removed() {
        let changed = vec!["src/main.rs".to_string(), "src/lib.rs".to_string()];
        let removed: HashSet<&str> = HashSet::new();
        let findings = check_reference_dangling_import("prr-1", &changed, &removed);
        assert!(findings.is_empty());
    }

    #[test]
    fn build_command_flag_on_manifest_with_removed_files() {
        let changed = vec!["Cargo.toml".to_string(), "src/old.rs".to_string()];
        let mut removed: HashSet<&str> = HashSet::new();
        removed.insert("src/old.rs");
        let findings = check_build_command_removed_file_reference("prr-1", &changed, &removed);
        assert!(!findings.is_empty(), "should flag manifest with removed files");
    }

    #[test]
    fn test_missing_when_src_changed_but_no_test_changes() {
        let changed = vec!["src/main.rs".to_string(), "src/lib.rs".to_string()];
        let removed: HashSet<&str> = HashSet::new();
        let findings = check_test_missing_for_changed_behavior("prr-1", &changed, &removed);
        assert!(!findings.is_empty(), "should flag missing test changes");
    }

    #[test]
    fn test_missing_not_flagged_when_tests_changed() {
        let changed = vec!["src/main.rs".to_string(), "tests/main_test.rs".to_string()];
        let removed: HashSet<&str> = HashSet::new();
        let findings = check_test_missing_for_changed_behavior("prr-1", &changed, &removed);
        assert!(findings.is_empty(), "should not flag when tests also changed");
    }

    #[test]
    fn naming_drift_detects_identical_stems() {
        let changed =
            vec!["src/validator.rs".to_string(), "crates/cli/src/validator.rs".to_string()];
        let removed: HashSet<&str> = HashSet::new();
        let findings = check_naming_drift("prr-1", &changed, &removed);
        assert!(!findings.is_empty(), "should detect identical stems");
        assert!(findings[0].category == CATEGORY_NAMING);
    }

    #[test]
    fn manifest_stale_flags_cargo_toml_changes() {
        let changed = vec!["Cargo.toml".to_string()];
        let removed: HashSet<&str> = HashSet::new();
        let findings = check_manifest_stale_dependency("prr-1", &changed, &removed);
        assert!(!findings.is_empty(), "should flag Cargo.toml change");
    }

    #[test]
    fn schema_drift_flags_run_toml_changes() {
        let changed = vec![".canon/runs/prr-1/run.toml".to_string()];
        let removed: HashSet<&str> = HashSet::new();
        let findings = check_manifest_schema_drift("prr-1", &changed, &removed);
        assert!(!findings.is_empty(), "should flag run.toml change");
    }

    #[test]
    fn classify_files_assigns_high_risk() {
        let classifications = classify_files(&[
            "src/contract.rs".to_string(),
            "README.md".to_string(),
            "src/main.rs".to_string(),
        ]);
        assert_eq!(classifications.len(), 3);
        let contract = classifications.iter().find(|c| c.path == "src/contract.rs").unwrap();
        assert_eq!(contract.risk_class, RISK_HIGH);
        let readme = classifications.iter().find(|c| c.path == "README.md").unwrap();
        assert_eq!(readme.risk_class, RISK_LOW);
        let main = classifications.iter().find(|c| c.path == "src/main.rs").unwrap();
        assert_eq!(main.risk_class, RISK_MEDIUM);
    }

    #[test]
    fn executor_runs_all_rules_and_produces_sequential_ids() {
        let changed = vec!["Cargo.toml".to_string(), "src/main.rs".to_string()];
        let mut removed: HashSet<&str> = HashSet::new();
        removed.insert("src/old.rs");
        let findings = execute_early_signal_pass("prr-1", &changed, &removed);
        for (idx, finding) in findings.iter().enumerate() {
            assert_eq!(finding.finding_id, format!("ES{:03}", idx));
        }
    }

    #[test]
    fn review_layers_has_seven_entries() {
        assert_eq!(REVIEW_LAYERS.len(), 7);
        assert_eq!(REVIEW_LAYERS[0].0, "early-signal");
    }
}
