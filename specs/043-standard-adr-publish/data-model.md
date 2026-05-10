# Data Model: Standard ADR Publish Artifacts

## Entity: ADR Registry Entry

- **Purpose**: Represents one durable repository-local ADR published from a governed Canon packet.
- **Fields**:
  - `identifier`: sequential ADR number rendered in `ADR-XXXX` form.
  - `slug`: stable filename suffix derived from the packet title or descriptor.
  - `title`: human-readable ADR title.
  - `date`: publish date recorded in the ADR body.
  - `status`: lifecycle state for this slice, bounded to `Accepted`.
  - `context`: standard ADR context section synthesized from the source packet.
  - `decision`: standard ADR decision section synthesized from the source packet.
  - `consequences`: standard ADR consequences section synthesized from the source packet.
  - `alternatives_considered`: optional standard extension section synthesized when the source packet contains explicit alternatives.
  - `source_mode`: originating Canon mode.
  - `source_run_id`: originating Canon run identifier.
  - `source_packet_path`: published packet directory or override path associated with the source publish.
  - `source_artifacts`: list of source packet artifact paths used to synthesize the ADR.
- **Validation rules**:
  - `identifier` must be unique within `docs/adr/`.
  - `status` must be `Accepted` in this slice.
  - `source_mode` must be one of `architecture`, `change`, or `migration`.
  - `context`, `decision`, and `consequences` must preserve explicit missing-context markers rather than being replaced with fabricated filler.

## Entity: ADR Publish Policy

- **Purpose**: Encodes mode-specific ADR export eligibility and whether export is automatic, opt-in, or unsupported.
- **Fields**:
  - `mode`: Canon mode name.
  - `behavior`: one of `default`, `opt-in`, or `unsupported`.
  - `registry_path`: fixed ADR registry destination.
  - `status_default`: lifecycle state assigned to generated ADRs.
- **Validation rules**:
  - `architecture` maps to `default`.
  - `change` and `migration` map to `opt-in`.
  - all other modes map to `unsupported`.
  - `registry_path` remains `docs/adr/` regardless of packet `--to` destination overrides.

## Entity: ADR Source Mapping

- **Purpose**: Captures how Canon maps existing packet artifacts into the standard ADR sections.
- **Fields**:
  - `source_mode`: mode being projected.
  - `context_sources`: ordered list of packet artifacts or sections used to build ADR context.
  - `decision_sources`: ordered list of packet artifacts or sections used to build ADR decision.
  - `consequence_sources`: ordered list of packet artifacts or sections used to build ADR consequences.
  - `alternatives_sources`: ordered list of packet artifacts or sections used for optional alternatives coverage.
  - `honesty_rules`: missing-context preservation rules inherited from the source packet.
- **Validation rules**:
  - mappings must be deterministic for a given mode.
  - unsupported modes have no mapping.
  - missing sections do not short-circuit publish silently; they surface the original packet honesty markers in the synthesized ADR text.

## State Transitions

- A publishable supported packet starts as a normal Canon publish candidate.
- ADR policy evaluation classifies the run as `default`, `opt-in`, or `unsupported`.
- If the mode is `default`, ADR synthesis runs automatically.
- If the mode is `opt-in`, ADR synthesis runs only when the CLI request explicitly asks for ADR export.
- If the mode is `unsupported` and ADR export was requested, publish fails with a validation error.
- Once written, the ADR registry entry is immutable within this slice; future lifecycle changes require a new ADR or follow-on workflow.