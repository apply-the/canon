# Discovery Brief: Legacy Auth Migration

## Problem
The current authentication system is written in a deprecated framework and relies on MD5 hashes. We need to decide if we should build a new auth service from scratch, adopt a third-party identity provider, or incrementally upgrade the existing database.

## Current Context
- The current service (`auth-v1`) handles 50,000 logins per day.
- It is deeply coupled with the primary user profile database.
- We have 3 active enterprise customers requiring SAML support, which `auth-v1` cannot natively provide.

## Known Facts
- We cannot take downtime during peak business hours (9 AM - 5 PM EST).
- MD5 hashes must be eliminated to satisfy upcoming compliance audits.

## Unknowns
- How many inactive users exist in the database and need to be migrated?
- Can third-party providers (e.g., Auth0, Okta) map directly to our existing internal user IDs without breaking legacy API consumers?
- How much effort is required to decouple `auth-v1` from the user profile database?

## Constraints
- Compliance deadline is in 4 months.
- Budget allows for licensing a 3rd-party provider if implementation time is < 2 months.

## In Scope
- Securing active user credentials.
- Adding SAML support for enterprise tiers.
- Establishing the data migration path for active users.

## Out of Scope
- Rewriting the user profile management UI.
- Changes to the RBAC authorization logic (permissions are handled in a separate service).

## Exploration Options
1. **Build New**: A containerized internal Auth Service supporting OIDC/SAML, migrating data in the background.
2. **Buy**: Integrate Okta or Auth0 and run a shadow-migration script to sync user accounts.
3. **In-Place Upgrade**: Add bcrypt to the existing node monolith and bolt-on a SAML middleware.

## Questions
- What is the latency impact of calling an external identity provider vs a local internal service for API token validation?
- Are enterprise customers willing to accept a mandatory password reset for the migration?