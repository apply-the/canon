# Decision Log: Governed Reasoning Posture v2

## Purpose

Record finalized implementation decisions for the `governed_reasoning_posture_v2`
feature so semantic changes do not live only in chat, code, or review comments.

## Required Decision Topics

- v1/v2 migration
- active versus legacy contract lines
- profile selector shape
- confidence handoff shape
- provenance shape
- independence shape
- release metadata compatibility
- rejection behavior

## Entries

### D001: v2 Is A New Contract Line, Not A v1 Patch

- Decision: publish `governed_reasoning_posture_v2` as a new stable line and
	keep `governed_reasoning_posture_v1` frozen as legacy context.
- Reason: the semantic delta is breaking because selector shape, independence,
	confidence handoff, provenance, and compatibility windows all become typed
	fail-closed subcontracts.

### D002: Dual-Line Publication Uses One Active Line And One Legacy Line

- Decision: mixed `v1`/`v2` publication is valid only when exactly one line is
	`active` and the other is explicitly `legacy`.
- Reason: the reviewer and consumer need one authoritative line; implicit
	fallback and dual-active publication make the release state ambiguous.

### D003: Selector Shape Is Explicit And Single-Branch

- Decision: every `v2` payload publishes `profile_selector.selector_kind` plus
	exactly one of `required_profile_family` or `required_profile_id`.
- Reason: `v1` left branch semantics too implicit; `v2` must reject both-present
	and neither-present states without consumer-side precedence rules.

### D004: Independence Is Split Into Hard Minima And Optional Guidance

- Decision: `minimum_independence` now separates required `hard_minima` from
	optional `guidance`.
- Reason: reviewers and consumers need to distinguish non-negotiable contract
	requirements from advisory strengthening without reading implementation code.

### D005: Confidence Handoff Remains Explicit Even When Not Required

- Decision: `confidence_handoff` is always present and publishes a typed
	`state`, with required subfields only when the state is `required`.
- Reason: omission is ambiguous; explicit none-versus-required state allows
	deterministic failure on contradictory or incomplete handoff data.

### D006: Provenance Is Typed And Checked Against Confidence Handoff

- Decision: `provenance` always carries a typed `state` plus stable
	`reference_kind` values, and required handoff must pair with
	evidence-backed provenance.
- Reason: `v2` must reject stale, contradictory, or under-specified evidence
	instead of inferring provenance from release metadata or contract line alone.

### D007: Compatibility Window Is Part Of The Contract Surface

- Decision: `compatibility_window` is a required typed block and the active
	`v2` window is Boundline `0.63.x` with Canon `0.64.x`.
- Reason: release metadata and stable docs must carry the same truth surface,
	so stale or contradictory version claims fail closed in contract validation.

### D008: Fixture Outcomes Are Contract Evidence

- Decision: the executable fixture corpus under
	`tests/fixtures/governed_reasoning_posture_v2/` is part of the contract
	surface, including valid, malformed, release-drift, coexistence, and
	migration-rejection cases.
- Reason: the reviewer must be able to confirm semantics from repository
	artifacts alone, and CI needs deterministic expected reasons for every reject
	path.