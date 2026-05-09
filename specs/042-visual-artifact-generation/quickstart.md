# Quickstart: Pragmatic C4 Architecture Packets And Visual Artifacts

## Scenario 1: Publish a small-system architecture packet

1. Author `canon-input/architecture.md` for a bounded system that clearly describes System Context and Containers but does not justify deeper views.
2. Run Canon in `architecture` mode until the run becomes publishable.
3. Publish the packet.
4. Verify the published directory exposes:
   - `architecture-overview.md` as the first reviewable document,
   - `system-context.md` and `container-view.md`,
   - deployment coverage or an explicit omission reason,
   - `view-manifest.json`,
   - Mermaid files for every included visual diagram.
5. Confirm the packet does not emit unnecessary component or dynamic artifacts.

## Scenario 2: Publish a complex architecture packet

1. Author an `architecture.md` brief with justified internal complexity or asynchronous interaction flow.
2. Run and publish the packet.
3. Verify `view-manifest.json` records the included deeper views and the formats emitted for them.
4. Confirm the primary overview document explains why the extra depth was included.

## Scenario 3: Render-capability fallback

1. Execute the packet flow in an environment where Mermaid source generation is available but SVG or PNG rendering is unavailable.
2. Publish the packet.
3. Verify the packet still exposes Mermaid source files and marks rendered assets as unsupported or skipped instead of claiming they exist.

## Validation Commands

```bash
cargo test --test architecture_c4_renderer --test architecture_c4_run
cargo test --test architecture_c4_contract
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info
```