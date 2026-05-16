# Adaptive Governance V1 Contract

- **Owner**: Canon
- **Status**: Derived, non-normative brief
- **Normative Stable Paths**: `docs/integration/governance-adapter.md` and the delivered Canon governance semantics guide
- **Required Baseline Relationship**: `authority-governance-v1` remains the required S3 posture contract
- **Optional Companion Contract Line**: `adaptive-governance-v1`

## Purpose

Summarize the Canon-owned S4 adaptive-governance companion semantics for
downstream consumers without turning Canon into a runtime orchestrator.

## Authority Rule

- This brief mirrors the planned stable Canon contract and exists for feature-local planning and validation.
- If this brief and delivered Canon docs diverge, the stable Canon docs win.
- Downstream runtimes remain responsible for confidence, trust, councils, degradation, escalation, and stop behavior.

## Machine-Readable Carrier

When Canon emits machine-readable S4 companion semantics, they are carried as a
typed `adaptive_governance` object adjacent to the required
`authority_governance` baseline.

Example shape:

```json
{
  "authority_governance": {
    "contract_line": "authority-governance-v1",
    "authority_zone": "yellow",
    "change_class": "bounded-impact",
    "intended_persona": "system-architect",
    "approval_state": "not_needed",
    "packet_readiness": "reusable",
    "risk": "bounded-impact"
  },
  "adaptive_governance": {
    "contract_line": "adaptive-governance-v1",
    "governance_state": "advisory",
    "rollout_profile": "guided"
  }
}
```

## Required Field Profile

If the companion object is present, consumers must be able to recover these
required fields:

- `contract_line`
- `governance_state`
- `rollout_profile`

## Optional Additive Metadata

The first companion contract line may also include:

- `state_rationale`
- `profile_rationale`

Missing optional metadata does not invalidate an otherwise compatible companion
contract.

## Pairing Rules

- `authority-governance-v1` remains the required posture baseline.
- `adaptive-governance-v1` is optional and additive.
- Consumers may use the baseline without the companion.
- Consumers must not treat the companion as a replacement for the baseline.

## Compatibility Rules

- Unsupported companion contract lines fail closed for companion semantics.
- Missing required companion fields make the companion semantics unavailable.
- Missing optional companion fields leave the compatible remainder of the companion usable.
- Unknown optional metadata may be ignored by older consumers.
- Missing required `authority-governance-v1` baseline semantics still invalidate the required posture even if a companion object is present.

## Explicit Exclusions

- runtime confidence scores
- trust evolution state
- council profiles
- reviewer assignments
- provider routes
- model routes
- degradation mode selection
- escalation target selection
- stop-transition policy
- final decision authority for downstream runtimes