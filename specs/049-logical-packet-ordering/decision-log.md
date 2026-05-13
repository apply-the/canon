# Decision Log: Logical Packet Ordering

## D-001: Create a new feature line for logical packet ordering

**Date**: 2026-05-13  
**Decision**: Open `049-logical-packet-ordering` as a new feature rather than mutating `046-ordered-artifact-filenames` in place.  
**Rationale**: The new work includes metadata, publish behavior, status and inspect semantics, and documentation scope beyond filename prefixing alone.

## D-002: Every new packet has a single primary artifact

**Date**: 2026-05-13  
**Decision**: Require exactly one primary artifact per packet-emitting mode, and make it the `01-*` artifact for new packets.  
**Rationale**: Reviewers and downstream tools need one unambiguous starting point for packet interpretation.

## D-003: Packet ordering is explicit metadata, not reconstructed behavior

**Date**: 2026-05-13  
**Decision**: Represent packet order explicitly in metadata through `primary_artifact` and `artifact_order`, with optional compatibility fields when needed.  
**Rationale**: Canon should not force downstream consumers to guess packet semantics from path sort order.

## D-004: Historical runs are preserved, not rewritten

**Date**: 2026-05-13  
**Decision**: Keep historical packets readable through compatibility behavior instead of rewriting old governed runs.  
**Rationale**: Historical runs are evidence artifacts and should not be mutated solely to match a newer packet contract.

## D-005: Runtime sidecars remain outside the ordered packet body

**Date**: 2026-05-13  
**Decision**: Keep runtime sidecars such as `packet-metadata.json` and `view-manifest.json` outside the ordered packet-body sequence unless a future Canon contract declares otherwise.  
**Rationale**: Readers need a clean artifact narrative and should not have support files mixed into the reading path by default.

## D-006: Publish and summary surfaces follow packet metadata instead of slug heuristics

**Date**: 2026-05-13  
**Decision**: Merge runtime packet-order metadata into published `packet-metadata.json` and let summary surfaces resolve the primary artifact from ordered packet metadata when available.  
**Rationale**: Canon should carry one authoritative packet-order contract across runtime, publish, status, and inspect instead of preserving multiple filename heuristics.