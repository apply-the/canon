# Research: Project Memory Promotion Policy

## Decision 1: Promotion State Resolution Strategy

**Decision**: Derive promotion state from the combination of run state, mode,
and per-mode policy configuration rather than requiring the operator to specify
it explicitly at publish time.

**Rationale**: The operator already selects a mode and the run already has a
state (Completed, AwaitingApproval, Blocked). The promotion policy is a
deterministic function of those inputs. Forcing explicit state selection would
duplicate information and create a new surface for operator error.

**Alternatives considered**:
- Explicit `--promotion-state` CLI flag: rejected because it decouples
  promotion intent from governed run state, violating the invariant that
  project-visible output is a projection of governed results.
- Infer from file content analysis: rejected as non-deterministic and outside
  Canon's governance model.

## Decision 2: Update Strategy Selection

**Decision**: Update strategy is configured per-target in the publish profile
TOML, not selected at runtime by the operator.

**Rationale**: The appropriate strategy depends on the target document's
structure (managed-block vs. index vs. standalone), which is a stable property
of the target path. Runtime selection would force operators to remember which
strategy each target uses.

**Alternatives considered**:
- Runtime `--update-strategy` flag: rejected because the target file's
  structure, not operator preference, determines the safe update mechanism.
- Auto-detect from file content markers: considered viable as a future
  enhancement but too fragile for the initial slice.

## Decision 3: Lineage Metadata Location

**Decision**: Emit lineage metadata as a JSON sidecar file (`*.lineage.json`)
alongside each promoted artifact, and optionally embed a summary in
YAML front matter for Markdown targets.

**Rationale**: A sidecar file is always parseable regardless of target format.
YAML front matter is convenient for Markdown reviewers but insufficient as the
sole storage mechanism because not all targets are Markdown.

**Alternatives considered**:
- Embed only in YAML front matter: rejected because non-Markdown targets
  (indexes, evidence directories) would lose lineage.
- Central lineage registry file: rejected because it couples all promotion
  outputs to a single-file bottleneck.

## Decision 4: Managed-Block Marker Format

**Decision**: Use HTML comment markers `<!-- canon:managed:start -->` and
`<!-- canon:managed:end -->` to delimit Canon-owned ranges in Markdown
documents.

**Rationale**: HTML comments are invisible in rendered Markdown, widely
supported across Markdown renderers, and unlikely to conflict with
human-authored content. They also survive common reformatting tools.

**Alternatives considered**:
- Custom YAML front matter range indicators: rejected because they cannot
  delimit arbitrary ranges within a document.
- Sentinel text strings: rejected because they would be visible in rendered
  output.

## Decision 5: Contract Stable Path

**Decision**: Promote the accepted contract to
`tech-docs/integration/project-memory-promotion-contract.md` rather than
`tech-docs/contracts/`.

**Rationale**: `tech-docs/integration/` signals that the document defines a
cross-repo integration boundary, which is more discoverable for consumers than
a generic `tech-docs/contracts/` path. Existing Canon docs use `tech-docs/` subdirectories
organized by topic.

**Alternatives considered**:
- `tech-docs/contracts/`: less descriptive; does not communicate the cross-repo
  integration purpose.
- Root-level `CONTRACT.md`: too prominent for a single integration contract
  when Canon may have multiple in the future.

## Decision 6: Default Promotion Policy Storage

**Decision**: Store default per-mode promotion policies in a TOML file at
`defaults/policies/publish-profiles.toml` and allow workspace-local overrides
through `.canon/config.toml`.

**Rationale**: A checked-in default file provides a reviewable baseline.
Workspace-local overrides let projects customize promotion behavior without
forking Canon.

**Alternatives considered**:
- Hard-coded in Rust: rejected because promotion policy tuning should not
  require a Canon release.
- Environment variables: rejected because they are not inspectable as
  artifacts.
