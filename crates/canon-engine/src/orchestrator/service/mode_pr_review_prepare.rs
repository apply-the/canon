//! Orchestrator for the `pr-review prepare` phase.
//!
//! Collects the diff, classifies changed files, builds the context index,
//! and generates staged 7-layer review packets with concrete target lists,
//! layer-specific instructions, and populated context references so the
//! LLM agent receives actionable work packets — not empty placeholders.

use std::fs;
use std::path::Path;

use crate::domain::review_coverage::FileBucket;
use crate::review::classifier;

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
        // Build context index and classify files BEFORE generating layers,
        // so each layer receives populated required-context.tsv and
        // concrete review targets instead of empty placeholders.
        let index = crate::review::context::ContextIndex::build(
            &diff_output.changed_files,
            &diff_output.patch,
        );
        let classifications = classifier::classify_files(&diff_output.changed_files);
        generate_layer_directories(&run_dir, &diff_output.changed_files, &classifications, &index)?;
        write_review_plan_md(&run_dir)?;

        // ── Existing onion-layer logic ──────────────────────────────────────
        let high_risk: Vec<&String> =
            diff_output.changed_files.iter().filter(|f| is_high_risk(f)).collect();

        write_review_brief(&run_dir, base_ref, head_ref, &diff_output.changed_files)?;
        write_review_plan(&run_dir)?;
        write_context_index(&run_dir, &index)?;
        write_changed_files(&run_dir, &diff_output.changed_files)?;
        write_high_risk_files(&run_dir, &high_risk)?;
        write_relation_hints(&run_dir, &diff_output.changed_files)?;
        write_diff_patch(&run_dir, &diff_output.patch)?;
        write_run_state(&run_dir, "awaiting_early_signal")?;
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

// ── Layer instruction generators ─────────────────────────────────────────

/// Builds layer-specific instructions with concrete methodology,
/// output contracts, and completion criteria.
fn build_layer_instructions(
    slug: &str,
    ordinal: u32,
    changed_files: &[String],
    classifications: &[(String, FileBucket)],
) -> String {
    let total = changed_files.len();
    let app_source: Vec<_> =
        classifications.iter().filter(|(_, b)| *b == FileBucket::ApplicationSource).collect();
    let high_risk_count = app_source.len();
    let test_count = classifications.iter().filter(|(_, b)| *b == FileBucket::Tests).count();

    match slug {
        "application-source" => application_source_template(ordinal, total, high_risk_count),
        "high-risk-surfaces" => high_risk_surfaces_template(ordinal, total, classifications),
        "related-context" => related_context_template(ordinal, total),
        "logical-stress" => logical_stress_template(ordinal, high_risk_count),
        "tests" => tests_template(ordinal, test_count, app_source.len()),
        // Canon-executed layers (early-signal, coverage-accounting) get a simple template
        _ => format!(
            "# {slug} (Layer {ordinal})\n\n\
             ## Instructions\n\n\
             This layer is executed deterministically by Canon.\n\
             See `required-context.tsv` for context entries relevant to this layer.\n\n\
             ## Output\n\n\
             Canon writes findings to `output.md` in this directory.\n"
        ),
    }
}

