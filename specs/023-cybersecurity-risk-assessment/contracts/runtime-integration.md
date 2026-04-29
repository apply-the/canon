# Contract: Runtime Integration For Security Assessment

## Runtime Surfaces

- The new mode name is `security-assessment`.
- Canonical authored input locations are `canon-input/security-assessment.md`
  and `canon-input/security-assessment/`.
- The publish destination is `docs/security-assessments/<RUN_ID>/`.
- The mode stays recommendation-only in the current release.

## Required Runtime Hooks

- Mode parsing and display in the core mode registry
- System-context validation and usage guidance
- Orchestrator dispatch and mode-specific service implementation
- Artifact contract selection and markdown rendering
- Gate evaluation using existing risk, architecture, and release-readiness
  semantics
- Publish path resolution and mode-result summarization
- Shared runtime compatibility references and helper scripts

## Test Expectations

- Contract test proving the artifact family is registered correctly
- Renderer test proving positive preservation and missing-body honesty behavior
- Run test proving the mode executes and persists artifacts
- Publish test proving packets land in the dedicated docs path
- Docs test proving skills and guidance surfaces stay synchronized

## Release Expectations

- Version surfaces report `0.22.0`
- `cargo fmt --check` passes
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` passes
- Focused security-assessment tests pass
- Full `cargo nextest run` passes