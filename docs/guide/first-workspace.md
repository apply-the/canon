# First Workspace

This page explains what needs to exist before Canon can produce governed packets in a repository, and how the workspace is structured.

## Repository Setup

Run Canon inside the repository whose engineering knowledge you want to govern. Initialization creates the local Canon runtime surface, including `.canon/`.

The repository should have a clear working directory and a place for authored input:

- `.canon/` for Canon runtime state, runs, artifacts, evidence, and manifests
- `canon-input/` for authored mode input
- Published packet destinations under visible repository paths such as `tech-docs/` or `specs/`

Authored input should be explicit. Do not rely on open editor tabs, incidental files, or previous generated artifacts as the current run input.

## Assistant Integration

Canon can materialize repo-local AI skills for supported assistant surfaces. Those skills guide the assistant to:

- read the authored input
- start or inspect the Canon run
- produce grounded packet content
- critique its own output
- preserve provenance
- avoid mutating authored input while generating packet artifacts

Assistant integration improves the quality of generated packet content, but Canon's governance model is still anchored in the CLI and recorded artifacts.

Canon's repo-local skill surface includes explicit runtime contracts for the
migrated `canon-implementation`, `canon-change`, and `canon-publish` skills:

- structured preflight JSON via `canon-preflight.sh` and `canon-preflight.ps1`
- declarative `preflight:` YAML frontmatter for machine-readable prerequisites
- detect/propose lifecycle hooks from `.canon/hooks.toml` with trace recording in `ai-provenance.md`

## Workspace Resolution

Canon resolves the active repository automatically. It prefers the nearest initialized `.canon/` ancestor, then the nearest `.git/` root, and finally falls back to the current working directory. You can run `canon` commands from any subdirectory within your project without needing to re-specify the repository root.

## Workspace Assumptions

Canon assumes:

- the resolved workspace is the repository being governed
- authored input is workspace-relative
- generated packet artifacts stay under Canon-managed paths until publication
- risk, zone, owner, and system context are explicit where the mode requires them
- downstream tools consume stable packet refs rather than scraping human prose

For file-backed modes, use the canonical `canon-input/<mode>.md` or `canon-input/<mode>/` locations described in [Canon Modes](./canon-modes).

## Troubleshooting Setup Failures

If setup fails, check these first:

- `canon` is installed and on `PATH`
- the command is running from a directory within a valid repository root (with a `.canon/` or `.git/` folder upstream)
- `.canon/` can be created or read
- the authored input path exists and is not empty
- the mode name matches a supported mode
- required `risk`, `zone`, `owner`, or `system-context` values are provided
- assistant skills were initialized for the assistant actually being used
