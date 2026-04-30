use serde_json::Value;
use std::fs;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command as ProcessCommand;
use tempfile::TempDir;

fn repo_path(relative_path: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(relative_path)
}

fn read_repo_text(relative_path: &str) -> String {
    fs::read_to_string(repo_path(relative_path))
        .unwrap_or_else(|error| panic!("read {relative_path}: {error}"))
}

fn run_command(command: &mut ProcessCommand, context: &str) -> String {
    let output = command.output().unwrap_or_else(|error| panic!("{context}: {error}"));
    assert!(
        output.status.success(),
        "{context} failed: stdout=`{}` stderr=`{}`",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    String::from_utf8(output.stdout).expect("utf8 stdout")
}

#[cfg(unix)]
fn write_unix_binary(path: &Path) {
    fs::write(path, "#!/usr/bin/env sh\necho canon\n").expect("write unix binary");
    let permissions = fs::Permissions::from_mode(0o755);
    fs::set_permissions(path, permissions).expect("chmod unix binary");
}

fn create_windows_zip(dist_dir: &Path, artifact_name: &str) {
    let work_dir = TempDir::new().expect("windows work dir");
    let binary_path = work_dir.path().join("canon.exe");
    fs::write(&binary_path, "canon windows binary\n").expect("write windows binary");

    let artifact_path = dist_dir.join(artifact_name);
    let mut command = ProcessCommand::new("zip");
    command.current_dir(work_dir.path()).args([
        "-q",
        artifact_path.to_str().expect("artifact path"),
        "canon.exe",
    ]);
    run_command(&mut command, "zip windows artifact");
}

#[cfg(unix)]
fn create_release_bundle(version: &str) -> TempDir {
    let temp_dir = TempDir::new().expect("release bundle temp dir");
    let dist_dir = temp_dir.path().join("dist");
    let bin_dir = temp_dir.path().join("bin");
    fs::create_dir_all(&dist_dir).expect("dist dir");
    fs::create_dir_all(&bin_dir).expect("bin dir");

    let unix_targets = [
        ("aarch64-apple-darwin", "canon-macos-arm64"),
        ("x86_64-apple-darwin", "canon-macos-x86_64"),
        ("aarch64-unknown-linux-gnu", "canon-linux-arm64"),
        ("x86_64-unknown-linux-gnu", "canon-linux-x86_64"),
    ];

    for (target, binary_stub) in unix_targets {
        let binary_path = bin_dir.join(binary_stub);
        write_unix_binary(&binary_path);

        let mut command = ProcessCommand::new("/bin/bash");
        command.arg(repo_path("scripts/release/package-unix.sh")).args([
            "--version",
            version,
            "--target",
            target,
            "--output-dir",
            dist_dir.to_str().expect("dist dir path"),
            "--binary-path",
            binary_path.to_str().expect("binary path"),
        ]);
        let artifact_path = run_command(&mut command, "package unix artifact");
        let artifact_path = artifact_path.trim();
        fs::write(format!("{artifact_path}.version.txt"), format!("canon {version}\n"))
            .expect("write unix version evidence");
    }

    let windows_artifact = format!("canon-{version}-windows-x86_64.zip");
    create_windows_zip(&dist_dir, &windows_artifact);
    fs::write(
        dist_dir.join(format!("{windows_artifact}.version.txt")),
        format!("canon {version}\n"),
    )
    .expect("write windows version evidence");

    let release_notes = format!(
        "# Canon {version}\n\nTag: v{version}\n\n- canon-{version}-macos-arm64.tar.gz\n- canon-{version}-macos-x86_64.tar.gz\n- canon-{version}-linux-arm64.tar.gz\n- canon-{version}-linux-x86_64.tar.gz\n- canon-{version}-windows-x86_64.zip\n"
    );
    fs::write(dist_dir.join("release-notes.md"), release_notes).expect("write release notes");

    let mut command = ProcessCommand::new("/bin/bash");
    command.arg(repo_path("scripts/release/verify-release-surface.sh")).args([
        "--version",
        version,
        "--dist-dir",
        dist_dir.to_str().expect("dist dir path"),
        "--release-notes",
        dist_dir.join("release-notes.md").to_str().expect("release notes path"),
        "--write-checksums",
    ]);
    run_command(&mut command, "verify release surface");

    temp_dir
}

#[cfg(unix)]
fn write_distribution_metadata(dist_dir: &Path, version: &str) -> PathBuf {
    let metadata_path = dist_dir.join(format!("canon-{version}-distribution-metadata.json"));

    let mut command = ProcessCommand::new("/bin/bash");
    command.arg(repo_path("scripts/release/write-distribution-metadata.sh")).args([
        "--version",
        version,
        "--dist-dir",
        dist_dir.to_str().expect("dist dir path"),
        "--output",
        metadata_path.to_str().expect("metadata path"),
    ]);
    run_command(&mut command, "write distribution metadata");

    metadata_path
}

#[cfg(unix)]
#[test]
fn distribution_metadata_describes_release_assets_for_homebrew_and_future_channels() {
    let version = "0.26.0";
    let bundle = create_release_bundle(version);
    let dist_dir = bundle.path().join("dist");
    let metadata_path = write_distribution_metadata(&dist_dir, version);

    let metadata: Value =
        serde_json::from_slice(&fs::read(&metadata_path).expect("read distribution metadata"))
            .expect("parse distribution metadata");

    assert_eq!(metadata["version"], version);
    assert_eq!(metadata["tag"], format!("v{version}"));

    let assets = metadata["assets"].as_array().expect("assets array");
    assert_eq!(assets.len(), 5, "metadata should enumerate all release assets");

    let homebrew_assets = assets
        .iter()
        .filter(|asset| {
            asset["channels"]
                .as_array()
                .expect("channels array")
                .iter()
                .any(|channel| channel == "homebrew")
        })
        .count();

    assert_eq!(homebrew_assets, 4, "Homebrew should consume the four macOS/Linux tarballs only");
    assert!(
        assets.iter().any(|asset| asset["os"] == "windows"),
        "metadata must preserve Windows assets for future channels"
    );
}

#[cfg(unix)]
#[test]
fn homebrew_formula_renders_platform_specific_release_assets() {
    let version = "0.26.0";
    let bundle = create_release_bundle(version);
    let dist_dir = bundle.path().join("dist");
    let metadata_path = write_distribution_metadata(&dist_dir, version);
    let formula_path = dist_dir.join(format!("canon-{version}-homebrew-formula.rb"));

    let mut command = ProcessCommand::new("/bin/bash");
    command.arg(repo_path("scripts/release/render-homebrew-formula.sh")).args([
        "--metadata",
        metadata_path.to_str().expect("metadata path"),
        "--output",
        formula_path.to_str().expect("formula path"),
    ]);
    run_command(&mut command, "render homebrew formula");

    let formula = fs::read_to_string(&formula_path).expect("read formula");
    assert!(formula.contains("class Canon < Formula"));
    assert!(formula.contains("on_macos do"));
    assert!(formula.contains("on_linux do"));
    assert!(formula.contains("bin.install \"canon\""));
    assert!(formula.contains("system bin/\"canon\", \"init\", \"--output\", \"json\""));

    for artifact in [
        "canon-0.26.0-macos-arm64.tar.gz",
        "canon-0.26.0-macos-x86_64.tar.gz",
        "canon-0.26.0-linux-arm64.tar.gz",
        "canon-0.26.0-linux-x86_64.tar.gz",
    ] {
        assert!(formula.contains(artifact), "formula should reference {artifact}: {formula}");
    }

    assert!(
        !formula.contains("windows-x86_64.zip"),
        "formula must not reference the Windows asset"
    );
}

#[cfg(unix)]
#[test]
fn release_surface_verifies_distribution_metadata_and_homebrew_formula() {
    let version = "0.26.0";
    let bundle = create_release_bundle(version);
    let dist_dir = bundle.path().join("dist");
    let metadata_path = write_distribution_metadata(&dist_dir, version);
    let formula_path = dist_dir.join(format!("canon-{version}-homebrew-formula.rb"));

    let mut render = ProcessCommand::new("/bin/bash");
    render.arg(repo_path("scripts/release/render-homebrew-formula.sh")).args([
        "--metadata",
        metadata_path.to_str().expect("metadata path"),
        "--output",
        formula_path.to_str().expect("formula path"),
    ]);
    run_command(&mut render, "render homebrew formula");

    let mut verify = ProcessCommand::new("/bin/bash");
    verify.arg(repo_path("scripts/release/verify-release-surface.sh")).args([
        "--version",
        version,
        "--dist-dir",
        dist_dir.to_str().expect("dist dir path"),
        "--release-notes",
        dist_dir.join("release-notes.md").to_str().expect("release notes path"),
        "--distribution-metadata",
        metadata_path.to_str().expect("metadata path"),
        "--homebrew-formula",
        formula_path.to_str().expect("formula path"),
    ]);
    run_command(&mut verify, "verify release distribution surface");
}

#[cfg(unix)]
#[test]
fn sync_homebrew_tap_updates_formula_destination_when_tap_root_is_provided() {
    let version = "0.26.0";
    let bundle = create_release_bundle(version);
    let dist_dir = bundle.path().join("dist");
    let metadata_path = write_distribution_metadata(&dist_dir, version);
    let formula_path = dist_dir.join(format!("canon-{version}-homebrew-formula.rb"));
    let tap_root = TempDir::new().expect("tap root temp dir");

    let mut render = ProcessCommand::new("/bin/bash");
    render.arg(repo_path("scripts/release/render-homebrew-formula.sh")).args([
        "--metadata",
        metadata_path.to_str().expect("metadata path"),
        "--output",
        formula_path.to_str().expect("formula path"),
    ]);
    run_command(&mut render, "render homebrew formula");

    let mut sync = ProcessCommand::new("/bin/bash");
    sync.arg(repo_path("scripts/release/sync-homebrew-tap.sh")).args([
        "--formula",
        formula_path.to_str().expect("formula path"),
        "--tap-root",
        tap_root.path().to_str().expect("tap root path"),
    ]);
    let stdout = run_command(&mut sync, "sync homebrew tap");

    assert!(stdout.contains("STATUS=updated"), "unexpected sync status: {stdout}");

    let synced_formula = fs::read_to_string(tap_root.path().join("Formula").join("canon.rb"))
        .expect("read synced formula");
    let original_formula = fs::read_to_string(&formula_path).expect("read original formula");
    assert_eq!(synced_formula, original_formula);
}

#[cfg(unix)]
#[test]
fn sync_homebrew_tap_reports_artifact_only_when_no_tap_root_is_provided() {
    let version = "0.26.0";
    let bundle = create_release_bundle(version);
    let dist_dir = bundle.path().join("dist");
    let metadata_path = write_distribution_metadata(&dist_dir, version);
    let formula_path = dist_dir.join(format!("canon-{version}-homebrew-formula.rb"));

    let mut render = ProcessCommand::new("/bin/bash");
    render.arg(repo_path("scripts/release/render-homebrew-formula.sh")).args([
        "--metadata",
        metadata_path.to_str().expect("metadata path"),
        "--output",
        formula_path.to_str().expect("formula path"),
    ]);
    run_command(&mut render, "render homebrew formula");

    let mut sync = ProcessCommand::new("/bin/bash");
    sync.arg(repo_path("scripts/release/sync-homebrew-tap.sh"))
        .args(["--formula", formula_path.to_str().expect("formula path")]);
    let stdout = run_command(&mut sync, "sync homebrew tap artifact-only");

    assert!(stdout.contains("STATUS=artifact-only"), "unexpected artifact-only status: {stdout}");
    assert!(
        stdout.contains(formula_path.to_str().expect("formula path")),
        "artifact-only mode should report the rendered formula path: {stdout}"
    );
}

#[test]
fn release_workflow_wires_distribution_artifacts_and_optional_tap_sync() {
    let workflow = read_repo_text(".github/workflows/release.yml");

    for snippet in [
        "scripts/release/write-distribution-metadata.sh",
        "scripts/release/render-homebrew-formula.sh",
        "scripts/release/sync-homebrew-tap.sh",
        "canon-${{ needs.prepare-release-metadata.outputs.version }}-distribution-metadata.json",
        "canon-${{ needs.prepare-release-metadata.outputs.version }}-homebrew-formula.rb",
        "HOMEBREW_TAP_REPOSITORY",
        "HOMEBREW_TAP_GITHUB_TOKEN",
    ] {
        assert!(workflow.contains(snippet), "release workflow missing {snippet}");
    }
}

#[test]
fn release_docs_describe_the_025_distribution_surface() {
    let release_notes = read_repo_text(".github/release-notes-template.md");
    for snippet in [
        "Homebrew",
        "canon-{{VERSION}}-distribution-metadata.json",
        "canon-{{VERSION}}-homebrew-formula.rb",
    ] {
        assert!(release_notes.contains(snippet), "release notes template missing {snippet}");
    }
    let readme = read_repo_text("README.md");
    assert!(
        readme.contains("brew tap apply-the/canon")
            && readme.contains("brew install canon")
            && readme.contains("Download the latest release"),
        "README must describe Homebrew plus direct-download fallback"
    );

    let changelog = read_repo_text("CHANGELOG.md");
    assert!(changelog.contains("## [0.26.0]"), "CHANGELOG missing 0.26.0 entry");
    assert!(
        changelog.contains("Distribution Channels Beyond GitHub Releases")
            && changelog.contains("Homebrew"),
        "CHANGELOG must describe feature 025"
    );

    let roadmap = read_repo_text("ROADMAP.md");
    assert!(
        roadmap.contains("Distribution Channels Beyond GitHub Releases")
            && roadmap.contains("Homebrew")
            && roadmap.contains("winget")
            && !roadmap.contains("## Feature: Protocol Interoperability"),
        "ROADMAP must reflect delivered Homebrew support and the current Windows distribution baseline"
    );
}
