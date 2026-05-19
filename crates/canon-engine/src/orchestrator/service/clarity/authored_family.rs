use crate::domain::mode::Mode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AuthoredClarityFamily {
    Planning,
    Execution,
    Assessment,
}

impl AuthoredClarityFamily {
    pub(super) fn label(self) -> &'static str {
        match self {
            Self::Planning => "planning",
            Self::Execution => "execution",
            Self::Assessment => "assessment",
        }
    }
}

pub(super) fn authored_clarity_family(mode: Mode) -> AuthoredClarityFamily {
    match mode {
        Mode::SystemShaping | Mode::Architecture | Mode::Change | Mode::Backlog => {
            AuthoredClarityFamily::Planning
        }
        Mode::Implementation | Mode::Refactor | Mode::Migration => AuthoredClarityFamily::Execution,
        Mode::Review
        | Mode::Verification
        | Mode::Incident
        | Mode::SecurityAssessment
        | Mode::SystemAssessment
        | Mode::DomainLanguage
        | Mode::DomainModel => AuthoredClarityFamily::Assessment,
        Mode::Requirements | Mode::Discovery | Mode::PrReview | Mode::SupplyChainAnalysis => {
            AuthoredClarityFamily::Planning
        }
    }
}

struct AuthoredFamilyMarkers {
    primary: &'static [&'static str],
    boundary: &'static [&'static str],
    support: &'static [&'static str],
    decision: &'static [&'static str],
    tradeoff: &'static [&'static str],
    option: &'static [&'static str],
    gap: &'static [&'static str],
}

impl AuthoredFamilyMarkers {
    const fn new(
        primary: &'static [&'static str],
        boundary: &'static [&'static str],
        support: &'static [&'static str],
        decision: &'static [&'static str],
        tradeoff: &'static [&'static str],
        option: &'static [&'static str],
        gap: &'static [&'static str],
    ) -> Self {
        Self { primary, boundary, support, decision, tradeoff, option, gap }
    }
}

struct AuthoredFamilyFallbacks {
    primary: &'static str,
    boundary: &'static str,
    support: &'static str,
    decision: &'static str,
}

impl AuthoredFamilyFallbacks {
    const fn new(
        primary: &'static str,
        boundary: &'static str,
        support: &'static str,
        decision: &'static str,
    ) -> Self {
        Self { primary, boundary, support, decision }
    }
}

struct AuthoredFamilyMessages {
    missing_primary_subject: &'static str,
    missing_boundary: &'static str,
    missing_support: &'static str,
}

impl AuthoredFamilyMessages {
    const fn new(
        missing_primary_subject: &'static str,
        missing_boundary: &'static str,
        missing_support: &'static str,
    ) -> Self {
        Self { missing_primary_subject, missing_boundary, missing_support }
    }
}

struct AuthoredFamilyPrompts {
    target: &'static str,
    boundary: &'static str,
    support: &'static str,
}

impl AuthoredFamilyPrompts {
    const fn new(target: &'static str, boundary: &'static str, support: &'static str) -> Self {
        Self { target, boundary, support }
    }
}

struct AuthoredFamilyProfile {
    markers: AuthoredFamilyMarkers,
    fallbacks: AuthoredFamilyFallbacks,
    messages: AuthoredFamilyMessages,
    prompts: AuthoredFamilyPrompts,
}

impl AuthoredFamilyProfile {
    const fn new(
        markers: AuthoredFamilyMarkers,
        fallbacks: AuthoredFamilyFallbacks,
        messages: AuthoredFamilyMessages,
        prompts: AuthoredFamilyPrompts,
    ) -> Self {
        Self { markers, fallbacks, messages, prompts }
    }
}

