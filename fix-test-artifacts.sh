#!/bin/bash
# Fix ordinal-prefixed artifact filenames in test assertions
# This script handles three categories:
# 1. MODE/SLUG.md patterns in format! and ends_with (safe global replacement)
# 2. .join("SLUG.md") in artifact contexts (per-mode-file replacement)
# 3. Sorted vectors that need reordering (manual via sed)

set -euo pipefail
cd "$(dirname "$0")"

# --- Category 1: Replace MODE/SLUG.md patterns globally in tests/ ---
# These are safe because the mode directory name provides unambiguous context.

# Requirements
sed -i '' 's|/requirements/problem-statement\.md|/requirements/01-problem-statement.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/requirements/constraints\.md|/requirements/02-constraints.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/requirements/options\.md|/requirements/03-options.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/requirements/tradeoffs\.md|/requirements/04-tradeoffs.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/requirements/scope-cuts\.md|/requirements/05-scope-cuts.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/requirements/decision-checklist\.md|/requirements/06-decision-checklist.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/requirements/prd\.md|/requirements/07-prd.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true

# Discovery
sed -i '' 's|/discovery/problem-map\.md|/discovery/01-problem-map.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/discovery/unknowns-and-assumptions\.md|/discovery/02-unknowns-and-assumptions.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/discovery/context-boundary\.md|/discovery/03-context-boundary.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/discovery/exploration-options\.md|/discovery/04-exploration-options.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/discovery/decision-pressure-points\.md|/discovery/05-decision-pressure-points.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true

# System-shaping
sed -i '' 's|/system-shaping/system-shape\.md|/system-shaping/01-system-shape.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/system-shaping/domain-model\.md|/system-shaping/02-domain-model.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/system-shaping/architecture-outline\.md|/system-shaping/03-architecture-outline.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/system-shaping/capability-map\.md|/system-shaping/04-capability-map.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/system-shaping/delivery-options\.md|/system-shaping/05-delivery-options.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/system-shaping/risk-hotspots\.md|/system-shaping/06-risk-hotspots.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true

# Architecture
sed -i '' 's|/architecture/architecture-overview\.md|/architecture/01-architecture-overview.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/architecture/architecture-decisions\.md|/architecture/02-architecture-decisions.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/architecture/invariants\.md|/architecture/03-invariants.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/architecture/tradeoff-matrix\.md|/architecture/04-tradeoff-matrix.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/architecture/boundary-map\.md|/architecture/05-boundary-map.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/architecture/context-map\.md|/architecture/06-context-map.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/architecture/readiness-assessment\.md|/architecture/07-readiness-assessment.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/architecture/system-context\.md|/architecture/08-system-context.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/architecture/system-context\.mmd|/architecture/09-system-context.mmd|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/architecture/container-view\.md|/architecture/10-container-view.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/architecture/container-view\.mmd|/architecture/11-container-view.mmd|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/architecture/deployment-view\.md|/architecture/12-deployment-view.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/architecture/deployment-view\.mmd|/architecture/13-deployment-view.mmd|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/architecture/view-manifest\.json|/architecture/14-view-manifest.json|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/architecture/packet-metadata\.json|/architecture/15-packet-metadata.json|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/architecture/component-view\.md|/architecture/16-component-view.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/architecture/component-view\.mmd|/architecture/17-component-view.mmd|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/architecture/dynamic-view\.md|/architecture/18-dynamic-view.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/architecture/dynamic-view\.mmd|/architecture/19-dynamic-view.mmd|g' tests/*.rs tests/**/*.rs 2>/dev/null || true

# Change
sed -i '' 's|/change/system-slice\.md|/change/01-system-slice.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/change/legacy-invariants\.md|/change/02-legacy-invariants.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/change/change-surface\.md|/change/03-change-surface.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/change/implementation-plan\.md|/change/04-implementation-plan.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/change/validation-strategy\.md|/change/05-validation-strategy.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/change/decision-record\.md|/change/06-decision-record.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true

# Backlog
sed -i '' 's|/backlog/backlog-overview\.md|/backlog/01-backlog-overview.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/backlog/epic-tree\.md|/backlog/02-epic-tree.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/backlog/capability-to-epic-map\.md|/backlog/03-capability-to-epic-map.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/backlog/dependency-map\.md|/backlog/04-dependency-map.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/backlog/delivery-slices\.md|/backlog/05-delivery-slices.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/backlog/sequencing-plan\.md|/backlog/06-sequencing-plan.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/backlog/acceptance-anchors\.md|/backlog/07-acceptance-anchors.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/backlog/planning-risks\.md|/backlog/08-planning-risks.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true

