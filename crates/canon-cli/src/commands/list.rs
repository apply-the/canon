use canon_engine::EngineService;
use canon_engine::persistence::layout::ProjectLayout;
use canon_engine::persistence::lookup::{RunHandle, scan_all};

use crate::app::{ListCommand, OutputFormat};
use crate::error::CliResult;

pub fn execute(service: &EngineService, command: ListCommand) -> CliResult<i32> {
    match command {
        ListCommand::Runs { output } => list_runs(service, output),
    }
}

fn list_runs(service: &EngineService, output: OutputFormat) -> CliResult<i32> {
    let layout = ProjectLayout::new(service.repo_root());
    let runs = scan_all(&layout).map_err(|e| {
        crate::error::CliError::Engine(canon_engine::EngineError::Validation(e.to_string()))
    })?;

    println!("{}", render_runs(runs, output)?);

    Ok(0)
}

fn render_runs(mut runs: Vec<RunHandle>, output: OutputFormat) -> CliResult<String> {
    // Newest first by created_at, then by run_id descending for determinism.
    runs.sort_by(|a, b| b.created_at.cmp(&a.created_at).then_with(|| b.run_id.cmp(&a.run_id)));

    match output {
        OutputFormat::Json => Ok(serde_json::to_string_pretty(&run_values(&runs))?),
        OutputFormat::Yaml => Ok(serde_yaml::to_string(&run_values(&runs)).unwrap_or_default()),
        _ => {
            if runs.is_empty() {
                Ok("(no runs)".to_string())
            } else {
                let mut lines =
                    vec!["RUN_ID                          SHORT_ID  CREATED_AT".to_string()];
                for handle in &runs {
                    lines.push(format!(
                        "{:<32} {:<9} {}",
                        handle.run_id, handle.short_id, handle.created_at
                    ));
                }
                Ok(lines.join("\n"))
            }
        }
    }
}

fn run_values(runs: &[RunHandle]) -> Vec<serde_json::Value> {
    runs.iter()
        .map(|handle| {
            serde_json::json!({
                "run_id": handle.run_id,
                "uuid": handle.uuid,
                "short_id": handle.short_id,
                "created_at": handle.created_at.to_string(),
                "is_legacy": handle.is_legacy,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::{Path, PathBuf};

    use tempfile::tempdir;
    use time::OffsetDateTime;

    use super::{execute, render_runs};
    use crate::app::{ListCommand, OutputFormat};
    use canon_engine::EngineService;
    use canon_engine::persistence::layout::ProjectLayout;
    use canon_engine::persistence::lookup::RunHandle;

    fn sample_run(run_id: &str, short_id: &str, created_at: i64, is_legacy: bool) -> RunHandle {
        RunHandle {
            run_id: run_id.to_string(),
            uuid: format!("{short_id}000070008000000000000001"),
            short_id: short_id.to_string(),
            directory: PathBuf::from(format!("/tmp/{run_id}")),
            is_legacy,
            created_at: OffsetDateTime::from_unix_timestamp(created_at).expect("timestamp"),
        }
    }

    fn write_run_manifest(run_dir: &Path, run_id: &str, short_id: &str, created_at: &str) {
        fs::create_dir_all(run_dir).expect("create run dir");
        fs::write(
            run_dir.join("run.toml"),
            format!(
                "run_id = \"{run_id}\"\n\
uuid = \"{short_id}000070008000000000000001\"\n\
short_id = \"{short_id}\"\n\
title = \"Sample run\"\n\
mode = \"requirements\"\n\
risk = \"low-impact\"\n\
zone = \"green\"\n\
system_context = \"existing\"\n\
owner = \"Owner <owner@example.com>\"\n\
created_at = \"{created_at}\"\n\
\n\
[classification.risk]\n\
source = \"explicit\"\n\
rationale = \"Risk class was supplied explicitly at run start.\"\n\
\n\
[classification.zone]\n\
source = \"explicit\"\n\
rationale = \"Usage zone was supplied explicitly at run start.\"\n"
            ),
        )
        .expect("write run manifest");
    }

    #[test]
    fn render_runs_formats_json_and_yaml_output() {
        let runs = vec![sample_run("R-20260422-11111111", "11111111", 2, false)];

        let json = render_runs(runs.clone(), OutputFormat::Json).expect("json output");
        let yaml = render_runs(runs, OutputFormat::Yaml).expect("yaml output");

        let json_value: Vec<serde_json::Value> = serde_json::from_str(&json).expect("parse json");
        assert_eq!(json_value[0]["run_id"], "R-20260422-11111111");
        assert_eq!(json_value[0]["short_id"], "11111111");
        assert_eq!(json_value[0]["is_legacy"], false);

        let yaml_value: Vec<serde_json::Value> = serde_yaml::from_str(&yaml).expect("parse yaml");
        assert_eq!(yaml_value[0]["run_id"], "R-20260422-11111111");
        assert_eq!(yaml_value[0]["short_id"], "11111111");
    }

    #[test]
    fn render_runs_formats_empty_and_sorted_text_output() {
        assert_eq!(render_runs(Vec::new(), OutputFormat::Text).expect("empty output"), "(no runs)");

        let output = render_runs(
            vec![
                sample_run("R-20260422-11111111", "11111111", 1, false),
                sample_run("R-20260422-22222222", "22222222", 2, false),
            ],
            OutputFormat::Markdown,
        )
        .expect("text output");

        assert!(output.contains("RUN_ID"), "expected header row: {output}");
        assert!(
            output.find("R-20260422-22222222").expect("newer run")
                < output.find("R-20260422-11111111").expect("older run"),
            "expected newest run first: {output}"
        );
    }

    #[test]
    fn execute_reads_runs_from_workspace() {
        let workspace = tempdir().expect("temp workspace");
        let layout = ProjectLayout::new(workspace.path());
        write_run_manifest(
            &layout.new_run_dir("R-20260422-11111111", None),
            "R-20260422-11111111",
            "11111111",
            "2026-04-22T00:00:00Z",
        );

        let service = EngineService::new(workspace.path());
        let exit_code = execute(&service, ListCommand::Runs { output: OutputFormat::Json })
            .expect("execute list runs");

        assert_eq!(exit_code, 0);
    }
}
