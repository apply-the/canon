//! Run resolution: map a user-supplied query (display id, full UUID,
//! prefix short id, or `@last`) to a unique run on disk.
//!
//! See `specs/009-run-id-display/contracts/run-identity-contract.md` §C-6
//! and `data-model.md` "RunHandle" / "LookupQuery".

use std::fmt;
use std::fs;
use std::path::PathBuf;

use time::OffsetDateTime;

use crate::domain::run::{is_canonical_display_id, short_id_from_uuid};
use crate::persistence::layout::{ProjectLayout, parse_dated_display_id, parse_run_dir_name};
use crate::persistence::manifests::RunManifest;

/// Result of resolving a [`LookupQuery`] against the on-disk run set.
#[derive(Debug, Clone)]
pub struct RunHandle {
    /// Canonical filesystem key (display id for new runs, UUID for legacy).
    pub run_id: String,
    /// Canonical machine identity (UUID), reconstructed for legacy runs.
    pub uuid: String,
    /// 8-char hex short id derived from `uuid`.
    pub short_id: String,
    /// Absolute path to the run directory.
    pub directory: PathBuf,
    /// True when this run lives at the legacy `runs/<uuid>/` location.
    pub is_legacy: bool,
    /// Manifest creation timestamp.
    pub created_at: OffsetDateTime,
}

/// User-supplied query forms.
#[derive(Debug, Clone)]
pub enum LookupQuery {
    /// Full display id `R-YYYYMMDD-XXXXXXXX`.
    FullRunId(String),
    /// Full UUID string (any canonical form).
    FullUuid(String),
    /// Prefix match against `short_id` (1–8 hex chars).
    ShortId(String),
    /// `@last` — most recent run by `created_at`.
    Last,
}

impl LookupQuery {
    /// Smart-parse a user-supplied token into a query variant.
    pub fn parse(input: &str) -> Self {
        let trimmed = input.trim();
        if trimmed == "@last" {
            return Self::Last;
        }
        if is_canonical_display_id(trimmed) {
            return Self::FullRunId(trimmed.to_string());
        }
        if let Ok(parsed) = trimmed.parse::<uuid::Uuid>() {
            return Self::FullUuid(parsed.as_simple().to_string());
        }
        // Treat anything else that looks like hex as a short-id prefix; fall
        // through to `FullRunId` as a last resort so the resolver can return
        // a clear `NotFound` error if it really is unknown.
        if !trimmed.is_empty()
            && trimmed.len() <= 16
            && trimmed.chars().all(|c| c.is_ascii_hexdigit())
        {
            return Self::ShortId(trimmed.to_ascii_lowercase());
        }
        Self::FullRunId(trimmed.to_string())
    }
}

#[derive(Debug)]
pub enum LookupError {
    NotFound { query: String },
    Ambiguous { query: String, matches: Vec<String> },
    EmptyHistory,
    Io(std::io::Error),
}

impl fmt::Display for LookupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound { query } => {
                write!(f, "no run matches `{query}`")
            }
            Self::Ambiguous { query, matches } => {
                writeln!(f, "ambiguous run reference `{query}`; matches:")?;
                for m in matches {
                    writeln!(f, "  {m}")?;
                }
                Ok(())
            }
            Self::EmptyHistory => write!(f, "no runs exist in this repository"),
            Self::Io(error) => write!(f, "io error scanning runs: {error}"),
        }
    }
}

impl std::error::Error for LookupError {}

impl From<std::io::Error> for LookupError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

/// Resolve a query against the on-disk run set.
pub fn resolve(layout: &ProjectLayout, query: &LookupQuery) -> Result<RunHandle, LookupError> {
    let runs = scan_all(layout)?;
    match query {
        LookupQuery::Last => runs
            .into_iter()
            .max_by(|a, b| a.created_at.cmp(&b.created_at).then(a.run_id.cmp(&b.run_id)))
            .ok_or(LookupError::EmptyHistory),
        LookupQuery::FullRunId(needle) => {
            let matches: Vec<RunHandle> =
                runs.into_iter().filter(|h| h.run_id.eq_ignore_ascii_case(needle)).collect();
            unique_or_error(needle, matches)
        }
        LookupQuery::FullUuid(needle) => {
            let needle_lower = needle.to_ascii_lowercase();
            let matches: Vec<RunHandle> =
                runs.into_iter().filter(|h| h.uuid.to_ascii_lowercase() == needle_lower).collect();
            unique_or_error(needle, matches)
        }
        LookupQuery::ShortId(needle) => {
            let needle_lower = needle.to_ascii_lowercase();
            let matches: Vec<RunHandle> = runs
                .into_iter()
                .filter(|h| h.short_id.to_ascii_lowercase().starts_with(&needle_lower))
                .collect();
            unique_or_error(needle, matches)
        }
    }
}

