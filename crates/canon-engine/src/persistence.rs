/// Atomic file-write helpers.
pub mod atomic;
/// Invocation persistence: persisted requests, decisions, and attempts.
pub mod invocations;
/// Workspace directory layout helpers.
pub mod layout;
/// Run ID lookup and resolution.
pub mod lookup;
/// Run manifest persistence: run, state, context, and artifact manifests.
pub mod manifests;
/// Run ID slug normalization.
pub mod slug;
/// Workspace store: the primary persistence API for Canon runs.
pub mod store;
/// Trace event persistence for adapter invocation audit logs.
pub mod traces;

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use tempfile::tempdir;
    use time::OffsetDateTime;

    use crate::domain::mode::Mode;
    use crate::domain::policy::{RiskClass, UsageZone};
    use crate::domain::run::{ClassificationProvenance, SystemContext};
    use crate::persistence::layout::ProjectLayout;
    use crate::persistence::lookup::{LookupError, LookupQuery, resolve, scan_all};
    use crate::persistence::manifests::RunManifest;

    fn sample_manifest(
        run_id: &str,
        uuid: Option<&str>,
        short_id: Option<&str>,
        created_at: OffsetDateTime,
    ) -> RunManifest {
        RunManifest {
            run_id: run_id.to_string(),
            uuid: uuid.map(str::to_string),
            short_id: short_id.map(str::to_string),
            slug: None,
            title: Some("Lookup fixture".to_string()),
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
    fn lookup_errors_render_all_public_variants() {
        let ambiguous = LookupError::Ambiguous {
            query: "abcd".to_string(),
            matches: vec!["R-20260422-abcd1234".to_string(), "R-20260422-abcd5678".to_string()],
        };
        let ambiguous_text = ambiguous.to_string();
        assert!(ambiguous_text.contains("ambiguous run reference `abcd`; matches:"));
        assert!(ambiguous_text.contains("R-20260422-abcd1234"));
        assert!(ambiguous_text.contains("R-20260422-abcd5678"));

        assert_eq!(LookupError::EmptyHistory.to_string(), "no runs exist in this repository");

        let io_error = LookupError::from(std::io::Error::other("disk failure"));
        assert_eq!(io_error.to_string(), "io error scanning runs: disk failure");
    }

    #[test]
    fn scan_all_skips_incomplete_unreadable_and_non_utf_run_entries() {
        let workspace = tempdir().expect("temp workspace");
        let layout = ProjectLayout::new(workspace.path());

        let month_dir = layout.runs_dir().join("2026").join("04");
        fs::create_dir_all(&month_dir).expect("create month dir");
        fs::write(month_dir.join("notes.txt"), "ignore").expect("write month file");

        fs::create_dir_all(layout.runs_dir().join("R-20260422-missing000"))
            .expect("create missing manifest dir");
        fs::create_dir_all(layout.runs_dir().join("R-20260422-unreadable").join("run.toml"))
            .expect("create unreadable manifest dir");

        #[cfg(unix)]
        {
            use std::ffi::OsString;
            use std::os::unix::ffi::OsStringExt;

            let invalid_name = OsString::from_vec(vec![0xff]);
            let invalid_path = layout.runs_dir().join(invalid_name);
            match fs::create_dir_all(&invalid_path) {
                Ok(()) => {}
                Err(error) if error.raw_os_error() == Some(92) => {}
                Err(error) => panic!("create non-utf directory: {error}"),
            }
        }

        assert!(scan_all(&layout).expect("scan runs").is_empty());
    }

    #[test]
    fn resolve_uses_run_id_fallback_when_manifest_uuid_is_unparseable() {
        let workspace = tempdir().expect("temp workspace");
        let layout = ProjectLayout::new(workspace.path());
        let created_at = OffsetDateTime::from_unix_timestamp(1_700_000_060).expect("timestamp");
        let run_dir = layout.runs_dir().join("019db71e-f1bb-7dc2-b535-213e556d16fe");

        write_manifest(
            &run_dir,
            &sample_manifest("deadbeef-run", Some("not-a-uuid"), None, created_at),
        );

        let handle = resolve(&layout, &LookupQuery::ShortId("dead".to_string()))
            .expect("resolve fallback short id");

        assert_eq!(handle.run_id, "deadbeef-run");
        assert_eq!(handle.uuid, "not-a-uuid");
        assert_eq!(handle.short_id, "deadbeef");
        assert!(handle.is_legacy);
    }
}
