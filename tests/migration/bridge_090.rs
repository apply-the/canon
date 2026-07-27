//! Transactional bridge tests derived from the immutable Canon 0.72.6 state schema.

use std::{
    collections::BTreeMap,
    error::Error,
    fs::{self, OpenOptions},
    path::Path,
    process::Command,
    sync::Arc,
    thread,
};

use canon_engine::policy::migration::{
    MigrationBoundary, MigrationControl, MigrationReasonCode, MigrationSource, MigrationStatus,
    apply_migration, apply_migration_with_control, inspect_legacy_state, plan_migration,
    recover_migration,
};
use tempfile::TempDir;

type TestResult = Result<(), Box<dyn Error>>;

const FIXTURE: &str = "tests/fixtures/migration/canon-0.72.6/state";
const FIXTURE_PROVENANCE: &str = "tests/fixtures/migration/canon-0.72.6/PROVENANCE.toml";
const SOURCE_VERSION: &str = "0.72.6";
const CHILD_SOURCE_ENV: &str = "CANON_M1D_CHILD_SOURCE";
const CHILD_BOUNDARY_ENV: &str = "CANON_M1D_CHILD_BOUNDARY";
const FORCED_EXIT_CODE: i32 = 86;
const MIGRATIONS_DIRECTORY: &str = ".canon-migrations";

const IMPLEMENTATION_RUN: &str = "22222222-2222-7222-8222-222222222222";
const REQUIREMENTS_RUN: &str = "11111111-1111-7111-8111-111111111111";

struct Fixture {
    root: TempDir,
}

impl Fixture {
    fn new() -> Result<Self, Box<dyn Error>> {
        let root = tempfile::tempdir()?;
        copy_tree(Path::new(FIXTURE), root.path())?;
        Ok(Self { root })
    }

    fn source(&self) -> MigrationSource {
        MigrationSource::new(self.root.path().join(".canon"), SOURCE_VERSION)
    }

    fn run(&self, id: &str) -> std::path::PathBuf {
        self.root.path().join(".canon/runs").join(id)
    }
}

fn copy_tree(source: &Path, target: &Path) -> std::io::Result<()> {
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let destination = target.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            fs::create_dir_all(&destination)?;
            copy_tree(&entry.path(), &destination)?;
        } else {
            fs::copy(entry.path(), destination)?;
        }
    }
    Ok(())
}

fn snapshot(root: &Path) -> std::io::Result<BTreeMap<String, Vec<u8>>> {
    fn walk(
        base: &Path,
        current: &Path,
        output: &mut BTreeMap<String, Vec<u8>>,
    ) -> std::io::Result<()> {
        for entry in fs::read_dir(current)? {
            let entry = entry?;
            let path = entry.path();
            if entry.file_type()?.is_dir() {
                walk(base, &path, output)?;
            } else {
                let relative =
                    path.strip_prefix(base).map_err(std::io::Error::other)?.to_string_lossy();
                output.insert(relative.replace('\\', "/"), fs::read(path)?);
            }
        }
        Ok(())
    }
    let mut output = BTreeMap::new();
    walk(root, root, &mut output)?;
    Ok(output)
}

fn ensure(condition: bool, message: impl Into<String>) -> TestResult {
    if condition { Ok(()) } else { Err(std::io::Error::other(message.into()).into()) }
}

fn plan(
    source: &MigrationSource,
) -> Result<canon_engine::policy::migration::BridgeMigrationPlan, Box<dyn Error>> {
    Ok(plan_migration(&inspect_legacy_state(source)?)?)
}

fn boundary_from_label(label: &str) -> Option<MigrationBoundary> {
    MigrationBoundary::all().iter().copied().find(|boundary| boundary.as_str() == label)
}