fn unique_or_error(needle: &str, mut matches: Vec<RunHandle>) -> Result<RunHandle, LookupError> {
    match matches.len() {
        0 => Err(LookupError::NotFound { query: needle.to_string() }),
        1 => Ok(matches.pop().expect("len == 1")),
        _ => {
            let mut ids: Vec<String> = matches.into_iter().map(|h| h.run_id).collect();
            ids.sort();
            Err(LookupError::Ambiguous { query: needle.to_string(), matches: ids })
        }
    }
}

/// Walk the on-disk run set and return one [`RunHandle`] per discovered run.
/// Includes both the new dated layout (`runs/YYYY/MM/<dir>/`) and legacy
/// UUID-keyed run directories (`runs/<uuid>/`).
pub fn scan_all(layout: &ProjectLayout) -> Result<Vec<RunHandle>, LookupError> {
    let mut handles = Vec::new();
    let runs_root = layout.runs_dir();
    if !runs_root.exists() {
        return Ok(handles);
    }
    for entry in fs::read_dir(&runs_root)? {
        let entry = entry?;
        let name = entry.file_name();
        let Some(name_str) = name.to_str() else {
            continue;
        };
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        if name_str.starts_with("R-") || looks_like_uuid(name_str) {
            // Legacy UUID-keyed run directory directly under runs/.
            push_handle_if_present(&path, &mut handles);
        } else if name_str.len() == 4 && name_str.chars().all(|c| c.is_ascii_digit()) {
            // YYYY/ bucket — recurse one level.
            scan_year(&path, &mut handles)?;
        }
    }
    Ok(handles)
}

fn scan_year(year_dir: &std::path::Path, handles: &mut Vec<RunHandle>) -> Result<(), LookupError> {
    for entry in fs::read_dir(year_dir)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        // MM/ bucket.
        for run_entry in fs::read_dir(&path)? {
            let run_entry = run_entry?;
            let run_path = run_entry.path();
            if run_path.is_dir() {
                push_handle_if_present(&run_path, handles);
            }
        }
    }
    Ok(())
}

fn push_handle_if_present(run_dir: &std::path::Path, handles: &mut Vec<RunHandle>) {
    let manifest_path = run_dir.join("run.toml");
    if !manifest_path.exists() {
        return;
    }
    let Ok(text) = fs::read_to_string(&manifest_path) else {
        return;
    };
    let Ok(manifest): Result<RunManifest, _> = toml::from_str(&text) else {
        return;
    };
    let manifest = manifest.canonicalize();
    let uuid = manifest.uuid.clone().unwrap_or_else(|| manifest.run_id.clone());
    let short_id = manifest.short_id.clone().unwrap_or_else(|| {
        manifest
            .uuid
            .as_deref()
            .and_then(|s| s.parse::<uuid::Uuid>().ok())
            .as_ref()
            .map(short_id_from_uuid)
            .unwrap_or_else(|| {
                let id = manifest.run_id.replace('-', "");
                id.chars().take(8).collect()
            })
    });
    let dir_name = run_dir.file_name().and_then(|n| n.to_str()).unwrap_or("");
    let (display, _slug) = parse_run_dir_name(dir_name);
    let is_legacy = parse_dated_display_id(display).is_none();
    handles.push(RunHandle {
        run_id: manifest.run_id,
        uuid,
        short_id,
        directory: run_dir.to_path_buf(),
        is_legacy,
        created_at: manifest.created_at,
    });
}

