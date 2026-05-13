# Research: Logical Packet Ordering

## Decision 1: Canon-owned ordering registry

**Decision**: Drive packet ordering from a Canon-owned per-mode ordering registry rather than from alphabetical sort or implicit emitter order.

**Rationale**: Packet readability is a product contract. A dedicated ordering registry makes the intended reading sequence explicit for emitters, metadata, publish, and docs without asking downstream consumers to guess from filenames.

**Alternatives considered**:
- Reconstruct order from alphabetical filenames; rejected because it encodes implementation detail, not reading intent.
- Infer order only from emitted packet contents; rejected because optional artifacts and mode evolution would make ordering unstable.

## Decision 2: Metadata carries packet order explicitly

**Decision**: Extend packet metadata to record `primary_artifact` and `artifact_order`, with optional compatibility surfaces such as `legacy_aliases` and `publish_order` when needed.

**Rationale**: Status, inspect, publish, and downstream tooling need a stable machine-readable contract that survives future packet-shape changes. Declared ordering is more reliable than deriving order from paths or sort order.

**Alternatives considered**:
- Expose order only through filenames; rejected because summaries and integrations would still need heuristics.
- Store order in a separate manifest only for some modes; rejected because cross-mode consistency is part of the feature value.

## Decision 3: Legacy packets remain readable without rewrite

**Decision**: Preserve existing governed runs as historical artifacts and implement compatibility behavior for legacy artifact names instead of rewriting old packets.

**Rationale**: Canon artifacts are audit surfaces. Rewriting historical runs would create unnecessary governance risk and blur the distinction between legacy and current contracts.

**Alternatives considered**:
- Rewrite all prior packets in place; rejected because it mutates historical evidence.
- Ignore legacy packets and only support new ones; rejected because status, inspect, and review flows still need to resolve historical runs.

## Decision 4: Sidecars stay outside packet-body ordering

**Decision**: Treat packet sidecars such as `packet-metadata.json` and AI provenance artifacts as support material outside the ordered packet body unless Canon explicitly promotes them into the reading sequence.

**Rationale**: Readers need a clear narrative order for packet-body artifacts. Mixing sidecars into that sequence would make packet browsing noisier and weaken the primary-artifact contract.

**Alternatives considered**:
- Prefix all sidecars numerically; rejected because support files are not part of the primary reading path.
- Leave sidecar handling undefined; rejected because the feature must state why ordered artifacts and sidecars behave differently.

## Decision 5: 049 supersedes the narrower 046 draft

**Decision**: Treat `049-logical-packet-ordering` as the authoritative feature for packet ordering and record that it broadens the older `046-ordered-artifact-filenames` draft.

**Rationale**: The new scope includes metadata, publish and summary behavior, and documentation semantics beyond filename prefixing alone. A clear supersession note prevents future planning drift.

**Alternatives considered**:
- Mutate 046 in place; rejected because the user explicitly requested a new spec.
- Keep 046 and 049 independent; rejected because the overlap would create conflicting packet-ordering directives.