#[test]
fn migration_reason_codes_have_frozen_display_and_wire_values() -> TestResult {
    let cases = [
        (MigrationReasonCode::UnsupportedSourceVersion, "unsupported_source_version"),
        (MigrationReasonCode::UnsupportedFutureSchema, "unsupported_future_schema"),
        (MigrationReasonCode::CorruptLegacyState, "corrupt_legacy_state"),
        (MigrationReasonCode::MixedLegacyState, "mixed_legacy_state"),
        (MigrationReasonCode::SourceChanged, "source_changed"),
        (MigrationReasonCode::MigrationInProgress, "migration_in_progress"),
        (MigrationReasonCode::BackupFailed, "backup_failed"),
        (MigrationReasonCode::BackupVerificationFailed, "backup_verification_failed"),
        (MigrationReasonCode::StagingFailed, "staging_failed"),
        (MigrationReasonCode::StagedVerificationFailed, "staged_verification_failed"),
        (MigrationReasonCode::ReplacementFailed, "replacement_failed"),
        (MigrationReasonCode::RecoveryRequired, "recovery_required"),
        (MigrationReasonCode::ArchiveRequired, "archive_required"),
        (MigrationReasonCode::ArchiveVerificationFailed, "archive_verification_failed"),
        (MigrationReasonCode::InsufficientSpace, "insufficient_space"),
        (MigrationReasonCode::PermissionDenied, "permission_denied"),
        (MigrationReasonCode::UnsafePath, "unsafe_path"),
        (MigrationReasonCode::UnsupportedFilesystem, "unsupported_filesystem"),
        (MigrationReasonCode::IdempotencyConflict, "idempotency_conflict"),
    ];
    for (reason, expected) in cases {
        ensure(reason.to_string() == expected, format!("{reason:?} display value drifted"))?;
        ensure(
            serde_json::to_string(&reason)? == format!("\"{expected}\""),
            format!("{reason:?} wire value drifted"),
        )?;
    }
    Ok(())
}

#[test]
fn inspection_plan_and_declared_source_identity_are_revalidated() -> TestResult {
    let fixture = Fixture::new()?;
    let source = fixture.source();
    let mut inspection = inspect_legacy_state(&source)?;
    inspection.source_schema_version = "future-schema".to_string();
    ensure(
        plan_migration(&inspection).err().map(|error| error.reason_code().as_str())
            == Some("unsupported_source_version"),
        "an inspection from another schema was accepted",
    )?;

    let mut migration_plan = plan(&source)?;
    migration_plan.implementation_version = "different-implementation".to_string();
    ensure(
        apply_migration(&migration_plan).err().map(|error| error.reason_code().as_str())
            == Some("idempotency_conflict"),
        "a plan from another implementation was accepted",
    )?;

    let mismatched = MigrationSource::new(source.root, "0.72.5");
    ensure(
        inspect_legacy_state(&mismatched).err().map(|error| error.reason_code().as_str())
            == Some("unsupported_source_version"),
        "a caller-declared version outside the frozen bridge was accepted",
    )
}

#[test]
fn forced_termination_child() -> TestResult {
    let Some(root) = std::env::var_os(CHILD_SOURCE_ENV) else {
        return Ok(());
    };
    let label = std::env::var(CHILD_BOUNDARY_ENV)?;
    let boundary = boundary_from_label(&label)
        .ok_or_else(|| std::io::Error::other("unknown forced-termination boundary"))?;
    let source = MigrationSource::new(root, SOURCE_VERSION);
    let migration_plan = plan(&source)?;
    let _ = apply_migration_with_control(&migration_plan, MigrationControl::stop_after(boundary))?;
    if boundary != MigrationBoundary::BeforeBackup {
        let lock = source
            .root
            .parent()
            .ok_or_else(|| std::io::Error::other("missing source parent"))?
            .join(MIGRATIONS_DIRECTORY)
            .join(&migration_plan.migration_id)
            .join("owner.lock");
        let stale_owner =
            OpenOptions::new().read(true).write(true).create(true).truncate(false).open(lock)?;
        stale_owner.try_lock().map_err(std::io::Error::from)?;
    }
    std::process::exit(FORCED_EXIT_CODE);
}

#[test]
fn forced_process_termination_at_every_durable_boundary_recovers() -> TestResult {
    let executable = std::env::current_exe()?;
    for boundary in MigrationBoundary::all() {
        let fixture = Fixture::new()?;
        let source = fixture.source();
        let status = Command::new(&executable)
            .arg("--exact")
            .arg("bridge_090::forced_termination_child")
            .arg("--nocapture")
            .env(CHILD_SOURCE_ENV, &source.root)
            .env(CHILD_BOUNDARY_ENV, boundary.as_str())
            .status()?;
        ensure(
            status.code() == Some(FORCED_EXIT_CODE),
            format!("{} child did not terminate at the injected boundary", boundary.as_str()),
        )?;
        let recovered = if *boundary == MigrationBoundary::BeforeBackup {
            let migration_root = fixture.root.path().join(".canon-migrations");
            ensure(
                !migration_root.exists() || fs::read_dir(&migration_root)?.next().is_none(),
                "pre-backup termination left recovery state",
            )?;
            apply_migration(&plan(&source)?)?
        } else {
            recover_migration(&source)?
        };
        ensure(
            matches!(
                recovered.status,
                MigrationStatus::Complete | MigrationStatus::AlreadyMigrated
            ),
            format!("{} did not recover after process termination", boundary.as_str()),
        )?;
    }
    Ok(())
}

