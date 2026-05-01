# Contract: Clarity And Reasoning Evidence

## Scope

This contract defines the runtime-facing reasoning-evidence expectations for
feature 033 across clarity inspection, packet posture, and review-family
honesty.

## File-Backed Clarity Family

| Mode Group | Required Clarity Outcome | Maintainer Outcome |
|------------|--------------------------|--------------------|
| authored planning and framing modes | summary, missing-context findings when applicable, targeted clarification questions when applicable, and non-empty reasoning signals | Maintainer can see whether the authored packet is bounded, shallow, or materially closed before starting the run |
| authored execution and preservation modes | summary, mutation or preservation boundary findings, and reasoning signals grounded in the authored packet | Maintainer can see whether the packet captures real execution reasoning rather than only headings |
| authored assessment and incident modes | summary, evidence-gap or scope-boundary findings when applicable, and reasoning signals about support strength | Maintainer can distinguish a reviewable packet from one that is still structurally shallow |

## Diff-Backed Review Family

| Mode | Required Reasoning Outcome | Reviewer Outcome |
|------|----------------------------|------------------|
| `pr-review` | diff-backed findings, missing evidence when present, unsupported verdicts when warranted, and explicit no-finding posture when no direct contradiction exists | Reviewer can trust that the packet is honest whether it found issues or not |
| `review` | packet-backed missing-evidence, disposition, and boundary findings posture | Reviewer can tell whether the packet justifies acceptance, deferral, or more evidence |
| `verification` | explicit challenge findings, contradictions when present, unresolved findings when present, and explicit no-contradiction posture when absent | Reviewer can tell whether the claims under test are actually supported |

## Honesty Rules

- Heading presence alone must never be treated as proof of strong reasoning.
- If the authored input materially closes the decision, Canon must say so
  directly instead of inventing balanced alternatives.
- If authored support is weak or absent, Canon must preserve explicit missing
  body, missing evidence, closure, blocked, or unsupported posture rather than
  generating generic filler.
- If no contradiction or no direct finding exists, the packet must say that
  explicitly instead of manufacturing adversity for shape compliance.