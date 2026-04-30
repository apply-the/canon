use std::fs;
use std::path::{Path, PathBuf};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
#[cfg(unix)]
use std::process::Command as ProcessCommand;

use serde_json::Value;

#[cfg(unix)]
use tempfile::TempDir;

fn repo_path(path: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(path)
}

fn read_repo_text(path: &str) -> String {
    fs::read_to_string(repo_path(path)).unwrap_or_else(|err| panic!("read {path}: {err}"))
}

#[cfg(unix)]
fn run_command(current_dir: &Path, program: &str, args: &[&str]) {
    let output = ProcessCommand::new(program)
        .args(args)
        .current_dir(current_dir)
        .output()
        .unwrap_or_else(|err| panic!("run {program} {:?}: {err}", args));

    assert!(
        output.status.success(),
        "command failed: {program} {:?}\nstdout:\n{}\nstderr:\n{}",
        args,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[cfg(unix)]
fn create_unix_archive(dist_dir: &Path, version: &str, label: &str) {
    let work_dir = dist_dir.join(format!("work-{label}"));
    fs::create_dir_all(&work_dir).expect("create unix work dir");

    let binary = work_dir.join("canon");
    fs::write(&binary, format!("canon {label} {version}\n")).expect("write unix binary");
    fs::set_permissions(&binary, fs::Permissions::from_mode(0o755)).expect("chmod unix binary");

    let archive = dist_dir.join(format!("canon-{version}-{label}.tar.gz"));
    run_command(
        dist_dir,
        "tar",
        &[
            "-czf",
            archive.to_str().expect("archive path"),
            "-C",
            work_dir.to_str().expect("work dir"),
            "canon",
        ],
    );

    fs::write(
        dist_dir.join(format!("canon-{version}-{label}.tar.gz.version.txt")),
        format!("canon {version}\n"),
    )
    .expect("write unix version evidence");
}

#[cfg(unix)]
fn create_windows_zip(dist_dir: &Path, version: &str) {
    let work_dir = dist_dir.join("work-windows-x86_64");
    fs::create_dir_all(&work_dir).expect("create windows work dir");

    let binary = work_dir.join("canon.exe");
    fs::write(&binary, format!("canon windows {version}\r\n")).expect("write windows binary");

    let archive = dist_dir.join(format!("canon-{version}-windows-x86_64.zip"));
    run_command(
        dist_dir,
        "zip",
        &[
            "-jq",
            archive.to_str().expect("windows archive path"),
            binary.to_str().expect("windows binary path"),
        ],
    );

    fs::write(
        dist_dir.join(format!("canon-{version}-windows-x86_64.zip.version.txt")),
        format!("canon {version}\n"),
    )
    .expect("write windows version evidence");
}

#[cfg(unix)]
fn write_release_notes(dist_dir: &Path, version: &str) {
    let notes = format!(
        "# Canon {version}\n\n## Assets\n\n- canon-{version}-macos-arm64.tar.gz\n- canon-{version}-macos-x86_64.tar.gz\n- canon-{version}-linux-arm64.tar.gz\n- canon-{version}-linux-x86_64.tar.gz\n- canon-{version}-windows-x86_64.zip\n"
    );
    fs::write(dist_dir.join("release-notes.md"), notes).expect("write release notes");
}

#[cfg(unix)]
fn write_checksums(dist_dir: &Path, version: &str) {
    let output = ProcessCommand::new("shasum")
        .args([
            "-a",
            "256",
            &format!("canon-{version}-macos-arm64.tar.gz"),
            &format!("canon-{version}-macos-x86_64.tar.gz"),
            &format!("canon-{version}-linux-arm64.tar.gz"),
            &format!("canon-{version}-linux-x86_64.tar.gz"),
            &format!("canon-{version}-windows-x86_64.zip"),
        ])
        .current_dir(dist_dir)
        .output()
        .expect("run shasum");

    assert!(output.status.success(), "shasum failed");
    fs::write(
        dist_dir.join(format!("canon-{version}-SHA256SUMS.txt")),
        String::from_utf8(output.stdout).expect("checksum output utf8"),
    )
    .expect("write checksum manifest");
}

#[cfg(unix)]
fn create_release_bundle(dist_dir: &Path, version: &str) {
    fs::create_dir_all(dist_dir).expect("create dist dir");

    for label in ["macos-arm64", "macos-x86_64", "linux-arm64", "linux-x86_64"] {
        create_unix_archive(dist_dir, version, label);
    }

    create_windows_zip(dist_dir, version);
    write_release_notes(dist_dir, version);
    write_checksums(dist_dir, version);
}

#[cfg(unix)]
fn write_distribution_metadata(dist_dir: &Path, version: &str, output: &Path) {
    let script = repo_path("scripts/release/write-distribution-metadata.sh");
    run_command(
        &repo_path("."),
        "bash",
        &[
            script.to_str().expect("metadata script"),
            "--version",
            version,
            "--dist-dir",
            dist_dir.to_str().expect("dist dir"),
            "--output",
            output.to_str().expect("metadata output"),
        ],
    );
}

#[cfg(unix)]
#[test]
fn distribution_metadata_marks_windows_asset_for_winget() {
    let temp_dir = TempDir::new().expect("temp dir");
    let dist_dir = temp_dir.path().join("dist");
    let version = "0.26.0";
    create_release_bundle(&dist_dir, version);

    let metadata_path = temp_dir.path().join(format!("canon-{version}-distribution-metadata.json"));
    write_distribution_metadata(&dist_dir, version, &metadata_path);

    let metadata: Value = serde_json::from_str(
        &fs::read_to_string(&metadata_path).expect("read distribution metadata"),
    )
    .expect("parse distribution metadata");

    let windows_asset = metadata["assets"]
        .as_array()
        .expect("assets array")
        .iter()
        .find(|asset| asset["asset_id"] == "windows-x86_64")
        .expect("windows asset entry");

    assert_eq!(windows_asset["channels"], serde_json::json!(["winget"]));
}

#[cfg(unix)]
#[test]
fn render_winget_manifests_builds_multi_file_bundle_from_release_metadata() {
    let temp_dir = TempDir::new().expect("temp dir");
    let dist_dir = temp_dir.path().join("dist");
    let version = "0.26.0";
    create_release_bundle(&dist_dir, version);

    let metadata_path = temp_dir.path().join(format!("canon-{version}-distribution-metadata.json"));
    write_distribution_metadata(&dist_dir, version, &metadata_path);

    let output_dir = temp_dir.path().join("winget-manifests");
    let script = repo_path("scripts/release/render-winget-manifests.sh");
    run_command(
        &repo_path("."),
        "bash",
        &[
            script.to_str().expect("winget renderer script"),
            "--metadata",
            metadata_path.to_str().expect("metadata path"),
            "--output-dir",
            output_dir.to_str().expect("winget output dir"),
        ],
    );

    let version_manifest =
        fs::read_to_string(output_dir.join("ApplyThe.Canon.yaml")).expect("read version manifest");
    let locale_manifest = fs::read_to_string(output_dir.join("ApplyThe.Canon.locale.en-US.yaml"))
        .expect("read locale manifest");
    let installer_manifest = fs::read_to_string(output_dir.join("ApplyThe.Canon.installer.yaml"))
        .expect("read installer manifest");

    assert!(version_manifest.contains("PackageIdentifier: ApplyThe.Canon"));
    assert!(version_manifest.contains("ManifestType: version"));

    assert!(locale_manifest.contains("PackageName: Canon"));
    assert!(locale_manifest.contains("ManifestType: defaultLocale"));

    assert!(installer_manifest.contains("InstallerType: zip"));
    assert!(installer_manifest.contains("NestedInstallerType: portable"));
    assert!(installer_manifest.contains("RelativeFilePath: canon.exe"));
    assert!(installer_manifest.contains("PortableCommandAlias: canon"));
    assert!(installer_manifest.contains(&format!(
        "InstallerUrl: https://github.com/apply-the/canon/releases/download/v{version}/canon-{version}-windows-x86_64.zip"
    )));
}

#[cfg(unix)]
#[test]
fn release_surface_verifies_winget_bundle() {
    let temp_dir = TempDir::new().expect("temp dir");
    let dist_dir = temp_dir.path().join("dist");
    let version = "0.26.0";
    create_release_bundle(&dist_dir, version);

    let metadata_path = temp_dir.path().join(format!("canon-{version}-distribution-metadata.json"));
    write_distribution_metadata(&dist_dir, version, &metadata_path);

    let winget_dir = temp_dir.path().join("winget-manifests");
    let render_script = repo_path("scripts/release/render-winget-manifests.sh");
    run_command(
        &repo_path("."),
        "bash",
        &[
            render_script.to_str().expect("winget renderer script"),
            "--metadata",
            metadata_path.to_str().expect("metadata path"),
            "--output-dir",
            winget_dir.to_str().expect("winget dir"),
        ],
    );

    let verify_script = repo_path("scripts/release/verify-release-surface.sh");
    run_command(
        &repo_path("."),
        "bash",
        &[
            verify_script.to_str().expect("verify script"),
            "--version",
            version,
            "--dist-dir",
            dist_dir.to_str().expect("dist dir"),
            "--release-notes",
            dist_dir.join("release-notes.md").to_str().expect("release notes"),
            "--distribution-metadata",
            metadata_path.to_str().expect("metadata path"),
            "--winget-manifest-dir",
            winget_dir.to_str().expect("winget dir"),
        ],
    );
}

#[test]
fn release_workflow_and_docs_describe_the_winget_surface_and_remove_protocol_interoperability() {
    let workflow = read_repo_text(".github/workflows/release.yml");
    assert!(
        workflow.contains("render-winget-manifests.sh")
            && workflow.contains("winget")
            && workflow.contains("distribution-metadata"),
        "release workflow must wire winget manifest generation and publication"
    );

    let readme = read_repo_text("README.md");
    assert!(
        readme.contains("winget install") && readme.contains("ApplyThe.Canon"),
        "README must describe the Windows winget install path"
    );
    assert!(
        readme.contains("windows-x86_64.zip"),
        "README must keep the Windows archive fallback visible"
    );

    let release_notes = read_repo_text(".github/release-notes-template.md");
    assert!(
        release_notes.contains("winget") && release_notes.contains("ApplyThe.Canon"),
        "release notes template must mention the winget publication artifact"
    );

    let changelog = read_repo_text("CHANGELOG.md");
    assert!(changelog.contains("## [0.26.0]"), "CHANGELOG missing 0.26.0 entry");
    assert!(changelog.contains("winget"), "CHANGELOG must describe the winget slice");

    let roadmap = read_repo_text("ROADMAP.md");
    assert!(
        !roadmap.contains("## Feature: Protocol Interoperability"),
        "ROADMAP must remove Protocol Interoperability as active next work"
    );
    assert!(
        roadmap.contains("winget") && roadmap.contains("Windows"),
        "ROADMAP must keep the Windows distribution priority visible"
    );
}