/// Template for layer 2: Application-Source Review.
fn application_source_template(ordinal: u32, total: usize, high_risk: usize) -> String {
    format!(
        "# Application-Source Review (Layer {ordinal})\n\n\
         ## Objective\n\n\
         Deeply review every file listed in `review-targets.tsv` with \
         `expected_depth = deep`. Lightly review files with \
         `expected_depth = light`.\n\n\
         ## Targets\n\n\
         **Total targets**: {total}\n\
         **High-risk targets**: {high_risk}\n\n\
         See `review-targets.tsv` for the complete list.\n\
         See `required-context.tsv` for context IDs.\n\n\
         ## Review Methodology\n\n\
         For each deep-reviewed file, inspect:\n\n\
         - Correctness: logic errors, off-by-one, inverted conditions\n\
         - Control flow: reachable branches, early returns, error paths\n\
         - Error handling: propagated/logged errors, swallowed errors\n\
         - Async behavior: missing .await, blocking in async, concurrency\n\
         - Retries & timeouts: bounded retry, configured timeouts\n\
         - Resource usage: connection/file/memory leaks\n\
         - Performance: O(n²) loops, unnecessary allocations\n\
         - Security: input validation, injection vectors, auth checks\n\
         - Compatibility: caller breakage, backward-incompatible changes\n\n\
         ## Output Contract\n\n\
         For each file with a finding:\n\
         ```\n\
         ### Finding F{{{{id}}}}\n\
         - **Severity**: blocking|major|minor|question|nitpick\n\
         - **Path**: `src/file.rs`\n\
         - **Line**: N\n\
         - **Context IDs**: C001\n\
         - **Summary**: one-line description\n\
         - **Why it matters**: impact\n\
         - **Suggested remediation**: fix suggestion\n\
         ```\n\n\
         For each file reviewed with no finding:\n\
         ```\n\
         ### Reviewed: file_path\n\
         - **Depth**: deep|light\n\
         - **Concerns inspected**: error handling, async, perf\n\
         - **Result**: no finding\n\
         ```\n\n\
         ## Completion Criteria\n\n\
         - [ ] Every high-risk deep target has a review record\n\
         - [ ] Every finding includes severity, context IDs, and remediation\n\
         - [ ] At least {high_risk} high-risk files are deeply reviewed\n",
        ordinal = ordinal,
        total = total,
        high_risk = high_risk,
    )
}

/// Template for layer 3: High-Risk Surfaces Review.
fn high_risk_surfaces_template(
    ordinal: u32,
    total: usize,
    classifications: &[(String, FileBucket)],
) -> String {
    use std::collections::BTreeMap;
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    for (_, b) in classifications {
        if matches!(
            b,
            FileBucket::ApiContracts
                | FileBucket::DatabaseMigrations
                | FileBucket::Configuration
                | FileBucket::BuildCi
        ) {
            *counts.entry(format!("{:?}", b)).or_default() += 1;
        }
    }
    let mut bucket_list = String::new();
    for (name, count) in &counts {
        bucket_list.push_str(&format!("- `{name}`: {count} files\n"));
    }

    format!(
        "# High-Risk Surfaces Review (Layer {ordinal})\n\n\
         ## Objective\n\n\
         Review files touching high-risk surface areas: build/CI, API contracts, \
         database migrations, configuration, deployment.\n\n\
         ## Targets\n\n\
         **Total targets**: {total}\n\n\
         **Targets by bucket**:\n{buckets}\n\
         See `review-targets.tsv` for the complete list.\n\n\
         ## Review Methodology\n\n\
         - Build/CI: build pipeline correctness, new dependencies, test invocation\n\
         - API contracts: schema drift, enum changes, type mismatches\n\
         - Database migrations: reversibility, data integrity, breaking schema changes\n\
         - Configuration: new keys documented, safe defaults, env overrides\n\
         - Deployment: manifests match code, resource limits adequate\n\n\
         ## Output Contract\n\n\
         Same format as layer 2. Write findings to `output.md`.\n",
        ordinal = ordinal,
        total = total,
        buckets = bucket_list,
    )
}

/// Template for layer 4: Related-Context Review.
fn related_context_template(ordinal: u32, total: usize) -> String {
    format!(
        "# Related-Context Review (Layer {ordinal})\n\n\
         ## Objective\n\n\
         Inspect call sites, dependents, imports, exports, examples, and docs \
         connected to changed behavior.\n\n\
         ## Targets\n\n\
         **Total targets**: {total}\n\n\
         See `review-targets.tsv` for the complete list.\n\n\
         ## Review Methodology\n\n\
         - Callers/imports: will callers break? new imports missing?\n\
         - Docs/examples: do docs match new behavior? examples up-to-date?\n\
         - Tests related to changed files: test fixtures still valid?\n\
         - Public contracts: trait impls, public fns, type exports consistent?\n\n\
         ## Output Contract\n\n\
         Same format as layer 2. Write findings to `output.md`.\n",
        ordinal = ordinal,
        total = total,
    )
}