# Implementation
sed -i '' 's|/implementation/task-mapping\.md|/implementation/01-task-mapping.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/implementation/mutation-bounds\.md|/implementation/02-mutation-bounds.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/implementation/implementation-notes\.md|/implementation/03-implementation-notes.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/implementation/completion-evidence\.md|/implementation/04-completion-evidence.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/implementation/validation-hooks\.md|/implementation/05-validation-hooks.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/implementation/rollback-notes\.md|/implementation/06-rollback-notes.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true

# Refactor
sed -i '' 's|/refactor/preserved-behavior\.md|/refactor/01-preserved-behavior.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/refactor/refactor-scope\.md|/refactor/02-refactor-scope.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/refactor/structural-rationale\.md|/refactor/03-structural-rationale.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/refactor/regression-evidence\.md|/refactor/04-regression-evidence.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/refactor/contract-drift-check\.md|/refactor/05-contract-drift-check.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/refactor/no-feature-addition\.md|/refactor/06-no-feature-addition.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true

# Incident
sed -i '' 's|/incident/incident-frame\.md|/incident/01-incident-frame.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/incident/hypothesis-log\.md|/incident/02-hypothesis-log.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/incident/blast-radius-map\.md|/incident/03-blast-radius-map.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/incident/containment-plan\.md|/incident/04-containment-plan.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/incident/incident-decision-record\.md|/incident/05-incident-decision-record.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/incident/follow-up-verification\.md|/incident/06-follow-up-verification.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true

# Migration
sed -i '' 's|/migration/source-target-map\.md|/migration/01-source-target-map.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/migration/compatibility-matrix\.md|/migration/02-compatibility-matrix.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/migration/sequencing-plan\.md|/migration/03-sequencing-plan.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/migration/fallback-plan\.md|/migration/04-fallback-plan.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/migration/migration-verification-report\.md|/migration/05-migration-verification-report.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/migration/decision-record\.md|/migration/06-decision-record.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true

# Review
sed -i '' 's|/review/review-brief\.md|/review/01-review-brief.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/review/boundary-assessment\.md|/review/02-boundary-assessment.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/review/missing-evidence\.md|/review/03-missing-evidence.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/review/decision-impact\.md|/review/04-decision-impact.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/review/review-disposition\.md|/review/05-review-disposition.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true

# Verification
sed -i '' 's|/verification/invariants-checklist\.md|/verification/01-invariants-checklist.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/verification/contract-matrix\.md|/verification/02-contract-matrix.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/verification/adversarial-review\.md|/verification/03-adversarial-review.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/verification/verification-report\.md|/verification/04-verification-report.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/verification/unresolved-findings\.md|/verification/05-unresolved-findings.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true

# PR-review
sed -i '' 's|/pr-review/pr-analysis\.md|/pr-review/01-pr-analysis.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/pr-review/boundary-check\.md|/pr-review/02-boundary-check.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/pr-review/conventional-comments\.md|/pr-review/03-conventional-comments.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/pr-review/duplication-check\.md|/pr-review/04-duplication-check.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/pr-review/contract-drift\.md|/pr-review/05-contract-drift.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/pr-review/missing-tests\.md|/pr-review/06-missing-tests.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/pr-review/decision-impact\.md|/pr-review/07-decision-impact.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/pr-review/review-summary\.md|/pr-review/08-review-summary.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true

# Security-assessment
sed -i '' 's|/security-assessment/assessment-overview\.md|/security-assessment/01-assessment-overview.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/security-assessment/threat-model\.md|/security-assessment/02-threat-model.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/security-assessment/risk-register\.md|/security-assessment/03-risk-register.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/security-assessment/mitigations\.md|/security-assessment/04-mitigations.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/security-assessment/assumptions-and-gaps\.md|/security-assessment/05-assumptions-and-gaps.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/security-assessment/compliance-anchors\.md|/security-assessment/06-compliance-anchors.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/security-assessment/assessment-evidence\.md|/security-assessment/07-assessment-evidence.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true

