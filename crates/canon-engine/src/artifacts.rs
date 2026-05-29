/// Artifact contract declarations: per-mode artifact requirements.
pub mod contract;
/// JSON artifact serialization helpers.
pub mod json;
/// Run manifest serialization and deserialization.
pub mod manifest;
/// Markdown artifact rendering functions, one per governed mode.
pub mod markdown;
/// YAML artifact serialization helpers.
pub mod yaml;

pub use markdown::render_refinement_working_brief;
