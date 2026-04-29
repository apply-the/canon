# Supply Chain Analysis Brief

System Surface: The bounded repository, package set, or release surface this packet evaluates.
Primary Upstream Mode: discovery | architecture | change | security-assessment | direct
Upstream Sources:
- Cargo.toml
Carried-Forward Decisions:
- Existing release, licensing, or dependency-policy decisions carried into this packet.
Excluded Upstream Scope: Adjacent services, external vendor trees, and unrelated release channels that remain out of scope.

## Declared Scope
- Name the bounded repository or manifest surface this packet covers.

## Licensing Posture
- State whether the project is commercial, OSS-permissive, OSS-copyleft, or mixed.

## Distribution Model
- State whether the analyzed dependencies are distributed or used internally only.

## Ecosystems In Scope
- List the package ecosystems and manifest files in scope.

## Out Of Scope Components
- State the directories, vendored code, or third-party assets this packet excludes.

## Scanner Selection Rationale
- Name the bounded scanner set and why each tool is needed.

## SBOM Outputs
- State the expected machine-readable SBOM outputs or attachments.

## Findings By Severity
- Record the vulnerability findings grouped by severity.

## Exploitability Notes
- Explain exploitability or operational relevance for the key findings.

## Triage Decisions
- Record whether each key finding is accepted, mitigated, deferred, or escalated.

## Compatibility Classes
- Group dependency licenses by compatibility class for the declared posture.

## Flagged Incompatibilities
- Record the incompatibilities or policy conflicts that need follow-up.

## Obligations
- Record the obligations the project must honor for the in-scope licenses.

## Outdated Dependencies
- List outdated or lagging dependencies that matter for the bounded surface.

## End Of Life Signals
- Record end-of-life signals or unsupported runtime surfaces.

## Abandonment Signals
- Record maintainer or ecosystem abandonment indicators when relevant.

## Modernization Slices
- Propose bounded modernization or dependency-replacement slices.

## Scanner Decisions
- Record install, skip, or replacement decisions for missing scanners.

## Coverage Gaps
- State the ecosystems or findings that remain uncovered and why.

## Source Inputs
- List the manifests, lockfiles, docs, configs, or prior Canon packets that ground the packet.

## Independent Checks
- Name the checks that will independently challenge the packet.

## Deferred Verification
- Record the verification work that must happen later.

Risk Level: bounded-impact
Zone: yellow