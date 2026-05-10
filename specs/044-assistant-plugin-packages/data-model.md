# Data Model: Assistant Plugin Packages

## AssistantPluginPackage

Represents one host-specific install/discovery package.

**Fields**:

- `host`: host identifier such as `claude`, `codex`, or `cursor`
- `package_path`: repository-relative package folder
- `manifest_path`: repository-relative manifest path when a manifest exists
- `metadata_fields`: required common identity and discovery fields
- `capability_refs`: host-facing references to shared governed method surfaces
- `skills_refs`: references to Canon-owned skill directories or files
- `method_refs`: references to Canon-owned method metadata where supported
- `asset_refs`: icon or logo references where supported

**Validation rules**:

- `package_path` must exist.
- JSON manifests must parse successfully.
- Common identity fields must align with shared metadata.
- Referenced repository paths must exist.
- Package text must not contain prohibited positioning phrases.

## SharedPluginMetadata

Represents the common Canon package identity.

**Fields**:

- `name`
- `display_name`
- `version`
- `description`
- `author`
- `homepage`
- `repository`
- `license`
- `keywords`
- `positioning`
- `required_capabilities`
- `required_paths`
- `supported_hosts`
- `prohibited_positioning`

**Validation rules**:

- `version` must match `[workspace.package].version` in `Cargo.toml`.
- `required_capabilities` must include clarify input, start governed packet, inspect status, inspect evidence, review packet, verify claims, and publish packet.
- `required_paths` must resolve to existing repository paths.
- `prohibited_positioning` must include the disallowed Canon descriptions from the specification.

## HostCapabilityBinding

Represents a host-facing command, prompt, skill, hook, or method binding.

**Fields**:

- `id`
- `label`
- `purpose`
- `canonical_method`
- `canon_surface`
- `skill_ref`
- `method_ref`
- `prompt`
- `host_overrides`

**Validation rules**:

- `id` must be unique.
- `canonical_method` must match one required method surface.
- `skill_ref` and `method_ref`, when present, must resolve to existing paths.
- `purpose` and `prompt` must preserve Canon runtime authority boundaries.

## CopilotCommandPromptPack

Represents the documented Copilot support boundary.

**Fields**:

- `document_path`
- `starter_prompts`
- `command_mappings`
- `limitations`

**Validation rules**:

- The document must exist and mention that no stable repository-local Copilot plugin manifest is claimed.
- Command mappings must point back to shared governed method surfaces.
- Limitations must state that Canon CLI and the governance adapter remain authoritative.

