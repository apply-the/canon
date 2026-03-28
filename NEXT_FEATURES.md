# Next Features

This file captures candidate product features that are intentionally beyond the
current implementation slice.

## Feature: Distribution Without Cargo

### Outcome

Users can install Canon from prebuilt binaries without needing a local Rust or
Cargo toolchain.

### First Slice

- Publish canonical release artifacts through GitHub Releases.
- Use `cargo-dist` to build release assets and installer scripts.
- Support Homebrew as the first package-manager channel for macOS and Linux.
- Support `winget` as the primary Windows package-manager channel.
- Support Scoop as a secondary Windows channel.

### Deferred

- `apt` or Debian packaging, until Canon has a stable release and repository
  publishing process worth maintaining.

### Why This Feature

- GitHub Releases should be the canonical source of downloadable binaries.
- `cargo-dist` reduces cross-platform release plumbing.
- Homebrew gives the strongest first install story on macOS and a viable one on
  Linux.
- `winget` is the most credible first-class Windows distribution target.
- Debian packaging adds maintenance cost too early.

## Feature: Protocol Interoperability

### Outcome

Canon remains protocol-agnostic at the core while gaining a practical path to
govern structured external-tool invocation and future interoperable exposure.

### Direction

- Canon core must remain protocol-agnostic.
- First protocol adapter: MCP consumer support.
- Future external surface: a minimal MCP server.
- A2A remains a later architectural option, not a near-term implementation
  priority.

### First Slice

- Treat MCP as the first protocol Canon actively consumes for governed external
  tool invocation.
- Route MCP requests through the same request, decision, approval, and evidence
  pipeline used by other execution adapters.
- Keep policy, capability typing, lineage, and verification semantics shared
  across protocols.
- Only ship MCP runtime behavior if it fits the common invocation pipeline
  cleanly, without introducing protocol-specific special cases into the core.

### Future Minimal MCP Server Surface

- Start a governed run.
- Inspect run state and evidence.
- Request a structured review.
- Avoid exposing the full internal engine surface.

### Explicitly Deferred

- A2A support, unless Canon needs to operate as a remote interoperable agent.
- Agent discovery, remote delegation, and network-visible multi-agent topology.
- Broad protocol exposure beyond narrowly governed capabilities.

### Why This Order

- MCP solves tool and resource interoperability, which matches Canon's current
  need to govern calls to external tools.
- A2A solves agent-to-agent interoperability, which is a different layer of the
  system.
- Canon is currently closer to a governed local execution runtime than to a
  remote agent platform.
- Supporting MCP first strengthens the execution layer without forcing premature
  multi-agent architecture.

### Planning Principle

MCP first, A2A later if Canon becomes a network-visible agent.
