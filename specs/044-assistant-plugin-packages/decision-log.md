# Decision Log: Assistant Plugin Packages

- **D-001**: Keep assistant plugin packages as install, discovery, metadata, command binding, prompt, asset, and host glue surfaces. **Rationale**: Canon CLI and the governance adapter already own governed packet behavior.
- **D-002**: Add root package folders for Claude Code, Codex, and Cursor. **Rationale**: The requested package shape names `.claude-plugin/`, `.codex-plugin/`, and `.cursor-plugin/` directly, and root folders make installation instructions easy to follow.
- **D-003**: Add shared package metadata and governed method definitions under `assistant/`. **Rationale**: Shared Canon-owned metadata prevents host manifest drift while still allowing host-specific package glue.
- **D-004**: Treat Copilot support as a documented command/prompt pack. **Rationale**: No stable Copilot plugin manifest shape exists in this repository, so claiming one would mislead users.
- **D-005**: Validate assistant plugin packages through a focused Rust integration test plus shell wrapper. **Rationale**: Rust tests align with existing repository validation and provide robust JSON/path/version checks without adding runtime behavior.
- **D-006**: Bump Canon to `0.44.0` for this feature. **Rationale**: The package surface is public install/discovery metadata and should have a distinct pre-1.0 minor version.
- **D-007**: Keep Claude Code and Cursor command files as host binding maps that point at `assistant/commands/governed-methods.json`. **Rationale**: Host folders need native glue, but the governed method definitions should stay shared.
- **D-008**: Keep Codex-specific interface metadata in `.codex-plugin/plugin.json` while also preserving the common top-level fields validated across hosts. **Rationale**: Codex needs interface presentation fields, and common fields keep the package aligned with the shared Canon identity.
