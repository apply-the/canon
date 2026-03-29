# Decision Log: Governed Execution Adapters

## Baseline Guardrails

- Risk classification for this increment remains `SystemicImpact`; no fourth
  risk tier is introduced.
- The baseline already includes gate-scoped approvals, gate-oriented resume
  behavior, and coarse JSONL traces, but it does not yet have first-class
  invocation governance, invocation-scoped approvals, or run-level evidence
  bundles.
- This tranche proves governed execution through the shared invocation layer
  and `requirements` before broadening deeper into other modes.

## D-001: Add a dedicated execution domain to `canon-engine`

**Decision**: introduce `domain/execution.rs` and related orchestration and
persistence modules instead of pushing invocation semantics into adapter code.

**Rationale**: Canon must own invocation governance semantics centrally.

## D-002: Keep capability typing enum-first

**Decision**: capability semantics remain Rust enums with orthogonal metadata.

**Rationale**: this keeps the type system strong and prevents a generic plugin
runtime.

## D-003: Store summary-first traces

**Decision**: JSONL traces remain append-only summaries with optional retained
payload files referenced by digest.

**Rationale**: Canon needs durable evidence, not transcript sprawl.

## D-004: Approval targets may be invocation-scoped

**Decision**: approval records will support `invocation:<request-id>` targets
in addition to gates.

**Rationale**: governed execution requires approval at the request level, not
only at broad gate boundaries.

## D-005: MCP remains modeled but runtime-staged

**Decision**: keep `McpStdio` in the domain and policy model, but defer full
runtime support unless it fits the common invocation pipeline cleanly.

**Rationale**: avoid over-generalization before the core execution model is
proven.

## D-006: First tranche closes on US1 stability

**Decision**: do not broaden into `brownfield-change` or `pr-review` governed
execution until `requirements` proves invocation inspection, evidence
inspection, invocation-scoped approvals, trace persistence, and end-to-end
stability.

**Rationale**: if the shared invocation spine is weak, mode-specific delivery
will multiply the weakness instead of proving the model.

## D-007: Persist policy-only requests as first-class invocation attempts

**Decision**: denied and approval-gated requests persist `attempt-01.toml`
records with `Denied` or `AwaitingApproval` outcomes even when no external tool
was dispatched.

**Rationale**: the governed execution contract is about attempted work in
motion, not only successful tool dispatch. Each request directory must remain
inspectable and complete even when policy blocks or gates execution.

## D-008: Hold expansion until US1 stability is proven

**Decision**: do not broaden into deeper mode work until `requirements`
proves invocation inspection, evidence inspection, invocation-scoped
approvals, trace persistence, and end-to-end stability.

**Rationale**: the shared invocation spine needed to be proven before Canon
could safely multiply it across more consequential modes.

## D-009: Operator guidance must surface execution evidence, not only artifacts

**Decision**: documentation for this tranche must lead operators to `inspect
invocations`, `inspect evidence`, and `approve --target invocation:<request-id>`
instead of presenting artifact emission as the main product outcome.

**Rationale**: this increment exists to realign Canon around governed
execution. If the operator guide still centers only on markdown outputs, the
product identity drifts back toward artifact generation.

## D-010: Brownfield invocation approval must satisfy the brownfield risk gate

**Decision**: a positive `invocation:<request-id>` approval for pending
brownfield generation counts as sufficient approval for the brownfield risk
gate.

**Rationale**: request-scoped approval must unlock the exact consequential work
it was granted for. Requiring a second redundant risk approval would weaken the
governance model instead of strengthening it.

## D-011: `pr-review` artifacts derive from governed diff inspection and critique evidence

**Decision**: `pr-review` now persists a governed diff-inspection request, a
governed critique request, retained diff payload references, and an evidence
bundle that review artifacts link back to through provenance.

**Rationale**: review output should be inspectable as evidence-backed work in
motion, not as a markdown packet that happens to appear after a diff was read.

## D-012: Close the governed execution increment on three delivered slices

**Decision**: this increment closes with governed execution delivered for
`requirements`, `brownfield-change`, and `pr-review`, while the remaining nine
typed modes stay modeled but not yet executed end to end.

**Rationale**: these three slices prove pre-execution authorization,
invocation-scoped evidence, approval handling, retained diff payloads, and
mode-specific gate behavior without expanding into generic adapter-runner
territory.

## D-013: Serialize root-package CLI integration tests under `nextest`

**Decision**: add `.config/nextest.toml`
to run the root package test binaries in a single test group under
`cargo nextest run`.

**Rationale**: those tests intentionally execute `cargo run` against the Canon
binary. Serializing that package keeps CI stable without weakening the product
runtime model or reducing normal unit-test parallelism in the crate-level
packages.

## D-014: Final acceptance of increment `002-governed-execution-adapters`

**Decision**: accept increment `002-governed-execution-adapters` as complete
for release-closeout purposes.

**Rationale**: the increment now has explicit structural validation, logical
validation across all three delivered user stories, recorded independent review
of lineage and constraint posture, contract conformance against the CLI and
runtime evidence contracts, and operator documentation aligned to the real
implemented workflow.

**Confirmed exclusions**:

- `McpStdio` remains modeled but runtime-denied
- the remaining nine typed modes remain outside this increment
- `verify` remains unimplemented

**Residual backlog**:

- deepen semantic review beyond the current local heuristics and synthetic AI
  critique
- broaden governed execution to the remaining typed modes in later increments
