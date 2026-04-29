# Data Model: Supply Chain And Legacy Analysis Mode

## SupplyChainAnalysisIntent

- **Purpose**: Captures the authored run intent Canon must have before scanner
  orchestration begins.
- **Fields**:
  - `run_id`: Canon run identifier
  - `owner`: human owner for the governed run
  - `risk`: declared risk classification
  - `zone`: declared usage zone
  - `system_context`: must resolve to `existing`
  - `licensing_posture`: `commercial`, `oss-permissive`, `oss-copyleft`, or `mixed`
  - `distribution_model`: whether dependencies are distributed or used internally only
  - `ecosystems_in_scope`: confirmed ecosystems Canon is allowed to analyze
  - `excluded_components`: bounded exclusions such as vendored or generated paths
  - `non_oss_tool_policy`: whether non-OSS tool proposals are allowed
- **Validation rules**:
  - `licensing_posture` and `non_oss_tool_policy` cannot remain implicit
  - `system_context` must remain `existing`
  - `ecosystems_in_scope` must not be empty once manifests are detected

## EcosystemScope

- **Purpose**: Binds the repository analysis to a concrete set of manifests,
  lockfiles, and dependency ecosystems.
- **Fields**:
  - `ecosystem_id`: Rust, Node, Python, Go, Java, etc.
  - `manifest_paths`: primary files that define dependencies
  - `lockfile_paths`: lockfiles or generated dependency snapshots
  - `status`: `in-scope`, `excluded`, or `detected-but-unconfirmed`
  - `exclusion_rationale`: optional rationale when excluded
- **Relationships**:
  - One `SupplyChainAnalysisIntent` can own many `EcosystemScope` entries
  - One `EcosystemScope` can drive many `ScannerRequirement` entries

## ScannerRequirement

- **Purpose**: Represents a required tool capability for a given ecosystem and
  analysis objective.
- **Fields**:
  - `requirement_id`: stable identifier per ecosystem and purpose
  - `ecosystem_id`: owning ecosystem
  - `purpose`: SBOM, vulnerability triage, license analysis, or legacy posture
  - `preferred_tools`: ordered list of acceptable tools
  - `oss_only_default`: whether proposals default to OSS-only choices
  - `availability`: `available`, `missing`, `skipped`, or `replaced`
- **Relationships**:
  - One `ScannerRequirement` can produce one `ScannerDecisionRecord`
  - One `ScannerRequirement` can produce zero or more findings

## ScannerDecisionRecord

- **Purpose**: Captures the user's explicit response when Canon cannot use the
  preferred scanner set as planned.
- **Fields**:
  - `requirement_id`: linked scanner requirement
  - `decision`: `installed`, `skipped`, or `replaced`
  - `rationale`: user or run-time rationale
  - `replacement_tool`: optional replacement identifier
  - `coverage_impact`: summary of what surface is now uncovered or redirected
  - `recorded_at`: timestamp
- **Validation rules**:
  - `skipped` and `replaced` must always include `coverage_impact`
  - `replaced` must include `replacement_tool`

## SBOMReference

- **Purpose**: Links the human packet to a machine-readable SBOM emitted for a
  bounded ecosystem surface.
- **Fields**:
  - `ecosystem_id`
  - `format`: CycloneDX, SPDX, or equivalent
  - `relative_path`: persisted artifact or payload reference
  - `source_manifests`: manifest paths covered by the SBOM
  - `generated_at`: timestamp

## VulnerabilityFinding

- **Purpose**: Records a dependency vulnerability finding grounded in scanner
  output.
- **Fields**:
  - `package_id`
  - `ecosystem_id`
  - `source_tool`
  - `severity`
  - `affected_version`
  - `fixed_version`: optional
  - `exploitability_notes`
  - `triage_disposition`: `accept`, `mitigate`, `defer`, or `needs-review`
  - `evidence_ref`: scanner output or advisory reference

## LicenseFinding

- **Purpose**: Evaluates dependency-license results against the declared
  project posture and distribution model.
- **Fields**:
  - `package_id`
  - `license_expression`
  - `compatibility_class`
  - `obligations`
  - `verdict`: `compatible`, `needs-review`, or `incompatible`
  - `evidence_ref`

## LegacyFinding

- **Purpose**: Records outdated, EOL, or abandonment signals that create
  modernization pressure even without a vulnerability finding.
- **Fields**:
  - `package_id`
  - `ecosystem_id`
  - `current_version`
  - `latest_known_version`: optional
  - `eol_status`
  - `abandonment_signals`
  - `modernization_slice`
  - `evidence_ref`

## CoverageGap

- **Purpose**: Makes incomplete analysis explicit wherever required scanner,
  evidence, or user posture decisions are unavailable.
- **Fields**:
  - `surface`: ecosystem, manifest set, or artifact section that is uncovered
  - `reason`: missing scanner, denied policy input, unsupported ecosystem, or excluded path
  - `impacted_artifacts`: affected packet artifacts
  - `resolution_hint`: next step for a follow-on run

## Relationships Summary

- `SupplyChainAnalysisIntent` owns many `EcosystemScope` entries.
- Each `EcosystemScope` drives many `ScannerRequirement` entries.
- `ScannerRequirement` may resolve to one `ScannerDecisionRecord` and many
  findings.
- `SBOMReference`, `VulnerabilityFinding`, `LicenseFinding`, and `LegacyFinding`
  feed the final packet.
- `CoverageGap` can attach to any requirement, ecosystem, or packet artifact.

## State Transitions

- `SupplyChainAnalysisIntent`: `authored` -> `clarified` -> `ready-for-run`
- `ScannerRequirement`: `planned` -> `available` or `missing` -> `executed`, `skipped`, or `replaced`
- `Run state`: `Blocked`, `AwaitingApproval`, or `Completed` according to the
  existing governed gate model