# System-assessment  
sed -i '' 's|/system-assessment/assessment-overview\.md|/system-assessment/01-assessment-overview.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/system-assessment/coverage-map\.md|/system-assessment/02-coverage-map.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/system-assessment/asset-inventory\.md|/system-assessment/03-asset-inventory.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/system-assessment/functional-view\.md|/system-assessment/04-functional-view.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/system-assessment/component-view\.md|/system-assessment/05-component-view.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/system-assessment/deployment-view\.md|/system-assessment/06-deployment-view.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/system-assessment/technology-view\.md|/system-assessment/07-technology-view.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/system-assessment/integration-view\.md|/system-assessment/08-integration-view.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/system-assessment/risk-register\.md|/system-assessment/09-risk-register.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/system-assessment/assessment-evidence\.md|/system-assessment/10-assessment-evidence.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true

# Supply-chain-analysis
sed -i '' 's|/supply-chain-analysis/analysis-overview\.md|/supply-chain-analysis/01-analysis-overview.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/supply-chain-analysis/sbom-bundle\.md|/supply-chain-analysis/02-sbom-bundle.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/supply-chain-analysis/vulnerability-triage\.md|/supply-chain-analysis/03-vulnerability-triage.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/supply-chain-analysis/license-compliance\.md|/supply-chain-analysis/04-license-compliance.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/supply-chain-analysis/legacy-posture\.md|/supply-chain-analysis/05-legacy-posture.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/supply-chain-analysis/policy-decisions\.md|/supply-chain-analysis/06-policy-decisions.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true
sed -i '' 's|/supply-chain-analysis/analysis-evidence\.md|/supply-chain-analysis/07-analysis-evidence.md|g' tests/*.rs tests/**/*.rs 2>/dev/null || true


# --- Category 2: .join("SLUG.md") patterns for artifact file paths ---
# These use unique artifact slugs that only appear as output artifact names.
# We handle them per-file based on the mode context.

# Architecture tests - .join() for architecture artifacts
for f in tests/architecture_c4_run.rs tests/integration/architecture_run.rs tests/architecture_037_clarification_readiness.rs; do
    [ -f "$f" ] || continue
    sed -i '' 's|\.join("architecture-overview\.md")|.join("01-architecture-overview.md")|g' "$f"
    sed -i '' 's|\.join("architecture-decisions\.md")|.join("02-architecture-decisions.md")|g' "$f"
    sed -i '' 's|\.join("invariants\.md")|.join("03-invariants.md")|g' "$f"
    sed -i '' 's|\.join("tradeoff-matrix\.md")|.join("04-tradeoff-matrix.md")|g' "$f"
    sed -i '' 's|\.join("boundary-map\.md")|.join("05-boundary-map.md")|g' "$f"
    sed -i '' 's|\.join("context-map\.md")|.join("06-context-map.md")|g' "$f"
    sed -i '' 's|\.join("readiness-assessment\.md")|.join("07-readiness-assessment.md")|g' "$f"
    sed -i '' 's|\.join("system-context\.md")|.join("08-system-context.md")|g' "$f"
    sed -i '' 's|\.join("system-context\.mmd")|.join("09-system-context.mmd")|g' "$f"
    sed -i '' 's|\.join("container-view\.md")|.join("10-container-view.md")|g' "$f"
    sed -i '' 's|\.join("container-view\.mmd")|.join("11-container-view.mmd")|g' "$f"
    sed -i '' 's|\.join("deployment-view\.md")|.join("12-deployment-view.md")|g' "$f"
    sed -i '' 's|\.join("deployment-view\.mmd")|.join("13-deployment-view.mmd")|g' "$f"
    sed -i '' 's|\.join("view-manifest\.json")|.join("14-view-manifest.json")|g' "$f"
    sed -i '' 's|\.join("packet-metadata\.json")|.join("15-packet-metadata.json")|g' "$f"
    sed -i '' 's|\.join("component-view\.md")|.join("16-component-view.md")|g' "$f"
    sed -i '' 's|\.join("component-view\.mmd")|.join("17-component-view.mmd")|g' "$f"
    sed -i '' 's|\.join("dynamic-view\.md")|.join("18-dynamic-view.md")|g' "$f"
    sed -i '' 's|\.join("dynamic-view\.mmd")|.join("19-dynamic-view.mmd")|g' "$f"
done

# System-shaping tests - .join()
for f in tests/integration/system_shaping_run.rs; do
    [ -f "$f" ] || continue
    sed -i '' 's|\.join("system-shape\.md")|.join("01-system-shape.md")|g' "$f"
    sed -i '' 's|\.join("domain-model\.md")|.join("02-domain-model.md")|g' "$f"
    sed -i '' 's|\.join("architecture-outline\.md")|.join("03-architecture-outline.md")|g' "$f"
    sed -i '' 's|\.join("capability-map\.md")|.join("04-capability-map.md")|g' "$f"
    sed -i '' 's|\.join("delivery-options\.md")|.join("05-delivery-options.md")|g' "$f"
    sed -i '' 's|\.join("risk-hotspots\.md")|.join("06-risk-hotspots.md")|g' "$f"
