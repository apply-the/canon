# Feature 011: Execution-Mode Approval and Action Chips

## Origin

Carry-forward from `specs/010-controlled-execution-modes/` T044
(independent review). Dogfooding the promoted `implementation` and
`refactor` modes against an external repository surfaced three blocking
gaps that make those modes operationally indistinguishable from `change`
and unusable for host integrations.

See `specs/010-controlled-execution-modes/decision-log.md` D-013, D-014,
D-015 for the design decisions that govern this slice. D-011 is
superseded by D-013.

## Problem Statement

After T044 dogfood:

1. **No execution approval.** Every `implementation` / `refactor` run
   completes immediately as `recommendation-only`. The run summary has
   no `approval_targets`, so `canon approve` and `canon resume` have no
   target to act on. The two modes collapse into "discovery with extra
   metadata".
2. **Hardcoded posture.** `recommendation_only` is fixed in
   `defaults/policies/adapters.toml` and the orchestrator constraint
   profiles, so even an explicit human approval cannot move the run
   into a different posture. Posture must be approval-controlled.
3. **No host-renderable next steps.** The chip contract documented in
   `defaults/embedded-skills/canon-shared/references/output-shapes.md`
   exists but is never populated. Hosts get raw paths and no Approve /
   Resume / Inspect / Open actions.
4. **Cosmetic owner placeholder.** Brief templates and examples ship
   the literal `Owner: maintainer`, which then propagates into the
   bundle summary even when a real git identity is available. (Already
   addressed in this feature; see Phase B in `plan.md`.)

## Scope

### In Scope

- Add an `ExecutionApproval` gate (`gate:execution`) to `implementation`
  and `refactor` run flows; gate is unconditional in v0.1 (D-013).
- Split mutation constraint profiles into pre-approval
  (`*-mutation-recommendation`, `recommendation_only = true`) and
  post-approval (`*-mutation-bounded`, `recommendation_only = false`)
  variants in both `defaults/policies/adapters.toml` and
  `crates/canon-engine/src/orchestrator/invocation.rs` (D-014).
- Reorder `policy_decision_attempt()` so `NeedsApproval` short-circuits
  before the recommendation-only short-circuit, and surface
  `gate:execution` in `RunSummary.approval_targets`.
- Wire `canon resume` so that, post-approval, the run re-evaluates the
  gate, picks the bounded constraint profile, re-emits the artifact
  bundle, and reports `execution_posture = approved-recommendation`
  (D-014).
- Add an `ActionChip` type and `ModeResultSummary.action_chips:
  Vec<ActionChip>`. Implement a single `build_action_chips_for(...)`
  helper used by every mode summarizer. Render chips in the CLI text
  and JSON outputs (D-015).
- Replace the brief-derived owner with the git-derived
  `RunSummary.owner` in the bundle summary; remove `Owner: maintainer`
  from templates and examples.
- Update `canon-implementation` and `canon-refactor` skills (and their
  embedded sources) to describe the new approve+resume cycle and the
  expected chip set.

### Out of Scope (deferred)

- Real bounded patch application after approval. There is still no
  mutation executor for `*-mutation-bounded`. That work is owned by
  Feature 012 (`bounded-mutation-execution`) and gates D-016+.
- Drift detection, post-mutation independent checks, and adapter-level
  patch format decisions — also Feature 012.
- Action chip rollout to `incident`, `migration`, and `verification`
  beyond what already ships, since those modes remain modeled-only.

## Acceptance Criteria

- AC-1: `canon run --mode implementation ...` and `canon run --mode
  refactor ...` against a valid folder-backed packet finish in
  `RunState::AwaitingApproval` with
  `approval_targets = ["gate:execution"]` and `execution_posture =
  recommendation-only`.
- AC-2: `canon approve --run <id> --target gate:execution` records the
  approval, and `canon resume --run <id>` progresses the run to
  `RunState::Completed` with `approval_targets = []` and
  `execution_posture = approved-recommendation`. The artifact bundle is
  re-emitted with the approval lineage attached.
- AC-3: `canon run ... --output json` and the resume output both
  include a non-null `action_chips` array on `mode_result`. The chip
  set matches the visibility rules in D-015 for the run's current
  state.
- AC-4: The CLI text output contains an `Action Chips:` section listing
  the same chips, in the same order they appear in JSON.
- AC-5: Bundle summaries show the git-derived owner when the brief does
  not declare one, and never the literal `maintainer` fallback when a
  git identity is available.
- AC-6: `cargo test`, `cargo nextest run`,
  `cargo clippy --workspace --all-targets --all-features -- -D warnings`,
  and `bash scripts/validate-canon-skills.sh` all pass.

## Risk and Zone

- Risk: `bounded-impact` (touches gating, policy decision, and CLI
  output but stays within the existing `implementation` /
  `refactor` slice).
- Zone: yellow.
- Mode: change (governed change to live execution behavior of
  promoted modes).

## Invariants

- Existing `change` mode behavior is unchanged.
- Existing `RunState` enum is unchanged; new behavior is expressed
  through `approval_targets` and `execution_posture` strings.
- No silent workspace mutation. The bounded profile is selected only
  after explicit human approval, and even then no patch is enacted in
  v0.1 (deferred to Feature 012).
- Backward compatibility for existing run JSON consumers: new fields
  are additive; `primary_artifact_action` is retained.
