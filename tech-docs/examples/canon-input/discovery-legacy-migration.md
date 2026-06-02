# Discovery Brief: Legacy Auth Migration

## Problem Domain
The current authentication system is written in a deprecated framework and relies on MD5 hashes. The blocked job is modernizing identity and enterprise login flows without breaking active users or missing the compliance deadline. We need to understand whether the right next step is a fresh internal auth service, a third-party identity provider, or an incremental hardening path around the current service.

## Repo Surface
- `services/auth-v1/` for the current login flow and password verification logic.
- `services/user-profile/` for the shared user identifier coupling that blocks a clean swap.
- `infra/sso/` for the partial enterprise SAML setup that currently terminates outside `auth-v1`.

## Immediate Tensions
- Compliance requires eliminating MD5 hashes within four months, but the current service is deeply coupled to the user profile store.
- Enterprise customers need SAML support now, while a full rewrite could overrun the compliance window.
- Zero-downtime migration is required, but the current coupling makes the blast radius unclear.

## Downstream Handoff
If this discovery packet lands cleanly, the next packet should be a requirements or change packet that names the chosen migration path, bounded rollout phases, and the preserved identity invariants for active users.

## Unknowns
- How many inactive users remain in the database and would need a migration path?
- Can Auth0 or Okta map directly to our internal user ids without breaking legacy API consumers?
- How much effort is required to decouple `auth-v1` from the user profile database?

## Assumptions
- Active-user migration can be prioritized ahead of long-tail inactive accounts.
- Enterprise SAML support is a first-slice requirement even if the long-term identity platform remains unsettled.
- The profile service team can support schema-preserving identifier bridges if a provider path is chosen.

## Validation Targets
- Measure the latency impact of external token validation versus a local internal service as an assumption test for the provider path.
- Confirm whether the profile coupling can be isolated behind a stable id-mapping layer.
- Verify whether a forced password reset is commercially acceptable for enterprise tenants.

## Confidence Levels
- High confidence: MD5 elimination and zero-downtime migration are non-negotiable.
- Medium confidence: a provider path is viable if identifier mapping can stay transparent.
- Low confidence: the true cost of decoupling `auth-v1` from the profile database.

## In-Scope Context
- Securing active user credentials.
- Adding SAML support for enterprise tiers.
- Establishing the migration path for active users.

## Out-of-Scope Context
- Rewriting the user profile management UI.
- Changes to the RBAC authorization logic handled by a separate service.

## Translation Trigger
Move to requirements once the team chooses the migration direction and can name the preserved identity boundary, rollout phases, and acceptance criteria.

## Options
1. Build a new internal auth service that supports OIDC and SAML, then migrate users in the background.
2. Buy a third-party identity provider and bridge internal ids with a shadow-migration path.
3. Upgrade the existing monolith in place with stronger hashing and a SAML add-on.

## Constraints
- Compliance deadline is in 4 months.
- Budget allows for licensing a 3rd-party provider if implementation time is < 2 months.

## Recommended Direction
Treat the provider path as the current favorite, but only if the identifier-bridge validation proves that legacy API consumers can stay stable. If the identifier bridge fails, fall back to a bounded internal service replacement rather than an in-place upgrade of the legacy monolith.

## Next-Phase Shape
The next packet should compare the top two viable migration paths with explicit rollout sequencing, preserved identity invariants, and a recommendation that can survive compliance review.

## Pressure Points
- The compliance deadline can force a short-term decision before the long-term platform choice is fully mature.
- SAML needs can pull the team toward a provider even if internal-service ownership would be cleaner long term.
- The current profile coupling can turn any migration into a cross-team coordination problem.

## Blocking Decisions
- Decide whether preserving internal user ids transparently is a hard invariant or a negotiable migration tradeoff.
- Decide whether enterprise SAML support must land in the same slice as hash elimination.

## Open Questions
- What is the latency impact of calling an external identity provider vs a local internal service for API token validation?
- Are enterprise customers willing to accept a mandatory password reset for the migration?

## Recommended Owner
- Identity platform lead with profile-service support for identifier mapping validation.