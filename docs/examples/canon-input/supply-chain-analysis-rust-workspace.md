# Supply Chain Analysis Brief: Rust Workspace Dependency Posture Review

System Surface: The Canon Rust workspace dependency, licensing, and release surface.
Primary Upstream Mode: direct
Upstream Sources:
- Cargo.toml
- deny.toml
- scripts/validate-canon-skills.sh
Carried-Forward Decisions:
- Canon remains a local-first CLI and does not auto-remediate dependencies.
- Existing release validation already depends on `cargo fmt`, `cargo clippy`, and workspace tests.
Excluded Upstream Scope: GitHub Actions marketplace actions, published release archives, and unrelated external tooling repos remain out of scope.

## Declared Scope
- Assess the Rust workspace dependency posture for the Canon repository itself.

## Licensing Posture
- Mixed: the repository is open source, but the packet must still expose obligations and compatibility posture for shipped dependencies.

## Distribution Model
- Distributed as prebuilt CLI binaries and source releases.

## Ecosystems In Scope
- Rust workspace dependencies from `Cargo.toml`.
- Rust advisory and license posture from `deny.toml`.

## Out Of Scope Components
- Editor extensions and marketplace-hosted dependencies not vendored into this repository.
- CI service configuration outside the repo-local workflow files.

## Scanner Selection Rationale
- Use Rust-native dependency and advisory tooling first because the repo is a Rust workspace.
- Use a license-policy tool to evaluate compatibility and obligations against the declared release posture.

## SBOM Outputs
- Emit a machine-readable SBOM for the workspace crates and reference it from the packet.

## Findings By Severity
- No critical findings are yet confirmed from the authored input alone.
- Advisory and outdated-dependency results must be pulled from the scanner outputs before the packet can claim clean posture.

## Exploitability Notes
- Dependency findings affecting CLI parsing, artifact persistence, or shell invocation have the highest operational relevance.

## Triage Decisions
- Hold final triage disposition until scanner outputs are available.
- Treat missing scanner coverage as an explicit packet gap rather than an accepted clean bill of health.

## Compatibility Classes
- Core Rust crate dependencies are expected to remain compatible with Canon's current release posture, subject to scanner confirmation.

## Flagged Incompatibilities
- None confirmed yet from the authored input alone.

## Obligations
- Preserve upstream license notices and attribution obligations for shipped dependencies.

## Outdated Dependencies
- Review dependencies that lag materially behind current stable releases when they affect CLI safety, persistence, or shell execution.

## End Of Life Signals
- No EOL findings are confirmed yet from the authored input alone.

## Abandonment Signals
- No abandonment findings are confirmed yet from the authored input alone.

## Modernization Slices
- Prioritize bounded updates for dependencies that touch CLI parsing, evidence persistence, or governed shell execution.

## Scanner Decisions
- Prefer OSS-native Rust tooling for the first slice.
- If a required scanner is missing, record whether it was skipped or replaced before the packet is treated as complete.

## Coverage Gaps
- Advisory, license, and outdated-dependency conclusions remain provisional until the required scanner outputs are attached.

## Source Inputs
- Cargo.toml
- deny.toml
- scripts/validate-canon-skills.sh

## Independent Checks
- cargo test --test direct_runtime_coverage
- cargo clippy --workspace --all-targets --all-features -- -D warnings

## Deferred Verification
- Record the final scanner outputs, triage decisions, and SBOM references in the emitted packet before treating the workspace supply-chain posture as review-ready.