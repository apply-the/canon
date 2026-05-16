# Decision Log: Authority Zone Contract

## 2026-05-15

- **D-001**: Canon introduces a named first-slice contract line,
  `authority-governance-v1`.
  **Rationale**: a named contract line lets downstream consumers fail closed on
  incompatible semantic changes instead of guessing compatibility.

- **D-002**: `authority-governance-v1` uses an explicit required versus
  optional field profile.
  **Rationale**: downstream runtimes need deterministic fail-closed behavior for
  control inputs while still being able to ignore missing provenance-only data.

- **D-003**: `stage_role_hints` remain advisory metadata.
  **Rationale**: Canon owns governed semantics, while downstream runtimes such
  as Boundline own runtime role selection and operational control flow.

- **D-004**: `AuthorityZone` is introduced as a distinct cross-repo contract
  type rather than as a direct rename of existing `UsageZone`.
  **Rationale**: Canon needs the new S3 vocabulary, including `restricted`,
  without silently changing current runtime policy semantics.