done

# Change tests - .join()
for f in tests/integration/change_run.rs; do
    [ -f "$f" ] || continue
    sed -i '' 's|\.join("system-slice\.md")|.join("01-system-slice.md")|g' "$f"
    sed -i '' 's|\.join("legacy-invariants\.md")|.join("02-legacy-invariants.md")|g' "$f"
    sed -i '' 's|\.join("change-surface\.md")|.join("03-change-surface.md")|g' "$f"
    sed -i '' 's|\.join("implementation-plan\.md")|.join("04-implementation-plan.md")|g' "$f"
    sed -i '' 's|\.join("validation-strategy\.md")|.join("05-validation-strategy.md")|g' "$f"
    sed -i '' 's|\.join("decision-record\.md")|.join("06-decision-record.md")|g' "$f"
done

# Implementation tests - .join()
for f in tests/integration/implementation_run.rs; do
    [ -f "$f" ] || continue
    sed -i '' 's|\.join("task-mapping\.md")|.join("01-task-mapping.md")|g' "$f"
    sed -i '' 's|\.join("mutation-bounds\.md")|.join("02-mutation-bounds.md")|g' "$f"
    sed -i '' 's|\.join("implementation-notes\.md")|.join("03-implementation-notes.md")|g' "$f"
    sed -i '' 's|\.join("completion-evidence\.md")|.join("04-completion-evidence.md")|g' "$f"
    sed -i '' 's|\.join("validation-hooks\.md")|.join("05-validation-hooks.md")|g' "$f"
    sed -i '' 's|\.join("rollback-notes\.md")|.join("06-rollback-notes.md")|g' "$f"
done

# Incident tests - .join()
for f in tests/integration/incident_run.rs; do
    [ -f "$f" ] || continue
    sed -i '' 's|\.join("incident-frame\.md")|.join("01-incident-frame.md")|g' "$f"
    sed -i '' 's|\.join("hypothesis-log\.md")|.join("02-hypothesis-log.md")|g' "$f"
    sed -i '' 's|\.join("blast-radius-map\.md")|.join("03-blast-radius-map.md")|g' "$f"
    sed -i '' 's|\.join("containment-plan\.md")|.join("04-containment-plan.md")|g' "$f"
    sed -i '' 's|\.join("incident-decision-record\.md")|.join("05-incident-decision-record.md")|g' "$f"
    sed -i '' 's|\.join("follow-up-verification\.md")|.join("06-follow-up-verification.md")|g' "$f"
done

# Review tests - .join()
for f in tests/integration/review_run.rs; do
    [ -f "$f" ] || continue
    sed -i '' 's|\.join("review-brief\.md")|.join("01-review-brief.md")|g' "$f"
    sed -i '' 's|\.join("boundary-assessment\.md")|.join("02-boundary-assessment.md")|g' "$f"
    sed -i '' 's|\.join("missing-evidence\.md")|.join("03-missing-evidence.md")|g' "$f"
    sed -i '' 's|\.join("decision-impact\.md")|.join("04-decision-impact.md")|g' "$f"
    sed -i '' 's|\.join("review-disposition\.md")|.join("05-review-disposition.md")|g' "$f"
done

# PR-review tests - .join()
for f in tests/integration/pr_review_run.rs tests/integration/pr_review_publish.rs; do
    [ -f "$f" ] || continue
    sed -i '' 's|\.join("pr-analysis\.md")|.join("01-pr-analysis.md")|g' "$f"
    sed -i '' 's|\.join("boundary-check\.md")|.join("02-boundary-check.md")|g' "$f"
    sed -i '' 's|\.join("conventional-comments\.md")|.join("03-conventional-comments.md")|g' "$f"
    sed -i '' 's|\.join("duplication-check\.md")|.join("04-duplication-check.md")|g' "$f"
    sed -i '' 's|\.join("contract-drift\.md")|.join("05-contract-drift.md")|g' "$f"
    sed -i '' 's|\.join("missing-tests\.md")|.join("06-missing-tests.md")|g' "$f"
    sed -i '' 's|\.join("decision-impact\.md")|.join("07-decision-impact.md")|g' "$f"
    sed -i '' 's|\.join("review-summary\.md")|.join("08-review-summary.md")|g' "$f"
