# 07 - Interactive Publish and Handoff Routing

## Problem
From the Open Ideas left over from `ROADMAP.md` in 0.64.0: `canon init` currently handles only local runtime materialization and basic initialization choices in an interactive guided flow. It does not capture Canon-local routing intent such as where publishable artifacts should go or which downstream backlog or review target the workspace expects to use.

## Proposal
Extend the Interactive Init surface with Canon-local routing setup. Focus on:
- Publish routing: collect choices for local publish destinations or documentation targets that Canon should record as metadata.
- Handoff routing: collect choices for backlog or review handoff targets as stable local references.
- Preserve the exact non-interactive contract (`--non-interactive`) so every interactive choice has a documented config or flag equivalent.

This feature intentionally excludes provider discovery, secret capture, permission grants, and connectivity checks. Those remain runtime concerns outside Canon.

## Risk Profile

**Governance Zone**: Green (local metadata configuration only).
The flow records local routing intent and generates local configuration. It does not perform network calls, persist credentials, or manage provider health.

## Why Existing Modes Are Not Enough
- The current interactive init flow materializes local Canon runtime state, but it stops short of capturing workspace-local publish and handoff intent.
- Generic documentation is not enough because these routing choices must be recorded as stable, machine-readable local metadata rather than prose.

## Dependencies

- **Stable init flow** (already shipping): this feature extends `canon init` rather than replacing it.
- **No provider prerequisite in Canon**: this seed records only Canon-local refs. If a chosen target later requires provider health, permissions, or credentials, that setup belongs to the consuming runtime.

## Related Modes

| Existing Mode | Relationship |
|---|---|
| `backlog` | Downstream consumer: handoff targets give backlog packets a stable local destination. |
| `review` | Downstream consumer: review-ready packets can project an intended publish or handoff destination. |
| `migration` | Adjacent: changing established routing targets is a migration-grade configuration change. |

## Entry Gates
- The flow must remain explicitly interactive; no hidden changes to `--non-interactive` behavior are allowed.
- Supported local routing targets and config surfaces must be declared up front.
- No secret collection, provider discovery, or network validation is allowed within Canon's side of this flow.

## Operational Mechanics
- **Inputs**: User interaction via the `canon init` TUI (leveraging `ratatui` + `crossterm`) and a target `.canon/routing.json` or equivalent local config scope.
- **Workflow Steps**:
  1. **Target Selection**: Display the declared local publish destinations and handoff targets that Canon knows how to reference.
  2. **Routing Materialization**: Save only stable local identifiers, path refs, or logical target names required by Canon packets.
  3. **Preview and Next Steps**: Render the resulting routing map and any external runtime prerequisites that still need to be completed outside Canon.
- **Required Artifacts**: Safe updates to the local `.canon/routing.json` or equivalent mapping, plus a concise onboarding summary that records which routing choices were captured.

## Exit Gates
- No raw secret, token, permission grant, or network check may occur in Canon's flow.
- Every saved value must be a stable logical target, local ref, or local path mapping.
- Every interactive choice must have a non-interactive equivalent.

## Packet Shape
- `01-targets.md`: chosen publish and handoff targets plus intended use.
- `02-routing-map.md`: canonical logical destinations and local refs.
- `03-generated-config.md`: local files or mappings created by the flow.
- `04-external-prereqs.md`: provider or runtime setup still required outside Canon.
- `05-operator-next-steps.md`: remaining manual setup and validation steps.

## Success Criteria

- Operators can configure Canon-local publish and handoff routing without touching provider credentials.
- No network calls or secret prompts occur during the Canon flow.
- The `--non-interactive` contract is preserved: all choices have documented flag or config equivalents.