#[test]
fn inspect_and_plan_are_deterministic_and_non_mutating() -> TestResult {
    let fixture = Fixture::new()?;
    let before = snapshot(fixture.root.path())?;
    let first = inspect_legacy_state(&fixture.source())?;
    let second = inspect_legacy_state(&fixture.source())?;
    ensure(first == second, "inspection changed across reads")?;
    ensure(plan_migration(&first)? == plan_migration(&second)?, "plan changed across reads")?;
    ensure(before == snapshot(fixture.root.path())?, "inspection mutated Canon state")?;
    ensure(!fixture.root.path().join(".canon-migrations").exists(), "inspect created journal state")
}

#[test]
fn historical_fixture_matches_its_immutable_provenance_digest() -> TestResult {
    let fixture = Fixture::new()?;
    let provenance = fs::read_to_string(FIXTURE_PROVENANCE)?.parse::<toml::Table>()?;
    let expected = provenance
        .get("fixture_digest")
        .and_then(toml::Value::as_str)
        .ok_or_else(|| std::io::Error::other("fixture provenance digest missing"))?;
    let inspection = inspect_legacy_state(&fixture.source())?;
    ensure(
        inspection.source_digest == expected,
        "Canon historical fixture drifted from its provenance record",
    )
}

#[test]
fn supported_terminal_records_preserve_authority_evidence_as_historical() -> TestResult {
    let fixture = Fixture::new()?;
    let outcome = apply_migration(&plan(&fixture.source())?)?;
    ensure(outcome.status == MigrationStatus::Complete, "migration did not complete")?;
    ensure(outcome.report.converted_item_count >= 1, "requirements run was not converted")?;
    ensure(
        outcome
            .backup_root
            .parent()
            .ok_or_else(|| std::io::Error::other("missing migration root"))?
            .join("backup-manifest.json")
            .is_file(),
        "durable backup manifest missing",
    )?;
    ensure(outcome.report.archived_item_count >= 1, "implementation mode was not archived")?;
    ensure(
        outcome
            .report
            .semantic_losses
            .iter()
            .any(|loss| loss.code == "legacy_verification_not_fresh"),
        "label-only verification was promoted silently",
    )?;
    ensure(
        fixture.run(REQUIREMENTS_RUN).join("verification/verification-1.toml").is_file(),
        "historical verification disappeared",
    )?;
    ensure(
        !fixture.run(IMPLEMENTATION_RUN).exists(),
        "implementation mode remained in the active run registry",
    )?;
    let archive_root = fixture.root.path().join(".canon/archives").join(IMPLEMENTATION_RUN);
    ensure(
        archive_root.join("manifest.json").is_file(),
        "implementation archive manifest missing",
    )?;
    ensure(
        fs::metadata(archive_root.join("manifest.json"))?.permissions().readonly(),
        "implementation archive manifest is not read-only",
    )?;
    ensure(
        fs::metadata(&archive_root)?.permissions().readonly()
            && fs::metadata(archive_root.join("source"))?.permissions().readonly(),
        "implementation archive directories are not read-only",
    )
}

#[test]
fn rerun_is_idempotent_and_does_not_duplicate_backups_or_archives() -> TestResult {
    let fixture = Fixture::new()?;
    let migration_plan = plan(&fixture.source())?;
    let first = apply_migration(&migration_plan)?;
    let target = snapshot(&fixture.root.path().join(".canon"))?;
    let second = apply_migration(&migration_plan)?;
    ensure(first.status == MigrationStatus::Complete, "first migration failed")?;
    ensure(second.status == MigrationStatus::AlreadyMigrated, "rerun was not idempotent")?;
    ensure(target == snapshot(&fixture.root.path().join(".canon"))?, "rerun changed target state")?;
    let mut conflicting = migration_plan;
    conflicting.source_digest = "different-source-digest".to_string();
    ensure(
        apply_migration(&conflicting).err().map(|error| error.reason_code().as_str())
            == Some("idempotency_conflict"),
        "same migration identity accepted a conflicting source digest",
    )
}

