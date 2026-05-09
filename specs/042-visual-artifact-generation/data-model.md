# Data Model: Pragmatic C4 Architecture Packets And Visual Artifacts

## Architecture Documentation Packet

- **Purpose**: Represents the full publishable architecture handoff bundle for one governed `architecture` run.
- **Fields**:
  - `run_id`: Canon run identifier for traceability.
  - `primary_document`: The required `architecture-overview.md` handoff file.
  - `supporting_documents`: Markdown artifacts such as decision, context-map, system-context, container-view, and deployment coverage files.
  - `visual_views`: Included visual view artifacts with their available formats.
  - `view_manifest`: Machine-readable manifest describing included and omitted views.
  - `packet_metadata`: Existing publish metadata linking back to `.canon/artifacts/...` sources.
- **Validation rules**:
  - The packet must expose one primary document.
  - Supporting documents remain traceable to governed artifacts.
  - Omitted views must be recorded explicitly rather than disappearing silently.

## Visual View Artifact

- **Purpose**: Represents one architectural view in the packet, such as System Context, Container, Deployment, Component, or Dynamic.
- **Fields**:
  - `view_kind`: Canonical view identifier.
  - `status`: Included, omitted, or degraded.
  - `justification`: Why the view exists, was omitted, or was downgraded.
  - `markdown_document`: Optional human-readable markdown file for the view.
  - `mermaid_source`: Optional `.mmd` source file.
  - `rendered_assets`: Zero or more rendered outputs such as SVG or PNG.
  - `confidence_notes`: Evidence limits for the view.
- **Validation rules**:
  - Included views must expose at least one durable artifact.
  - A Mermaid source is required for every included visual diagram view.
  - Rendered assets are optional and capability-dependent.

## View Coverage Manifest

- **Purpose**: Machine-readable summary of which architecture views are present, omitted, or degraded.
- **Fields**:
  - `run_id`: Source run id.
  - `primary_document`: Relative path to the primary overview file.
  - `required_views`: The baseline pragmatic set expected for the packet.
  - `included_views`: Array of included view descriptors.
  - `omitted_views`: Array of omitted view descriptors with reasons.
  - `available_formats`: Mapping from view kind to emitted formats.
  - `rendering_capabilities`: Notes about SVG or PNG generation availability.
- **Validation rules**:
  - Required baseline views must appear either in `included_views` or `omitted_views` with a reason.
  - Format claims must match emitted files.

## Render Capability Note

- **Purpose**: Explicitly records why a rendered asset is missing, skipped, or degraded.
- **Fields**:
  - `target_format`: `svg` or `png`.
  - `status`: available, skipped, unsupported, or degraded.
  - `reason`: Human-readable explanation.
  - `source_view`: The visual view this capability note belongs to.
- **Validation rules**:
  - A missing rendered asset must have an explicit reason whenever the packet claims the view itself was included.