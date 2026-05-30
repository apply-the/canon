use canon_workspace::assistant_plugin_validation::{
    REQUIRED_METHODS, capability_ids, command_ids, manifest_errors, string_array,
    workspace_version_from_toml,
};
use serde_json::{Value, json};
use std::fs;
use std::path::PathBuf;

const MANIFESTS: &[&str] =
    &[".claude-plugin/manifest.json", ".codex-plugin/plugin.json", ".cursor-plugin/manifest.json"];

const PACKAGE_FILES: &[&str] = &[
    ".claude-plugin/manifest.json",
    ".claude-plugin/commands.json",
    ".codex-plugin/plugin.json",
    ".cursor-plugin/manifest.json",
    ".cursor-plugin/commands.json",
    "assistant/plugin-metadata.json",
    "assistant/commands/governed-methods.json",
    "assistant/prompts/starter-prompts.md",
    "assistant/prompts/copilot-command-pack.md",
    "assistant/assets/canon-plugin-icon.svg",
    "assistant/assets/canon-plugin-logo.svg",
    "docs/guides/assistant-plugin-packages.md",
];

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read_text(relative_path: &str) -> String {
    let path = repo_root().join(relative_path);
    fs::read_to_string(&path).unwrap_or_else(|error| panic!("failed to read {path:?}: {error}"))
}

fn read_json(relative_path: &str) -> Value {
    serde_json::from_str(&read_text(relative_path))
        .unwrap_or_else(|error| panic!("invalid JSON in {relative_path}: {error}"))
}

fn workspace_version() -> String {
    workspace_version_from_toml(&read_text("Cargo.toml")).expect("workspace version must parse")
}

#[test]
fn package_folders_and_docs_are_present() {
    let root = repo_root();
    for folder in [".claude-plugin", ".codex-plugin", ".cursor-plugin", "assistant"] {
        assert!(root.join(folder).is_dir(), "missing package folder {folder}");
    }
    for file in PACKAGE_FILES {
        assert!(root.join(file).is_file(), "missing package file {file}");
    }

    let guide = read_text("docs/guides/assistant-plugin-packages.md");
    for expected in [
        ".claude-plugin/",
        ".codex-plugin/",
        ".cursor-plugin/",
        "assistant/prompts/copilot-command-pack.md",
        "Canon CLI and the governance adapter remain authoritative",
    ] {
        assert!(guide.contains(expected), "guide must mention {expected}");
    }
}

#[test]
fn manifests_expose_required_governed_methods() {
    let metadata = read_json("assistant/plugin-metadata.json");
    let commands = read_json("assistant/commands/governed-methods.json");
    let required_capabilities = capability_ids(&metadata).expect("metadata capabilities parse");
    let command_ids = command_ids(&commands).expect("command ids parse");

    for required in REQUIRED_METHODS {
        assert!(
            required_capabilities.iter().any(|capability| capability == required),
            "shared metadata must include {required}"
        );
        assert!(
            command_ids.iter().any(|command| command == required),
            "shared command definitions must include {required}"
        );
    }

    for manifest_path in MANIFESTS {
        let manifest = read_json(manifest_path);
        let ids = capability_ids(&manifest).expect("manifest capabilities parse");
        for required in REQUIRED_METHODS {
            assert!(ids.iter().any(|id| id == required), "{manifest_path} must include {required}");
        }
    }
}

#[test]
fn metadata_paths_and_versions_are_aligned() {
    let root = repo_root();
    let version = workspace_version();
    let metadata = read_json("assistant/plugin-metadata.json");

    assert_eq!(metadata["version"], version);
    assert_eq!(metadata["description"], "Governed packet runtime for AI-assisted engineering work");

    for path in string_array(&metadata, "requiredPaths").expect("required paths parse") {
        assert!(root.join(path).exists(), "shared metadata path is missing: {path}");
    }

    for manifest_path in MANIFESTS {
        let manifest = read_json(manifest_path);
        assert!(
            manifest_errors(&manifest, &version, &root).is_empty(),
            "{manifest_path} failed validation"
        );
    }
}