/// Template for layer 5: Logical Stress Review.
fn logical_stress_template(ordinal: u32, high_risk: usize) -> String {
    format!(
        "# Logical Stress Review (Layer {ordinal})\n\n\
         ## Objective\n\n\
         Act as QA/security reviewer. Stress-test changed high-risk files \
         against edge cases, failure modes, and adversarial inputs.\n\n\
         ## Targets\n\n\
         **High-risk application source files**: {high_risk}\n\n\
         See `review-targets.tsv` for the complete list.\n\n\
         ## Review Methodology\n\n\
         Check each target for:\n\n\
         - Null/empty input: empty strings, null values, missing fields\n\
         - Malformed input: invalid UTF-8, huge payloads, deep nesting\n\
         - Boundary values: MAX/MIN, empty collections, epoch/overflow\n\
         - Failed downstream: DB down, HTTP timeout, channel closed\n\
         - Partial failures: N of M shards succeed, partial results\n\
         - Timeout & retry: bounded timeout, exponential backoff, circuit breaker\n\
         - Idempotency: retried operations safe? double-charge?\n\
         - Concurrency: data races, lock across await, state corruption\n\
         - Cancellation: future dropped, resources leaked, inconsistent state\n\
         - Resource exhaustion: unbounded queues, disk/memory/connection fill\n\
         - State transitions: all valid? invalid state reachable?\n\n\
         ## Output Contract\n\n\
         Same format as layer 2. Write findings to `output.md`.\n\n\
         ## Completion Criteria\n\n\
         - [ ] Every high-risk file has a stress review record\n\
         - [ ] At least one edge case per category is explicitly tested\n\
         - [ ] Any crash, panic, hang, or data corruption is blocking\n",
        ordinal = ordinal,
        high_risk = high_risk,
    )
}

/// Template for layer 6: Tests Review.
fn tests_template(ordinal: u32, test_count: usize, missing_candidates: usize) -> String {
    format!(
        "# Tests Review (Layer {ordinal})\n\n\
         ## Objective\n\n\
         Compare changed behavior against existing tests. Identify missing \
         test coverage.\n\n\
         ## Targets\n\n\
         **Test files to review**: {test_count}\n\
         **Missing-test candidates**: {missing}\n\n\
         See `review-targets.tsv` for the complete list.\n\n\
         ## Review Methodology\n\n\
         For each changed source file, determine coverage for:\n\n\
         - Success path: normal operation with valid inputs\n\
         - Failure path: error handling, invalid inputs, edge cases\n\
         - Compatibility: old behavior still works? regression tests?\n\
         - Integration: end-to-end contract tests exercise changed behavior?\n\n\
         Missing-test findings should be severity `major` or `blocking`.\n\n\
         ## Output Contract\n\n\
         Same format as layer 2. Write findings to `output.md`.\n",
        ordinal = ordinal,
        test_count = test_count,
        missing = missing_candidates,
    )
}

// ── required-context.tsv builder ──────────────────────────────────────────

/// Builds the `required-context.tsv` content for a specific layer,
/// populated from the global context index. Returns a header-only TSV
/// when no relevant context entries exist.
fn build_required_context_tsv(
    slug: &str,
    _ordinal: u32,
    index: &crate::review::context::ContextIndex,
    _classifications: &[(String, FileBucket)],
) -> String {
    // Map layer slugs to the context index layer names
    let ctx_layer = match slug {
        "application-source" => "application_source",
        "high-risk-surfaces" => "high_risk",
        "related-context" => "related_context",
        "logical-stress" => "logical_stress",
        "tests" => "test",
        _ => slug,
    };

    let ctx_entries: Vec<_> =
        index.entries.iter().filter(|e| e.layer == ctx_layer || e.layer == "diff").collect();
    if ctx_entries.is_empty() {
        // No matching layer entries — include all diff entries as baseline
        let all_diff: Vec<_> = index.entries.iter().filter(|e| e.layer == "diff").collect();
        if all_diff.is_empty() {
            return "id\ttype\tpath\trink\treason\n".to_string();
        }
        let header = "id\ttype\tpath\trisk\treason".to_string();
        let rows: Vec<String> = all_diff
            .iter()
            .map(|e| format!("{}\t{}\t{}\t{}\t{}", e.id, e.entry_type, e.path, e.risk, e.reason))
            .collect();
        return format!("{header}\n{}", rows.join("\n"));
    }
    let header = "id\ttype\tpath\trisk\treason".to_string();
    let rows: Vec<String> = ctx_entries
        .iter()
        .map(|e| format!("{}\t{}\t{}\t{}\t{}", e.id, e.entry_type, e.path, e.risk, e.reason))
        .collect();
    format!("{header}\n{}", rows.join("\n"))
}

