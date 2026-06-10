//! File classification by path-pattern heuristics.
//!
//! Assigns every changed file to one of ten [`FileBucket`] categories based
//! on its path prefix, suffix, and directory location. Classification is
//! deterministic and never inspects file content — Canon does not perform
//! semantic analysis.

use crate::domain::review_coverage::FileBucket;

/// The file extension for Rust source code.
const RUST_EXT: &str = "rs";

/// The file extension for SQL migration scripts.
const SQL_EXT: &str = "sql";

/// Known CI configuration files.
const CI_FILES: &[&str] = &["Dockerfile", "Makefile", "Justfile", ".gitlab-ci.yml", "Jenkinsfile"];

/// Known CI directory prefixes.
const CI_DIRS: &[&str] = &[".github/workflows", ".github/actions", ".circleci", ".buildkite"];

/// Known API contract directory prefixes.
const API_CONTRACT_DIRS: &[&str] = &["api/", "openapi/", "contracts/", "schemas/"];

/// Known database migration directory prefixes.
const MIGRATION_DIRS: &[&str] = &["migrations/", "etl/", "db/"];

/// Known test directory prefixes.
const TEST_DIRS: &[&str] = &["tests/", "test/", "testing/", "spec/"];

/// Known generated/vendor directory prefixes.
const GENERATED_DIRS: &[&str] = &["generated/", "vendor/", "node_modules/", "dist/", "build/"];

/// Known documentation extensions.
const DOC_EXTENSIONS: &[&str] = &["md", "adoc", "rst"];

/// Known asset extensions.
const ASSET_EXTENSIONS: &[&str] = &[
    "png", "jpg", "jpeg", "gif", "svg", "ico", "webp", "woff", "woff2", "ttf", "eot", "otf", "mp4",
    "webm", "pdf", "zip", "tar", "gz",
];

/// Classifies a single file path into a [`FileBucket`].
///
/// Classification rules (first-match):
/// 1. Generated/vendor directories → `GeneratedOrVendor`
/// 2. Asset extensions → `Assets`
/// 3. CI directories or known CI files → `BuildCi`
/// 4. Documentation extensions → `Documentation`
/// 5. `.rs` under test directories → `Tests`
/// 6. `.rs` files → `ApplicationSource`
/// 7. SQL or files under migration directories → `DatabaseMigrations`
/// 8. API contract directories or JSON/YAML under contract-like paths →
///    `ApiContracts`
/// 9. Configuration files (`.toml`, `.json`, `.yaml`, `.yml`, `.env`) at root
///    or under `config/` → `Configuration`
/// 10. Everything else → `Unknown`
pub fn classify_file(path: &str) -> FileBucket {
    let lower = path.to_ascii_lowercase();
    let ext = path.rsplit('.').next().unwrap_or("").to_ascii_lowercase();
    let file_name = path.rsplit('/').next().unwrap_or(path);

    // Steps 1-4: early-stage buckets (generated, assets, build-ci, docs)
    if let Some(bucket) = classify_early_bucket(&lower, &ext, file_name) {
        return bucket;
    }

    // Steps 5-7: source-related buckets (tests, app source, migrations)
    if let Some(bucket) = classify_source_bucket(&lower, &ext) {
        return bucket;
    }

    // Steps 8-9: tail buckets (api contracts, configuration)
    if let Some(bucket) = classify_tail_bucket(&lower, &ext) {
        return bucket;
    }

    FileBucket::Unknown
}

/// Checks bucket categories 1-4: generated/vendor, assets, build/CI, documentation.
fn classify_early_bucket(lower: &str, ext: &str, file_name: &str) -> Option<FileBucket> {
    if GENERATED_DIRS.iter().any(|d| lower.starts_with(d)) {
        return Some(FileBucket::GeneratedOrVendor);
    }
    if ASSET_EXTENSIONS.contains(&ext) {
        return Some(FileBucket::Assets);
    }
    if CI_DIRS.iter().any(|d| lower.starts_with(d)) {
        return Some(FileBucket::BuildCi);
    }
    if CI_FILES.contains(&file_name) {
        return Some(FileBucket::BuildCi);
    }
    if ext == "lock" {
        return Some(FileBucket::BuildCi);
    }
    if DOC_EXTENSIONS.contains(&ext) {
        return Some(FileBucket::Documentation);
    }
    None
}

/// Checks bucket categories 5-7: tests, application source, database migrations.
fn classify_source_bucket(lower: &str, ext: &str) -> Option<FileBucket> {
    if ext == RUST_EXT && TEST_DIRS.iter().any(|d| lower.starts_with(d)) {
        return Some(FileBucket::Tests);
    }
    if ext == RUST_EXT {
        return Some(FileBucket::ApplicationSource);
    }
    if ext == SQL_EXT || MIGRATION_DIRS.iter().any(|d| lower.starts_with(d)) {
        return Some(FileBucket::DatabaseMigrations);
    }
    None
}

