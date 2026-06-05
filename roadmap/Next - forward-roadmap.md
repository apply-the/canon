# Canon Forward Roadmap

## Where Canon Is Today

Canon 0.64.0 governs packet-based workflows across 18 first-class modes
covering the full lifecycle from discovery through operations. What the runtime
does well: bounded packet authoring, evidence-linked approval, fail-closed
validation surfaces, and release-aware contract publication.

What the runtime does not yet do well:

- It trusts agent completion claims without demanding fresh proof.
- It has no disciplined reproduction-first debugging path.
- It lacks a standard progress and handoff schema for long-running external execution.
- It offers no divergent ideation space before formal convergence.
- It treats policy evolution as a generic document change.
- It plans observability only after incidents, never before deployment.
- It does not capture Canon-local publish and handoff routing during guided setup.

This roadmap exists to close those gaps in a deliberate order.

---

## Prioritization Logic

Features are ordered by compounding impact on the daily delivery loop:

1. Items that improve trust in existing packet completion come first, because
   they reduce rework across every other mode.
2. Items that add missing lifecycle phases come next, because they fill semantic
   gaps that currently force users into generic workarounds.
3. Items that extend the product surface come last, because they serve
   onboarding rather than governance correctness.

Within each tier, the feature that unblocks the most downstream value ranks
higher.

---

## Feature Categories

### Tier 1: Runtime Integrity (trust in what Canon already does)

These features do not add new packet types. They harden the existing runtime so
that completion, readiness, and verification claims are provably true.

| # | Feature | Type | Unlocks |
|---|---------|------|---------|
| 02 | [Completion Verification Gates](features/02-completion-verification-gates.md) | Cross-mode runtime rule | Fresh proof before any success claim |

### Tier 2: Missing Lifecycle Phases (capabilities Canon lacks)

These features introduce new semantic packet types or governed metadata contracts that
the current mode surface cannot express without workarounds.

| # | Feature | Type | Unlocks |
|---|---------|------|---------|
| 03 | [Plan Progress and Handoff Schemas](features/03-plan-execution-orchestration.md) | Governed metadata contract | Standard progress and handoff packets for external orchestrators |
| 06 | [Observability Design](../specs/071-observability-design/feat-observability-design.md) | New mode | Proactive telemetry contracts before deployment |

### Tier 3: Product Surface (extend Canon's reach)

These features improve operator experience and external integration but do not
change governance semantics.

| # | Feature | Type | Unlocks |
|---|---------|------|---------|
| 07 | [Interactive Publish and Handoff Routing](features/07-interactive-integration-onboarding.md) | Local setup feature | Guided Canon-local routing in `canon init` |

---

## Sequencing and Dependencies

```
  ┌─────────────────────────────────────────────────────────────────────┐
  │ Tier 1: Runtime Integrity                                           │
  │                                                                     │
  │  02 Completion Verification Gates                                   │
  └─────────────────────────────────────────────────────────────────────┘
            │
            ▼
  ┌─────────────────────────────────────────────────────────────────────┐
  │ Tier 2: Missing Lifecycle Phases                                    │
  │                                                                     │
  │  03 Progress/Handoff ──── requires 02 (evidence_ref contract)       │
  │  06 Observability Design ─ independent, can start in parallel       │
  └─────────────────────────────────────────────────────────────────────┘
            │
            ▼
  ┌─────────────────────────────────────────────────────────────────────┐
  │ Tier 3: Product Surface                                             │
  │                                                                     │
  │  07 Publish/Handoff Routing ───── independent local setup           │
  └─────────────────────────────────────────────────────────────────────┘
```

Key sequencing rules:

- **02 before 03**: progress and handoff packets need a stable
  `claim -> proof -> evidence_ref` contract rather than vague completion text.
- **06 stays independent**: it addresses lifecycle gaps and does not
  share internal dependencies.
- **07 stays low priority**: it improves local setup and routing intent, not core packet semantics.

---

## What Stays Out Of Scope

- No new distribution channel work in this cycle (Homebrew, Scoop, Winget are
  already shipping).
- No MCP server implementation, provider discovery, provider health checks,
  secret routing, or connectivity dry-runs in Canon.
- No Boundline runtime orchestration in Canon; 03 may define handoff and
  progress schemas, but task locking, dispatch, checkpointing, and resume stay
  runtime-owned.
- No rewrites of the existing 18 first-class modes; new modes may be introduced if they cover missing lifecycle phases, and enrichment happens through
  runtime rules (02) rather than mode-level rewrites.
- No AI model routing, prompt engineering, or provider abstraction; Canon
  governs packets, not inference.

---

## Technical Debt

No formal technical debt tracked at this time. Future items will be filed
in this directory with prefix `TD-<n>` and indexed below as they are
identified.

---

## How To Propose A New Feature

1. Draft a feature file under `roadmap/features/` following the established
  structure: Problem, Proposal, Risk Profile, Why Existing Modes Are Not
  Enough, Dependencies, Related Modes, Entry Gates, Operational Mechanics,
  Exit Gates, Packet Shape, and Success Criteria.
2. Determine the correct tier and sequencing position.
3. Update this index document with the new entry.
4. If the feature is substantial enough to warrant a full Speckit lifecycle,
   create a numbered `specs/` directory and run the governed packet flow.
