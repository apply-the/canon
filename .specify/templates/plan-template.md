# Implementation Plan: [FEATURE]

**Branch**: `[###-feature-name]` | **Date**: [DATE] | **Spec**: [link]
**Input**: Feature specification from `/specs/[###-feature-name]/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See
`.specify/templates/plan-template.md` for the execution workflow.

## Summary

[Extract from feature spec: primary requirement + technical approach from
research]

## Governance Context

**Execution Mode**: [e.g., system-shaping, brownfield, review, debugging or NEEDS
CLARIFICATION]
**Risk Classification**: [low-impact | bounded-impact | systemic-impact with rationale]
**Scope In**: [Explicitly included work]
**Scope Out**: [Explicitly excluded work]

**Invariants**:

- [Invariant that MUST remain true]
- [Invariant that bounds the change]

**Decision Log**: [Path to the durable decision record for this feature]  
**Validation Ownership**: [Who/what generates output vs who/what validates it]  
**Approval Gates**: [Human approvals required by the risk level]

## Technical Context

<!--
  ACTION REQUIRED: Replace the content in this section with the technical details
  for the project. The structure here is presented in advisory capacity to guide
  the iteration process.
-->

**Language/Version**: [e.g., Python 3.11, Swift 5.9, Rust 1.75 or NEEDS
CLARIFICATION]  
**Primary Dependencies**: [e.g., FastAPI, UIKit, LLVM or NEEDS CLARIFICATION]  
**Storage**: [if applicable, e.g., PostgreSQL, CoreData, files or N/A]  
**Testing**: [e.g., pytest, XCTest, cargo test or NEEDS CLARIFICATION]  
**Target Platform**: [e.g., Linux server, iOS 15+, WASM or NEEDS
CLARIFICATION]  
**Project Type**: [e.g., library/cli/web-service/mobile-app/compiler/desktop-app
or NEEDS CLARIFICATION]  
**Existing System Touchpoints**: [services, packages, files, or interfaces this
feature will affect]  
**Performance Goals**: [domain-specific, e.g., 1000 req/s, 10k lines/sec, 60
fps or NEEDS CLARIFICATION]  
**Constraints**: [domain-specific, e.g., <200ms p95, <100MB memory,
offline-capable or NEEDS CLARIFICATION]  
**Scale/Scope**: [domain-specific, e.g., 10k users, 1M LOC, 50 screens or NEEDS
CLARIFICATION]

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [ ] Execution mode is declared and matches the requested work
- [ ] Risk classification is explicit and autonomy is appropriate for that risk
- [ ] Scope boundaries and exclusions are recorded
- [ ] Invariants are explicit before implementation
- [ ] Required artifacts and owners are identified
- [ ] Decision logging is planned and linked to a durable artifact
- [ ] Validation plan separates generation from validation
- [ ] Declared-risk approval checkpoints are named where required by the risk classification
- [ ] Any constitution deviations are documented in Complexity Tracking

## Project Structure

### Documentation (this feature)

```text
specs/[###-feature]/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── decision-log.md
├── validation-report.md
├── contracts/
└── tasks.md
```

### Source Code (repository root)
<!--
  ACTION REQUIRED: Replace the placeholder tree below with the concrete layout
  for this feature. Delete unused options and expand the chosen structure with
  real paths (e.g., apps/admin, packages/something). The delivered plan must
  not include Option labels.
-->

```text
# [REMOVE IF UNUSED] Option 1: Single project (DEFAULT)
src/
├── models/
├── services/
├── cli/
└── lib/

tests/
├── contract/
├── integration/
└── unit/

# [REMOVE IF UNUSED] Option 2: Web application (when "frontend" + "backend" detected)
backend/
├── src/
│   ├── models/
│   ├── services/
│   └── api/
└── tests/

frontend/
├── src/
│   ├── components/
│   ├── pages/
│   └── services/
└── tests/

# [REMOVE IF UNUSED] Option 3: Mobile + API (when "iOS/Android" detected)
api/
└── [same as backend above]

ios/ or android/
└── [platform-specific structure: feature modules, UI flows, platform tests]
```

**Structure Decision**: [Document the selected structure and reference the real
directories captured above]

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., broader scope] | [current need] | [why tighter scope is insufficient] |
| [e.g., weaker validation split] | [specific constraint] | [why independent validation is still preserved adequately] |
