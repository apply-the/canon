use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use serde_json::{Value, json};
use tempfile::TempDir;

const WRITE_METADATA_SCRIPT: &str = "scripts/release/write-distribution-metadata.sh";
const RENDER_SCOOP_SCRIPT: &str = "scripts/release/render-scoop-manifest.sh";
const VERIFY_RELEASE_SCRIPT: &str = "scripts/release/verify-release-surface.sh";
const RELEASE_WORKFLOW: &str = ".github/workflows/release.yml";
const RELEASE_NOTES_TEMPLATE: &str = ".github/release-notes-template.md";
const EMBEDDED_RUNTIME_COMPATIBILITY: &str =
    "defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml";
const AGENT_RUNTIME_COMPATIBILITY: &str =
    ".agents/skills/canon-shared/references/runtime-compatibility.toml";

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn script_path(relative: &str) -> PathBuf {
    repo_root().join(relative)
}

fn read(relative: &str) -> String {
    fs::read_to_string(repo_root().join(relative))
        .unwrap_or_else(|err| panic!("read {relative}: {err}"))
}

fn path_string(path: &Path) -> String {
    path.to_str().expect("path should be valid UTF-8").to_owned()
}

fn run_bash_script(script: &str, args: &[String]) -> String {
    let output = Command::new("bash")
        .arg(script_path(script))
        .args(args)
        .output()
        .unwrap_or_else(|err| panic!("run {script}: {err}"));

    assert!(
        output.status.success(),
        "{script} failed with status {:?}\nstdout:\n{}\nstderr:\n{}",
        output.status.code(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    String::from_utf8(output.stdout).expect("stdout should be UTF-8").trim().to_owned()
}

fn create_tarball(dist_dir: &Path, filename: &str) {
    let staging = TempDir::new().expect("create temp dir for tarball staging");
    fs::write(staging.path().join("canon"), "canon").expect("write staged canon binary");

    let output = dist_dir.join(filename);
    let status = Command::new("tar")
        .args(["-czf", output.to_str().expect("output path"), "-C"])
        .arg(staging.path())
        .arg("canon")
        .status()
        .expect("run tar");

    assert!(status.success(), "tar should succeed for {filename}");
}

fn create_zip(dist_dir: &Path, filename: &str) {
    let staging = TempDir::new().expect("create temp dir for zip staging");
    fs::write(staging.path().join("canon.exe"), "canon").expect("write staged Windows binary");

    let output = dist_dir.join(filename);
    let status = Command::new("zip")
        .current_dir(staging.path())
        .args(["-q", output.to_str().expect("output path"), "canon.exe"])
        .status()
        .expect("run zip");

    assert!(status.success(), "zip should succeed for {filename}");
}

fn artifact_names(version: &str) -> Vec<String> {
    vec![
        format!("canon-{version}-macos-arm64.tar.gz"),
        format!("canon-{version}-macos-x86_64.tar.gz"),
        format!("canon-{version}-linux-arm64.tar.gz"),
        format!("canon-{version}-linux-x86_64.tar.gz"),
        format!("canon-{version}-windows-x86_64.zip"),
    ]
}

fn stage_release_bundle(version: &str) -> TempDir {
    let dist = TempDir::new().expect("create dist temp dir");

    for filename in artifact_names(version) {
        if filename.ends_with(".zip") {
            create_zip(dist.path(), &filename);
        } else {
            create_tarball(dist.path(), &filename);
        }

        fs::write(dist.path().join(format!("{filename}.version.txt")), format!("{version}\n"))
            .expect("write version evidence");
    }

    let release_notes = format!(
        "# Canon {version}\n\n## Artifacts\n\n- {}\n",
        artifact_names(version).join("\n- ")
    );
    fs::write(dist.path().join("release-notes.md"), release_notes).expect("write release notes");

    dist
}

fn generate_checksums(version: &str, dist_dir: &Path) {
    run_bash_script(
        VERIFY_RELEASE_SCRIPT,
        &[
            "--version".to_owned(),
            version.to_owned(),
            "--dist-dir".to_owned(),
            path_string(dist_dir),
            "--release-notes".to_owned(),
            path_string(&dist_dir.join("release-notes.md")),
            "--write-checksums".to_owned(),
        ],
    );
}

fn write_distribution_metadata(version: &str, dist_dir: &Path) -> PathBuf {
    let metadata = dist_dir.join(format!("canon-{version}-distribution-metadata.json"));
    run_bash_script(
        WRITE_METADATA_SCRIPT,
        &[
            "--version".to_owned(),
            version.to_owned(),
            "--dist-dir".to_owned(),
            path_string(dist_dir),
            "--output".to_owned(),
            path_string(&metadata),
        ],
    );
    metadata
}

fn read_json(path: &Path) -> Value {
    serde_json::from_str(
        &fs::read_to_string(path).unwrap_or_else(|err| panic!("read {}: {err}", path.display())),
    )
    .unwrap_or_else(|err| panic!("parse {}: {err}", path.display()))
}

fn lookup_sha(dist_dir: &Path, version: &str, artifact: &str) -> String {
    let manifest = fs::read_to_string(dist_dir.join(format!("canon-{version}-SHA256SUMS.txt")))
        .expect("read checksum manifest");

    manifest
        .lines()
        .find_map(|line| {
            let (sha, file) = line.split_once("  ")?;
            (file == artifact).then(|| sha.to_owned())
        })
        .unwrap_or_else(|| panic!("missing checksum for {artifact}"))
}

#[test]
fn distribution_metadata_marks_windows_asset_for_winget_and_scoop() {
    let version = "0.32.0";
    let dist = stage_release_bundle(version);
    generate_checksums(version, dist.path());
    let metadata_path = write_distribution_metadata(version, dist.path());
    let metadata = read_json(&metadata_path);

    let windows_asset = metadata["assets"]
        .as_array()
        .expect("assets should be an array")
        .iter()
        .find(|asset| asset["asset_id"] == "windows-x86_64")
        .expect("windows asset should exist");

    assert_eq!(windows_asset["filename"], format!("canon-{version}-windows-x86_64.zip"));
    assert_eq!(windows_asset["binary_name"], "canon.exe");
    assert_eq!(windows_asset["channels"], json!(["winget", "scoop"]));
    assert_eq!(
        windows_asset["download_url"],
        format!(
            "https://github.com/apply-the/canon/releases/download/v{version}/canon-{version}-windows-x86_64.zip"
        )
    );
}

#[test]
fn rendered_scoop_manifest_matches_the_canonical_release_bundle() {
    let version = "0.32.0";
    let dist = stage_release_bundle(version);
    generate_checksums(version, dist.path());
    let metadata_path = write_distribution_metadata(version, dist.path());
    let manifest_path = dist.path().join(format!("canon-{version}-scoop-manifest.json"));

    run_bash_script(
        RENDER_SCOOP_SCRIPT,
        &[
            "--metadata".to_owned(),
            path_string(&metadata_path),
            "--output".to_owned(),
            path_string(&manifest_path),
        ],
    );

    let manifest = read_json(&manifest_path);
    let artifact = format!("canon-{version}-windows-x86_64.zip");
    let expected_sha = lookup_sha(dist.path(), version, &artifact);
    let expected_url =
        format!("https://github.com/apply-the/canon/releases/download/v{version}/{artifact}");

    assert_eq!(manifest["version"], version);
    assert_eq!(
        manifest["description"],
        "Governed local-first method engine for AI-assisted software engineering"
    );
    assert_eq!(manifest["homepage"], "https://github.com/apply-the/canon");
    assert_eq!(manifest["license"], "MIT");
    assert_eq!(manifest["architecture"]["64bit"]["url"], expected_url);
    assert_eq!(manifest["architecture"]["64bit"]["hash"], expected_sha);
    assert_eq!(manifest["bin"], "canon.exe");

    run_bash_script(
        VERIFY_RELEASE_SCRIPT,
        &[
            "--version".to_owned(),
            version.to_owned(),
            "--dist-dir".to_owned(),
            path_string(dist.path()),
            "--release-notes".to_owned(),
            path_string(&dist.path().join("release-notes.md")),
            "--distribution-metadata".to_owned(),
            path_string(&metadata_path),
            "--scoop-manifest".to_owned(),
            path_string(&manifest_path),
        ],
    );
}

#[test]
fn workflow_and_release_notes_reference_the_scoop_artifact_surface() {
    let workflow = read(RELEASE_WORKFLOW);
    assert!(
        workflow.contains("- name: Render Scoop manifest")
            && workflow.contains("scripts/release/render-scoop-manifest.sh")
            && workflow.contains(
                "canon-${{ needs.prepare-release-metadata.outputs.version }}-scoop-manifest.json"
            ),
        "release workflow must render and publish the Scoop manifest artifact"
    );
    assert!(
        workflow.contains("--scoop-manifest dist/canon-${{ needs.prepare-release-metadata.outputs.version }}-scoop-manifest.json"),
        "release workflow must verify the generated Scoop manifest"
    );

    let release_notes = read(RELEASE_NOTES_TEMPLATE);
    assert!(
        release_notes.contains("### Windows via Scoop")
            && release_notes.contains("scoop install canon")
            && release_notes.contains("scoop update canon")
            && release_notes.contains("canon-{{VERSION}}-scoop-manifest.json"),
        "release notes template must describe the Scoop install path and artifact"
    );
}

#[test]
fn runtime_refs_describe_the_0_32_0_scoop_slice() {
    for path in [EMBEDDED_RUNTIME_COMPATIBILITY, AGENT_RUNTIME_COMPATIBILITY] {
        let compatibility = read(path);
        assert!(
            compatibility.contains("expected_workspace_version = \"0.32.0\""),
            "{path} must carry the 0.32.0 workspace expectation"
        );
    }
}