#[test]
fn publish_command_surfaces_match_the_positional_cli_contract() {
    let commands = read_json("assistant/commands/governed-methods.json");
    let publish_command = commands["commands"]
        .as_array()
        .expect("commands array")
        .iter()
        .find(|entry| entry["id"] == "publish-packet")
        .expect("publish command entry");

    assert_eq!(
        publish_command["canonCommand"],
        json!("canon publish <RUN_ID>"),
        "assistant publish command metadata must use the positional CLI contract"
    );

    let copilot_pack = read_text("assistant/prompts/copilot-command-pack.md");
    assert!(
        copilot_pack.contains("| Publish packet | Publish this Canon packet after readiness is established. | `canon publish <RUN_ID>` |"),
        "Copilot command pack must document the positional publish syntax"
    );
    assert!(
        !copilot_pack.contains("canon publish --run <RUN_ID>"),
        "Copilot command pack must not document an unsupported --run publish form"
    );
}

#[test]
fn validation_rejects_drift_and_prohibited_positioning() {
    assert!(serde_json::from_str::<Value>("{").is_err(), "invalid JSON must be rejected");

    let root = repo_root();
    let valid = json!({
        "name": "canon",
        "displayName": "Canon Assistant Support",
        "version": "0.45.0",
        "description": "Governed packet runtime for AI-assisted engineering work",
        "author": {"name": "Apply The", "url": "https://github.com/apply-the"},
        "homepage": "https://github.com/apply-the/canon",
        "repository": "https://github.com/apply-the/canon",
        "license": "MIT",
        "keywords": ["canon", "governance"],
        "capabilities": REQUIRED_METHODS
            .iter()
            .map(|id| json!({"id": id, "label": id.replace('-', " ")}))
            .collect::<Vec<_>>(),
        "paths": {
            "skills": ".agents/skills",
            "methods": "defaults/methods"
        }
    });

    let mut missing_fields = valid.clone();
    missing_fields.as_object_mut().expect("valid manifest must be an object").remove("author");
    assert!(
        manifest_errors(&missing_fields, "0.45.0", &root)
            .iter()
            .any(|error| error.contains("missing required field"))
    );

    let mut version_drift = valid.clone();
    version_drift["version"] = json!("0.0.0");
    assert!(
        manifest_errors(&version_drift, "0.45.0", &root)
            .iter()
            .any(|error| error.contains("version"))
    );

    let mut missing_path = valid.clone();
    missing_path["paths"]["skills"] = json!("missing/skills");
    assert!(
        manifest_errors(&missing_path, "0.45.0", &root)
            .iter()
            .any(|error| error.contains("referenced path"))
    );

    let mut non_string_path = valid.clone();
    non_string_path["paths"]["skills"] = json!(false);
    assert!(
        manifest_errors(&non_string_path, "0.45.0", &root)
            .iter()
            .any(|error| error.contains("path references must be strings"))
    );

    let mut missing_paths = valid.clone();
    missing_paths.as_object_mut().expect("valid manifest must be an object").remove("paths");
    assert!(
        manifest_errors(&missing_paths, "0.45.0", &root)
            .iter()
            .any(|error| error.contains("missing paths object"))
    );

    let mut missing_method = valid.clone();
    missing_method["capabilities"] = json!([]);
    assert!(
        manifest_errors(&missing_method, "0.45.0", &root)
            .iter()
            .any(|error| error.contains("missing required governed method"))
    );

    let mut invalid_capability = valid.clone();
    invalid_capability["capabilities"] = json!([{"label": "missing id"}]);
    assert!(
        manifest_errors(&invalid_capability, "0.45.0", &root)
            .iter()
            .any(|error| error.contains("capability id must be a string"))
    );

    let mut prohibited = valid;
    prohibited["description"] = json!("Canon is an agent framework");
    assert!(
        manifest_errors(&prohibited, "0.45.0", &root)
            .iter()
            .any(|error| error.contains("prohibited positioning"))
    );
}

#[test]
fn validation_helpers_report_malformed_inputs() {
    assert!(workspace_version_from_toml("not = [toml").is_err());
    assert!(workspace_version_from_toml("[workspace]\n[workspace.package]\nversion = 44").is_err());
    assert!(string_array(&json!({"requiredPaths": [1]}), "requiredPaths").is_err());
    assert!(string_array(&json!({}), "requiredPaths").is_err());
    assert!(command_ids(&json!({"commands": [{"label": "missing id"}]})).is_err());
    assert!(command_ids(&json!({})).is_err());
}
