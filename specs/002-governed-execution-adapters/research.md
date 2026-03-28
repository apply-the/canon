# Research: Governed Execution Adapters

## Decision 1: Type adapter capabilities as a small enum plus orthogonal metadata

**Decision**: keep capability typing enum-based, but extend it with
`InvocationOrientation`, `MutabilityClass`, `TrustBoundaryKind`, and
`LineageClass`.

**Rationale**: a single flat enum will either stay too coarse or explode into
one-off variants. A small canonical enum plus metadata preserves type safety
without turning Canon into a generic tool schema engine.

**Alternatives considered**:

- fully trait-driven capability objects: rejected because it pushes Canon’s
  semantics into adapter-specific code
- stringly typed capability names in config: rejected because it weakens
  compile-time guarantees and encourages plugin drift

## Decision 2: Represent "allow with constraints" as a typed constraint set

**Decision**: use `InvocationPolicyDecision` with a typed
`InvocationConstraintSet`.

**Rationale**: constraints must be inspectable, serializable, and executable.
Free-form maps are easy to persist but hard to enforce. Typed constraints make
review and runtime enforcement match.

**Alternatives considered**:

- post-hoc advisory notes: rejected because the spec requires constraints to
  apply before execution
- per-adapter ad hoc constraint formats: rejected because it harms inspection
  and cross-adapter governance consistency

## Decision 3: Keep policy semantics in code, but matrices in TOML

**Decision**: code owns mode semantics, precedence rules, lineage rules, and
policy interpretation; TOML owns adapter/capability matrices, approval
requirements, retention caps, and path profiles.

**Rationale**: Canon’s product identity lives in semantics, not in editable
policy text. The configurable layer should tune allowed behavior, not redefine
what a mode or validation rule means.

**Alternatives considered**:

- fully code-defined policy: rejected because it makes routine operational
  tuning expensive
- fully config-defined policy DSL: rejected because it would reintroduce a
  generic workflow/config framework

## Decision 4: Persist summary-first traces, not transcript-first traces

**Decision**: JSONL remains the event stream, but per-invocation directories
hold request, decision, attempt, and optional retained payload references.

**Rationale**: Canon needs durable evidence without turning the runtime into a
prompt archive. Summary-first traces keep inspection useful and storage bounded.

**Alternatives considered**:

- store every prompt/output inline in JSONL: rejected because it creates noisy,
  bulky traces and raises secret-handling risk
- store only artifacts: rejected because it loses work-in-motion evidence

## Decision 5: Model validation independence with lineage classes

**Decision**: add lineage metadata and compute independence based on lineage
and trust boundary, not just adapter kind.

**Rationale**: two invocations can look different while sharing the same
reasoning family. Canon needs a better basis for independence than “different
command string”.

**Alternatives considered**:

- adapter name as the independence proxy: rejected because the same adapter can
  serve radically different roles
- human review only for all consequential work: rejected because it would
  unnecessarily narrow automation for bounded-impact flows

## Decision 6: Approvals attach to invocation requests, not opaque attempts

**Decision**: approval targets will reference `request_id`s. Retries create new
attempt records under the same request unless scope or context changes.

**Rationale**: approvals should authorize a governed request shape, not a
single low-level process attempt. If the request shape changes, the approval
must be re-earned.

**Alternatives considered**:

- approval per process attempt: rejected because it becomes brittle and noisy
- approval for the entire run only: rejected because it is too coarse for
  constrained execution

## Decision 7: MCP stays in-domain, but not in first runtime slice

**Decision**: keep `McpStdio` in the domain and policy model, but deliver real
runtime behavior only if it can fit the same request/decision/outcome pipeline
without special cases.

**Rationale**: the domain should not exclude structured tools, but Canon should
not over-generalize around an adapter that does not yet justify its runtime
weight.

**Alternatives considered**:

- full MCP executor in the first slice: rejected because it adds complexity
  before the core invocation model is proven
- remove MCP entirely from the model: rejected because it would artificially
  narrow the execution surface Canon is meant to govern

## Tranche Note: Runtime MCP execution stays explicitly blocked

The first implementation tranche keeps MCP in:

- capability typing
- policy surfaces
- inspection and evidence language

The first implementation tranche keeps MCP out of:

- runtime dispatch
- adapter execution
- resume or retry flows

This avoids the task generator or implementation from pulling MCP runtime work
back in "for completeness" before the shared invocation spine is stable.
