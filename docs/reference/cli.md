# Reference

This page is a compact reference for Canon wiki concepts. Use the source repository for exact CLI behavior and policy defaults.

## Machine-Facing Integration

For downstream runtimes (like Boundline), use the JSON-based `canon governance` commands instead of scraping human CLI prose:

```bash
canon governance capabilities --json
canon governance start --json < request.json
canon governance refresh --json < request.json
```

These commands resolve the workspace and provide machine-readable status, packet refs, and metadata.

## Governance Semantics

Canon separates semantic posture into two contracts:

- **Authority Governance (`authority-governance-v1`)**: The required baseline contract.
- **Adaptive Governance (`adaptive-governance-v1`)**: An optional companion contract.

For downstream reasoning-posture consumers such as Boundline, Canon also
publishes the stable `governed_reasoning_posture_v2` contract in
`tech-docs/integration/governed-reasoning-posture-contract.md`. That contract
is Canon-owned, versioned independently from runtime orchestration, and now
defines typed selector, `minimum_independence`, `confidence_handoff`,
`provenance`, `compatibility_window`, and active-versus-legacy migration rules.

| Type | Term | Definition |
| --- | --- | --- |
| Governance State | `advisory` | A state in the Adaptive Governance layer suggesting guidance. |
| Governance State | `catch` | A state in the Adaptive Governance layer indicating observation/alerting. |
| Governance State | `rule` | A state in the Adaptive Governance layer enforcing a constraint. |
| Governance State | `hook` | A state in the Adaptive Governance layer indicating a required process integration. |
| Rollout Profile | `minimal` | Earliest maturity label for a feature/system. |
| Rollout Profile | `guided` | Guided maturity label. |
| Rollout Profile | `governed` | Full governance maturity label. |
| Rollout Profile | `strict` | Highest level of strictness maturity. |

## Stable Profile Reference

`canon inspect modes`, the machine capabilities projection, and stable run
admission share one ordered registry. Identifiers are exact and case-sensitive.

| Profile | Primary Use |
| --- | --- |
| `discovery` | Explore ambiguous problem space. |
| `requirements` | Define bounded scope, outcomes, and acceptance. |
| `architecture` | Capture bounded structural decisions. |
| `backlog` | Decompose approved upstream knowledge into delivery slices. |
| `change` | Govern change intent, scope, risk, invariants, and required evidence. |
| `refactor` | Improve structure without expanding feature scope. |
| `verification` | Govern evidence requirements and validate supplied evidence deterministically. |
| `pr-review` | Review a real diff or worktree. |
| `incident` | Capture incident impact, containment, and follow-up. |

`implementation` and the prior extended mode taxonomy are not stable profiles.
Persisted legacy records remain inspectable, but their names are not aliases
and do not admit new runs.

### Verification boundary

`canon verify --run <RUN_ID>` checks that the persisted typed evidence
projection is readable. The command reports
`deterministic_structure_valid` and `external_semantic_judgment:
required_missing`, then exits nonzero until required external semantic evidence
has been supplied through a qualified boundary. Deterministic structural
success is not a semantic approval.

Canon validates externally supplied semantic evidence only for structural
completeness, exact claims and immutable references, freshness, terminality,
challenge tier, and independent lineage. It does not start a reviewer, invoke
Copilot or a model API, read provider credentials, use the network, or create
semantic evidence. Requests for Canon-owned semantic execution are explicitly
unsupported.

## Packet Document Reference

### PR-Review Conventional Comments

`pr-review` Conventional Comments always retain explicit derived scope.

- `scope:pr`: the comment applies at the whole-PR level.
- `scope:surface`: all changed surfaces for the finding stay within one functional group.
- `scope:file`: the finding spans more than one functional group.

When the stored diff resolves to one changed surface and one contiguous interval,
Canon may also render one host-agnostic anchor as `surface:start` or
`surface:start-end`.

When evidence is cross-surface, disjoint, stale, or otherwise insufficient,
Canon omits the anchor and keeps scope-only output.

Common ordered document roles:

- `01-context.md`: why this packet exists and what boundary applies
- `02-findings.md`: observations, facts, or conclusions
- `03-decisions.md`: choices, rationale, and alternatives
- `04-evidence.md`: supporting material and traceability
- `05-next-steps.md`: follow-up work, if the mode calls for it

Mode-specific packets may use different names, but should preserve an intentional reading order.

## Evidence Reference

Strong evidence:

- exact file refs
- command output
- test or CI results
- logs and timestamps
- reviewer comments
- prior packet refs
- source examples
- dependency or vulnerability data

Weak evidence:

- generic summaries
- uncited assumptions
- stale observations
- "the assistant thinks" claims
- conclusions without checks

## Approval And Readiness Reference

Approval answers: may downstream work rely on this?

Readiness answers: is this packet complete enough for its intended use?

Do not collapse the two. A reusable packet may still require approval for high-impact use. An approved packet may still be bounded to one context.

## Promotion Reference

Promote knowledge when it is:

- durable
- evidence-backed
- source-linked
- appropriately approved
- useful for downstream work
- compatible with the destination memory surface

Do not promote raw drafts, unresolved questions, or low-confidence generated prose.

## Glossary

**Authored input**: user- or team-authored material that grounds a Canon run.

**Boundline**: a downstream runtime delivery system that can consume Canon-governed knowledge.

**Evidence**: inspectable support for a packet claim.

**Governed packet**: structured, mode-specific engineering knowledge with evidence, readiness, approval, and lineage.

**Lineage**: trace from source input to packet, evidence, publication, promotion, and downstream consumption.

**Mode**: the kind of work Canon is governing.

**Project memory**: durable governed knowledge promoted for future use.

**Promotion**: moving selected packet knowledge into durable memory or documentation.

**Readiness**: whether a packet is usable for its stated purpose.

**Semantic authority**: Canon-owned governed meaning that downstream tools should respect.

**Zone**: governance context indicating operational confidence or criticality.
