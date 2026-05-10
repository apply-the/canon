# Feature Specification: Assistant Plugin Packages

**Feature Branch**: `044-assistant-plugin-packages`  
**Created**: 2026-05-10  
**Status**: Draft  
**Input**: User description: "Add host-specific assistant plugin packaging for Canon so its governed methods, skills, and command surfaces can be installed consistently across Claude Code, Codex, Cursor, Copilot, and future assistant hosts. Keep Canon as the governed packet runtime for AI-assisted engineering work, add host plugin manifests and package folders, avoid duplicating skill content, validate metadata/version/path alignment, update docs and README, bump the Canon version first, and close with 95% coverage on new or modified Rust files plus clean clippy, tests, and cargo fmt."

## Governance Context *(mandatory)*

**Mode**: change  
**Risk Classification**: bounded-impact, because this slice changes public assistant packaging metadata, documentation, validation, and version surfaces without redesigning Canon's runtime, approval semantics, governed packet model, or execution authority.  
**Scope In**:

- Add host plugin package folders for Claude Code, Codex, and Cursor using host-native manifest or metadata shapes where they can be represented cleanly.
- Add Copilot support only as a documented command and prompt pack unless a stable plugin manifest shape already exists in this repository.
- Add shared Canon-owned plugin metadata where useful to prevent manifest drift across host packages.
- Expose Canon's governed methods through host-facing commands or method bindings for clarify input, start governed packet, inspect status, inspect evidence, review packet, verify claims, and publish packet.
- Reference shared Canon skill and method source material from host packages rather than duplicating all skill content.
- Add validation that checks JSON manifests where applicable, required metadata fields, Canon version alignment, referenced paths, required commands or methods, and prohibited positioning language.
- Update installation and limitation documentation plus the README assistant plugin package summary.
- Bump Canon version surfaces from `0.43.0` to `0.44.0` before implementation tasks that depend on the new packaging surface.
- Close the feature with formatting, linting, tests, and at least 95% line coverage for every new or modified Rust source file touched by this slice.

**Scope Out**:

- Redesigning Canon's core governed packet runtime, `.canon/` artifact persistence, approval model, evidence model, provenance model, or governance adapter authority.
- Making any assistant host the source of truth for Canon behavior.
- Creating divergent Canon behavior per host.
- Duplicating full skill content into each host plugin package.
- Exposing private or unstable internals as plugin capabilities.
- Requiring Boundline or any other external project to use these assistant plugin packages.
- Claiming Canon is an agent framework, orchestrator, coding agent, or workspace mutation engine.

**Invariants**:

- Canon CLI and the machine-facing governance adapter MUST remain authoritative for governed packet behavior; host plugin packages are install, discovery, metadata, and glue surfaces only.
- Host-specific package folders MUST mostly contain manifests, metadata, assets, command bindings, prompts, and host glue while shared skill or method content stays in Canon-owned source directories.
- Plugin language MUST position Canon as a governed packet runtime for AI-assisted engineering work, with structured packets, evidence, approvals, and provenance.
- Plugin metadata MUST NOT imply that Canon executes code, orchestrates agents, bypasses approvals, or mutates workspaces on behalf of an assistant host.
- Validation MUST fail on version drift, missing required metadata, invalid JSON where applicable, missing referenced paths, missing required command or method exposure, or prohibited positioning language.

**Decision Traceability**: Design and implementation decisions for this slice will be recorded in `specs/044-assistant-plugin-packages/decision-log.md`, with validation evidence and closeout notes recorded in `specs/044-assistant-plugin-packages/validation-report.md`.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Install Canon Support For A Host (Priority: P1)

As a Canon user, I want to find a host-specific package folder and installation instructions for my assistant host so I can install Canon's governed methods consistently without guessing which files matter.

**Why this priority**: Install and discovery consistency is the core user value. Without host package folders and install documentation, the feature does not solve the current packaging gap.

**Independent Test**: Inspect the Claude Code, Codex, and Cursor package folders plus the installation guide and verify that each supported host has required metadata, a documented install path, and references to shared Canon skills or methods.

**Acceptance Scenarios**:

1. **Given** a user wants Canon support in Claude Code, Codex, or Cursor, **When** they inspect the assistant plugin documentation, **Then** they can identify the correct package folder, what it contains, and how to install or copy it.
2. **Given** a user wants Copilot support, **When** they inspect the assistant plugin documentation, **Then** they see a documented command and prompt pack boundary rather than an unstable plugin manifest claim.

---

### User Story 2 - Discover Governed Canon Capabilities Natively (Priority: P2)

