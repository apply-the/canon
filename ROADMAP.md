# Next Features

## Current 0.63.1

Canon 0.63.1 is the current public release line. The `canon-pr-review` intake,
skill validators, README, changelog, roadmap, and site now describe one
consistent `0.63.1` story.

## Recently Landed

Canon 0.63.0 shipped a guided interactive `canon init` flow in supported
terminals. The CLI now opens a branded assistant selector by default, keeps a
confirmation step before initialization, preserves the existing script-safe
contract behind `--non-interactive`, and restores the terminal cleanly on
success, `Ctrl+C`, and guided-path failures.

Canon 0.63.1 follows that release by tightening `canon-pr-review` intake with
guided comparison choices when base/head refs are missing or semantically
unclear.

The assistant surface also now includes Cursor and Antigravity across guided
init, non-interactive init, and repository-level assistant package metadata.

## Open Ideas

A future follow-on could extend the interactive init surface beyond local
runtime materialization into optional integration onboarding. If Canon later
needs to collect operator choices such as publish destinations, backlog
handoff targets, or credential references for MCP-backed services, that next
slice should build on the existing guided init contract without weakening the
non-interactive path used by automation.

This roadmap remains intentionally sparse: a macrofeature only moves forward
once its bounds, artifact contract, and validation story are explicit.
