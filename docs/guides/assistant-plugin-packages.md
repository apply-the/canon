# Assistant Plugin Packages

Canon includes repository-local assistant package surfaces for hosts that can consume package metadata or command bindings. These packages advertise Canon as a governed packet runtime for AI-assisted engineering work with structured packets, evidence, approvals, and provenance.

Canon CLI and the governance adapter remain authoritative for packet behavior, run state, evidence, approvals, and provenance. Host packages install guidance, prompts, and bindings only; they do not replace the Canon runtime.

## Supported Host Packages

| Host | Package Folder | Contents | Install Shape |
|------|----------------|----------|---------------|
| Claude Code | `.claude-plugin/` | `manifest.json` plus command bindings | Copy or link the folder into the host plugin/package location expected by Claude Code. |
| Codex | `.codex-plugin/` | `plugin.json` with interface metadata, capabilities, prompts, and paths | Use the folder as the Codex plugin package root for this repository. |
| Cursor | `.cursor-plugin/` | `manifest.json` plus command bindings | Copy or link the folder into the host extension/package location expected by Cursor. |
| Copilot | `assistant/prompts/copilot-command-pack.md` | Documented commands and prompts | Use the prompt pack directly; no stable repository-local Copilot manifest is claimed. |

## Shared Canon Sources

Host package folders reference shared Canon-owned files instead of copying the full skill or method corpus:

- `.agents/skills/`
- `.canon/methods/`
- `defaults/embedded-skills/`
- `assistant/commands/governed-methods.json`
- `assistant/prompts/starter-prompts.md`

## Governed Method Surfaces

Every supported host package exposes or documents:

- clarify input
- start governed packet
- inspect status
- inspect evidence
- review packet
- verify claims
- publish packet

## Validation

Run package validation from the repository root:

```bash
bash scripts/validate-assistant-plugins.sh
```

The validation checks JSON manifest syntax, required fields, Canon version alignment, referenced paths, required method surfaces, and prohibited package positioning.

## Limitations

The packages do not make assistant hosts authoritative for Canon behavior. They do not create a new runtime, bypass approvals, fabricate evidence, invent run ids, or expose private internals. Future host packages should extend `assistant/plugin-metadata.json` and `assistant/commands/governed-methods.json` rather than copying Canon skill content.