As an assistant host user, I want the installed package to advertise Canon's governed methods, skills, starter prompts, and commands in host-native terms so I can start, inspect, review, verify, and publish packets without memorizing internal file layouts.

**Why this priority**: The package is only useful if hosts can expose Canon capabilities accurately and consistently after installation.

**Independent Test**: Validate each supported host package and confirm that it declares Canon identity, version, positioning, capabilities, required method or command surfaces, starter prompts where supported, and references to existing shared skills or methods.

**Acceptance Scenarios**:

1. **Given** a supported host package manifest, **When** validation checks its declared capabilities, **Then** the required governed method surfaces are present: clarify input, start governed packet, inspect status, inspect evidence, review packet, verify claims, and publish packet.
2. **Given** a supported host package manifest, **When** validation checks its metadata, **Then** the package identifies Canon as a governed packet runtime and links to repository, homepage, license, keywords, version, author, and supported paths without prohibited runtime claims.

---

### User Story 3 - Prevent Plugin Metadata Drift (Priority: P3)

As a Canon maintainer, I want automated validation for plugin manifests, shared metadata, path references, and version alignment so host packages remain coherent as Canon evolves.

**Why this priority**: Multi-host packaging becomes a liability if metadata, version values, or referenced paths drift from the Canon source of truth.

**Independent Test**: Run the plugin package validation command and confirm it fails for intentionally invalid manifest JSON, version drift, missing fields, missing paths, missing required commands, or prohibited positioning text, then passes for the repository package set.

**Acceptance Scenarios**:

1. **Given** a plugin manifest references a non-existent skills path, **When** validation runs, **Then** validation fails with the missing path and host package identified.
2. **Given** a plugin manifest version differs from the Canon workspace version, **When** validation runs, **Then** validation fails and identifies the drift.
3. **Given** plugin metadata describes Canon as an agent framework, orchestrator, coding agent, or workspace mutation engine, **When** validation runs, **Then** validation fails and identifies the prohibited wording.

### Edge Cases

- A host package format supports JSON manifests while another host package is documented metadata or command bindings only; validation must apply JSON parsing only where JSON files are present.
- Copilot has no stable repository-local manifest shape; the package surface must remain a documented command/prompt pack and not claim unsupported install automation.
- A package references shared Canon skills, methods, hooks, or command packs through relative paths; validation must resolve paths from the repository root and catch missing targets.
- A future host package is added later; shared metadata and validation should make the required fields and prohibited claims explicit enough to extend without copying rules by hand.
- A host-specific package needs extra metadata that another host does not support; host glue may differ, but Canon behavior and shared governed method definitions must not diverge.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The repository MUST provide `.claude-plugin/`, `.codex-plugin/`, and `.cursor-plugin/` package folders for supported assistant hosts.
- **FR-002**: The repository MUST provide Copilot support only as a documented command and prompt pack unless a stable Copilot plugin manifest shape exists in this repository.
- **FR-003**: Each supported host package MUST declare name, display name, version, description, author, homepage, repository, license, keywords, capabilities, and supported paths to skills, commands, hooks, method packs, or starter prompts where the host surface supports them.
- **FR-004**: Host package descriptions and capability text MUST use Canon-specific positioning such as "Governed packet runtime for AI-assisted engineering work", "Structured packets, evidence, approvals, and provenance", and "Governed methods for requirements, architecture, backlog, implementation, review, verification, incident, and migration".
- **FR-005**: Host package descriptions and metadata MUST NOT describe Canon as an agent framework, orchestrator, coding agent, or workspace mutation engine.
- **FR-006**: Host packages MUST expose or document the required method surfaces: clarify input, start governed packet, inspect status, inspect evidence, review packet, verify claims, and publish packet.
- **FR-007**: Host package folders MUST reference shared Canon-owned skill and method source paths rather than duplicating full skill content into every package folder.
- **FR-008**: Shared plugin metadata MUST be added when it reduces drift across host packages, and host manifests MUST align with it for common fields.
- **FR-009**: Validation MUST check JSON manifest parseability where applicable, required metadata fields, version alignment with the Canon workspace version, referenced path existence, required method surface exposure, and prohibited positioning language.
- **FR-010**: Documentation MUST explain supported hosts, installation steps, package contents, Copilot limitations, future host extension expectations, and the boundary that Canon CLI and the governance adapter remain authoritative.
- **FR-011**: README MUST include a short "Assistant plugin packages" section that points users to the installation guide and summarizes the supported host package folders.
- **FR-012**: Canon version surfaces MUST be bumped consistently to `0.44.0` as the first implementation task for this feature slice.
- **FR-013**: The final validation closeout for this feature MUST prove that every new or modified Rust source file touched by the slice reaches at least 95% line coverage and that `cargo fmt`, clippy, and the test suite are green.