#[test]
fn active_or_ambiguous_runs_are_archived_and_never_become_fresh_proof() -> TestResult {
    for state in ["Executing", "AwaitingApproval"] {
        let fixture = Fixture::new()?;
        fs::write(
            fixture.run(REQUIREMENTS_RUN).join("state.toml"),
            format!("state = \"{state}\"\nupdated_at = \"2026-04-22T08:30:00Z\"\n"),
        )?;
        let outcome = apply_migration(&plan(&fixture.source())?)?;
        ensure(
            outcome
                .report
                .unsupported_states
                .iter()
                .any(|record| record.requires_new_admitted_session),
            format!("{state} run did not require a new admitted run"),
        )?;
        ensure(!fixture.run(REQUIREMENTS_RUN).exists(), format!("{state} run remained active"))?;
    }
    Ok(())
}

#[test]
fn future_corrupt_and_mixed_schema_state_fail_closed() -> TestResult {
    for (kind, expected) in [
        ("future", "unsupported_future_schema"),
        ("corrupt", "corrupt_legacy_state"),
        ("mixed", "mixed_legacy_state"),
    ] {
        let fixture = Fixture::new()?;
        let source = fixture.source();
        match kind {
            "future" => fs::write(
                fixture.root.path().join(".canon/schema-version.json"),
                br#"{"schema_version":"9"}"#,
            )?,
            "corrupt" => fs::write(fixture.run(REQUIREMENTS_RUN).join("run.toml"), b"[[[")?,
            _ => fs::write(fixture.run(REQUIREMENTS_RUN).join("schema-version"), b"0.90")?,
        }
        let result = inspect_legacy_state(&source);
        ensure(
            result.err().map(|error| error.reason_code().as_str()) == Some(expected),
            format!("{kind} state was not rejected with {expected}"),
        )?;
        ensure(
            !fixture.root.path().join(".canon-migrations").exists(),
            format!("{kind} inspection mutated state"),
        )?;
    }
    Ok(())
}

#[test]
fn source_change_and_tampered_backup_cannot_be_committed() -> TestResult {
    let fixture = Fixture::new()?;
    let source = fixture.source();
    let migration_plan = plan(&source)?;
    fs::write(fixture.run(REQUIREMENTS_RUN).join("external-edit"), b"changed")?;
    ensure(
        apply_migration(&migration_plan).err().map(|error| error.reason_code().as_str())
            == Some("source_changed"),
        "changed source was accepted",
    )?;

    let current = plan(&source)?;
    let interrupted = apply_migration_with_control(
        &current,
        MigrationControl::stop_after(MigrationBoundary::BackupVerified),
    )?;
    fs::write(interrupted.backup_root.join("tampered"), b"tampered")?;
    ensure(
        recover_migration(&source).err().map(|error| error.reason_code().as_str())
            == Some("backup_verification_failed"),
        "tampered backup was accepted",
    )?;

    let manifest_fixture = Fixture::new()?;
    let manifest_source = manifest_fixture.source();
    let interrupted = apply_migration_with_control(
        &plan(&manifest_source)?,
        MigrationControl::stop_after(MigrationBoundary::BackupVerified),
    )?;
    fs::write(
        interrupted
            .backup_root
            .parent()
            .ok_or_else(|| std::io::Error::other("missing migration root"))?
            .join("backup-manifest.json"),
        b"{}",
    )?;
    ensure(
        recover_migration(&manifest_source).err().map(|error| error.reason_code().as_str())
            == Some("backup_verification_failed"),
        "mismatching backup manifest was accepted",
    )?;

    let staging_fixture = Fixture::new()?;
    let staging_source = staging_fixture.source();
    let staged = apply_migration_with_control(
        &plan(&staging_source)?,
        MigrationControl::stop_after(MigrationBoundary::StagedTargetVerified),
    )?;
    let staging_root = staged
        .backup_root
        .parent()
        .ok_or_else(|| std::io::Error::other("missing migration root"))?
        .join("staging");
    fs::write(staging_root.join("tampered"), b"tampered")?;
    ensure(
        recover_migration(&staging_source).err().map(|error| error.reason_code().as_str())
            == Some("staged_verification_failed"),
        "mismatching staged identity was accepted",
    )
}

#[cfg(unix)]
#[test]
fn symlink_escape_is_rejected_without_cross_product_mutation() -> TestResult {
    use std::os::unix::fs::symlink;

    let fixture = Fixture::new()?;
    let external = fixture.root.path().join("boundline-owned");
    fs::write(&external, b"boundline")?;
    symlink(&external, fixture.root.path().join(".canon/escape"))?;
    ensure(
        inspect_legacy_state(&fixture.source()).err().map(|error| error.reason_code().as_str())
            == Some("unsafe_path"),
        "symlink escape was accepted",
    )?;
    ensure(fs::read(external)? == b"boundline", "Canon migration changed external state")
}