const PLANNING_PROFILE: AuthoredFamilyProfile = AuthoredFamilyProfile::new(
    AuthoredFamilyMarkers::new(
        &["system shape", "decision", "delivery intent", "intended change", "problem"],
        &[
            "constraints",
            "boundary decisions",
            "candidate boundaries",
            "change surface",
            "system slice",
            "desired granularity",
            "planning horizon",
            "domain slice",
            "excluded areas",
            "out of scope",
        ],
        &[
            "rationale",
            "decision evidence",
            "decision drivers",
            "sequencing rationale",
            "source references",
            "validation strategy",
            "independent checks",
        ],
        &[
            "selected boundaries",
            "recommendation",
            "decision record",
            "recommended direction",
            "recommended path",
            "decision",
        ],
        &[
            "boundary tradeoffs",
            "tradeoffs",
            "consequences",
            "pros",
            "cons",
            "cross-context risks",
            "risk per phase",
        ],
        &[
            "structural options",
            "options considered",
            "options",
            "candidate bounded contexts",
            "candidate boundaries",
            "why not the others",
        ],
        &[
            "boundary risks and open questions",
            "unresolved risks",
            "unresolved questions",
            "gaps",
            "open questions",
        ],
    ),
    AuthoredFamilyFallbacks::new(
        "NOT CAPTURED - Provide a planning subject such as `## System Shape`, `## Decision`, `## Delivery Intent`, or `## Intended Change`.",
        "NOT CAPTURED - Provide a planning boundary such as `## Constraints`, `## Boundary Decisions`, `## Candidate Boundaries`, or `## Change Surface`.",
        "NOT CAPTURED - Provide explicit rationale or support such as `## Rationale`, `## Decision Evidence`, or `## Decision Drivers`.",
        "NOT CAPTURED - Provide a `## Recommendation`, `## Selected Boundaries`, or `## Decision` section if the planning packet is already materially closed.",
    ),
    AuthoredFamilyMessages::new(
        "Planning intent is missing; Canon cannot tell what bounded problem or decision this packet is meant to drive.",
        "Planning boundary is missing; the packet does not yet state the scope, slice, or constraint Canon must preserve.",
        "Planning support is missing; the packet has no explicit rationale or decision evidence anchoring its direction.",
    ),
    AuthoredFamilyPrompts::new(
        "What concrete planning target or decision should this packet drive?",
        "Which boundary, slice, or scope limit must this packet preserve?",
        "What explicit rationale or evidence supports the chosen planning direction?",
    ),
);

const EXECUTION_PROFILE: AuthoredFamilyProfile = AuthoredFamilyProfile::new(
    AuthoredFamilyMarkers::new(
        &["task mapping", "refactor scope", "current state", "target state"],
        &[
            "bounded changes",
            "mutation bounds",
            "allowed paths",
            "transition boundaries",
            "rollback triggers",
            "fallback paths",
            "refactor scope",
        ],
        &[
            "decision evidence",
            "completion evidence",
            "safety-net evidence",
            "verification checks",
            "independent checks",
            "rollback steps",
            "contract drift",
        ],
        &["recommendation", "decision", "migration decisions", "approval notes"],
        &[
            "adoption implications",
            "tradeoff analysis",
            "remaining risks",
            "residual risks",
            "temporary incompatibilities",
        ],
        &["candidate frameworks", "options matrix", "parallelizable work", "why not the others"],
        &[
            "remaining risks",
            "residual risks",
            "deferred decisions",
            "regression findings",
            "feature audit",
        ],
    ),
    AuthoredFamilyFallbacks::new(
        "NOT CAPTURED - Provide an execution subject such as `## Task Mapping`, `## Refactor Scope`, `## Current State`, or `## Target State`.",
        "NOT CAPTURED - Provide a mutation boundary such as `## Bounded Changes`, `## Mutation Bounds`, `## Allowed Paths`, or `## Transition Boundaries`.",
        "NOT CAPTURED - Provide execution support such as `## Decision Evidence`, `## Safety-Net Evidence`, `## Verification Checks`, or `## Rollback Steps`.",
        "NOT CAPTURED - Provide a `## Recommendation`, `## Decision`, or `## Migration Decisions` section if the execution plan is already decided.",
    ),
    AuthoredFamilyMessages::new(
        "Execution target is missing; Canon cannot tell which bounded change, refactor, or migration surface this packet controls.",
        "Mutation boundary is missing; execution output would otherwise overreach beyond the authored scope.",
        "Execution evidence is missing; the packet lacks safety-net, rollback, or validation support for the proposed work.",
    ),
    AuthoredFamilyPrompts::new(
        "Which implementation, refactor, or migration surface is actually in scope?",
        "Which paths, mutation bounds, or transition boundaries are explicitly allowed?",
        "What safety-net, validation, or rollback evidence makes this execution plan safe?",
    ),
);

