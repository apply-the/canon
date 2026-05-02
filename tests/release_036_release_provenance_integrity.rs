use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command as ProcessCommand;

use serde_json::{Value, json};
use tempfile::TempDir;

const VERSION: &str = "0.36.0";

#[test]
fn distribution_metadata_includes_provenance_and_channel_contracts() {
    let workspace = TempDir::new().expect("temp dir");
    let dist_dir = workspace.path().join("dist");
    fs::create_dir_all(&dist_dir).expect("dist dir");
    stage_release_bundle(&dist_dir, VERSION);

    let metadata_path = dist_dir.join("distribution-metadata.json");
    run_write_distribution_metadata(VERSION, &dist_dir, &metadata_path);

    let metadata: Value = serde_json::from_str(
        &fs::read_to_string(&metadata_path).expect("read distribution metadata"),
    )
    .expect("parse distribution metadata");

    assert_eq!(metadata["version"], VERSION);
    assert_eq!(
        metadata["source_of_truth"],
        json!({
            "kind": "github-releases",
            "artifact_inventory": "assets",
            "checksum_source": format!("canon-{VERSION}-SHA256SUMS.txt"),
            "release_notes_source": "release-notes.md"
        })
    );

    let channels = metadata["channels"].as_array().expect("channel contracts array");
    assert_eq!(channels.len(), 3, "expected three explicit channel contracts");

    assert!(
        channels.iter().any(|channel| {
            channel
                == &json!({
                    "channel": "homebrew",
                    "asset_ids": ["macos-arm64", "macos-x86_64", "linux-arm64", "linux-x86_64"],
                    "generated_artifacts": ["canon.rb"]
                })
        }),
        "missing homebrew channel contract: {channels:?}"
    );

    assert!(
        channels.iter().any(|channel| {
            channel
                == &json!({
                    "channel": "winget",
                    "asset_ids": ["windows-x86_64"],
                    "generated_artifacts": [
                        "ApplyThe.Canon.yaml",
                        "ApplyThe.Canon.locale.en-US.yaml",
                        "ApplyThe.Canon.installer.yaml"
                    ]
                })
        }),
        "missing winget channel contract: {channels:?}"
    );

    assert!(
        channels.iter().any(|channel| {
            channel
                == &json!({
                    "channel": "scoop",
                    "asset_ids": ["windows-x86_64"],
                    "generated_artifacts": ["canon.json"]
                })
        }),
        "missing scoop channel contract: {channels:?}"
    );
}

