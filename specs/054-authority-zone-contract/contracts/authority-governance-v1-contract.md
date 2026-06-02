# Authority Governance V1 Contract

- **Owner**: Canon
- **Status**: Derived, non-normative brief
- **Normative Stable Paths**: `tech-docs/integration/governance-adapter.md` and the delivered Canon personas or authority-zones guide
- **Current Contract Line**: `authority-governance-v1`

## Purpose

Summarize the Canon-owned `authority-governance-v1` surface for downstream
consumers without turning Canon into a runtime orchestrator.

## Authority Rule

- This brief mirrors the stable Canon contract and exists for feature-local
  planning and validation.
- If this brief and the delivered Canon integration docs diverge, the stable
  Canon docs win.
- Downstream runtimes remain responsible for councils, runtime roles, and stop
  transitions.

## Machine-Readable Carrier

The contract is carried as a typed `authority_governance` object in governed
packet metadata and may also be projected through the machine-facing governance
adapter when packet metadata is available.

Example shape:

```json
{
  "authority_governance": {
    "contract_line": "authority-governance-v1",
    "authority_zone": "yellow",
    "change_class": "bounded-impact",
    "intended_persona": "system-architect",
    "approval_state": "requested",
    "packet_readiness": "incomplete",
    "risk": "bounded-impact"
  }
}
```

## Required Field Profile

Consumers must be able to recover these required fields:

- `contract_line`
- `authority_zone`
- `change_class`
- `intended_persona`
- `approval_state`
- `packet_readiness`
- `risk`

## Optional Additive Metadata

The first contract line may also include:

- `persona_anti_behaviors`
- `primary_artifact`
- `artifact_order`
- `promotion_refs`
- `stage_role_hints`

Missing optional metadata does not invalidate an otherwise compatible contract.

## Compatibility Rules

- Unsupported contract lines fail closed for consumers.
- Missing required fields make the authority contract unavailable.
- Missing optional fields leave the compatible remainder of the contract usable.
- Unknown optional metadata may be ignored by older consumers.

## Explicit Exclusions

- runtime council profiles
- reviewer assignments
- provider routes
- model routes
- retry policy
- stop-transition policy
- final decision authority for downstream runtimes