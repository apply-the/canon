# Stable Canon CLI and one-shot RPC

Canon 0.90 and 0.91 freeze nine visible root commands in this order:

```text
init run resume status approve inspect publish assistant rpc
```

`canon run --profile <profile> --bundle <file> --output json` admits one
typed governance draft through the deterministic decision-memory kernel.
`canon inspect decision-memory --output json` reads the same public projection
without changing durable state. The stable profile registry is `discovery`,
`requirements`, `architecture`, `backlog`, `change`, `refactor`,
`verification`, `pr-review`, and `incident`. `implementation` is not a stable
profile.

`canon rpc --stdio` reads exactly one complete JSON request, writes exactly one
JSON response, flushes stdout, and exits. The historical 0.90 operation
inventory remains:

```text
capabilities start refresh approve inspect publish
```

Canon 0.91 adds exactly one operation:

```text
record_outcome
```

The existing six semantics do not change. `start` and `approve` accept a typed
`bundle`; `capabilities`, `refresh`, `inspect`, and `publish` require an empty
payload. `publish` remains read-only and returns governance, evidence, and
decision-memory projections only.

Before T059, capabilities lists `record_outcome` with `available = false` and
`reason_code = unsupported_operation`. A valid invocation accepts the public
`RecordOutcomeRequest` payload but returns a typed rejected
`RecordOutcomeResponse`, no decision-memory revision or digest, exit code 8,
and no durable state. This is an unavailable service boundary, not a
successful stub.

The request limit remains 1,048,576 bytes. Empty, malformed, trailing,
concatenated, array, oversized, invalid-UTF-8, unknown-field, and
unknown-operation inputs fail closed.

The exit matrix is:

| Condition | Code |
| --- | ---: |
| accepted / capabilities | 0 |
| invalid input or framing | 1 |
| stale state | 2 |
| authority denied | 3 |
| required evidence missing or deterministic rejection | 5 |
| persistence failure | 6 |
| identity/digest conflict | 7 |
| unsupported operation or contract state | 8 |
| internal invariant failure | 1 |

Governance mutation identity remains bound to `request_id == bundle_id`. An
exact retry returns the recorded projection without rewriting decision memory;
different content under the same identity fails with
`identity_digest_conflict`. Read-only operations never rewrite the snapshot.

For `record_outcome`, `request_id` equals `event_id`. The request binds the
Boundline source and clone-local repository identity, governance bundle,
session and final transaction revision, terminal status, optional published
commit and final fingerprint, proof, deviations, terminal claims, authority,
approval, challenge, lineage, and an optional authoritative occurrence time.
`published` requires a commit and fingerprint; `no_change` requires a
fingerprint and forbids a commit; `failed`, `cancelled`, and `rejected` forbid a
commit. `blocked` and `stale` decode as closed candidate values but are rejected
as nonterminal.

The decoder recomputes `event_digest` as SHA-256 over:

```text
UTF-8("canon-boundline-outcome-c14n-v1")
0x00
canonical JSON of every authoritative request field except event_digest
```

Object keys are recursively sorted. Set-like collections are sorted and
duplicate-free. Sequence order is otherwise preserved. Floating-point values,
duplicate keys, self-attested lineage, unbound claims, secrets, raw prompts,
private conversations, and process-local paths fail closed.

The envelope remains `CanonContractVersion::V1`, serialized as `"1.0"`.
Package version `0.91.0` identifies the additive Rust API and discoverable
operation; V1 remains truthful because the original operations retain their
wire semantics and capability discovery advertises the addition.

These surfaces execute no provider, model, network call, semantic reviewer, or
background process. Semantic review remains externally supplied evidence.
`publish` cannot modify an authoritative workspace. MCP is not registered by
this milestone because T056 freezes only the JSON one-shot transport.
T057-T059 are not implemented by the 0.91 readiness amendment.