#[test]
fn homebrew_renderer_rejects_missing_homebrew_channel_contract() {
    let workspace = TempDir::new().expect("temp dir");
    let dist_dir = workspace.path().join("dist");
    fs::create_dir_all(&dist_dir).expect("dist dir");
    stage_release_bundle(&dist_dir, VERSION);

    let metadata_path = dist_dir.join("distribution-metadata.json");
    run_write_distribution_metadata(VERSION, &dist_dir, &metadata_path);

    let mut metadata = read_json(&metadata_path);
    metadata["channels"] = Value::Array(
        metadata["channels"]
            .as_array()
            .expect("channels array")
            .iter()
            .filter(|channel| channel["channel"] != "homebrew")
            .cloned()
            .collect(),
    );
    write_json(&metadata_path, &metadata);

    let output_path = dist_dir.join("canon.rb");
    let result = run_release_script(
        "render-homebrew-formula.sh",
        &[
            "--metadata",
            metadata_path.to_str().expect("metadata path"),
            "--output",
            output_path.to_str().expect("output path"),
        ],
    );

    assert!(
        !result.status.success(),
        "homebrew renderer unexpectedly succeeded: stdout=`{}` stderr=`{}`",
        String::from_utf8_lossy(&result.stdout),
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(
        String::from_utf8_lossy(&result.stderr)
            .contains("Missing required channel contract: homebrew"),
        "unexpected stderr: {}",
        String::from_utf8_lossy(&result.stderr)
    );
}

#[test]
fn winget_renderer_rejects_missing_generated_artifact_expectation() {
    let workspace = TempDir::new().expect("temp dir");
    let dist_dir = workspace.path().join("dist");
    fs::create_dir_all(&dist_dir).expect("dist dir");
    stage_release_bundle(&dist_dir, VERSION);

    let metadata_path = dist_dir.join("distribution-metadata.json");
    run_write_distribution_metadata(VERSION, &dist_dir, &metadata_path);

    let mut metadata = read_json(&metadata_path);
    let channels = metadata["channels"].as_array_mut().expect("channels array");
    let winget = channels
        .iter_mut()
        .find(|channel| channel["channel"] == "winget")
        .expect("winget channel contract");
    winget["generated_artifacts"] =
        json!(["ApplyThe.Canon.yaml", "ApplyThe.Canon.locale.en-US.yaml"]);
    write_json(&metadata_path, &metadata);

    let output_dir = dist_dir.join("winget");
    let result = run_release_script(
        "render-winget-manifests.sh",
        &[
            "--metadata",
            metadata_path.to_str().expect("metadata path"),
            "--output-dir",
            output_dir.to_str().expect("output dir"),
        ],
    );

    assert!(
        !result.status.success(),
        "winget renderer unexpectedly succeeded: stdout=`{}` stderr=`{}`",
        String::from_utf8_lossy(&result.stdout),
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(
        String::from_utf8_lossy(&result.stderr).contains(
            "Channel contract winget missing generated artifact: ApplyThe.Canon.installer.yaml"
        ),
        "unexpected stderr: {}",
        String::from_utf8_lossy(&result.stderr)
    );
}

#[test]
fn scoop_renderer_rejects_missing_scoop_channel_contract() {
    let workspace = TempDir::new().expect("temp dir");
    let dist_dir = workspace.path().join("dist");
    fs::create_dir_all(&dist_dir).expect("dist dir");
    stage_release_bundle(&dist_dir, VERSION);

    let metadata_path = dist_dir.join("distribution-metadata.json");
    run_write_distribution_metadata(VERSION, &dist_dir, &metadata_path);

    let mut metadata = read_json(&metadata_path);
    metadata["channels"] = Value::Array(
        metadata["channels"]
            .as_array()
            .expect("channels array")
            .iter()
            .filter(|channel| channel["channel"] != "scoop")
            .cloned()
            .collect(),
    );
    write_json(&metadata_path, &metadata);

    let output_path = dist_dir.join("canon.json");
    let result = run_release_script(
        "render-scoop-manifest.sh",
        &[
            "--metadata",
            metadata_path.to_str().expect("metadata path"),
            "--output",
            output_path.to_str().expect("output path"),
        ],
    );

    assert!(
        !result.status.success(),
        "scoop renderer unexpectedly succeeded: stdout=`{}` stderr=`{}`",
        String::from_utf8_lossy(&result.stdout),
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(
        String::from_utf8_lossy(&result.stderr)
            .contains("Missing required channel contract: scoop"),
        "unexpected stderr: {}",
        String::from_utf8_lossy(&result.stderr)
    );
}

#[test]
fn release_surface_verifier_rejects_mismatched_source_of_truth() {
    let workspace = TempDir::new().expect("temp dir");
    let dist_dir = workspace.path().join("dist");
    fs::create_dir_all(&dist_dir).expect("dist dir");
    stage_verifiable_release_bundle(&dist_dir, VERSION);

    let metadata_path = dist_dir.join("distribution-metadata.json");
    run_write_distribution_metadata(VERSION, &dist_dir, &metadata_path);

    let mut metadata = read_json(&metadata_path);
    metadata["source_of_truth"]["checksum_source"] = json!("canon-0.36.0-checksums.txt");
    write_json(&metadata_path, &metadata);

    let release_notes = dist_dir.join("release-notes.md");
    let result = run_release_script(
        "verify-release-surface.sh",
        &[
            "--version",
            VERSION,
            "--dist-dir",
            dist_dir.to_str().expect("dist dir path"),
            "--release-notes",
            release_notes.to_str().expect("release notes path"),
            "--distribution-metadata",
            metadata_path.to_str().expect("metadata path"),
        ],
    );

    assert!(
        !result.status.success(),
        "release verifier unexpectedly succeeded: stdout=`{}` stderr=`{}`",
        String::from_utf8_lossy(&result.stdout),
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(
        String::from_utf8_lossy(&result.stderr)
            .contains("Distribution metadata source_of_truth is invalid"),
        "unexpected stderr: {}",
        String::from_utf8_lossy(&result.stderr)
    );
}

#[test]
fn release_surface_verifier_rejects_channel_contract_asset_drift() {
    let workspace = TempDir::new().expect("temp dir");
    let dist_dir = workspace.path().join("dist");
    fs::create_dir_all(&dist_dir).expect("dist dir");
    stage_verifiable_release_bundle(&dist_dir, VERSION);

    let metadata_path = dist_dir.join("distribution-metadata.json");
    run_write_distribution_metadata(VERSION, &dist_dir, &metadata_path);

    let mut metadata = read_json(&metadata_path);
    let channels = metadata["channels"].as_array_mut().expect("channels array");
    let homebrew = channels
        .iter_mut()
        .find(|channel| channel["channel"] == "homebrew")
        .expect("homebrew channel contract");
    homebrew["asset_ids"] = json!(["windows-x86_64"]);
    write_json(&metadata_path, &metadata);

    let release_notes = dist_dir.join("release-notes.md");
    let result = run_release_script(
        "verify-release-surface.sh",
        &[
            "--version",
            VERSION,
            "--dist-dir",
            dist_dir.to_str().expect("dist dir path"),
            "--release-notes",
            release_notes.to_str().expect("release notes path"),
            "--distribution-metadata",
            metadata_path.to_str().expect("metadata path"),
        ],
    );

    assert!(
        !result.status.success(),
        "release verifier unexpectedly succeeded: stdout=`{}` stderr=`{}`",
        String::from_utf8_lossy(&result.stdout),
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(
        String::from_utf8_lossy(&result.stderr)
            .contains("Distribution metadata channel contract mismatch for homebrew"),
        "unexpected stderr: {}",
        String::from_utf8_lossy(&result.stderr)
    );
}

#[test]
fn canonical_release_contract_renders_and_verifies_all_channels() {
    let workspace = TempDir::new().expect("temp dir");
    let dist_dir = workspace.path().join("dist");
    fs::create_dir_all(&dist_dir).expect("dist dir");
    stage_verifiable_release_bundle(&dist_dir, VERSION);

    let metadata_path = dist_dir.join("distribution-metadata.json");
    run_write_distribution_metadata(VERSION, &dist_dir, &metadata_path);

    let homebrew_formula = dist_dir.join("canon.rb");
    let winget_dir = dist_dir.join("winget");
    let scoop_manifest = dist_dir.join("canon.json");

    assert_script_success(
        "render-homebrew-formula.sh",
        run_release_script(
            "render-homebrew-formula.sh",
            &[
                "--metadata",
                metadata_path.to_str().expect("metadata path"),
                "--output",
                homebrew_formula.to_str().expect("homebrew path"),
            ],
        ),
    );

    assert_script_success(
        "render-winget-manifests.sh",
        run_release_script(
            "render-winget-manifests.sh",
            &[
                "--metadata",
                metadata_path.to_str().expect("metadata path"),
                "--output-dir",
                winget_dir.to_str().expect("winget dir"),
            ],
        ),
    );

    assert_script_success(
        "render-scoop-manifest.sh",
        run_release_script(
            "render-scoop-manifest.sh",
            &[
                "--metadata",
                metadata_path.to_str().expect("metadata path"),
                "--output",
                scoop_manifest.to_str().expect("scoop path"),
            ],
        ),
    );

    let release_notes = dist_dir.join("release-notes.md");
    assert_script_success(
        "verify-release-surface.sh",
        run_release_script(
            "verify-release-surface.sh",
            &[
                "--version",
                VERSION,
                "--dist-dir",
                dist_dir.to_str().expect("dist dir"),
                "--release-notes",
                release_notes.to_str().expect("release notes"),
                "--distribution-metadata",
                metadata_path.to_str().expect("metadata path"),
                "--homebrew-formula",
                homebrew_formula.to_str().expect("homebrew path"),
                "--winget-manifest-dir",
                winget_dir.to_str().expect("winget dir"),
                "--scoop-manifest",
                scoop_manifest.to_str().expect("scoop path"),
            ],
        ),
    );
}

#[test]
fn release_docs_and_version_surfaces_align_on_0_36_0_provenance() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let cargo_manifest = fs::read_to_string(repo_root.join("Cargo.toml")).expect("read Cargo.toml");
    assert!(
        cargo_manifest.contains("version = \"0.36.0\""),
        "workspace manifest should be bumped to 0.36.0"
    );

    for compatibility_ref in [
        repo_root
            .join("defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml"),
        repo_root.join(".agents/skills/canon-shared/references/runtime-compatibility.toml"),
    ] {
        let content =
            fs::read_to_string(&compatibility_ref).expect("read runtime compatibility reference");
        assert!(
            content.contains("expected_workspace_version = \"0.36.0\""),
            "runtime compatibility reference should point at 0.36.0: {}",
            compatibility_ref.display()
        );
    }

    let readme = fs::read_to_string(repo_root.join("README.md")).expect("read README");
    let readme_compact = readme.split_whitespace().collect::<Vec<_>>().join(" ");
    assert!(
        readme.contains("The current delivery line in this repository targets Canon `0.36.0`."),
        "README should advertise Canon 0.36.0"
    );
    assert!(
        readme_compact.contains(
            "GitHub Releases remain the canonical source of truth for the Homebrew formula, `winget` bundle, and Scoop manifest."
        ),
        "README should describe GitHub Releases as the canonical provenance source"
    );

    let winget_guide = fs::read_to_string(repo_root.join("docs/guides/publishing-to-winget.md"))
        .expect("read winget guide");
    let winget_guide_compact = winget_guide.split_whitespace().collect::<Vec<_>>().join(" ");
    assert!(
        winget_guide_compact.contains("explicit `source_of_truth` and `channels` contracts"),
        "winget guide should describe the explicit provenance contract"
    );
    assert!(winget_guide.contains("0.36.0"), "winget guide should use 0.36.0 examples");

    let scoop_guide = fs::read_to_string(repo_root.join("docs/guides/publishing-to-scoop.md"))
        .expect("read scoop guide");
    let scoop_guide_compact = scoop_guide.split_whitespace().collect::<Vec<_>>().join(" ");
    assert!(
        scoop_guide_compact.contains("explicit `source_of_truth` and `channels` contracts"),
        "scoop guide should describe the explicit provenance contract"
    );
    assert!(scoop_guide.contains("0.36.0"), "scoop guide should use 0.36.0 examples");

    let roadmap = fs::read_to_string(repo_root.join("ROADMAP.md")).expect("read roadmap");
    let roadmap_compact = roadmap.split_whitespace().collect::<Vec<_>>().join(" ");
    assert!(
        roadmap.contains("## Delivered Feature: 036 Release Provenance And Channel Integrity"),
        "roadmap should record the delivered 036 slice"
    );
    assert!(
        roadmap_compact.contains(
            "There are no active remaining candidate feature blocks recorded immediately after the delivered 036 slice."
        ),
        "roadmap should be cleaned after 036"
    );

    let changelog = fs::read_to_string(repo_root.join("CHANGELOG.md")).expect("read changelog");
    assert!(changelog.contains("## [0.36.0]"), "changelog should record the 0.36.0 release");
    assert!(
        changelog.contains("Release Provenance And Channel Integrity"),
        "changelog should name the 036 feature"
    );
}

fn stage_release_bundle(dist_dir: &Path, version: &str) {
    let archives = expected_archives(version);

    for artifact in &archives {
        fs::write(dist_dir.join(artifact), format!("artifact: {artifact}\n"))
            .expect("write synthetic archive");
    }

    let release_notes = std::iter::once(format!("# Canon {version}"))
        .chain(archives.iter().map(|artifact| format!("- {artifact}")))
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(dist_dir.join("release-notes.md"), release_notes).expect("write release notes");

    let checksum_output = ProcessCommand::new("shasum")
        .arg("-a")
        .arg("256")
        .args(&archives)
        .current_dir(dist_dir)
        .output()
        .expect("run shasum");
    assert!(
        checksum_output.status.success(),
        "shasum failed: stdout=`{}` stderr=`{}`",
        String::from_utf8_lossy(&checksum_output.stdout),
        String::from_utf8_lossy(&checksum_output.stderr)
    );
    fs::write(dist_dir.join(format!("canon-{version}-SHA256SUMS.txt")), checksum_output.stdout)
        .expect("write checksum manifest");
}

fn stage_verifiable_release_bundle(dist_dir: &Path, version: &str) {
    fs::create_dir_all(dist_dir).expect("dist dir");

    let unix_staging_dir = dist_dir.join("unix-staging");
    fs::create_dir_all(&unix_staging_dir).expect("unix staging dir");
    fs::write(unix_staging_dir.join("canon"), format!("canon {version}\n"))
        .expect("write unix binary placeholder");

    let windows_staging_dir = dist_dir.join("windows-staging");
    fs::create_dir_all(&windows_staging_dir).expect("windows staging dir");
    fs::write(windows_staging_dir.join("canon.exe"), format!("canon.exe {version}\n"))
        .expect("write windows binary placeholder");

    for artifact in expected_archives(version) {
        let artifact_path = dist_dir.join(&artifact);

        if artifact.ends_with(".tar.gz") {
            let output = ProcessCommand::new("tar")
                .arg("-czf")
                .arg(&artifact_path)
                .arg("-C")
                .arg(&unix_staging_dir)
                .arg("canon")
                .output()
                .expect("run tar");
            assert!(
                output.status.success(),
                "tar failed for {artifact}: stdout=`{}` stderr=`{}`",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
        } else {
            let output = ProcessCommand::new("zip")
                .arg("-q")
                .arg(&artifact_path)
                .arg("canon.exe")
                .current_dir(&windows_staging_dir)
                .output()
                .expect("run zip");
            assert!(
                output.status.success(),
                "zip failed for {artifact}: stdout=`{}` stderr=`{}`",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
        }

        fs::write(dist_dir.join(format!("{artifact}.version.txt")), format!("{version}\n"))
            .expect("write version evidence");
    }

    let release_notes = expected_archives(version)
        .into_iter()
        .map(|artifact| format!("- {artifact}"))
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(dist_dir.join("release-notes.md"), format!("# Canon {version}\n\n{release_notes}\n"))
        .expect("write release notes");

    let archives = expected_archives(version);
    let checksum_output = ProcessCommand::new("shasum")
        .arg("-a")
        .arg("256")
        .args(&archives)
        .current_dir(dist_dir)
        .output()
        .expect("run shasum");
    assert!(
        checksum_output.status.success(),
        "shasum failed: stdout=`{}` stderr=`{}`",
        String::from_utf8_lossy(&checksum_output.stdout),
        String::from_utf8_lossy(&checksum_output.stderr)
    );
    fs::write(dist_dir.join(format!("canon-{version}-SHA256SUMS.txt")), checksum_output.stdout)
        .expect("write checksum manifest");
}

fn run_write_distribution_metadata(version: &str, dist_dir: &Path, output: &Path) {
    let result = run_release_script(
        "write-distribution-metadata.sh",
        &[
            "--version",
            version,
            "--dist-dir",
            dist_dir.to_str().expect("dist dir path"),
            "--output",
            output.to_str().expect("output path"),
        ],
    );

    assert!(
        result.status.success(),
        "metadata writer failed: stdout=`{}` stderr=`{}`",
        String::from_utf8_lossy(&result.stdout),
        String::from_utf8_lossy(&result.stderr)
    );
}

fn run_release_script(script_name: &str, args: &[&str]) -> std::process::Output {
    let script =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("scripts").join("release").join(script_name);

    ProcessCommand::new("/bin/bash").arg(&script).args(args).output().expect("run release script")
}

fn assert_script_success(script_name: &str, output: std::process::Output) {
    assert!(
        output.status.success(),
        "{script_name} failed: stdout=`{}` stderr=`{}`",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn expected_archives(version: &str) -> Vec<String> {
    [
        format!("canon-{version}-macos-arm64.tar.gz"),
        format!("canon-{version}-macos-x86_64.tar.gz"),
        format!("canon-{version}-linux-arm64.tar.gz"),
        format!("canon-{version}-linux-x86_64.tar.gz"),
        format!("canon-{version}-windows-x86_64.zip"),
    ]
    .into_iter()
    .collect()
}

fn read_json(path: &Path) -> Value {
    serde_json::from_str(&fs::read_to_string(path).expect("read JSON file"))
        .expect("parse JSON file")
}

fn write_json(path: &Path, value: &Value) {
    fs::write(path, serde_json::to_vec_pretty(value).expect("serialize JSON file"))
        .expect("write JSON file");
}