/// Checks bucket categories 8-9: API contracts, configuration.
fn classify_tail_bucket(lower: &str, ext: &str) -> Option<FileBucket> {
    if API_CONTRACT_DIRS.iter().any(|d| lower.starts_with(d)) {
        return Some(FileBucket::ApiContracts);
    }
    if matches!(ext, "json" | "yaml" | "yml")
        && (lower.contains("/api/") || lower.contains("/contract"))
    {
        return Some(FileBucket::ApiContracts);
    }
    if matches!(ext, "toml" | "env") {
        return Some(FileBucket::Configuration);
    }
    if matches!(ext, "json" | "yaml" | "yml")
        && (lower.starts_with("config/")
            || lower.starts_with("config.")
            || lower == "config.json"
            || lower == "config.yaml"
            || lower == "config.yml"
            || !lower.contains('/'))
    {
        return Some(FileBucket::Configuration);
    }
    None
}

/// Classifies a list of file paths, returning `(path, bucket)` pairs.
pub fn classify_files(paths: &[String]) -> Vec<(String, FileBucket)> {
    paths.iter().map(|p| (p.clone(), classify_file(p))).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Application source ───────────────────────────────────────────────

    #[test]
    fn classify_application_source() {
        assert_eq!(classify_file("src/main.rs"), FileBucket::ApplicationSource);
        assert_eq!(classify_file("src/domain/user.rs"), FileBucket::ApplicationSource);
        assert_eq!(classify_file("crates/lib/src/mod.rs"), FileBucket::ApplicationSource);
    }

    // ── Tests ────────────────────────────────────────────────────────────

    #[test]
    fn classify_tests() {
        assert_eq!(classify_file("tests/integration.rs"), FileBucket::Tests);
        assert_eq!(classify_file("test/unit.rs"), FileBucket::Tests);
        assert_eq!(classify_file("testing/fixture.rs"), FileBucket::Tests);
        assert_eq!(classify_file("spec/contract_test.rs"), FileBucket::Tests);
    }

    // ── API contracts ────────────────────────────────────────────────────

    #[test]
    fn classify_api_contracts() {
        assert_eq!(classify_file("api/openapi.json"), FileBucket::ApiContracts);
        assert_eq!(classify_file("openapi/spec.yaml"), FileBucket::ApiContracts);
        assert_eq!(classify_file("contracts/service.yml"), FileBucket::ApiContracts);
        assert_eq!(classify_file("schemas/user.json"), FileBucket::ApiContracts);
    }

    // ── Database migrations ─────────────────────────────────────────────

    #[test]
    fn classify_migrations() {
        assert_eq!(classify_file("migrations/001_init.sql"), FileBucket::DatabaseMigrations);
        assert_eq!(classify_file("etl/transform.sql"), FileBucket::DatabaseMigrations);
        assert_eq!(classify_file("db/schema.sql"), FileBucket::DatabaseMigrations);
    }

    // ── Configuration ───────────────────────────────────────────────────—

    #[test]
    fn classify_configuration() {
        assert_eq!(classify_file("config/app.toml"), FileBucket::Configuration);
        assert_eq!(classify_file("Cargo.toml"), FileBucket::Configuration);
        assert_eq!(classify_file(".env"), FileBucket::Configuration);
        assert_eq!(classify_file("config/default.json"), FileBucket::Configuration);
    }

    // ── Build/CI ─────────────────────────────────────────────────────────

    #[test]
    fn classify_build_ci() {
        assert_eq!(classify_file(".github/workflows/ci.yml"), FileBucket::BuildCi);
        assert_eq!(classify_file("Dockerfile"), FileBucket::BuildCi);
        assert_eq!(classify_file("Makefile"), FileBucket::BuildCi);
    }

    // ── Documentation ────────────────────────────────────────────────────

    #[test]
    fn classify_documentation() {
        assert_eq!(classify_file("README.md"), FileBucket::Documentation);
        assert_eq!(classify_file("docs/guide.adoc"), FileBucket::Documentation);
        assert_eq!(classify_file("CHANGELOG.md"), FileBucket::Documentation);
    }

    // ── Generated/vendor ─────────────────────────────────────────────────

    #[test]
    fn classify_generated_or_vendor() {
        assert_eq!(classify_file("generated/proto.rs"), FileBucket::GeneratedOrVendor);
        assert_eq!(classify_file("vendor/lib.rs"), FileBucket::GeneratedOrVendor);
        assert_eq!(classify_file("node_modules/pkg/index.js"), FileBucket::GeneratedOrVendor);
    }

    // ── Assets ───────────────────────────────────────────────────────────

    #[test]
    fn classify_assets() {
        assert_eq!(classify_file("logo.png"), FileBucket::Assets);
        assert_eq!(classify_file("assets/icon.svg"), FileBucket::Assets);
        assert_eq!(classify_file("fonts/roboto.woff2"), FileBucket::Assets);
    }

    // ── Unknown ──────────────────────────────────────────────────────────

    #[test]
    fn classify_unknown() {
        assert_eq!(classify_file("scripts/deploy.sh"), FileBucket::Unknown);
        assert_eq!(classify_file("data/export.csv"), FileBucket::Unknown);
    }

    // ── classify_files aggregates ───────────────────────────────────────

    #[test]
    fn classify_files_returns_all_entries() {
        let paths: Vec<String> =
            vec!["src/main.rs".to_string(), "tests/test.rs".to_string(), "README.md".to_string()];
        let classified = classify_files(&paths);
        assert_eq!(classified.len(), 3);
        assert_eq!(classified[0].1, FileBucket::ApplicationSource);
        assert_eq!(classified[1].1, FileBucket::Tests);
        assert_eq!(classified[2].1, FileBucket::Documentation);
    }
}
