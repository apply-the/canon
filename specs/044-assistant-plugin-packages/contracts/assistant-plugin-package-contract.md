# Contract: Assistant Plugin Package Surface

## Required Package Folders

- `.claude-plugin/`
- `.codex-plugin/`
- `.cursor-plugin/`

No `.copilot-plugin/` folder is required for this slice unless a stable Copilot manifest shape is introduced in the repository.

## Required Common Metadata

Every supported host package must expose or reference:

- `name`
- `display_name`
- `version`
- `description`
- `author`
- `homepage`
- `repository`
- `license`
- `keywords`
- `capabilities`
- skill, command, hook, prompt, or method paths where supported by the host
- default starter prompts where supported by the host
- icon or logo metadata where supported by the host

## Required Canon Positioning

Supported package metadata must include Canon positioning consistent with:

- "Governed packet runtime for AI-assisted engineering work"
- "Structured packets, evidence, approvals, and provenance"
- "Governed methods for requirements, architecture, backlog, implementation, review, verification, incident, and migration"

Supported package metadata must not describe Canon as:

- an agent framework
- an orchestrator
- a coding agent
- a workspace mutation engine

## Required Governed Method Surfaces

Each supported host package must expose or document bindings for:

- clarify input
- start governed packet
- inspect status
- inspect evidence
- review packet
- verify claims
- publish packet

## Shared Source Boundary

Host package folders may contain manifests, metadata, assets, command bindings, prompts, hooks, and host-specific glue. They must not duplicate the full contents of `.agents/skills/`, `.canon/methods/`, or `defaults/embedded-skills/`.

## Validation Contract

The package validation command must fail when:

- a JSON manifest cannot be parsed
- a required field is absent
- a package version differs from the workspace version
- a referenced repository path does not exist
- a required governed method surface is missing
- prohibited positioning text appears in package metadata

The package validation command must pass for the committed package set.

