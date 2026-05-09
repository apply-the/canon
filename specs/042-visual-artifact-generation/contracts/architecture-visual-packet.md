# Contract: Architecture Visual Packet

## Purpose

Define the intended publish surface for the pragmatic architecture packet.

## Required Published Outputs

A publishable `architecture` run MUST produce a directory containing at least:

- `architecture-overview.md`
- `architecture-decisions.md`
- `context-map.md`
- `system-context.md`
- `container-view.md`
- one deployment coverage artifact or an explicit omission record
- `view-manifest.json`
- `packet-metadata.json`

## Optional Published Outputs

When justified by the authored packet and supported by the environment, the same published directory MAY also contain:

- `component-view.md`
- `dynamic-view.md`
- `system-context.mmd`
- `container-view.mmd`
- `deployment-view.mmd`
- `component-view.mmd`
- `dynamic-view.mmd`
- rendered `*.svg` assets for included views
- rendered `*.png` assets for included views

## Behavioral Rules

- `architecture-overview.md` is the primary human-readable review entrypoint.
- Mermaid source files are the canonical machine-readable diagram artifacts.
- Rendered SVG or PNG files are optional and MUST never be implied when they were not emitted.
- `view-manifest.json` MUST describe included views, omitted views, available formats, and capability notes for rendered assets.
- Omitted Component or Dynamic views MUST be recorded explicitly in `view-manifest.json` and referenced from `architecture-overview.md`.
- The existing publish flow remains authoritative: all published files are copied from governed artifacts under `.canon/artifacts/...`.

## Governance Rules

- The packet MUST remain honest about weak or unsupported deployment, component, or dynamic evidence.
- No rendered asset may be emitted if the source Mermaid artifact is missing.
- The primary overview document MUST not replace the governed supporting artifacts; it layers on top of them.