#[cfg(unix)]
#[test]
fn permission_and_write_failures_preserve_the_authoritative_source() -> TestResult {
    use std::os::unix::fs::PermissionsExt;

    let permission_fixture = Fixture::new()?;
    let permission_source = permission_fixture.source();
    let permission_plan = plan(&permission_source)?;
    let before = snapshot(permission_fixture.root.path())?;
    fs::set_permissions(permission_fixture.root.path(), fs::Permissions::from_mode(0o500))?;
    let result = apply_migration(&permission_plan);
    fs::set_permissions(permission_fixture.root.path(), fs::Permissions::from_mode(0o700))?;
    ensure(
        result.err().map(|error| error.reason_code().as_str()) == Some("permission_denied"),
        "insufficient permissions did not return permission_denied",
    )?;
    ensure(
        before == snapshot(permission_fixture.root.path())?,
        "permission failure changed source",
    )?;

    let write_fixture = Fixture::new()?;
    let write_source = write_fixture.source();
    let write_plan = plan(&write_source)?;
    let before = snapshot(write_fixture.root.path())?;
    fs::write(write_fixture.root.path().join(".canon-migrations"), b"not-a-directory")?;
    let result = apply_migration(&write_plan);
    ensure(
        result.err().map(|error| error.reason_code().as_str()) == Some("staging_failed"),
        "write failure did not fail before replacement",
    )?;
    let mut after = snapshot(write_fixture.root.path())?;
    after.remove(".canon-migrations");
    ensure(before == after, "write failure changed authoritative source")
}

#[test]
fn concurrent_migrators_cannot_commit_competing_targets() -> TestResult {
    let fixture = Fixture::new()?;
    let migration_plan = Arc::new(plan(&fixture.source())?);
    let left = {
        let plan = Arc::clone(&migration_plan);
        thread::spawn(move || apply_migration(&plan))
    };
    let right = {
        let plan = Arc::clone(&migration_plan);
        thread::spawn(move || apply_migration(&plan))
    };
    let left = left.join().map_err(|_| "first migrator panicked")?;
    let right = right.join().map_err(|_| "second migrator panicked")?;
    ensure(left.is_ok() || right.is_ok(), "neither migrator obtained ownership")?;
    ensure(
        fixture.root.path().join(".canon/schema-version.json").is_file(),
        "no verified target was committed",
    )
}

#[test]
fn every_durable_boundary_recovers_to_one_typed_outcome() -> TestResult {
    for boundary in MigrationBoundary::all() {
        let fixture = Fixture::new()?;
        let source = fixture.source();
        let interrupted =
            apply_migration_with_control(&plan(&source)?, MigrationControl::stop_after(*boundary))?;
        ensure(
            matches!(
                interrupted.status,
                MigrationStatus::SafeRetry
                    | MigrationStatus::RecoveryRequired
                    | MigrationStatus::Complete
            ),
            format!("{} produced an untyped partial state", boundary.as_str()),
        )?;
        let recovered = if interrupted.status == MigrationStatus::SafeRetry {
            apply_migration(&plan(&source)?)?
        } else {
            recover_migration(&source)?
        };
        ensure(
            matches!(
                recovered.status,
                MigrationStatus::Complete | MigrationStatus::AlreadyMigrated
            ),
            format!("{} did not recover", boundary.as_str()),
        )?;
    }
    Ok(())
}

#[test]
fn normalized_reports_are_deterministic_complete_and_portable() -> TestResult {
    let first = Fixture::new()?;
    let second = Fixture::new()?;
    let first_outcome = apply_migration(&plan(&first.source())?)?;
    let second_outcome = apply_migration(&plan(&second.source())?)?;
    ensure(
        first_outcome.report.normalized() == second_outcome.report.normalized(),
        "equivalent reports differ after volatile normalization",
    )?;
    let rendered = serde_json::to_string(&first_outcome.report)?;
    ensure(
        !rendered.contains(&first.root.path().to_string_lossy().to_string()),
        "portable report leaked an absolute path",
    )?;
    ensure(
        first_outcome.report.terminal_reason_code == "migration_complete",
        "stable terminal reason missing",
    )
}
