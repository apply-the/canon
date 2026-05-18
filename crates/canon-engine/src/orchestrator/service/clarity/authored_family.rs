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

pub(super) fn authored_primary_markers(family: AuthoredClarityFamily) -> &'static [&'static str] {
    match family {
        AuthoredClarityFamily::Planning => {
            &["system shape", "decision", "delivery intent", "intended change", "problem"]
        }
        AuthoredClarityFamily::Execution => {
            &["task mapping", "refactor scope", "current state", "target state"]
        }
        AuthoredClarityFamily::Assessment => &[
            "review target",
            "claims under test",
            "incident scope",
            "assessment scope",
            "assessment objective",
        ],
    }
}

pub(super) fn authored_boundary_markers(family: AuthoredClarityFamily) -> &'static [&'static str] {
    match family {
        AuthoredClarityFamily::Planning => &[
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
        AuthoredClarityFamily::Execution => &[
            "bounded changes",
            "mutation bounds",
            "allowed paths",
            "transition boundaries",
            "rollback triggers",
            "fallback paths",
            "refactor scope",
        ],
        AuthoredClarityFamily::Assessment => &[
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
    }
}

pub(super) fn authored_support_markers(family: AuthoredClarityFamily) -> &'static [&'static str] {
    match family {
        AuthoredClarityFamily::Planning => &[
            "rationale",
            "decision evidence",
            "decision drivers",
            "sequencing rationale",
            "source references",
            "validation strategy",
            "independent checks",
        ],
        AuthoredClarityFamily::Execution => &[
            "decision evidence",
            "completion evidence",
            "safety-net evidence",
            "verification checks",
            "independent checks",
            "rollback steps",
            "contract drift",
        ],
        AuthoredClarityFamily::Assessment => &[
            "evidence basis",
            "known facts",
            "observed findings",
            "inferred findings",
            "evidence sources",
            "collection priorities",
            "control families",
            "verified claims",
        ],
    }
}

pub(super) fn authored_decision_markers(family: AuthoredClarityFamily) -> &'static [&'static str] {
    match family {
        AuthoredClarityFamily::Planning => &[
            "selected boundaries",
            "recommendation",
            "decision record",
            "recommended direction",
            "recommended path",
            "decision",
        ],
        AuthoredClarityFamily::Execution => {
            &["recommendation", "decision", "migration decisions", "approval notes"]
        }
        AuthoredClarityFamily::Assessment => &[
            "final disposition",
            "overall verdict",
            "verification outcome",
            "decision impact",
            "immediate actions",
        ],
    }
}

pub(super) fn authored_tradeoff_markers(family: AuthoredClarityFamily) -> &'static [&'static str] {
    match family {
        AuthoredClarityFamily::Planning => &[
            "boundary tradeoffs",
            "tradeoffs",
            "consequences",
            "pros",
            "cons",
            "cross-context risks",
            "risk per phase",
        ],
        AuthoredClarityFamily::Execution => &[
            "adoption implications",
            "tradeoff analysis",
            "remaining risks",
            "residual risks",
            "temporary incompatibilities",
        ],
        AuthoredClarityFamily::Assessment => &[
            "accepted risks",
            "reversibility concerns",
            "tradeoffs",
            "likelihood and impact",
            "impact notes",
        ],
    }
}

pub(super) fn authored_option_markers(family: AuthoredClarityFamily) -> &'static [&'static str] {
    match family {
        AuthoredClarityFamily::Planning => &[
            "structural options",
            "options considered",
            "options",
            "candidate bounded contexts",
            "candidate boundaries",
            "why not the others",
        ],
        AuthoredClarityFamily::Execution => {
            &["candidate frameworks", "options matrix", "parallelizable work", "why not the others"]
        }
        AuthoredClarityFamily::Assessment => &[],
    }
}

pub(super) fn authored_gap_markers(family: AuthoredClarityFamily) -> &'static [&'static str] {
    match family {
        AuthoredClarityFamily::Planning => &[
            "boundary risks and open questions",
            "unresolved risks",
            "unresolved questions",
            "gaps",
            "open questions",
        ],
        AuthoredClarityFamily::Execution => &[
            "remaining risks",
            "residual risks",
            "deferred decisions",
            "regression findings",
            "feature audit",
        ],
        AuthoredClarityFamily::Assessment => &[
            "missing evidence",
            "evidence gaps",
            "assessment gaps",
            "confidence and unknowns",
            "open findings",
            "risk findings",
            "deferred verification",
            "unobservable surfaces",
        ],
    }
}

pub(super) fn authored_primary_fallback(family: AuthoredClarityFamily) -> &'static str {
    match family {
        AuthoredClarityFamily::Planning => {
            "NOT CAPTURED - Provide a planning subject such as `## System Shape`, `## Decision`, `## Delivery Intent`, or `## Intended Change`."
        }
        AuthoredClarityFamily::Execution => {
            "NOT CAPTURED - Provide an execution subject such as `## Task Mapping`, `## Refactor Scope`, `## Current State`, or `## Target State`."
        }
        AuthoredClarityFamily::Assessment => {
            "NOT CAPTURED - Provide an assessment target such as `## Review Target`, `## Claims Under Test`, `## Incident Scope`, or `## Assessment Scope`."
        }
    }
}

