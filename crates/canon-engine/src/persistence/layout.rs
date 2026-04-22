use std::path::{Path, PathBuf};

use crate::domain::mode::Mode;
use crate::domain::run::is_canonical_display_id;

#[derive(Debug, Clone)]
pub struct ProjectLayout {
    pub repo_root: PathBuf,
    pub canon_root: PathBuf,
}

impl ProjectLayout {
    pub fn new(repo_root: impl AsRef<Path>) -> Self {
        let repo_root = repo_root.as_ref().to_path_buf();
        Self { canon_root: repo_root.join(".canon"), repo_root }
    }

    pub fn sessions_dir(&self) -> PathBuf {
        self.canon_root.join("sessions")
    }

    pub fn artifacts_dir(&self) -> PathBuf {
        self.canon_root.join("artifacts")
    }

    pub fn decisions_dir(&self) -> PathBuf {
        self.canon_root.join("decisions")
    }

    pub fn traces_dir(&self) -> PathBuf {
        self.canon_root.join("traces")
    }

    pub fn methods_dir(&self) -> PathBuf {
        self.canon_root.join("methods")
    }

    pub fn policies_dir(&self) -> PathBuf {
        self.canon_root.join("policies")
    }

    pub fn runs_dir(&self) -> PathBuf {
        self.canon_root.join("runs")
    }

    /// Resolve the on-disk directory for a run.
    ///
    /// For new-style display ids `R-YYYYMMDD-SHORTID` the runtime returns a
    /// dated path `runs/YYYY/MM/<run_id>/`, falling back to a sibling
    /// directory whose name starts with `<run_id>--` when a slug suffix is
    /// present on disk. For any other (legacy UUID-shaped) value, the
    /// runtime returns the flat legacy path `runs/<run_id>/`. The path
    /// returned for new-style ids is the same regardless of whether the
    /// directory currently exists, so this function remains pure for
    /// callers that are about to create the run.
    pub fn run_dir(&self, run_id: &str) -> PathBuf {
        if let Some((year, month)) = parse_dated_display_id(run_id) {
            let month_dir = self.runs_dir().join(year).join(month);
            // Slug-less canonical path
            let canonical = month_dir.join(run_id);
            if canonical.exists() {
                return canonical;
            }
            // Look for a slugged sibling: `<run_id>--<slug>`
            if let Ok(read) = std::fs::read_dir(&month_dir) {
                let prefix = format!("{run_id}--");
                for entry in read.flatten() {
                    if let Some(name) = entry.file_name().to_str()
                        && name.starts_with(&prefix)
                    {
                        return entry.path();
                    }
                }
            }
            return canonical;
        }
        // Legacy: directory keyed directly by UUID under runs/.
        self.runs_dir().join(run_id)
    }

    /// Compute the on-disk directory for a brand-new run, given its display
    /// id and an optional slug. Used by the persistence layer at run
    /// creation. For legacy callers that have no slug, [`Self::run_dir`]
    /// returns the same path.
    pub fn new_run_dir(&self, run_id: &str, slug: Option<&str>) -> PathBuf {
        if let Some((year, month)) = parse_dated_display_id(run_id) {
            let month_dir = self.runs_dir().join(year).join(month);
            let dir_name = match slug {
                Some(s) if !s.is_empty() => format!("{run_id}--{s}"),
                _ => run_id.to_string(),
            };
            month_dir.join(dir_name)
        } else {
            self.runs_dir().join(run_id)
        }
    }

    pub fn run_gates_dir(&self, run_id: &str) -> PathBuf {
        self.run_dir(run_id).join("gates")
    }

    pub fn run_approvals_dir(&self, run_id: &str) -> PathBuf {
        self.run_dir(run_id).join("approvals")
    }

    pub fn run_verification_dir(&self, run_id: &str) -> PathBuf {
        self.run_dir(run_id).join("verification")
    }

    pub fn run_invocations_dir(&self, run_id: &str) -> PathBuf {
        self.run_dir(run_id).join("invocations")
    }

