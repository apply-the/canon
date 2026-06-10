/// File classification by path-pattern heuristics.
pub mod classifier;
/// Onion-layer review context index builder.
pub mod context;
/// Coverage-aware review analysis: buckets, review type, confidence.
pub mod coverage;
/// Critique note types for review runs.
pub mod critique;
/// Diff mapping and line extraction utilities.
pub mod diff;
/// PR review evaluator mapping and decision logic.
pub mod evaluator;
/// Review finding types: categories, severity, conventional comment scopes.
pub mod findings;
/// Markdown generators for review artifacts.
pub mod generators;
/// Onion-layer review state machine.
pub mod onion;
/// Artifact rendering and recommendation logic for pr-review finalize.
pub mod render;
/// Review summary and disposition types.
pub mod summary;
/// Reviewer output validation for pr-review accept phase.
pub mod validate;