done

# Migration tests - .join()
for f in tests/integration/migration_publish.rs; do
    [ -f "$f" ] || continue
    sed -i '' 's|\.join("source-target-map\.md")|.join("01-source-target-map.md")|g' "$f"
    sed -i '' 's|\.join("compatibility-matrix\.md")|.join("02-compatibility-matrix.md")|g' "$f"
    sed -i '' 's|\.join("sequencing-plan\.md")|.join("03-sequencing-plan.md")|g' "$f"
    sed -i '' 's|\.join("fallback-plan\.md")|.join("04-fallback-plan.md")|g' "$f"
    sed -i '' 's|\.join("migration-verification-report\.md")|.join("05-migration-verification-report.md")|g' "$f"
    sed -i '' 's|\.join("decision-record\.md")|.join("06-decision-record.md")|g' "$f"
done

# Incident publish - .join()
for f in tests/integration/incident_publish.rs; do
    [ -f "$f" ] || continue
    sed -i '' 's|\.join("follow-up-verification\.md")|.join("06-follow-up-verification.md")|g' "$f"
done

# Supply-chain-analysis tests - .join()
for f in tests/supply_chain_analysis_direct_runtime.rs tests/integration/supply_chain_analysis_run.rs; do
    [ -f "$f" ] || continue
    sed -i '' 's|\.join("analysis-overview\.md")|.join("01-analysis-overview.md")|g' "$f"
    sed -i '' 's|\.join("sbom-bundle\.md")|.join("02-sbom-bundle.md")|g' "$f"
    sed -i '' 's|\.join("vulnerability-triage\.md")|.join("03-vulnerability-triage.md")|g' "$f"
    sed -i '' 's|\.join("license-compliance\.md")|.join("04-license-compliance.md")|g' "$f"
    sed -i '' 's|\.join("legacy-posture\.md")|.join("05-legacy-posture.md")|g' "$f"
    sed -i '' 's|\.join("policy-decisions\.md")|.join("06-policy-decisions.md")|g' "$f"
    sed -i '' 's|\.join("analysis-evidence\.md")|.join("07-analysis-evidence.md")|g' "$f"
done

# Security-assessment tests - .join()
for f in tests/security_assessment_direct_runtime.rs; do
    [ -f "$f" ] || continue
    sed -i '' 's|\.join("assessment-overview\.md")|.join("01-assessment-overview.md")|g' "$f"
    sed -i '' 's|\.join("threat-model\.md")|.join("02-threat-model.md")|g' "$f"
    sed -i '' 's|\.join("risk-register\.md")|.join("03-risk-register.md")|g' "$f"
    sed -i '' 's|\.join("mitigations\.md")|.join("04-mitigations.md")|g' "$f"
    sed -i '' 's|\.join("assumptions-and-gaps\.md")|.join("05-assumptions-and-gaps.md")|g' "$f"
    sed -i '' 's|\.join("compliance-anchors\.md")|.join("06-compliance-anchors.md")|g' "$f"
    sed -i '' 's|\.join("assessment-evidence\.md")|.join("07-assessment-evidence.md")|g' "$f"
done

# Discovery authoring run - .join()
for f in tests/discovery_authoring_run.rs; do
    [ -f "$f" ] || continue
    sed -i '' 's|\.join("problem-map\.md")|.join("01-problem-map.md")|g' "$f"
done

# Requirements authoring run - .join()
for f in tests/requirements_authoring_run.rs; do
    [ -f "$f" ] || continue
    sed -i '' 's|\.join("problem-statement\.md")|.join("01-problem-statement.md")|g' "$f"
done

# Change authoring run - .join()
for f in tests/change_authoring_run.rs; do
    [ -f "$f" ] || continue
    sed -i '' 's|\.join("change-surface\.md")|.join("03-change-surface.md")|g' "$f"
    sed -i '' 's|\.join("implementation-plan\.md")|.join("04-implementation-plan.md")|g' "$f"
    sed -i '' 's|\.join("decision-record\.md")|.join("06-decision-record.md")|g' "$f"
done

# Backlog run - .join()
for f in tests/backlog_run.rs; do
    [ -f "$f" ] || continue
    sed -i '' 's|\.join("delivery-slices\.md")|.join("05-delivery-slices.md")|g' "$f"
done

