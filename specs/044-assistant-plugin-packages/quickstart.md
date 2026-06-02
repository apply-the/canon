# Quickstart: Assistant Plugin Packages

## Validate Package Metadata

From the repository root:

```bash
bash scripts/validate-assistant-plugins.sh
```

Expected result:

```text
PASS: Canon assistant plugin packages are valid.
```

## Inspect Supported Host Packages

```bash
ls .claude-plugin .codex-plugin .cursor-plugin
```

Expected result:

- `.claude-plugin/manifest.json`
- `.codex-plugin/plugin.json`
- `.cursor-plugin/manifest.json`
- host command glue where the host package supports it

## Install Guidance

Read:

```text
tech-docs/guides/assistant-plugin-packages.md
```

The guide identifies how to install or copy Canon support for Claude Code, Codex, and Cursor, and explains why Copilot support is a documented command/prompt pack instead of a claimed plugin manifest.

## Boundary Check

When reviewing package metadata, confirm that packages present Canon as a governed packet runtime and not as an assistant-host runtime. Canon CLI and the governance adapter remain authoritative for packet behavior.