const ASSESSMENT_PROFILE: AuthoredFamilyProfile = AuthoredFamilyProfile::new(
    AuthoredFamilyMarkers::new(
        &[
            "review target",
            "claims under test",
            "incident scope",
            "assessment scope",
            "assessment objective",
        ],
        &[
            "boundary findings",
            "assessment scope",
            "in-scope assets",
            "trust boundaries",
            "incident scope",
            "assessed views",
            "impacted surfaces",
            "scope limits",
            "out of scope",
        ],
        &[
            "evidence basis",
            "known facts",
            "observed findings",
            "inferred findings",
            "evidence sources",
            "collection priorities",
            "control families",
            "verified claims",
        ],
        &[
            "final disposition",
            "overall verdict",
            "verification outcome",
            "decision impact",
            "immediate actions",
        ],
        &[
            "accepted risks",
            "reversibility concerns",
            "tradeoffs",
            "likelihood and impact",
            "impact notes",
        ],
        &[],
        &[
            "missing evidence",
            "evidence gaps",
            "assessment gaps",
            "confidence and unknowns",
            "open findings",
            "risk findings",
            "deferred verification",
            "unobservable surfaces",
        ],
    ),
    AuthoredFamilyFallbacks::new(
        "NOT CAPTURED - Provide an assessment target such as `## Review Target`, `## Claims Under Test`, `## Incident Scope`, or `## Assessment Scope`.",
        "NOT CAPTURED - Provide an assessment boundary such as `## Boundary Findings`, `## Assessment Scope`, `## Assessed Views`, or `## Impacted Surfaces`.",
        "NOT CAPTURED - Provide an evidence basis such as `## Evidence Basis`, `## Known Facts`, `## Observed Findings`, or `## Evidence Sources`.",
        "NOT CAPTURED - Provide a `## Final Disposition`, `## Overall Verdict`, or equivalent conclusion section.",
    ),
    AuthoredFamilyMessages::new(
        "Assessment target is missing; Canon cannot tell what claim, incident, or system slice this packet is evaluating.",
        "Assessment boundary is missing; the packet does not yet say which evidence surface is actually in scope.",
        "Evidence basis is missing; assessment output would otherwise rely on inferred confidence instead of authored support.",
    ),
    AuthoredFamilyPrompts::new(
        "What exact review, verification, incident, or assessment target is under examination?",
        "Which evidence surfaces are in scope, and which ones are explicitly excluded?",
        "What evidence basis supports this packet instead of inferred reasoning?",
    ),
);

fn authored_profile(family: AuthoredClarityFamily) -> &'static AuthoredFamilyProfile {
    match family {
        AuthoredClarityFamily::Planning => &PLANNING_PROFILE,
        AuthoredClarityFamily::Execution => &EXECUTION_PROFILE,
        AuthoredClarityFamily::Assessment => &ASSESSMENT_PROFILE,
    }
}

pub(super) fn authored_primary_markers(family: AuthoredClarityFamily) -> &'static [&'static str] {
    authored_profile(family).markers.primary
}

pub(super) fn authored_boundary_markers(family: AuthoredClarityFamily) -> &'static [&'static str] {
    authored_profile(family).markers.boundary
}

pub(super) fn authored_support_markers(family: AuthoredClarityFamily) -> &'static [&'static str] {
    authored_profile(family).markers.support
}

pub(super) fn authored_decision_markers(family: AuthoredClarityFamily) -> &'static [&'static str] {
    authored_profile(family).markers.decision
}

pub(super) fn authored_tradeoff_markers(family: AuthoredClarityFamily) -> &'static [&'static str] {
    authored_profile(family).markers.tradeoff
}

pub(super) fn authored_option_markers(family: AuthoredClarityFamily) -> &'static [&'static str] {
    authored_profile(family).markers.option
}

pub(super) fn authored_gap_markers(family: AuthoredClarityFamily) -> &'static [&'static str] {
    authored_profile(family).markers.gap
}

pub(super) fn authored_primary_fallback(family: AuthoredClarityFamily) -> &'static str {
    authored_profile(family).fallbacks.primary
}

pub(super) fn authored_boundary_fallback(family: AuthoredClarityFamily) -> &'static str {
    authored_profile(family).fallbacks.boundary
}

pub(super) fn authored_support_fallback(family: AuthoredClarityFamily) -> &'static str {
    authored_profile(family).fallbacks.support
}

pub(super) fn authored_decision_fallback(family: AuthoredClarityFamily) -> &'static str {
    authored_profile(family).fallbacks.decision
}

pub(super) fn authored_preserved_fallback() -> &'static str {
    "NOT CAPTURED - Provide a preservation boundary such as `## Preserved Behavior`, `## Guaranteed Compatibility`, or `## Invariant Checks`."
}

pub(super) fn authored_preserved_markers() -> &'static [&'static str] {
    &[
        "preserved behavior",
        "guaranteed compatibility",
        "coexistence rules",
        "invariant checks",
        "operational constraints",
    ]
}

pub(super) fn authored_missing_primary_subject_message(family: AuthoredClarityFamily) -> String {
    authored_profile(family).messages.missing_primary_subject.to_string()
}

pub(super) fn authored_missing_boundary_message(family: AuthoredClarityFamily) -> String {
    authored_profile(family).messages.missing_boundary.to_string()
}

pub(super) fn authored_missing_support_message(family: AuthoredClarityFamily) -> String {
    authored_profile(family).messages.missing_support.to_string()
}

pub(super) fn authored_target_prompt(family: AuthoredClarityFamily) -> &'static str {
    authored_profile(family).prompts.target
}

pub(super) fn authored_boundary_prompt(family: AuthoredClarityFamily) -> &'static str {
    authored_profile(family).prompts.boundary
}

pub(super) fn authored_support_prompt(family: AuthoredClarityFamily) -> &'static str {
    authored_profile(family).prompts.support
}
