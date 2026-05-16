# Governance Adapter Integration

Use this guide when an external orchestrator needs machine-stable control flow
from Canon.

Canon remains the governed packet runtime. Your orchestrator remains the
higher-level coordinator. The adapter is the machine-facing boundary around the
same runtime, not the higher-level orchestrator.

## Decision Rule

- Use `canon run`, `status`, `inspect`, `approve`, `resume`, and `publish`
  when a human is driving the repository directly.
- Use `canon governance` when a tool needs stable JSON for capabilities,
  start, or refresh decisions.

## Commands

```bash
canon governance capabilities --json
canon governance start --json < request.json
canon governance refresh --json < request.json
```

## Stable Response Fields

The `v1` surface returns flat JSON. External tools should rely on these fields:

| Field | Meaning |
| --- | --- |
| `status` | Canon's projected lifecycle status for the request or run |
| `approval_state` | Whether approval is unnecessary, requested, granted, rejected, or expired |
| `packet_readiness` | Whether the packet is pending, incomplete, reusable, or rejected |
| `reason_code` | Machine-readable explanation for blocked, gated, or failed outcomes |
| `run_ref` | Canon run identifier when a run exists |
| `packet_ref` | Canonical workspace-relative packet location under `.canon/artifacts/...` |
| `document_refs` | Canonical workspace-relative document refs that currently exist |
| `expected_document_refs` | Canonical workspace-relative document refs Canon expects for the packet |

The adapter returns canonical workspace-relative refs so downstream systems can
store or display stable pointers without scraping human CLI prose.

Governed packets may also carry `authority-governance-v1` in the adjacent
`packet-metadata.json` sidecar, and may also carry the optional
`adaptive-governance-v1` companion in the same metadata. Downstream consumers
should treat that sidecar as the stable source for Canon governance semantics
rather than inferring them from filenames or human prose.

## Start Request Envelope

For file-backed modes, the request body is JSON on stdin:

```json
{
  "request_kind": "start",
  "governance_attempt_id": "ga-start-001",
  "stage_key": "analysis",
  "goal": "Create a governed packet",
  "workspace_ref": "/absolute/path/to/workspace",
  "mode": "change",
  "system_context": "existing",
  "risk": "bounded-impact",
  "zone": "yellow",
  "owner": "change-owner",
  "input_documents": [
    { "path": "canon-input/change.md" }
  ]
}
```

`workspace_ref` must bind to the current Canon workspace. `input_documents`
and `bounded_context.stage_brief_ref` resolve to workspace-relative document
refs inside that workspace.

## Response Example

When Canon produces a reusable packet, the adapter returns a projection like:

```json
{
  "adapter_schema_version": "v1",
  "status": "governed_ready",
  "approval_state": "not_needed",
  "message": "run `R-20260503-abc123` produced a reusable governed packet",
  "run_ref": "R-20260503-abc123",
  "packet_ref": ".canon/artifacts/R-20260503-abc123/change",
  "document_refs": [
    ".canon/artifacts/R-20260503-abc123/change/change-summary.md"
  ],
  "expected_document_refs": [
    ".canon/artifacts/R-20260503-abc123/change/change-summary.md"
  ],
  "packet_readiness": "reusable"
}
```

When the request is blocked or approval-gated, Canon keeps the same flat shape
and adds a machine-readable `reason_code`.

## Authority Governance Sidecar

Canon publishes first-slice authority semantics in `packet-metadata.json` as a
typed `authority_governance` object.

Representative shape:

```json
{
  "authority_governance": {
    "contract_line": "authority-governance-v1",
    "authority_zone": "yellow",
    "change_class": "systemic-impact",
    "intended_persona": "system-architect",
    "approval_state": "granted",
    "packet_readiness": "reusable",
    "risk": "systemic-impact",
    "primary_artifact": "01-architecture-summary.md",
    "artifact_order": ["01-architecture-summary.md"]
  }
}
```

Required fields:

- `contract_line`
- `authority_zone`
- `change_class`
- `intended_persona`
- `approval_state`
- `packet_readiness`
- `risk`

Optional additive fields:

- `persona_anti_behaviors`
- `primary_artifact`
- `artifact_order`
- `promotion_refs`
- `stage_role_hints`

Compatibility rules:

- Unsupported contract lines fail closed for consumers.
- Missing required fields make the authority semantics unavailable without
  making the packet unreadable.
- Missing optional fields leave the compatible remainder usable.
- Unknown optional fields are additive by default.

## Adaptive Governance Companion

Canon may also publish first-slice adaptive semantics in `packet-metadata.json`
as a typed `adaptive_governance` object beside the required
`authority_governance` baseline.

Representative shape:

```json
{
  "adaptive_governance": {
    "contract_line": "adaptive-governance-v1",
    "governance_state": "advisory",
    "rollout_profile": "guided"
  }
}
```

Required companion fields:

- `contract_line`
- `governance_state`
- `rollout_profile`

Optional additive fields:

- `state_rationale`
- `profile_rationale`

Compatibility rules:

- `authority-governance-v1` remains the required baseline.
- `adaptive-governance-v1` is optional unless the downstream runtime requires a
  compatible companion for a given stage.
- Missing or unsupported companion semantics must remain distinguishable from a
  missing required baseline.
- An optional companion cannot repair missing required authority semantics.

This boundary is intentional: Canon publishes semantic maturity labels, while
downstream runtimes decide confidence, trust, degradation, escalation,
councils, and stop behavior.

## Example: change

Use `mode: "change"` with a bounded authored brief such as
`canon-input/change.md` or `canon-input/change/brief.md`. A bounded-impact
change run is the normal happy path for an orchestrator that already knows the
target surface and wants Canon to govern packet production.

## Example: implementation

Use `mode: "implementation"` with the current feature brief as the
authoritative input and any carry-forward context kept as supporting files in
the same packet folder. The adapter still returns the same runtime-level
projection fields such as `status`, `approval_state`, `packet_readiness`, and
workspace-relative refs.

## Example: verification

Use `canon governance refresh --json < request.json` with a prior `run_ref`
when an external tool needs to re-check whether a verification run now exposes
reusable packet refs, approval requirements, or blocker details.

## Example: pr-review

Keep the boundary explicit: `pr-review` remains a diff-driven mode, not a
file-backed authored packet. The current `v1` governance request envelope is
best suited to workspace-relative document refs, so do not pretend that Canon
is the higher-level orchestrator by synthesizing fake file inputs for
`pr-review`. If your system already owns base or head ref selection, keep that
selection logic above Canon and treat the adapter contract as the same runtime
boundary rather than a workflow engine.

## Human vs Machine Boundary

- Canon owns governed run state, approvals, evidence, packet projection, and
  durable refs under `.canon/`.
- Your orchestrator owns the higher-level sequencing, retries, and any product-
  specific stage model above Canon.
- Canon is not a generic agent framework. It exposes one governed runtime that
  can be driven either by the human CLI or by this machine-facing adapter.