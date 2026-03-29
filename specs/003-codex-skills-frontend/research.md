# Research: Codex Skills Frontend for Canon

## Decision 1: Make the full Canon taxonomy discoverable in phase 1

**Decision**: implement all Canon skills as discoverable repo-local skills and
use brutally explicit support-state labeling instead of soft-hiding modeled
workflows.

**Rationale**: repo-local Codex skills are discovered from filesystem layout.
Phase 1 should improve UX by making the whole Canon surface visible while
preserving trust through honest support-state behavior.

**Alternatives considered**:

- Soft-hide modeled-only skills in docs and routing: rejected because the
  product correction requires discoverability for the full Canon taxonomy.
- Ship only runnable skills and omit the rest entirely: rejected because the
  feature requires a complete visible skill contract for the Canon taxonomy.

## Decision 2: Keep two skill classes only

**Decision**: use two concrete frontend skill types:

- executable wrapper skills
- support-state wrapper skills

**Rationale**: this keeps the skill surface explicit and avoids inventing a
third generic orchestration layer.

**Alternatives considered**:

- One generic `canon` router skill: rejected because it weakens mode semantics
  and turns the frontend into a chat workflow.
- One-off custom logic per skill: rejected because it would duplicate
  boilerplate across many skill folders.

## Decision 3: Use a small shared deterministic support layer

**Decision**: add a shared `canon-shared` area with:

- runtime compatibility references
- support-state and output references
- shell-specific helper scripts for preflight and standard response shaping

**Rationale**: the feature needs reuse for version checks, repo-context checks,
support-state messages, and next-step guidance, but it does not need a generic
skill runtime.

**Alternatives considered**:

- No shared helpers at all: rejected because it would spread identical failure
  handling and messaging across many `SKILL.md` files.
- A generic `run-skill` framework: rejected because it would recentralize
  logic and blur Canon's runtime authority.

## Decision 4: Make version compatibility explicit in repo files

**Decision**: store the expected Canon version contract in a shared reference
file and validate `canon --version` against it in helper scripts.

**Rationale**: Codex skills need a deterministic way to reject incompatible
Canon binaries with actionable guidance.

**Alternatives considered**:

- Blindly trust whatever `canon` is on PATH: rejected because the skill layer
  would drift from the repo's expected CLI surface.
- Infer compatibility from README text only: rejected because that is not
  deterministic enough for preflight behavior.

## Decision 5: Validate supported skills by walkthrough, not by a fake skill runner

**Decision**: validate high-value skills with deterministic repo checks and
end-to-end walkthroughs against the real Canon CLI rather than building a
simulated Codex skill executor.

**Rationale**: the value of the feature is the contract between skill text,
shared helpers, and the actual Canon runtime. A fake skill runner would be a
second system and would not prove real Codex usage quality.

**Alternatives considered**:

- Build a local test harness that executes skill logic abstractly: rejected
  because that would recreate a skill runtime instead of validating the real
  repo-local layer.
- Rely on documentation review alone: rejected because it would not test
  failure handling or Canon command mappings.
