# Research: Assistant Plugin Packages

## Decision: Root-Level Host Package Folders

**Decision**: Use `.claude-plugin/`, `.codex-plugin/`, and `.cursor-plugin/` at the repository root, with host-specific manifests and command glue inside each folder.

**Rationale**: The feature request names these package folders directly, and root-level folders make installation/discovery instructions unambiguous for users copying host support into assistant environments.

**Alternatives considered**:

- Nest all host packages under `assistant/plugins/`: rejected because it does not match the requested package shape.
- Publish one generic package folder for every host: rejected because hosts need distinct metadata and discovery conventions.

## Decision: Shared Metadata And Command Pack Under `assistant/`

**Decision**: Add `assistant/plugin-metadata.json`, `assistant/commands/governed-methods.json`, and shared prompt/asset files that host packages reference.

**Rationale**: Common package fields, required commands, positioning, and asset paths should have one Canon-owned source to reduce version and language drift.

**Alternatives considered**:

- Duplicate command definitions into each host folder: rejected because the feature explicitly forbids duplicating shared content and divergent Canon behavior.
- Infer required metadata from host manifests only: rejected because validation would have no independent common source for drift checks.

## Decision: Codex Uses The Local `.codex-plugin/plugin.json` Shape

**Decision**: Use the Codex plugin manifest shape with interface metadata, capabilities, default prompts, skill path references, and icon/logo fields.

**Rationale**: The local plugin creator guidance documents `.codex-plugin/plugin.json` as the required Codex manifest file and defines the interface fields Canon needs for install/discovery metadata.

**Alternatives considered**:

- Put Codex metadata in a generic JSON file: rejected because it would not be a host-native Codex package surface.
- Create a nested Codex plugin under `plugins/canon/`: rejected because the requested package shape is root `.codex-plugin/`.

## Decision: Copilot Is A Documented Command/Prompt Pack

**Decision**: Provide Copilot support through `assistant/prompts/copilot-command-pack.md` and installation docs, with no `.copilot-plugin/` manifest.

**Rationale**: This repository does not currently define a stable Copilot plugin manifest shape. A documented command/prompt pack is useful without implying unsupported install automation.

**Alternatives considered**:

- Invent `.copilot-plugin/manifest.json`: rejected because it would advertise a stability guarantee the repository cannot validate.
- Omit Copilot entirely: rejected because the feature asks for Copilot support where the surface can be represented cleanly.

## Decision: Rust Integration Test Plus Shell Wrapper

**Decision**: Add `tests/assistant_plugin_packages.rs` for validation logic and `scripts/validate-assistant-plugins.sh` as the documented command wrapper.

**Rationale**: Rust tests fit existing repository validation patterns and can check JSON, shared metadata, path references, version alignment, required method surfaces, and prohibited wording without adding dependencies. The shell wrapper gives users and CI a single command for package validation.

**Alternatives considered**:

- Bash-only validation: rejected because robust JSON/path assertions are clearer in Rust with `serde_json`.
- Production CLI validation command: rejected because plugin packaging is repository validation, not Canon runtime behavior.