// ── review-targets.tsv builder ────────────────────────────────────────────

/// Builds the `review-targets.tsv` content for a specific layer,
/// listing concrete files with bucket, risk level, reason, and expected depth.
fn build_review_targets_tsv(
    slug: &str,
    prefix: &str,
    ordinal: u32,
    changed_files: &[String],
    classifications: &[(String, FileBucket)],
) -> String {
    let header = "id\tpath\tbucket\trisk\treason\texpected_depth".to_string();
    let mut counter = 0u32;
    let mut rows = Vec::new();

    for (path, bucket) in classifications {
        // Determine if this file is relevant to this layer
        if !layer_includes_file(slug, bucket) {
            continue;
        }

        counter += 1;
        let id = format!("{prefix}{:02}{:03}", ordinal, counter);
        let risk = risk_for_bucket(bucket);
        let reason = bucket_reason(bucket);
        let depth = expected_depth_for(slug, bucket);

        rows.push(format!("{id}\t{path}\t{:?}\t{risk}\t{reason}\t{depth}", bucket));
    }

    if rows.is_empty() {
        // Include all changed files as baseline context
        for path in changed_files {
            counter += 1;
            let id = format!("{prefix}{:02}{:03}", ordinal, counter);
            rows.push(format!("{id}\t{path}\tUnknown\tlow\tincluded in layer {ordinal}\tindexed"));
        }
    }

    format!("{header}\n{}", rows.join("\n"))
}

/// Returns whether a file from the given bucket should be included in the
/// specified review layer.
fn layer_includes_file(slug: &str, bucket: &FileBucket) -> bool {
    match slug {
        "application-source" => matches!(bucket, FileBucket::ApplicationSource),
        "high-risk-surfaces" => matches!(
            bucket,
            FileBucket::ApiContracts
                | FileBucket::DatabaseMigrations
                | FileBucket::Configuration
                | FileBucket::BuildCi
        ),
        "related-context" => matches!(
            bucket,
            FileBucket::ApplicationSource | FileBucket::ApiContracts | FileBucket::Documentation
        ),
        "logical-stress" => matches!(bucket, FileBucket::ApplicationSource),
        "tests" => matches!(bucket, FileBucket::Tests | FileBucket::ApplicationSource),
        // Canon-executed layers include everything for context
        _ => true,
    }
}

/// Returns the risk level for a file bucket.
const fn risk_for_bucket(bucket: &FileBucket) -> &str {
    match bucket {
        FileBucket::ApplicationSource
        | FileBucket::ApiContracts
        | FileBucket::DatabaseMigrations => "high",
        FileBucket::Configuration | FileBucket::BuildCi => "medium",
        _ => "low",
    }
}

/// Returns the reason text for why a file in the given bucket is in review.
const fn bucket_reason(bucket: &FileBucket) -> &str {
    match bucket {
        FileBucket::ApplicationSource => "application behavior changed",
        FileBucket::Tests => "test coverage for changed behavior",
        FileBucket::ApiContracts => "API contract may have changed",
        FileBucket::DatabaseMigrations => "schema or data migration changed",
        FileBucket::Configuration => "configuration changed",
        FileBucket::BuildCi => "build or CI pipeline changed",
        FileBucket::Documentation => "documentation may need update",
        FileBucket::GeneratedOrVendor => "generated or vendored file changed",
        FileBucket::Assets => "asset changed",
        FileBucket::Unknown => "file classification unknown",
    }
}

/// Returns the expected review depth for a file in the given bucket and layer.
fn expected_depth_for(slug: &str, bucket: &FileBucket) -> &'static str {
    match slug {
        "application-source" => {
            if *bucket == FileBucket::ApplicationSource {
                "deep"
            } else {
                "light"
            }
        }
        "high-risk-surfaces" => {
            if matches!(bucket, FileBucket::ApiContracts | FileBucket::DatabaseMigrations) {
                "deep"
            } else {
                "light"
            }
        }
        "logical-stress" => "deep",
        "tests" => {
            if *bucket == FileBucket::Tests {
                "deep"
            } else {
                "light"
            }
        }
        "related-context" => "light",
        _ => "indexed",
    }
}

