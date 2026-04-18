# Canon Skill Index

All Canon skills are discoverable through `$`. Available-now skills drive the
real Canon CLI. Modeled-only and intentionally-limited skills stay visible, but
they must stay brutally explicit about the boundary and nearest honest
alternative.

| Skill | Class | Support State | Visibility | Nearest Related Skills |
| --- | --- | --- | --- | --- |
| `canon-init` | executable-wrapper | `available-now` | prominent | `canon-requirements`, `canon-status` |
| `canon-status` | executable-wrapper | `available-now` | prominent | `canon-inspect-invocations`, `canon-inspect-evidence` |
| `canon-inspect-invocations` | executable-wrapper | `available-now` | prominent | `canon-status`, `canon-inspect-evidence` |
| `canon-inspect-evidence` | executable-wrapper | `available-now` | prominent | `canon-status`, `canon-inspect-invocations` |
| `canon-inspect-artifacts` | executable-wrapper | `available-now` | discoverable-standard | `canon-status`, `canon-inspect-evidence` |
| `canon-inspect-clarity` | executable-wrapper | `available-now` | discoverable-standard | `canon-requirements`, `canon-discovery` |
| `canon-approve` | executable-wrapper | `available-now` | discoverable-standard | `canon-resume`, `canon-status` |
| `canon-resume` | executable-wrapper | `available-now` | discoverable-standard | `canon-status`, `canon-inspect-evidence` |
| `canon-requirements` | executable-wrapper | `available-now` | prominent | `canon-inspect-clarity`, `canon-status` |
| `canon-brownfield` | executable-wrapper | `available-now` | discoverable-standard | `canon-status`, `canon-approve` |
| `canon-pr-review` | executable-wrapper | `available-now` | discoverable-standard | `canon-status`, `canon-inspect-evidence` |
| `canon-discovery` | executable-wrapper | `available-now` | discoverable-standard | `canon-inspect-clarity`, `canon-status` |
| `canon-system-shaping` | executable-wrapper | `available-now` | discoverable-standard | `canon-status`, `canon-inspect-artifacts` |
| `canon-architecture` | executable-wrapper | `available-now` | discoverable-standard | `canon-status`, `canon-approve` |
| `canon-implementation` | support-state-wrapper | `modeled-only` | discoverable-standard | `canon-brownfield`, `canon-pr-review` |
| `canon-refactor` | support-state-wrapper | `modeled-only` | discoverable-standard | `canon-brownfield`, `canon-review` |
| `canon-verification` | support-state-wrapper | `intentionally-limited` | discoverable-standard | `canon-inspect-evidence`, `canon-pr-review` |
| `canon-review` | support-state-wrapper | `modeled-only` | discoverable-standard | `canon-pr-review`, `canon-inspect-evidence` |
| `canon-incident` | support-state-wrapper | `modeled-only` | discoverable-standard | `canon-brownfield`, `canon-requirements` |
| `canon-migration` | support-state-wrapper | `modeled-only` | discoverable-standard | `canon-brownfield`, `canon-architecture` |
