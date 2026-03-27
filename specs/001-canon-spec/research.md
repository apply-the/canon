# Research: Canon v0.1

## 1. Workspace Topology

**Decision**: use a three-crate workspace: `canon-cli`,
`canon-engine`, and `canon-adapters`.

**Rationale**: this keeps the public command surface, core product semantics,
and external execution surfaces separated without creating the maintenance cost
of five or more crates. The engine remains the product core.

**Alternatives considered**:

- single crate monolith: simpler at first, but it would blur domain,
  persistence, and adapter boundaries immediately
- many small crates for policy, artifacts, persistence, and review: cleaner on
  paper, but premature for v0.1 and likely to create architectural ceremony

## 2. Mode and Method Modeling

**Decision**: model all modes as explicit Rust enums and code-owned
`ModeProfile` definitions. Use a hybrid method approach where code defines the
semantics and TOML carries versioned metadata and safe defaults.

**Rationale**: the product thesis depends on mode semantics being durable and
non-generic. A pure config system would turn the product into a loose workflow
framework. Pure code would make policy updates unnecessarily rigid.

**Alternatives considered**:

- fully config-defined methods: rejected because it invites DSL drift
- fully code-defined everything: rejected because policy and method defaults
  need inspectable, versioned data without recompilation

## 3. Runtime Model

**Decision**: keep v0.1 synchronous and blocking.

**Rationale**: local filesystem operations and external command invocations are
easier to audit and reason about in a synchronous orchestrator. Async does not
solve a pressing product problem in v0.1 and would complicate testing,
tracing, and cross-platform process control.

**Alternatives considered**:

- async runtime with concurrent adapters: rejected because the product is a
  governed control plane, not a throughput server
- background worker model: rejected because it weakens local auditability and
  resumability

## 4. Persistence and Artifact Memory

**Decision**: store all runtime memory under `.canon/` with TOML manifests,
Markdown or structured artifacts, JSONL traces, and append-only decision
records.

**Rationale**: the product exists to make engineering memory durable and
inspectable. The filesystem is the correct v0.1 persistence layer because it is
local, reviewable, and easy to version with the repository.

**Alternatives considered**:

- SQLite or embedded database: rejected because it adds operational weight and
  hides memory behind tooling rather than plain files
- ad hoc markdown only: rejected because contracts, traces, and approvals need
  machine-readable metadata

## 5. Policy and Gate Enforcement

**Decision**: gate evaluation is fail-closed and schema-bound. Overrides require
both a persisted approval and a decision record.

**Rationale**: governance only matters if the engine can stop unsafe momentum.
Soft warnings would recreate the same failure mode the product is meant to
prevent.

**Alternatives considered**:

- warning-only gates: rejected because they do not create trustworthy control
- fully dynamic policy expressions: rejected because they create a shadow
  programming language

## 6. Adapter Surface

**Decision**: implement concrete adapters for filesystem, shell, Copilot CLI,
and optional MCP stdio. Govern them through typed capabilities and invocation
purposes.

**Rationale**: the engine must govern external tools, not replace them. The
chosen adapter set covers local-first execution without becoming a generic
integration marketplace.

**Alternatives considered**:

- direct shelling out everywhere: rejected because it would bypass capability
  tracking and trace quality
- broad plugin API: rejected because it optimizes for extensibility instead of
  governed clarity

## 7. Verification Separation

**Decision**: keep verification as a distinct artifact-producing phase, even
when the same AI surface is reused for self-critique.

**Rationale**: the system must preserve separation between generation and
validation. Self-critique is the cheapest layer, but bounded and systemic work
still requires adversarial and human review artifacts.

**Alternatives considered**:

- treat green tests as enough: rejected because tests can mirror the same wrong
  assumptions as the generator
- store only a final review summary: rejected because it hides which challenge
  layers actually ran

## 8. Platform Support Strategy

**Decision**: pin the contributor toolchain to Rust 1.94.0 with the requested
Linux targets, then stage Apple, Windows, and Linux ARM support in CI and
release jobs.

**Rationale**: this satisfies the immediate contributor setup requirement while
avoiding premature cross-target installation on every machine. The release
process, not the local toolchain file alone, is the right place to prove
cross-platform support.

**Alternatives considered**:

- add every target to `rust-toolchain.toml` immediately: rejected because it
  imposes cross-platform cost before release automation is in place
- claim support for fewer platforms: rejected because the product contract
  explicitly includes macOS, Linux, and Windows on ARM and x86_64

## 9. Public Surface Area

**Decision**: treat the CLI contract and runtime filesystem contract as the two
public surfaces of v0.1.

**Rationale**: this keeps the external identity of the tool concrete and
auditable. Internal modules remain refactorable so long as these contracts are
preserved.

**Alternatives considered**:

- expose a library API immediately: rejected because the primary product is a
  CLI, not an embedded SDK
- rely on undocumented file layout: rejected because persistence is part of the
  product value

