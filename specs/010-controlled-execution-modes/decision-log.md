# Design Decision Log: Controlled Execution Modes

## D-001: Promote both execution-heavy modes in one change package

- **Status**: Proposed
- **Context**: `implementation` and `refactor` share execution-heavy posture, mutation-bounds enforcement, canonical authored-input binding, and recommendation-only behavior, but differ in artifact bundles and failure semantics.
- **Decision**: Deliver the two modes in one design package so shared runtime primitives are implemented once and mode-specific gates/contracts stay aligned.
- **Consequences**: The change set is broader than promoting only one mode, but it avoids two partial execution systems.

## D-002: Reuse `context.toml` and invocation constraints for machine-checkable execution metadata

- **Status**: Proposed
- **Context**: The feature needs durable mutation bounds, preservation targets, and safety-net references without creating parallel storage.
- **Decision**: Extend run context with mode-specific execution metadata and project enforceable path bounds into `InvocationConstraintSet.allowed_paths`.
- **Consequences**: Runtime persistence changes stay localized to existing manifests and enforcement surfaces.

## D-003: Keep recommendation-only as posture, not as a new run state

- **Status**: Proposed
- **Context**: Recommendation-only already exists for constrained mutation decisions and tool outcomes.
- **Decision**: Preserve the current `RunState` enum and surface recommendation-only through policy decisions, tool outcomes, and summaries.
- **Consequences**: `status`, `inspect`, `list`, and `publish` remain backward-compatible, but the summary layer must become explicit enough that users do not confuse completed recommendation-only work with enacted mutation.

## D-004: Build dedicated gatekeeper evaluators instead of reusing `change`

- **Status**: Proposed
- **Context**: `change` already models preserved-behavior pressure and recommendation-only mutation, but it does not encode refactor no-feature-addition proof or implementation task mapping.
- **Decision**: Add dedicated implementation and refactor gate evaluators using existing gate vocabulary where possible.
- **Consequences**: Slightly more orchestrator code, but clearer mode semantics and stronger invariant enforcement.

## D-005: Preserve existing publish destinations and CLI entrypoints

- **Status**: Proposed
- **Context**: Publish already routes `implementation` and `refactor` to distinct visible directories, and the CLI already supports `canon run --mode <MODE>` plus inspect/status/publish.
- **Decision**: Keep the current entrypoints and publish destinations; change only the runtime contracts and emitted content.
- **Consequences**: User workflows stay stable and documentation updates are mostly about availability and semantics, not command discovery.

## D-006: Update skills only after runtime truth changes

- **Status**: Proposed
- **Context**: Current implementation/refactor skills intentionally advertise `modeled-only` support.
- **Decision**: Keep support-state honesty until the runtime paths are complete, then promote the skill contracts and validate them with the existing skill validator.
- **Consequences**: Delivery is sequenced: runtime first, messaging second, no temporary period of inflated capability claims.

## D-007: Treat `change` test suites as the reference model for new coverage

- **Status**: Proposed
- **Context**: `change` already proves bounded mutation, release gating, and recommendation-only policy behavior.
- **Decision**: Mirror that structure for new implementation/refactor contract and integration tests, then update staged-mode assertions.
- **Consequences**: Faster test design and stronger non-regression mapping across execution-heavy modes.

## D-008: Implement in the order implementation -> refactor -> recommendation-only

- **Status**: Accepted for implementation
- **Context**: The feature promotes two execution-heavy modes plus a shared recommendation-only fallback. The runtime already has more precedent for bounded execution posture in `change` than for preservation-specific refactor semantics.
- **Decision**: Execute delivery in the order captured by `tasks.md`: shared governance/foundations, `implementation`, `refactor`, then recommendation-only cross-mode behavior.
- **Consequences**: The first shipped slice exercises the smallest bounded-execution path, reduces concurrent moving parts in the gatekeeper, and gives refactor-specific preservation logic a stable execution substrate.

## D-009: Start implementation in bounded recommendation-only posture with explicit readiness proof

- **Status**: Accepted for implementation
- **Context**: `implementation` must be runnable end-to-end before Canon can safely promote more mutating execution, but the runtime still needs a safety net that prevents silent workspace mutation when authored evidence is incomplete or risk expands.
- **Decision**: Make `implementation` a governed execution mode that emits its distinct artifact bundle, records `recommendation-only` posture in run context and summaries, and treats task mapping, mutation bounds, safety-net evidence, and rollback notes as `implementation-readiness` prerequisites.
- **Consequences**: Users can run `implementation` end-to-end now, inspect bounded recommendations and trace evidence through existing surfaces, and only see completed runs when the authored inputs satisfy the readiness contract.

## D-010: Start refactor in preservation-first recommendation-only posture with explicit no-feature-addition proof

- **Status**: Accepted for implementation
- **Context**: `refactor` needs a real runtime path, but the first safe slice must prove preserved behavior, bounded structural scope, and no-feature-addition intent before Canon can treat the packet as complete.
- **Decision**: Make `refactor` a governed execution mode that emits its distinct preservation bundle, records `recommendation-only` posture in run context and summaries, and treats preserved behavior, bounded refactor scope, structural rationale, drift review, and no-feature-addition evidence as blocking prerequisites.
- **Consequences**: Users can run `refactor` end to end now, publish the preservation packet through the existing `docs/refactors/<RUN_ID>/` surface, and get explicit blocking output instead of silent structural drift when authored evidence is incomplete.

## D-011: Keep high-risk implementation/refactor runs on recommendation-only completion, not risk-gate approval