fn looks_like_uuid(s: &str) -> bool {
    s.parse::<uuid::Uuid>().is_ok()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use tempfile::tempdir;
    use time::OffsetDateTime;
    use uuid::Uuid;

    use super::{LookupError, LookupQuery, resolve, scan_all};
    use crate::domain::mode::Mode;
    use crate::domain::policy::{RiskClass, UsageZone};
    use crate::domain::run::{ClassificationProvenance, SystemContext};
    use crate::persistence::layout::ProjectLayout;
    use crate::persistence::manifests::RunManifest;

    fn sample_manifest(
        run_id: &str,
        uuid: Option<Uuid>,
        created_at: OffsetDateTime,
    ) -> RunManifest {
        let short_id = uuid.as_ref().map(crate::domain::run::short_id_from_uuid);
        RunManifest {
            run_id: run_id.to_string(),
            uuid: uuid.map(|value| value.as_simple().to_string()),
            short_id,
            slug: None,
            title: Some("Sample run".to_string()),
            mode: Mode::Requirements,
            risk: RiskClass::LowImpact,
            zone: UsageZone::Green,
            system_context: Some(SystemContext::Existing),
            classification: ClassificationProvenance::explicit(),
            owner: "Owner <owner@example.com>".to_string(),
            created_at,
        }
    }

    fn write_manifest(run_dir: &Path, manifest: &RunManifest) {
        fs::create_dir_all(run_dir).expect("create run dir");
        fs::write(run_dir.join("run.toml"), toml::to_string(manifest).expect("serialize manifest"))
            .expect("write run manifest");
    }

    #[test]
    fn lookup_query_parse_recognizes_supported_tokens() {
        let uuid = Uuid::parse_str("abcd1234-0000-7000-8000-000000000001").expect("uuid");

        assert!(matches!(LookupQuery::parse("@last"), LookupQuery::Last));
        assert!(matches!(
            LookupQuery::parse("R-20260422-abcd1234"),
            LookupQuery::FullRunId(value) if value == "R-20260422-abcd1234"
        ));
        assert!(matches!(
            LookupQuery::parse(&uuid.to_string()),
            LookupQuery::FullUuid(value) if value == uuid.as_simple().to_string()
        ));
        assert!(
            matches!(LookupQuery::parse("AbCd"), LookupQuery::ShortId(value) if value == "abcd")
        );
        assert!(matches!(
            LookupQuery::parse("not-a-run"),
            LookupQuery::FullRunId(value) if value == "not-a-run"
        ));
    }

    #[test]
    fn scan_all_returns_empty_when_runs_root_is_missing() {
        let workspace = tempdir().expect("temp workspace");
        let layout = ProjectLayout::new(workspace.path());

        assert!(scan_all(&layout).expect("scan runs").is_empty());
    }

    #[test]
    fn scan_all_discovers_canonical_and_legacy_runs() {
        let workspace = tempdir().expect("temp workspace");
        let layout = ProjectLayout::new(workspace.path());
        let canonical_uuid = Uuid::parse_str("11111111-0000-7000-8000-000000000001").expect("uuid");
        let legacy_uuid = Uuid::parse_str("019db71e-f1bb-7dc2-b535-213e556d16fe").expect("uuid");

        write_manifest(
            &layout.new_run_dir("R-20260422-11111111", Some("slugged-run")),
            &sample_manifest(
                "R-20260422-11111111",
                Some(canonical_uuid),
                OffsetDateTime::from_unix_timestamp(1_700_000_010).expect("timestamp"),
            ),
        );
        write_manifest(
            &layout.runs_dir().join(legacy_uuid.to_string()),
            &sample_manifest(
                &legacy_uuid.to_string(),
                None,
                OffsetDateTime::from_unix_timestamp(1_700_000_020).expect("timestamp"),
            ),
        );

        let invalid_dir = layout.runs_dir().join("2026").join("04").join("R-20260422-deadbeef");
        fs::create_dir_all(&invalid_dir).expect("create invalid dir");
        fs::write(invalid_dir.join("run.toml"), "not = [valid").expect("write invalid manifest");
        fs::write(layout.runs_dir().join("README.txt"), "skip me").expect("write root file");

        let mut handles = scan_all(&layout).expect("scan runs");
        handles.sort_by(|left, right| left.run_id.cmp(&right.run_id));

        assert_eq!(handles.len(), 2);

        let canonical = handles
            .iter()
            .find(|handle| handle.run_id == "R-20260422-11111111")
            .expect("canonical run");
        assert_eq!(canonical.uuid, canonical_uuid.as_simple().to_string());
        assert_eq!(canonical.short_id, "11111111");
        assert!(!canonical.is_legacy);
        assert!(canonical.directory.to_string_lossy().contains("R-20260422-11111111--slugged-run"));

        let legacy = handles
            .iter()
            .find(|handle| handle.run_id == legacy_uuid.to_string())
            .expect("legacy run");
        assert_eq!(legacy.uuid, legacy_uuid.as_simple().to_string());
        assert_eq!(legacy.short_id, "019db71e");
        assert!(legacy.is_legacy);
    }

    #[test]
    fn resolve_supports_last_run_id_uuid_and_short_id_queries() {
        let workspace = tempdir().expect("temp workspace");
        let layout = ProjectLayout::new(workspace.path());
        let first_uuid = Uuid::parse_str("11111111-0000-7000-8000-000000000001").expect("uuid");
        let second_uuid = Uuid::parse_str("22222222-0000-7000-8000-000000000002").expect("uuid");
        let created_at = OffsetDateTime::from_unix_timestamp(1_700_000_030).expect("timestamp");

        write_manifest(
            &layout.new_run_dir("R-20260422-11111111", None),
            &sample_manifest("R-20260422-11111111", Some(first_uuid), created_at),
        );
        write_manifest(
            &layout.new_run_dir("R-20260422-22222222", None),
            &sample_manifest("R-20260422-22222222", Some(second_uuid), created_at),
        );

        let last = resolve(&layout, &LookupQuery::Last).expect("resolve last");
        assert_eq!(last.run_id, "R-20260422-22222222");

        let by_run_id =
            resolve(&layout, &LookupQuery::FullRunId("R-20260422-11111111".to_string()))
                .expect("resolve run id");
        assert_eq!(by_run_id.uuid, first_uuid.as_simple().to_string());

        let by_uuid = resolve(&layout, &LookupQuery::FullUuid(second_uuid.as_simple().to_string()))
            .expect("resolve uuid");
        assert_eq!(by_uuid.run_id, "R-20260422-22222222");

        let by_short_id =
            resolve(&layout, &LookupQuery::ShortId("1111".to_string())).expect("resolve short id");
        assert_eq!(by_short_id.run_id, "R-20260422-11111111");
    }

    #[test]
    fn resolve_reports_empty_history_missing_runs_and_ambiguous_short_ids() {
        let workspace = tempdir().expect("temp workspace");
        let layout = ProjectLayout::new(workspace.path());

        assert!(matches!(resolve(&layout, &LookupQuery::Last), Err(LookupError::EmptyHistory)));

        let first_uuid = Uuid::parse_str("abcd1234-0000-7000-8000-000000000001").expect("uuid");
        let second_uuid = Uuid::parse_str("abcd5678-0000-7000-8000-000000000002").expect("uuid");

        write_manifest(
            &layout.new_run_dir("R-20260422-abcd1234", None),
            &sample_manifest(
                "R-20260422-abcd1234",
                Some(first_uuid),
                OffsetDateTime::from_unix_timestamp(1_700_000_040).expect("timestamp"),
            ),
        );
        write_manifest(
            &layout.new_run_dir("R-20260422-abcd5678", None),
            &sample_manifest(
                "R-20260422-abcd5678",
                Some(second_uuid),
                OffsetDateTime::from_unix_timestamp(1_700_000_050).expect("timestamp"),
            ),
        );

        match resolve(&layout, &LookupQuery::FullRunId("R-20260422-deadbeef".to_string())) {
            Err(LookupError::NotFound { query }) => assert_eq!(query, "R-20260422-deadbeef"),
            other => panic!("expected not found error, got {other:?}"),
        }

        match resolve(&layout, &LookupQuery::ShortId("abcd".to_string())) {
            Err(LookupError::Ambiguous { query, matches }) => {
                assert_eq!(query, "abcd");
                assert_eq!(matches, vec!["R-20260422-abcd1234", "R-20260422-abcd5678"]);
            }
            other => panic!("expected ambiguous error, got {other:?}"),
        }
    }
}
