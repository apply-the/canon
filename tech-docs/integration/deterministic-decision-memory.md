# Deterministic decision memory

Canon's decision-memory kernel validates governance data and records exact
dependencies. It does not run models, providers, semantic reviewers, network
clients, or subprocesses.

## Boundary

The public `canon-contracts 0.90.0` types remain the exchange format.
Repository-local metadata binds the public bundle to packet digests, packet
and decision-memory revisions, exact subject-artifact revisions and digests,
claims, evidence, approvals,
verification requirements, authority, risk acceptances, assumptions,
alternatives, triggers, owners, freshness, and the terminal deterministic
result. Internal metadata does not add public wire fields.

An accepted result means that deterministic structure, exact bindings,
authority, evidence policy, and freshness checks passed. It does not assert
the semantic truth of externally supplied findings.

## Validation phases

Validation uses the following order:

1. `parse`
2. `normalize`
3. `validate_structure`
4. `validate_cross_references`
5. `validate_authority`
6. `validate_evidence_requirements`
7. `validate_freshness`
8. `record_decision`
9. `project`

An earlier failure prevents later policy checks from producing success.
Terminal failure is still recorded as a typed decision.

## Identity, history, and freshness

Nodes have stable typed identities and versioned SHA-256 content digests.
Replaying the same identity and digest is idempotent. Reusing an identity with
different content is a conflict. Explicit supersession preserves the old node
and marks it stale.

Freshness propagation starts from an exact changed digest and traverses only
typed freshness, evidence, and authority edges. Direct and transitive
dependents become stale with a stable reason chain; unrelated branches remain
fresh. Restoring display text or adding replacement evidence does not revive
an old approval or decision. Bundle admission, deterministic validation,
successful propagation, and supersession share one ordered journal inside the
graph. Persisted freshness and the terminal decision can therefore be
reconstructed rather than trusted from mutable envelope fields. The terminal
snapshot must end with deterministic validation; a later mutation cannot
leave an earlier accepted result presented as current.

## Authority and evidence

Evidence cannot grant authority. Approval must match the exact packet,
subject-artifact set, claim set, evidence set, verification-requirement set,
authority zone, owner, decision-memory revision, validity range, and
freshness state. These references cannot be borrowed across packet
boundaries.

External semantic evidence remains externally produced. Canon reuses the
external-evidence validator to check exact claims, immutable digest
references, lineage, independent context, freshness, and challenge tier.
Shared lineage fails closed. A named Tier 2 degradation is accepted only when
it matches a fresh, justified risk acceptance owned by required authority.
Tier 3 has no automatic override.

## Persistence

The graph and its terminal deterministic result are one typed snapshot under
the existing Canon state root:

```text
.canon/decision-memory/state.json
```

The snapshot also retains the normalized admitted bundles needed to
reconstruct the normative repository topology. On persistence and load, Canon
revalidates each bundle digest, re-admits the bundles in order, recomputes
their deterministic decisions, replays the graph event journal in its exact
admission/validation/mutation order, and recomputes the terminal result.
Bundle root order must match journal admission order, and the reconstructed
graph and result must exactly match the snapshot. Supersession may advance only
between revisions of the same bundle. Canon writes the temporary
file in the same directory, flushes file content, atomically replaces the
snapshot, and flushes the parent directory. Loading revalidates schema and
contract identity, graph/result digest equality, declared freshness, every
non-decision node, every base and terminal dependency edge, supersession,
dangling references, dependency cycles, and the recorded terminal decision.
It also rejects terminal records that claim semantic truth, external
execution, non-canonical phase order, or incoherent findings. This adds no
second state root and performs no outcome ingestion.