- **Status**: Superseded by D-013 (see Feature 011)
- **Context**: `change` already uses explicit risk-gate approval for systemic or red-zone work, but `implementation` and `refactor` in this feature slice never enact consequential mutation. Reusing the same approval gate made high-risk runs stop in `AwaitingApproval`, which violated the US3 contract.
- **Decision (original)**: For `implementation` and `refactor`, require human ownership at the risk gate but do not require `gate:risk` approval for red-zone or systemic-impact classifications. The high-risk fallback is recommendation-only completion with the same artifact surfaces, plus explicit posture labeling in status and inspect output.
- **Consequences (original)**: High-risk execution-heavy runs finish as readable recommendation-only packets, `change` keeps its approval-gated semantics, and publish/run-lookup surfaces remain unchanged because the fallback is expressed through posture rather than a new run-state branch.
- **Why superseded**: T044 independent review (dogfood against `java-html-sanitizer`) showed that auto-completing every `implementation` / `refactor` run as `recommendation-only` collapses the modes into `change`: there is no approval surface, no `$canon-approve` / `$canon-resume` cycle, and no host-renderable Action Chips, so the modes are practically indistinguishable from a discovery output. Feature 011 reintroduces an explicit execution-approval gate (D-013), reframes recommendation-only as a posture controlled by approval state (D-014), and adds the missing chip emission contract (D-015).

## D-012: Treat folder-backed carry-forward packets as current-brief plus provenance, not implicit upstream ingestion

- **Status**: Accepted for implementation
- **Context**: `implementation` and `refactor` need a durable way to continue prior bounded work without making old `.canon/` state or published docs implicitly authoritative for the new run.
- **Decision**: For folder-backed `canon-input/implementation/` and `canon-input/refactor/` packets, treat `brief.md` as the authoritative current-mode contract, `source-map.md` as explicit provenance, and optional `selected-context.md` as narrowed excerpts. Persist any recognized lineage to `upstream_context` in `context.toml` and `inspect evidence`, but never infer readiness by re-reading prior `.canon/` runs or published packets outside the current authored inputs.
- **Consequences**: Carry-forward remains explicit and machine-readable, inspect surfaces gain durable lineage, and the runtime avoids hidden dependency on prior run state while preserving the principle that the current brief governs execution.

## D-013: Require an `ExecutionApproval` gate for `implementation` and `refactor`

- **Status**: Accepted (Feature 011)
- **Context**: D-011 made every `implementation` and `refactor` run finish as `recommendation-only`, with no approval target on the run summary and no path from generation to enacted mutation. Independent review (T044) confirmed that this makes the two modes operationally indistinguishable from `change`/discovery and removes the entire `$canon-approve` / `$canon-resume` cycle.
- **Decision**: Both `implementation` and `refactor` issue a Canon-managed `gate:execution` approval target on first generation, regardless of risk or zone, because mutation intent is the defining property of these modes. The run halts at `AwaitingApproval` until a maintainer records `canon approve --target gate:execution`. `canon resume` re-evaluates the gate and progresses the run. The gate is unconditional in v0.1; finer-grained risk-aware variants are deferred.
- **Consequences**: `implementation` and `refactor` regain a real approval surface; the run summary now exposes `approval_targets = ["gate:execution"]` pre-approval; `change`'s `gate:risk` semantics are unchanged; all existing integration tests that asserted immediate `Completed` for these modes must add an approve+resume step.

## D-014: Reframe `recommendation_only` as an approval-controlled posture

- **Status**: Accepted (Feature 011)
- **Context**: `defaults/policies/adapters.toml` and the orchestrator's constraint profiles hardcoded `recommendation_only = true` for `implementation-mutation` and `refactor-mutation`, so even an approved run could not select a different posture. With D-013 in place, posture must be a function of approval state, not a static policy constant.
- **Decision**: Split each mutation profile into two variants: `*-mutation-recommendation` (`recommendation_only = true`, used pre-approval) and `*-mutation-bounded` (`recommendation_only = false`, used post-approval). The active profile is selected at policy-decision time based on whether `gate:execution` has an approved record for the run. After approval, the run reports `execution_posture = approved-recommendation` to make the transition observable to hosts and humans without overloading the existing string. Actual bounded patch application remains out of scope here and is owned by Feature 012.
- **Consequences**: Posture becomes derivable from runtime state instead of policy constants; hosts can render approve / resume / inspect actions without inferring approval state from list contents; mutation execution wiring stays compatible with the future bounded executor because it only needs to swap behavior inside the existing `*-mutation-bounded` profile.

## D-015: Emit a structured `action_chips` contract on every mode summary

- **Status**: Accepted (Feature 011)
- **Context**: `defaults/embedded-skills/canon-shared/references/output-shapes.md` already defines an Action Chip shape (`id`, `label`, `skill`, `prefilled_args`, `required_user_inputs`, `visibility_condition`, `recommended`), but no mode summarizer ever populates it. Hosts therefore cannot render Approve / Resume / Inspect / Open chips, leaving the user with raw paths and no next-step affordance.
- **Decision**: Extend `ModeResultSummary` with `action_chips: Vec<ActionChip>` (always present, may be empty) and emit a small canonical chip set per run state via a single `build_action_chips_for(run_state, approval_targets, primary_artifact_path, mode)` helper. Available chips: `Open primary artifact` (whenever a primary artifact path exists), `Inspect evidence` (`AwaitingApproval` or `Completed`), `Approve generation...` (only when a real approval target exists in `approval_targets`), `Resume run` (`AwaitingApproval` plus a recorded approval for the active gate). All `available-now` modes emit chips; mandatory test coverage in Feature 011 is on `implementation` and `refactor`. CLI text output gains an `Action Chips:` section; JSON serialization is automatic via Serde derive.
- **Consequences**: Hosts gain a stable, mode-agnostic UI contract; the existing `primary_artifact_action` field stays for back-compat but is now redundant with the `Open primary artifact` chip; future modes can opt into more chips without changing the summary shape.
