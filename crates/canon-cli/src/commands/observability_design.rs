use clap::Args;
use std::path::PathBuf;

/// Arguments for the observability-design mode.
#[derive(Debug, Clone, Args)]
pub struct ObservabilityDesignArgs {
    /// Path to the target architecture, domain-model, or feature-spec document.
    #[arg(required = true)]
    pub input_file: PathBuf,

    /// Evaluate the document and detect boundaries, but do not write output artifacts to disk.
    #[arg(long, default_value_t = false)]
    pub dry_run: bool,

    /// Output reasoning steps during the LLM boundary inference phase.
    #[arg(long, default_value_t = false)]
    pub verbose: bool,

    /// Force the mode to ask for manual boundary confirmation, bypassing automatic inference.
    #[arg(long, default_value_t = false)]
    pub interactive: bool,
}

pub fn handle(args: &ObservabilityDesignArgs) -> Result<(), Box<dyn std::error::Error>> {
    use canon_engine::observability::{evaluator, generators};
    use std::fs;

    // Read input file
    let input_content = fs::read_to_string(&args.input_file)?;

    // Evaluate (T010 integration)
    let plan = evaluator::evaluate_architecture(&input_content, args.interactive)?;

    if args.dry_run {
        println!("Dry-run successful. Found {} boundaries.", plan.boundaries.len());
        if args.verbose {
            println!("{:#?}", plan);
        }
        return Ok(());
    }

    // Generate markdown (T011 integration)
    let telemetry_plan_md = generators::generate_telemetry_plan_markdown(&plan);
    let checklist_md = generators::generate_instrumentation_checklist(&plan);

    // Generate SLO and Runbooks (T016 integration)
    let (slos, runbooks) =
        evaluator::generate_slos_and_runbooks(&plan).map_err(|e| e.to_string())?;
    let slos_md = generators::generate_slo_alerts(&slos);
    let runbooks_md = generators::generate_runbook_stubs(&runbooks);

    // Write outputs to the same directory as the input file
    let parent_dir = args.input_file.parent().unwrap_or(std::path::Path::new(""));
    let artifacts = [
        ("telemetry-plan.md", telemetry_plan_md),
        ("05-instrumentation-checklist.md", checklist_md),
        ("03-slo-alerts.md", slos_md),
        ("04-runbook.md", runbooks_md),
    ];

    for (filename, content) in artifacts {
        let path = parent_dir.join(filename);
        fs::write(&path, content)?;
        println!("Success: Generated {}", path.display());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_observability_design_handle_dry_run() {
        let dir = tempdir().unwrap();
        let input_path = dir.path().join("arch.md");
        std::fs::write(&input_path, "test arch").unwrap();

        let args = ObservabilityDesignArgs {
            input_file: input_path.clone(),
            dry_run: true,
            verbose: true,
            interactive: false,
        };

        let res = handle(&args);
        assert!(res.is_ok());

        // Assert no output files were written
        assert!(!dir.path().join("telemetry-plan.md").exists());
    }

    #[test]
    fn test_observability_design_handle_generates_files() {
        let dir = tempdir().unwrap();
        let input_path = dir.path().join("arch.md");
        std::fs::write(&input_path, "test arch").unwrap();

        let args = ObservabilityDesignArgs {
            input_file: input_path.clone(),
            dry_run: false,
            verbose: false,
            interactive: true,
        };

        let res = handle(&args);
        assert!(res.is_ok());

        assert!(dir.path().join("telemetry-plan.md").exists());
        assert!(dir.path().join("05-instrumentation-checklist.md").exists());
        assert!(dir.path().join("03-slo-alerts.md").exists());
        assert!(dir.path().join("04-runbook.md").exists());
    }
}