pub(super) fn authored_boundary_fallback(family: AuthoredClarityFamily) -> &'static str {
    match family {
        AuthoredClarityFamily::Planning => {
            "NOT CAPTURED - Provide a planning boundary such as `## Constraints`, `## Boundary Decisions`, `## Candidate Boundaries`, or `## Change Surface`."
        }
        AuthoredClarityFamily::Execution => {
            "NOT CAPTURED - Provide a mutation boundary such as `## Bounded Changes`, `## Mutation Bounds`, `## Allowed Paths`, or `## Transition Boundaries`."
        }
        AuthoredClarityFamily::Assessment => {
            "NOT CAPTURED - Provide an assessment boundary such as `## Boundary Findings`, `## Assessment Scope`, `## Assessed Views`, or `## Impacted Surfaces`."
        }
    }
}

pub(super) fn authored_support_fallback(family: AuthoredClarityFamily) -> &'static str {
    match family {
        AuthoredClarityFamily::Planning => {
            "NOT CAPTURED - Provide explicit rationale or support such as `## Rationale`, `## Decision Evidence`, or `## Decision Drivers`."
        }
        AuthoredClarityFamily::Execution => {
            "NOT CAPTURED - Provide execution support such as `## Decision Evidence`, `## Safety-Net Evidence`, `## Verification Checks`, or `## Rollback Steps`."
        }
        AuthoredClarityFamily::Assessment => {
            "NOT CAPTURED - Provide an evidence basis such as `## Evidence Basis`, `## Known Facts`, `## Observed Findings`, or `## Evidence Sources`."
        }
    }
}

pub(super) fn authored_decision_fallback(family: AuthoredClarityFamily) -> &'static str {
    match family {
        AuthoredClarityFamily::Planning => {
            "NOT CAPTURED - Provide a `## Recommendation`, `## Selected Boundaries`, or `## Decision` section if the planning packet is already materially closed."
        }
        AuthoredClarityFamily::Execution => {
            "NOT CAPTURED - Provide a `## Recommendation`, `## Decision`, or `## Migration Decisions` section if the execution plan is already decided."
        }
        AuthoredClarityFamily::Assessment => {
            "NOT CAPTURED - Provide a `## Final Disposition`, `## Overall Verdict`, or equivalent conclusion section."
        }
    }
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
    match family {
        AuthoredClarityFamily::Planning => {
            "Planning intent is missing; Canon cannot tell what bounded problem or decision this packet is meant to drive.".to_string()
        }
        AuthoredClarityFamily::Execution => {
            "Execution target is missing; Canon cannot tell which bounded change, refactor, or migration surface this packet controls.".to_string()
        }
        AuthoredClarityFamily::Assessment => {
            "Assessment target is missing; Canon cannot tell what claim, incident, or system slice this packet is evaluating.".to_string()
        }
    }
}

pub(super) fn authored_missing_boundary_message(family: AuthoredClarityFamily) -> String {
    match family {
        AuthoredClarityFamily::Planning => {
            "Planning boundary is missing; the packet does not yet state the scope, slice, or constraint Canon must preserve.".to_string()
        }
        AuthoredClarityFamily::Execution => {
            "Mutation boundary is missing; execution output would otherwise overreach beyond the authored scope.".to_string()
        }
        AuthoredClarityFamily::Assessment => {
            "Assessment boundary is missing; the packet does not yet say which evidence surface is actually in scope.".to_string()
        }
    }
}

pub(super) fn authored_missing_support_message(family: AuthoredClarityFamily) -> String {
    match family {
        AuthoredClarityFamily::Planning => {
            "Planning support is missing; the packet has no explicit rationale or decision evidence anchoring its direction.".to_string()
        }
        AuthoredClarityFamily::Execution => {
            "Execution evidence is missing; the packet lacks safety-net, rollback, or validation support for the proposed work.".to_string()
        }
        AuthoredClarityFamily::Assessment => {
            "Evidence basis is missing; assessment output would otherwise rely on inferred confidence instead of authored support.".to_string()
        }
    }
}

pub(super) fn authored_target_prompt(family: AuthoredClarityFamily) -> &'static str {
    match family {
        AuthoredClarityFamily::Planning => {
            "What concrete planning target or decision should this packet drive?"
        }
        AuthoredClarityFamily::Execution => {
            "Which implementation, refactor, or migration surface is actually in scope?"
        }
        AuthoredClarityFamily::Assessment => {
            "What exact review, verification, incident, or assessment target is under examination?"
        }
    }
}

pub(super) fn authored_boundary_prompt(family: AuthoredClarityFamily) -> &'static str {
    match family {
        AuthoredClarityFamily::Planning => {
            "Which boundary, slice, or scope limit must this packet preserve?"
        }
        AuthoredClarityFamily::Execution => {
            "Which paths, mutation bounds, or transition boundaries are explicitly allowed?"
        }
        AuthoredClarityFamily::Assessment => {
            "Which evidence surfaces are in scope, and which ones are explicitly excluded?"
        }
    }
}

pub(super) fn authored_support_prompt(family: AuthoredClarityFamily) -> &'static str {
    match family {
        AuthoredClarityFamily::Planning => {
            "What explicit rationale or evidence supports the chosen planning direction?"
        }
        AuthoredClarityFamily::Execution => {
            "What safety-net, validation, or rollback evidence makes this execution plan safe?"
        }
        AuthoredClarityFamily::Assessment => {
            "What evidence basis supports this packet instead of inferred reasoning?"
        }
    }
}