# run_lookup - .join() for requirements and implementation and backlog artifacts
for f in tests/integration/run_lookup.rs; do
    [ -f "$f" ] || continue
    sed -i '' 's|\.join("problem-statement\.md")|.join("01-problem-statement.md")|g' "$f"
    sed -i '' 's|\.join("task-mapping\.md")|.join("01-task-mapping.md")|g' "$f"
    sed -i '' 's|\.join("backlog-overview\.md")|.join("01-backlog-overview.md")|g' "$f"
done

# Published file paths for supply-chain and security-assessment
# These are published to docs/ directories and use the artifact file_name
for f in tests/supply_chain_analysis_direct_runtime.rs; do
    [ -f "$f" ] || continue
    # Published overview path uses the artifact filename
    sed -i '' 's|\.join("analysis-overview\.md");|.join("01-analysis-overview.md");|g' "$f"
done
for f in tests/security_assessment_direct_runtime.rs; do
    [ -f "$f" ] || continue
    sed -i '' 's|\.join("assessment-overview\.md");|.join("01-assessment-overview.md");|g' "$f"
done


# --- Category 3: .join() in string literal arrays and for loops ---
# Architecture artifact lists in integration tests (for artifact in [...])
for f in tests/integration/architecture_run.rs; do
    [ -f "$f" ] || continue
    # These are in a for loop checking artifact_root.join(artifact).exists()
    sed -i '' 's|"architecture-overview\.md",|"01-architecture-overview.md",|g' "$f"
    sed -i '' 's|"architecture-decisions\.md",|"02-architecture-decisions.md",|g' "$f"
    sed -i '' 's|"invariants\.md",|"03-invariants.md",|g' "$f"
    sed -i '' 's|"tradeoff-matrix\.md",|"04-tradeoff-matrix.md",|g' "$f"
    sed -i '' 's|"boundary-map\.md",|"05-boundary-map.md",|g' "$f"
    sed -i '' 's|"context-map\.md",|"06-context-map.md",|g' "$f"
    sed -i '' 's|"readiness-assessment\.md",|"07-readiness-assessment.md",|g' "$f"
    sed -i '' 's|"system-context\.md",|"08-system-context.md",|g' "$f"
    sed -i '' 's|"system-context\.mmd",|"09-system-context.mmd",|g' "$f"
    sed -i '' 's|"container-view\.md",|"10-container-view.md",|g' "$f"
    sed -i '' 's|"container-view\.mmd",|"11-container-view.mmd",|g' "$f"
    sed -i '' 's|"deployment-view\.md",|"12-deployment-view.md",|g' "$f"
    sed -i '' 's|"deployment-view\.mmd",|"13-deployment-view.mmd",|g' "$f"
    sed -i '' 's|"view-manifest\.json",|"14-view-manifest.json",|g' "$f"
    sed -i '' 's|"packet-metadata\.json",|"15-packet-metadata.json",|g' "$f"
    sed -i '' 's|"component-view\.md",|"16-component-view.md",|g' "$f"
    sed -i '' 's|"component-view\.mmd",|"17-component-view.mmd",|g' "$f"
done

# System-shaping artifact lists
for f in tests/integration/system_shaping_run.rs; do
    [ -f "$f" ] || continue
    sed -i '' 's|"system-shape\.md",|"01-system-shape.md",|g' "$f"
    sed -i '' 's|"domain-model\.md",|"02-domain-model.md",|g' "$f"
    sed -i '' 's|"architecture-outline\.md",|"03-architecture-outline.md",|g' "$f"
    sed -i '' 's|"capability-map\.md",|"04-capability-map.md",|g' "$f"
    sed -i '' 's|"delivery-options\.md",|"05-delivery-options.md",|g' "$f"
    sed -i '' 's|"risk-hotspots\.md",|"06-risk-hotspots.md",|g' "$f"
done

# Change artifact lists
for f in tests/integration/change_run.rs; do
    [ -f "$f" ] || continue
    sed -i '' 's|"system-slice\.md",|"01-system-slice.md",|g' "$f"
    sed -i '' 's|"legacy-invariants\.md",|"02-legacy-invariants.md",|g' "$f"
    sed -i '' 's|"change-surface\.md",|"03-change-surface.md",|g' "$f"
    sed -i '' 's|"implementation-plan\.md",|"04-implementation-plan.md",|g' "$f"
    sed -i '' 's|"validation-strategy\.md",|"05-validation-strategy.md",|g' "$f"
    sed -i '' 's|"decision-record\.md",|"06-decision-record.md",|g' "$f"
done

echo "=== Replacements complete ==="
echo "Now check for remaining unprefixed artifact slugs in assertions..."
