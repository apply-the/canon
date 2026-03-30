# Decision Log: Runnable Skill Interaction and Ref-Safe Input Binding

## 2026-03-29 - D-001 Local-only ref acceptance for `canon-pr-review`

**Context**: the spec required the plan to close whether remote refs are
accepted and in what order resolution runs.

**Decision**: this patch accepts only `HEAD`, explicit `refs/heads/*`, and
short local branch names that resolve to local heads. Remote refs are rejected
with explicit guidance.

**Alternatives considered**:

- support tracked remote refs in this patch
- leave remote-ref behavior unresolved until implementation

**Rationale**: the trust-repair goal favors correctness and determinism over
convenience. Remote refs add ambiguity without being required by the current
Canon runtime.

**Consequences**:

- `canon-pr-review` retry guidance becomes narrower but more trustworthy
- future remote-ref support, if needed, must be deliberate and separately
  validated

## 2026-03-29 - D-002 Canonical CLI binding uses `HEAD` or explicit `refs/heads/*`

**Context**: short branch names are convenient for users, but the plan must
define what exact values are passed to Canon.

**Decision**: normalize short local branch names to `refs/heads/<name>`.
Preserve `HEAD` as `HEAD`.

**Alternatives considered**:

- pass short branch names through unchanged
- auto-map `main` to `master` or the reverse

**Rationale**: explicit canonical forms make retry guidance and command binding
match exactly while avoiding silent intent changes.

**Consequences**:

- affected skills must show the exact normalized command form in retry guidance
- shared preflight must return normalized ref values

## 2026-03-29 - D-003 Input preservation is intra-interaction only

**Context**: the spec requires valid inputs to be preserved across retry, but
the user explicitly warned against introducing hidden conversational memory.

**Decision**: preserve valid inputs only for the current active runnable-skill
interaction. Do not persist or reuse them across later turns or different
skills without explicit user confirmation.

**Alternatives considered**:

- persist typed inputs in repo-local state
- reuse prior slot values automatically across later invocations

**Rationale**: one-interaction preservation improves usability while staying
inside a clear trust boundary.

**Consequences**:

- skill instructions must restate preserved values when asking for one
  correction
- shared helpers must not become state stores

## 2026-03-29 - D-004 Extend the existing shared preflight instead of adding a framework

**Context**: the patch needs typed validation and deterministic retry guidance,
but scope excludes a generic interaction engine.

**Decision**: keep `check-runtime.sh` and `check-runtime.ps1` as the only
shared preflight entrypoints and add bounded helper logic inside them.

**Alternatives considered**:

- create a new generic interaction framework
- keep all logic inside individual `SKILL.md` files

**Rationale**: the current bug lives in the shared boundary, so the smallest
correct fix is to repair that boundary and tighten skill instructions around
it.

**Consequences**:

- shell and PowerShell parity becomes mandatory for new status codes and
  normalized outputs
- validators must check both helper behavior and affected skill text

## 2026-03-29 - D-005 Reuse run-id handling broadly, keep approve extras skill-local

**Context**: run-id-based skills can benefit from typed retry handling, but the
patch should not broaden into full shared validation for every operational
field.

**Decision**: apply shared `RunIdInput` handling to status, inspect, resume,
and approve flows, while leaving `TARGET`, `BY`, `DECISION`, and `RATIONALE`
as skill-local fields for this increment.

**Alternatives considered**:

- leave operational skills unchanged
- add a full shared taxonomy for approval metadata now

**Rationale**: run-id correction is low-cost reuse; approval-field taxonomy is
not needed to repair `canon-pr-review`.

**Consequences**:

- operational skills become more consistent without scope expansion
- approval-field typing can be reconsidered in a future focused patch

## 2026-03-29 - D-006 Feature risk classification is `bounded-impact`

**Context**: the feature plan originally used the generic label `High`, while
Canon policy, CLI contracts, and downstream task generation use canonical risk
tokens such as `low-impact`, `bounded-impact`, and `systemic-impact`.

**Decision**: classify this corrective patch as `bounded-impact`.

**Alternatives considered**:

- classify the patch as `systemic-impact`
- keep the generic `High` label and let downstream tooling map it implicitly

**Rationale**: the failure mode is important because a bad binding can start the
wrong governed command or damage trust in delivered skills, but the change
remains bounded to the repo-local skills frontend and keeps Canon CLI as the
only execution engine and system of record.

**Consequences**:

- feature artifacts can reference the same risk vocabulary the runtime and
  policies already enforce
- task generation no longer needs to invent an ad hoc mapping from `High` to a
  Canon-native risk token

## 2026-03-29 - D-007 Shared helpers remain the enforcement point

**Context**: this patch updates `SKILL.md` files for the direct-scope runnable
skills, but trust repair only works if typed preflight, ref classification, and
retry rendering are enforced by the shared helper boundary rather than by prose.

**Decision**: keep `.agents/skills/canon-shared/scripts/check-runtime.sh` and
`.agents/skills/canon-shared/scripts/check-runtime.ps1` as the source of truth
for typed preflight behavior, and require skill docs to mirror that behavior
without widening it.

**Alternatives considered**:

- encode most of the behavior only in `SKILL.md` guidance
- create a new interaction layer separate from the existing shared helpers

**Rationale**: prose-only fixes would leave the real enforcement point weak,
while a new layer would broaden scope beyond a narrow trust-repair patch.

**Consequences**:

- validation must prove helper behavior first and skill wording second
- direct-scope skill updates are constrained to documenting helper-backed
  behavior rather than inventing new interaction semantics
