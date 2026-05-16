//! Filesystem path helpers for the governance adapter.
//!
//! All path operations are pure and side-effect-free except
//! [`artifact_contains_missing_authored_body`], which reads a file from disk.
//! Functions in this module are shared by handlers, parsers, and the status
//! module through explicit `use super::paths::*` imports.

use super::*;

/// Resolves a potentially relative `value` against `repo_root`.
///
/// Absolute paths are returned unchanged; relative paths are joined to the
/// repository root so that all downstream I/O uses canonical absolute paths.
pub(super) fn resolve_request_path(repo_root: &Path, value: &str) -> PathBuf {
    let path = Path::new(value);
    if path.is_absolute() { path.to_path_buf() } else { repo_root.join(path) }
}

/// Returns the canonicalized form of `repo_root`, resolving symlinks.
///
/// Callers rely on canonicalization to produce a stable absolute path that can
/// be compared with artifact paths stored in `.canon/`.
pub(super) fn canonical_repo_root(repo_root: &Path) -> Result<PathBuf, std::io::Error> {
    repo_root.canonicalize()
}

/// Converts a filesystem path to a forward-slash string, dropping any
/// path prefix components (drive letters, UNC roots, etc.).
///
/// The resulting string matches the slash-separated artifact ref format used
/// throughout `.canon/` manifests and the governance response payload.
pub(super) fn path_to_slash_string(path: &Path) -> String {
    path.components()
        .filter_map(|component| match component {
            Component::Normal(segment) => Some(segment.to_string_lossy().into_owned()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("/")
}

/// Returns `true` when the artifact file at `repo_root/document_ref` still
/// contains the canonical missing-body placeholder marker.
///
/// This is used by the status module to detect authored sections that have not
/// yet been filled in, so the governance response can surface them as
/// `missing_sections` rather than falsely treating them as complete.
pub(super) fn artifact_contains_missing_authored_body(
    repo_root: &Path,
    document_ref: &str,
) -> bool {
    std::fs::read_to_string(repo_root.join(document_ref))
        .map(|contents| contents.contains(MISSING_AUTHORED_BODY_MARKER))
        .unwrap_or(false)
}

/// Extracts the file-name leaf of a slash-separated artifact reference.
///
/// Falls back to the original `reference` string when no file-name component
/// can be parsed (e.g. the reference is already a bare leaf with no slashes).
pub(super) fn packet_leaf(reference: &str) -> String {
    Path::new(reference)
        .file_name()
        .map(|value| value.to_string_lossy().into_owned())
        .unwrap_or_else(|| reference.to_string())
}

/// Returns `Some(trimmed)` when `value` is non-empty after trimming, or
/// `None` when it is absent or blank.
///
/// Used throughout request parsing to treat empty-string fields the same as
/// absent (`None`) fields in the incoming JSON request body.
pub(super) fn non_empty(value: Option<&str>) -> Option<&str> {
    value.map(str::trim).filter(|value| !value.is_empty())
}
