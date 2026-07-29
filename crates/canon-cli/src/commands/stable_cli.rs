//! Human CLI adapter for the deterministic governance service.
//!
//! Stable CLI input is an explicitly selected typed JSON draft. Provider
//! credentials and authoring runtimes are intentionally outside this path.

use std::fs;
use std::path::{Path, PathBuf};

use canon_engine::EngineService;
use canon_engine::decision_memory::GovernanceBundleDraft;

use super::stable_governance;
use crate::app::OutputFormat;
use crate::error::{CliError, CliResult};

/// Loads and admits one explicitly selected stable governance draft.
pub fn run(
    service: &EngineService,
    profile: &str,
    bundle_path: Option<PathBuf>,
    inputs: Vec<String>,
    inline_inputs: Vec<String>,
    output: OutputFormat,
) -> CliResult<i32> {
    let source = select_source(service.repo_root(), bundle_path, inputs, inline_inputs)?;
    let draft: GovernanceBundleDraft = serde_json::from_slice(&source)?;
    let encoded_profile = serde_json::to_value(draft.profile)?;
    if encoded_profile.as_str() != Some(profile) {
        return Err(CliError::InvalidInput(
            "selected profile does not match the typed bundle profile".to_string(),
        ));
    }
    let request_id = draft.bundle_id.clone();
    let result = stable_governance::mutate(service, &request_id, draft)?;
    emit(&result, output)?;
    Ok(stable_governance::exit_code(result.terminal_status))
}

/// Projects current decision memory without changing its durable bytes.
pub fn inspect(service: &EngineService, output: OutputFormat) -> CliResult<i32> {
    let result = stable_governance::inspect(service)?;
    emit(&result, output)?;
    Ok(stable_governance::exit_code(result.terminal_status))
}

fn select_source(
    repo_root: &Path,
    bundle_path: Option<PathBuf>,
    inputs: Vec<String>,
    inline_inputs: Vec<String>,
) -> CliResult<Vec<u8>> {
    let paths =
        bundle_path.into_iter().chain(inputs.into_iter().map(PathBuf::from)).collect::<Vec<_>>();
    if paths.len().saturating_add(inline_inputs.len()) != 1 {
        return Err(CliError::InvalidInput(
            "stable run requires exactly one --bundle, --input, or --input-text source".to_string(),
        ));
    }
    if let Some(path) = paths.first() {
        let resolved = if path.is_absolute() { path.clone() } else { repo_root.join(path) };
        return Ok(fs::read(resolved)?);
    }
    inline_inputs
        .into_iter()
        .next()
        .map(String::into_bytes)
        .ok_or_else(|| CliError::InvalidInput("stable run input was empty".to_string()))
}

fn emit(result: &impl serde::Serialize, output: OutputFormat) -> CliResult<()> {
    match output {
        OutputFormat::Yaml => println!("{}", serde_yaml::to_string(result)?),
        OutputFormat::Text | OutputFormat::Json | OutputFormat::Markdown => {
            println!("{}", serde_json::to_string_pretty(result)?);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use canon_engine::EngineService;
    use serde::Serialize;

    use super::{emit, inspect, run, select_source};
    use crate::app::OutputFormat;
    use crate::commands::stable_governance::tests::draft;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    fn require(condition: bool, message: &str) -> TestResult {
        if condition { Ok(()) } else { Err(message.to_string().into()) }
    }

    #[derive(Serialize)]
    struct Projection {
        status: &'static str,
    }

    #[test]
    fn direct_source_selection_and_projection_paths_are_strict() -> TestResult {
        let workspace = tempfile::tempdir()?;
        std::fs::write(workspace.path().join("bundle.json"), b"{\"bundle\":true}")?;
        require(
            select_source(workspace.path(), Some("bundle.json".into()), Vec::new(), Vec::new())?
                == b"{\"bundle\":true}",
            "file source drifted",
        )?;
        require(
            select_source(
                workspace.path(),
                None,
                vec![workspace.path().join("bundle.json").display().to_string()],
                Vec::new(),
            )? == b"{\"bundle\":true}",
            "absolute input source drifted",
        )?;
        require(
            select_source(
                workspace.path(),
                None,
                Vec::new(),
                vec!["{\"inline\":true}".to_string()],
            )? == b"{\"inline\":true}",
            "inline source drifted",
        )?;
        require(
            select_source(workspace.path(), None, Vec::new(), Vec::new()).is_err(),
            "missing source was accepted",
        )?;
        require(
            select_source(
                workspace.path(),
                Some("bundle.json".into()),
                Vec::new(),
                vec!["{}".to_string()],
            )
            .is_err(),
            "ambiguous sources were accepted",
        )?;
        for output in
            [OutputFormat::Text, OutputFormat::Json, OutputFormat::Yaml, OutputFormat::Markdown]
        {
            emit(&Projection { status: "accepted" }, output)?;
        }
        require(
            inspect(&EngineService::new(workspace.path()), OutputFormat::Json).is_err(),
            "inspection without durable state was accepted",
        )?;

        let admitted = draft("bundle-cli-direct", 1);
        std::fs::write(workspace.path().join("admitted.json"), serde_json::to_vec(&admitted)?)?;
        require(
            run(
                &EngineService::new(workspace.path()),
                "discovery",
                Some("admitted.json".into()),
                Vec::new(),
                Vec::new(),
                OutputFormat::Json,
            )? == 0,
            "stable CLI mutation did not return success",
        )?;
        require(
            inspect(&EngineService::new(workspace.path()), OutputFormat::Json)? == 0,
            "stable CLI inspection did not return success",
        )?;
        require(
            run(
                &EngineService::new(workspace.path()),
                "architecture",
                None,
                Vec::new(),
                vec![serde_json::to_string(&admitted)?],
                OutputFormat::Json,
            )
            .is_err(),
            "profile mismatch was accepted",
        )?;
        require(
            run(
                &EngineService::new(workspace.path()),
                "discovery",
                None,
                Vec::new(),
                vec!["not-json".to_string()],
                OutputFormat::Json,
            )
            .is_err(),
            "malformed typed bundle was accepted",
        )
    }
}