### Key Entities *(include if feature involves data)*

- **Assistant Plugin Package**: A host-specific folder containing the manifest, metadata, command bindings, prompts, assets, or host glue that let an assistant host discover Canon capabilities.
- **Shared Plugin Metadata**: Canon-owned metadata used to align common package fields such as version, description, author, repository, license, keywords, and required capabilities.
- **Host Capability Binding**: A host-facing declaration that maps a native command, prompt, skill, hook, or method entry to a Canon governed method without changing Canon runtime behavior.
- **Package Validation Report**: Evidence showing each host package has valid metadata, aligned version values, existing path references, required method exposure, and no prohibited positioning claims.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A user can identify install instructions and package location for Claude Code, Codex, Cursor, and the documented Copilot command/prompt pack in under five minutes using repository docs.
- **SC-002**: 100% of supported host packages declare the required metadata fields and the required Canon governed method surfaces.
- **SC-003**: 100% of JSON manifests in plugin package folders parse successfully during validation.
- **SC-004**: Validation fails for missing referenced paths, missing required metadata, missing required method surfaces, workspace-version drift, or prohibited positioning wording.
- **SC-005**: No supported host package duplicates the full shared Canon skill corpus; package folders remain metadata, manifest, command, prompt, asset, or glue surfaces.
- **SC-006**: README and installation docs explicitly state that Canon CLI and the governance adapter remain authoritative and that host packages do not create a new runtime.
- **SC-007**: Final closeout passes repository-required formatting, lint, test, and touched-Rust-file coverage validation with at least 95% line coverage for every created or modified Rust source file.

## Validation Plan *(mandatory)*

- **Structural validation**: Plugin validation script or test coverage for JSON parseability, required fields, version alignment, referenced path existence, required command/method surfaces, prohibited language, README guide links, and package folder presence.
- **Logical validation**: Focused tests or validation fixtures that prove invalid JSON, missing paths, version drift, missing capability bindings, and prohibited positioning text fail with useful diagnostics.
- **Independent validation**: Cross-artifact Speckit coherence review before implementation, followed by a manual readback of host package manifests and docs against the acceptance criteria to confirm they advertise Canon as a governance runtime rather than a host-owned runtime or agent system.
- **Evidence artifacts**: `specs/044-assistant-plugin-packages/validation-report.md`, plugin validation output, cargo test output, clippy output, cargo fmt output, and touched-file coverage evidence for any Rust source files created or modified by this slice.

## Decision Log *(mandatory)*

- **D-001**: Keep host plugin packages as installation, discovery, and metadata surfaces rather than runtime extensions, **Rationale**: Canon's CLI and governance adapter already define governed packet behavior and must remain authoritative.
- **D-002**: Treat Copilot as a documented command/prompt pack unless this repository already contains a stable manifest convention, **Rationale**: adding an invented plugin manifest would create false install guarantees and drift from actual host capabilities.
- **D-003**: Use a shared metadata source for common package fields and host folders for host-specific glue, **Rationale**: this prevents version and positioning drift without forcing one manifest shape onto every assistant host.
- **D-004**: Bump the workspace version to `0.44.0` for this packaging surface, **Rationale**: the project is pre-1.0 and the feature adds a public distribution and install surface.

## Non-Goals

- Redesign Canon's governed packet runtime, evidence persistence, approval flow, provenance model, or machine-facing governance adapter.
- Build a generic assistant plugin framework or marketplace integration layer.
- Make Claude Code, Codex, Cursor, Copilot, or future hosts authoritative for Canon governance decisions.
- Require Boundline to install or consume these plugin packages.
- Duplicate all Canon skill content into every host package.
- Expose private or unstable Canon internals as public plugin capabilities.

## Assumptions

- The current workspace version is `0.43.0`, and the correct feature release bump is `0.44.0`.
- Claude Code, Codex, and Cursor can be represented by repository-local package folders with manifests or metadata plus path references to Canon-owned skills and methods.
- Codex plugin metadata can live in `.codex-plugin/plugin.json` with interface metadata, default prompts, capabilities, and skills path.
- Claude and Cursor package shapes can be represented through repository-local manifest JSON plus host-facing command, prompt, hook, skill, or method reference files as needed by the existing project conventions.
- Copilot has no stable repository-local plugin manifest shape in this repo, so command/prompt pack documentation is the non-misleading supported surface.
- Existing `.agents/skills/`, `.canon/methods/`, and `defaults/embedded-skills/` directories remain the shared source material for host package references.
