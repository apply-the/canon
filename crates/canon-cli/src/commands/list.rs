use canon_engine::EngineService;
use canon_engine::persistence::layout::ProjectLayout;
use canon_engine::persistence::lookup::scan_all;

use crate::app::{ListCommand, OutputFormat};
use crate::error::CliResult;

pub fn execute(service: &EngineService, command: ListCommand) -> CliResult<i32> {
    match command {
        ListCommand::Runs { output } => list_runs(service, output),
    }
}

fn list_runs(service: &EngineService, output: OutputFormat) -> CliResult<i32> {
    let layout = ProjectLayout::new(service.repo_root());
    let mut runs = scan_all(&layout).map_err(|e| {
        crate::error::CliError::Engine(canon_engine::EngineError::Validation(e.to_string()))
    })?;
    // Newest first by created_at, then by run_id descending for determinism.
    runs.sort_by(|a, b| b.created_at.cmp(&a.created_at).then_with(|| b.run_id.cmp(&a.run_id)));

    match output {
        OutputFormat::Json => {
            let value: Vec<_> = runs
                .iter()
                .map(|h| {
                    serde_json::json!({
                        "run_id": h.run_id,
                        "uuid": h.uuid,
                        "short_id": h.short_id,
                        "created_at": h.created_at.to_string(),
                        "is_legacy": h.is_legacy,
                    })
                })
                .collect();
            println!("{}", serde_json::to_string_pretty(&value)?);
        }
        OutputFormat::Yaml => {
            let value: Vec<_> = runs
                .iter()
                .map(|h| {
                    serde_json::json!({
                        "run_id": h.run_id,
                        "uuid": h.uuid,
                        "short_id": h.short_id,
                        "created_at": h.created_at.to_string(),
                        "is_legacy": h.is_legacy,
                    })
                })
                .collect();
            println!("{}", serde_yaml::to_string(&value).unwrap_or_default());
        }
        _ => {
            if runs.is_empty() {
                println!("(no runs)");
            } else {
                println!("RUN_ID                          SHORT_ID  CREATED_AT");
                for h in &runs {
                    println!("{:<32} {:<9} {}", h.run_id, h.short_id, h.created_at);
                }
            }
        }
    }

    Ok(0)
}
