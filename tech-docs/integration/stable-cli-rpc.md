# Stable Canon CLI and one-shot RPC

Canon 0.90 freezes nine visible root commands in this order:

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
JSON response, flushes stdout, and exits. Its operations are `capabilities`,
`start`, `refresh`, `approve`, `inspect`, and `publish`. `start` and `approve`
accept a typed `bundle`; other operations require an empty payload. The request
limit is 1,048,576 bytes. Empty, malformed, trailing, concatenated, array,
oversized, invalid-UTF-8, unknown-field, and unknown-operation inputs fail
closed.

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

Mutation identity is bound to `request_id == bundle_id`. An exact retry returns
the recorded projection without rewriting decision memory; different content
under the same identity fails with `identity_digest_conflict`. Read-only
operations never rewrite the snapshot.

These surfaces execute no provider, model, network call, semantic reviewer, or
background process. Semantic review remains externally supplied evidence.
`publish` returns governance projections only; it cannot modify an authoritative
workspace. MCP is not registered by this milestone because T056 freezes only
the JSON one-shot transport.