/// Writes `run-state.json` with staged execution model.
///
/// Sets the initial state and creates per-layer status entries so the LLM
/// agent and accept/finalize phases can track which layers have been
/// completed, deferred, or failed.
fn write_run_state(dir: &Path, state: &str) -> Result<(), String> {
    let layer_states = serde_json::json!({
        "early-signal": {"status": if state == "awaiting_early_signal" { "pending" } else { "completed" }},
        "application-source": {"status": "pending"},
        "high-risk-surfaces": {"status": "pending"},
        "related-context": {"status": "pending"},
        "logical-stress": {"status": "pending"},
        "tests": {"status": "pending"},
        "coverage-accounting": {"status": "pending"},
    });
    let json = serde_json::json!({
        "run_state": state,
        "next_layer": if state == "awaiting_early_signal" { "early-signal" } else { "awaiting_application_source" },
        "layer_states": layer_states,
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
/// Each layer directory receives:
/// - `instructions.md` — layer-specific methodology, output contract, completion criteria
/// - `required-context.tsv` — context IDs from the global context index relevant to this layer
/// - `review-targets.tsv` — concrete files to review with bucket, risk, reason, and expected depth
/// - `output.md` — placeholder for LLM findings
pub(crate) fn generate_layer_directories(
    run_dir: &Path,
    changed_files: &[String],
    classifications: &[(String, FileBucket)],
    index: &crate::review::context::ContextIndex,
) -> Result<(), String> {
    let layers_dir = run_dir.join("layers");
    let target_id_prefix = "T";

    for (idx, (slug, display_name, _canonical)) in EARLY_SIGNAL_LAYERS.iter().enumerate() {
        let ordinal = idx as u32 + 1;
        let layer_dir = layers_dir.join(format!("{:02}-{}", ordinal, slug));
        fs::create_dir_all(&layer_dir).map_err(|e| e.to_string())?;

        // ── Layer-specific instructions ─────────────────────────────────
        let instructions = build_layer_instructions(slug, ordinal, changed_files, classifications);
        fs::write(layer_dir.join("instructions.md"), &instructions).map_err(|e| e.to_string())?;

        // ── Populated required-context.tsv ──────────────────────────────
        let ctx_tsv = build_required_context_tsv(slug, ordinal, index, classifications);
        fs::write(layer_dir.join("required-context.tsv"), &ctx_tsv).map_err(|e| e.to_string())?;

        // ── Concrete review targets ─────────────────────────────────────
        let targets_tsv = build_review_targets_tsv(
            slug,
            target_id_prefix,
            ordinal,
            changed_files,
            classifications,
        );
        fs::write(layer_dir.join("review-targets.tsv"), &targets_tsv).map_err(|e| e.to_string())?;

        // ── Output placeholder ──────────────────────────────────────────
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
        let empty_files: Vec<String> = Vec::new();
        let empty_classifications: Vec<(String, FileBucket)> = Vec::new();
        let empty_index = crate::review::context::ContextIndex::build(&[], "");
        generate_layer_directories(dir.path(), &empty_files, &empty_classifications, &empty_index)
            .unwrap();
        let layers = dir.path().join("layers");
        assert!(layers.is_dir());
        for (idx, (slug, _, _)) in EARLY_SIGNAL_LAYERS.iter().enumerate() {
            let ordinal = idx + 1;
            let layer_dir = layers.join(format!("{:02}-{}", ordinal, slug));
            assert!(layer_dir.is_dir(), "missing {:02}-{}", ordinal, slug);
            assert!(layer_dir.join("instructions.md").exists());
            assert!(layer_dir.join("required-context.tsv").exists());
            assert!(layer_dir.join("review-targets.tsv").exists());
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

    // ── Staged orchestration tests ───────────────────────────────────────

    #[test]
    fn prepare_populates_required_context_tsv_for_app_source() {
        let dir = TempDir::new().unwrap();
        let changed: Vec<String> = vec!["src/main.rs".to_string(), "src/lib.rs".to_string()];
        let classifications = classifier::classify_files(&changed);
        let index = crate::review::context::ContextIndex::build(&changed, "");
        generate_layer_directories(dir.path(), &changed, &classifications, &index).unwrap();

        let ctx_path =
            dir.path().join("layers").join("02-application-source").join("required-context.tsv");
        let content = fs::read_to_string(&ctx_path).unwrap();
        // Should contain header and data rows, not just the empty header
        assert!(content.contains("id\ttype\tpath\trisk\treason"));
        assert!(content.lines().count() > 1, "required-context.tsv should have data rows");
    }

    #[test]
    fn prepare_populates_review_targets_tsv_for_rust_files() {
        let dir = TempDir::new().unwrap();
        let changed: Vec<String> =
            vec!["src/main.rs".to_string(), "src/lib.rs".to_string(), "tests/test.rs".to_string()];
        let classifications = classifier::classify_files(&changed);
        let index = crate::review::context::ContextIndex::build(&changed, "");
        generate_layer_directories(dir.path(), &changed, &classifications, &index).unwrap();

        let targets_path =
            dir.path().join("layers").join("02-application-source").join("review-targets.tsv");
        let content = fs::read_to_string(&targets_path).unwrap();
        assert!(content.contains("id\tpath\tbucket\trisk\treason\texpected_depth"));
        assert!(content.contains("src/main.rs"));
        assert!(content.contains("src/lib.rs"));
        assert!(content.contains("ApplicationSource"));
        assert!(content.contains("deep"));
    }

    #[test]
    fn prepare_writes_specific_instructions_for_app_source() {
        let dir = TempDir::new().unwrap();
        let changed: Vec<String> = vec!["src/main.rs".to_string()];
        let classifications = classifier::classify_files(&changed);
        let index = crate::review::context::ContextIndex::build(&changed, "");
        generate_layer_directories(dir.path(), &changed, &classifications, &index).unwrap();

        let inst_path =
            dir.path().join("layers").join("02-application-source").join("instructions.md");
        let content = fs::read_to_string(&inst_path).unwrap();
        assert!(content.contains("Application-Source Review"));
        assert!(content.contains("Correctness"));
        assert!(content.contains("Error handling"));
        assert!(content.contains("Security"));
        assert!(content.contains("### Finding"));
        assert!(content.contains("### Reviewed"));
        assert!(
            !content.contains("Review the changed files according to the"),
            "instructions should NOT be generic one-liner"
        );
    }

    #[test]
    fn prepare_writes_specific_instructions_for_logical_stress() {
        let dir = TempDir::new().unwrap();
        let changed: Vec<String> = vec!["src/main.rs".to_string()];
        let classifications = classifier::classify_files(&changed);
        let index = crate::review::context::ContextIndex::build(&changed, "");
        generate_layer_directories(dir.path(), &changed, &classifications, &index).unwrap();

        let inst_path = dir.path().join("layers").join("05-logical-stress").join("instructions.md");
        let content = fs::read_to_string(&inst_path).unwrap();
        assert!(content.contains("Logical Stress Review"));
        assert!(content.contains("Null/empty input"));
        assert!(content.contains("Malformed input"));
        assert!(content.contains("Concurrency"));
        assert!(content.contains("Idempotency"));
    }

    #[test]
    fn review_targets_tsv_has_expected_columns_and_data() {
        let dir = TempDir::new().unwrap();
        let changed: Vec<String> =
            vec!["src/main.rs".to_string(), "Cargo.toml".to_string(), "README.md".to_string()];
        let classifications = classifier::classify_files(&changed);
        let index = crate::review::context::ContextIndex::build(&changed, "");
        generate_layer_directories(dir.path(), &changed, &classifications, &index).unwrap();

        // App source layer should only contain .rs files
        let targets = fs::read_to_string(
            dir.path().join("layers").join("02-application-source").join("review-targets.tsv"),
        )
        .unwrap();
        assert!(targets.contains("src/main.rs"));
        assert!(
            !targets.contains("Cargo.toml"),
            "non-Rust files should not be in app-source layer"
        );
        assert!(!targets.contains("README.md"));
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

        // Configure user for CI environments
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

        // Configure user for CI environments
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