    pub fn run_inputs_dir(&self, run_id: &str) -> PathBuf {
        self.run_dir(run_id).join("inputs")
    }

    pub fn run_invocation_dir(&self, run_id: &str, request_id: &str) -> PathBuf {
        self.run_invocations_dir(run_id).join(request_id)
    }

    pub fn run_evidence_path(&self, run_id: &str) -> PathBuf {
        self.run_dir(run_id).join("evidence.toml")
    }

    pub fn run_artifact_dir(&self, run_id: &str, mode: Mode) -> PathBuf {
        self.artifacts_dir().join(run_id).join(mode.as_str())
    }

    pub fn skills_dir(&self) -> PathBuf {
        self.repo_root.join(".agents").join("skills")
    }

    pub fn claude_skills_dir(&self) -> PathBuf {
        self.repo_root.join(".claude").join("skills")
    }

    pub fn claude_md_path(&self) -> PathBuf {
        self.repo_root.join("CLAUDE.md")
    }
}

/// Parse a canonical display id `R-YYYYMMDD-SHORTID` into (`YYYY`, `MM`)
/// strings. Returns `None` for legacy UUID-shaped ids or anything else.
pub fn parse_dated_display_id(value: &str) -> Option<(&str, &str)> {
    if !is_canonical_display_id(value) {
        return None;
    }
    Some((&value[2..6], &value[6..8]))
}

/// Split a run-directory name into `(display_id, optional_slug)` using the
/// **first** `--` as the separator. Slugs themselves may contain `--`.
pub fn parse_run_dir_name(name: &str) -> (&str, Option<&str>) {
    match name.split_once("--") {
        Some((display, slug)) => (display, Some(slug)),
        None => (name, None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_dated_display_id() {
        assert_eq!(parse_dated_display_id("R-20260413-6f2b8d4e"), Some(("2026", "04")));
    }

    #[test]
    fn rejects_non_canonical_ids() {
        assert!(parse_dated_display_id("not-a-run").is_none());
        assert!(parse_dated_display_id("R-2026041-6f2b8d4e").is_none());
        assert!(parse_dated_display_id("0190f4cf-3a91-7a1c-9e8b-fa9203b1f0d4").is_none());
    }

    #[test]
    fn parses_dir_name_first_double_dash_only() {
        assert_eq!(parse_run_dir_name("R-20260413-6f2b8d4e"), ("R-20260413-6f2b8d4e", None));
        assert_eq!(
            parse_run_dir_name("R-20260413-6f2b8d4e--auth-hardening"),
            ("R-20260413-6f2b8d4e", Some("auth-hardening")),
        );
        // Slug payload itself contains '--'; first split wins.
        assert_eq!(
            parse_run_dir_name("R-20260413-6f2b8d4e--foo--bar"),
            ("R-20260413-6f2b8d4e", Some("foo--bar")),
        );
    }

    #[test]
    fn new_run_dir_uses_dated_layout_for_canonical_id() {
        let layout = ProjectLayout::new("/tmp/canon-fixture");
        let p = layout.new_run_dir("R-20260413-6f2b8d4e", Some("auth-hardening"));
        assert!(p.ends_with("runs/2026/04/R-20260413-6f2b8d4e--auth-hardening"));
    }

    #[test]
    fn new_run_dir_omits_slug_when_absent() {
        let layout = ProjectLayout::new("/tmp/canon-fixture");
        let p = layout.new_run_dir("R-20260413-6f2b8d4e", None);
        assert!(p.ends_with("runs/2026/04/R-20260413-6f2b8d4e"));
    }

    #[test]
    fn new_run_dir_falls_back_to_legacy_for_non_canonical_id() {
        let layout = ProjectLayout::new("/tmp/canon-fixture");
        let p = layout.new_run_dir("0190f4cf-3a91-7a1c-9e8b-fa9203b1f0d4", None);
        assert!(p.ends_with("runs/0190f4cf-3a91-7a1c-9e8b-fa9203b1f0d4"));
    }
}
