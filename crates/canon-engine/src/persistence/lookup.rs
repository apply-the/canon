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
