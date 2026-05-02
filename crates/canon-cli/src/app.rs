use std::path::{Path, PathBuf};

use canon_engine::{AiTool, EngineService};
use clap::{Args, Parser, Subcommand, ValueEnum};
use strum_macros::Display;

use crate::commands;
use crate::error::CliResult;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, ValueEnum, Display)]
#[strum(serialize_all = "lowercase")]
pub enum OutputFormat {
    #[default]
    Text,
    Json,
    Yaml,
    Markdown,
}

#[derive(Debug, Subcommand, Clone)]
pub enum InspectCommand {
    Modes {
        #[arg(long, default_value_t = OutputFormat::Markdown)]
        output: OutputFormat,
    },
    Methods {
        #[arg(long, default_value_t = OutputFormat::Markdown)]
        output: OutputFormat,
    },
    Policies {
        #[arg(long, default_value_t = OutputFormat::Markdown)]
        output: OutputFormat,
    },
    RiskZone {
        #[arg(long)]
        mode: String,
        #[arg(long)]
        risk: Option<String>,
        #[arg(long)]
        zone: Option<String>,
        #[arg(long = "input", num_args = 1..)]
        inputs: Vec<String>,
        #[arg(long = "input-text")]
        inline_inputs: Vec<String>,
        #[arg(long, default_value_t = OutputFormat::Markdown)]
        output: OutputFormat,
    },
    Clarity {
        #[arg(long)]
        mode: String,
        #[arg(long = "input", num_args = 1..)]
        inputs: Vec<String>,
        #[arg(long, default_value_t = OutputFormat::Markdown)]
        output: OutputFormat,
    },
    Artifacts {
        #[arg(long)]
        run: String,
        #[arg(long, default_value_t = OutputFormat::Markdown)]
        output: OutputFormat,
    },
    Invocations {
        #[arg(long)]
        run: String,
        #[arg(long, default_value_t = OutputFormat::Markdown)]
        output: OutputFormat,
    },
    Evidence {
        #[arg(long)]
        run: String,
        #[arg(long, default_value_t = OutputFormat::Markdown)]
        output: OutputFormat,
    },
}

#[derive(Debug, Subcommand, Clone)]
pub enum SkillsCommand {
    Install {
        #[arg(long, value_enum)]
        ai: AiTarget,
        #[arg(long, default_value_t)]
        output: OutputFormat,
    },
    Update {
        #[arg(long, value_enum)]
        ai: AiTarget,
        #[arg(long, default_value_t)]
        output: OutputFormat,
    },
    List {
        #[arg(long, default_value_t)]
        output: OutputFormat,
    },
}

#[derive(Debug, Subcommand, Clone)]
pub enum GovernanceCommand {
    Start {
        #[arg(long)]
        json: bool,
    },
    Refresh {
        #[arg(long)]
        json: bool,
    },
    Capabilities {
        #[arg(long)]
        json: bool,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum AiTarget {
    Codex,
    Copilot,
    Claude,
}

impl From<AiTarget> for AiTool {
    fn from(value: AiTarget) -> Self {
        match value {
            AiTarget::Codex => AiTool::Codex,
            AiTarget::Copilot => AiTool::Copilot,
            AiTarget::Claude => AiTool::Claude,
        }
    }
}

#[derive(Debug, Clone, Args)]
pub struct RunCommand {
    #[arg(long)]
    mode: String,
    #[arg(long = "system-context")]
    system_context: Option<String>,
    #[arg(long)]
    risk: String,
    #[arg(long)]
    zone: String,
    #[arg(long, hide = true)]
    risk_source: Option<String>,
    #[arg(long, hide = true)]
    risk_rationale: Option<String>,
    #[arg(long = "risk-signal", hide = true)]
    risk_signals: Vec<String>,
    #[arg(long, hide = true)]
    zone_source: Option<String>,
    #[arg(long, hide = true)]
    zone_rationale: Option<String>,
    #[arg(long = "zone-signal", hide = true)]
    zone_signals: Vec<String>,
    #[arg(
        long,
        help = "Human owner for the run. If omitted, Canon tries git user.name and user.email."
    )]
    owner: Option<String>,
    #[arg(long = "input", num_args = 1..)]
    inputs: Vec<String>,
    #[arg(long = "input-text")]
    inline_inputs: Vec<String>,
    #[arg(long = "exclude")]
    excluded_paths: Vec<String>,
    #[arg(long)]
    policy_root: Option<String>,
    #[arg(long)]
    method_root: Option<String>,
    #[arg(long, default_value_t)]
    output: OutputFormat,
}

#[derive(Debug, Clone, Args)]
pub struct ApproveCommand {
    #[arg(long)]
    run: String,
    #[arg(long, help = "Approval target in the form gate:<gate-kind> or invocation:<request-id>")]
    target: Option<String>,
    #[arg(long, hide = true)]
    gate: Option<String>,
    #[arg(
        long,
        help = "Human approver for the decision. If omitted, Canon tries git user.name and user.email."
    )]
    by: Option<String>,
    #[arg(long)]
    decision: String,
    #[arg(long)]
    rationale: String,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Init {
        #[arg(long, value_enum)]
        ai: Option<AiTarget>,
        #[arg(long, default_value_t)]
        output: OutputFormat,
    },
    Run(Box<RunCommand>),
    Resume {
        #[arg(long)]
        run: String,
    },
    Status {
        #[arg(long)]
        run: String,
        #[arg(long, default_value_t)]
        output: OutputFormat,
    },
    Approve(ApproveCommand),
    Verify {
        #[arg(long)]
        run: String,
    },
    Inspect {
        #[command(subcommand)]
        command: InspectCommand,
    },
    Skills {
        #[command(subcommand)]
        command: SkillsCommand,
    },
    Governance {
        #[command(subcommand)]
        command: GovernanceCommand,
    },
    List {
        #[command(subcommand)]
        command: ListCommand,
    },
    Publish(PublishCommand),
}

#[derive(Debug, Subcommand, Clone)]
pub enum ListCommand {
    /// List all known runs (newest first).
    Runs {
        #[arg(long, default_value_t)]
        output: OutputFormat,
    },
}

#[derive(Debug, Clone, clap::Args)]
pub struct PublishCommand {
    /// The ID of the run to publish (short ID, UUID, or @last)
    pub run_id: String,

    /// Optional override for the destination directory
    #[arg(long)]
    pub to: Option<std::path::PathBuf>,
}

#[derive(Debug, Parser)]
#[command(name = "canon")]
#[command(about = "A governed method engine for AI-assisted software engineering.")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

fn dispatch_command(service: &EngineService, repo_root: &Path, command: Command) -> CliResult<i32> {
    match command {
        Command::Init { ai, output } => {
            commands::init::execute(service, ai.map(Into::into), output)
        }
        Command::Run(run_command) => {
            let RunCommand {
                mode,
                system_context,
                risk,
                zone,
                risk_source,
                risk_rationale,
                risk_signals,
                zone_source,
                zone_rationale,
                zone_signals,
                owner,
                inputs,
                inline_inputs,
                excluded_paths,
                policy_root,
                method_root,
                output,
            } = *run_command;

            commands::run::execute(
                service,
                mode,
                system_context,
                risk,
                zone,
                risk_source,
                risk_rationale,
                risk_signals,
                zone_source,
                zone_rationale,
                zone_signals,
                owner,
                inputs,
                inline_inputs,
                excluded_paths,
                policy_root,
                method_root,
                output,
            )
        }
        Command::Resume { run } => commands::resume::execute(service, &run),
        Command::Status { run, output } => commands::status::execute(service, &run, output),
        Command::Approve(ApproveCommand { run, target, gate, by, decision, rationale }) => {
            commands::approve::execute(service, &run, target, gate, by, decision, rationale)
        }
        Command::Verify { .. } => commands::verify::execute(),
        Command::Inspect { command } => commands::inspect::execute(service, command),
        Command::Skills { command } => commands::skills::execute(service, command),
        Command::Governance { command } => {
            commands::governance::execute(service, repo_root, command)
        }
        Command::List { command } => commands::list::execute(service, command),
        Command::Publish(cmd) => commands::publish::execute(service, cmd),
    }
}

fn run_with(cli: Cli, repo_root: PathBuf) -> CliResult<i32> {
    let service = EngineService::new(&repo_root);
    dispatch_command(&service, repo_root.as_path(), cli.command)
}

pub fn run() -> CliResult<i32> {
    tracing_subscriber::fmt::try_init().ok();

    run_with(Cli::parse(), std::env::current_dir()?)
}

#[cfg(test)]
mod tests {
    use clap::Parser;
    use tempfile::tempdir;

    use super::{
        AiTarget, ApproveCommand, Cli, Command, GovernanceCommand, InspectCommand, ListCommand,
        OutputFormat, PublishCommand, RunCommand, SkillsCommand, run_with,
    };
    use canon_engine::AiTool;

    macro_rules! assert_command {
        ($command:expr, $pattern:pat $(if $guard:expr)? $(,)?) => {
            assert!(matches!($command, $pattern $(if $guard)?));
        };
    }

    #[test]
    fn output_format_display_matches_cli_values() {
        assert_eq!(OutputFormat::Text.to_string(), "text");
        assert_eq!(OutputFormat::Json.to_string(), "json");
        assert_eq!(OutputFormat::Yaml.to_string(), "yaml");
        assert_eq!(OutputFormat::Markdown.to_string(), "markdown");
    }

    #[test]
    fn ai_target_converts_to_engine_tool() {
        assert_eq!(AiTool::from(AiTarget::Codex), AiTool::Codex);
        assert_eq!(AiTool::from(AiTarget::Copilot), AiTool::Copilot);
        assert_eq!(AiTool::from(AiTarget::Claude), AiTool::Claude);
    }

    #[test]
    fn inspect_modes_defaults_to_markdown_output() {
        let cli = Cli::parse_from(["canon", "inspect", "modes"]);

        assert_command!(
            cli.command,
            Command::Inspect { command: InspectCommand::Modes { output } }
                if output == OutputFormat::Markdown
        );
    }

    #[test]
    fn skills_install_parses_ai_and_default_output() {
        let cli = Cli::parse_from(["canon", "skills", "install", "--ai", "codex"]);

        assert_command!(
            cli.command,
            Command::Skills { command: SkillsCommand::Install { ai, output } }
                if ai == AiTarget::Codex && output == OutputFormat::Text
        );
    }

    #[test]
    fn list_runs_defaults_to_text_output() {
        let cli = Cli::parse_from(["canon", "list", "runs"]);

        assert_command!(
            cli.command,
            Command::List { command: ListCommand::Runs { output } }
                if output == OutputFormat::Text
        );
    }

    #[test]
    fn list_runs_accepts_yaml_output() {
        let cli = Cli::parse_from(["canon", "list", "runs", "--output", "yaml"]);

        assert_command!(
            cli.command,
            Command::List { command: ListCommand::Runs { output } }
                if output == OutputFormat::Yaml
        );
    }

    #[test]
    fn run_command_parses_expected_arguments() {
        let cli = Cli::parse_from([
            "canon",
            "run",
            "--mode",
            "requirements",
            "--risk",
            "low-impact",
            "--zone",
            "green",
            "--owner",
            "Owner <owner@example.com>",
            "--input",
            "idea.md",
            "--exclude",
            "target/",
            "--risk-source",
            "inferred-confirmed",
            "--risk-rationale",
            "Production boundary detected",
            "--risk-signal",
            "Detected boundary keyword",
        ]);

        assert_command!(
            cli.command,
            Command::Run(run)
                if run.mode == "requirements"
                    && run.system_context.is_none()
                    && run.risk == "low-impact"
                    && run.zone == "green"
                    && run.risk_source.as_deref() == Some("inferred-confirmed")
                    && run.risk_rationale.as_deref() == Some("Production boundary detected")
                    && run.risk_signals == vec!["Detected boundary keyword"]
                    && run.owner.as_deref() == Some("Owner <owner@example.com>")
                    && run.inputs == vec!["idea.md"]
                    && run.inline_inputs.is_empty()
                    && run.excluded_paths == vec!["target/"]
        );
    }

    #[test]
    fn run_command_accepts_execution_heavy_modes_without_explicit_input_flags() {
        let cli = Cli::parse_from([
            "canon",
            "run",
            "--mode",
            "implementation",
            "--system-context",
            "existing",
            "--risk",
            "bounded-impact",
            "--zone",
            "yellow",
            "--owner",
            "staff-engineer",
        ]);

        assert_command!(
            cli.command,
            Command::Run(run)
                if run.mode == "implementation"
                    && run.system_context.as_deref() == Some("existing")
                    && run.inputs.is_empty()
                    && run.inline_inputs.is_empty()
        );
    }

    #[test]
    fn run_command_accepts_multiple_inputs_after_single_flag() {
        let cli = Cli::parse_from([
            "canon",
            "run",
            "--mode",
            "requirements",
            "--risk",
            "low-impact",
            "--zone",
            "green",
            "--owner",
            "Owner <owner@example.com>",
            "--input",
            "idea.md",
            "constraints.md",
            "canon-input/requirements",
            "--output",
            "json",
        ]);

        assert_command!(
            cli.command,
            Command::Run(run)
                if run.inputs == vec!["idea.md", "constraints.md", "canon-input/requirements"]
                    && run.inline_inputs.is_empty()
                    && run.output == OutputFormat::Json
        );
    }

    #[test]
    fn run_command_accepts_inline_authored_input() {
        let cli = Cli::parse_from([
            "canon",
            "run",
            "--mode",
            "requirements",
            "--risk",
            "low-impact",
            "--zone",
            "green",
            "--input-text",
            "# Requirements Brief\n\n## Problem\nNeed explicit governance.",
        ]);

        assert_command!(
            cli.command,
            Command::Run(run)
                if run.inputs.is_empty()
                    && run.inline_inputs.len() == 1
                    && run.inline_inputs[0].contains("Need explicit governance")
        );
    }

    #[test]
    fn init_status_resume_verify_and_approve_commands_parse_expected_defaults() {
        let init_cli = Cli::parse_from(["canon", "init", "--ai", "claude", "--output", "yaml"]);
        assert_command!(
            init_cli.command,
            Command::Init { ai, output }
                if ai == Some(AiTarget::Claude) && output == OutputFormat::Yaml
        );

        let status_cli = Cli::parse_from(["canon", "status", "--run", "run-123"]);
        assert_command!(
            status_cli.command,
            Command::Status { run, output }
                if run == "run-123" && output == OutputFormat::Text
        );

        let resume_cli = Cli::parse_from(["canon", "resume", "--run", "run-123"]);
        assert_command!(resume_cli.command, Command::Resume { run } if run == "run-123");

        let verify_cli = Cli::parse_from(["canon", "verify", "--run", "run-123"]);
        assert_command!(verify_cli.command, Command::Verify { run } if run == "run-123");

        let publish_cli = Cli::parse_from(["canon", "publish", "run-123", "--to", "docs/public"]);
        assert_command!(
            publish_cli.command,
            Command::Publish(PublishCommand { run_id, to })
                if run_id == "run-123" && to == Some(std::path::PathBuf::from("docs/public"))
        );

        let approve_cli = Cli::parse_from([
            "canon",
            "approve",
            "--run",
            "run-123",
            "--target",
            "gate:risk",
            "--decision",
            "approve",
            "--rationale",
            "looks good",
        ]);
        assert_command!(
            approve_cli.command,
            Command::Approve(ApproveCommand { run, target, gate, by, decision, rationale })
                if run == "run-123"
                    && target.as_deref() == Some("gate:risk")
                    && gate.is_none()
                    && by.is_none()
                    && decision == "approve"
                    && rationale == "looks good"
        );
    }

    #[test]
    fn inspect_run_scoped_targets_parse_run_id_and_output() {
        let classification = Cli::parse_from([
            "canon",
            "inspect",
            "risk-zone",
            "--mode",
            "discovery",
            "--input",
            "canon-input/discovery.md",
        ]);
        assert_command!(
            classification.command,
            Command::Inspect {
                command: InspectCommand::RiskZone { mode, inputs, inline_inputs, output, .. },
            }
                if mode == "discovery"
                    && inputs == vec!["canon-input/discovery.md"]
                    && inline_inputs.is_empty()
                    && output == OutputFormat::Markdown
        );

        let inline_classification = Cli::parse_from([
            "canon",
            "inspect",
            "risk-zone",
            "--mode",
            "requirements",
            "--input-text",
            "# Requirements Brief\n\n## Problem\nNeed bounded runtime governance.",
            "--output",
            "json",
        ]);
        assert_command!(
            inline_classification.command,
            Command::Inspect {
                command: InspectCommand::RiskZone { mode, inputs, inline_inputs, output, .. },
            }
                if mode == "requirements"
                    && inputs.is_empty()
                    && inline_inputs.len() == 1
                    && output == OutputFormat::Json
        );

        let clarity = Cli::parse_from([
            "canon",
            "inspect",
            "clarity",
            "--mode",
            "requirements",
            "--input",
            "canon-input/requirements.md",
            "canon-input/requirements/supporting/constraints.md",
            "--output",
            "json",
        ]);
        assert_command!(
            clarity.command,
            Command::Inspect { command: InspectCommand::Clarity { mode, inputs, output } }
                if mode == "requirements"
                    && inputs
                        == vec![
                            "canon-input/requirements.md",
                            "canon-input/requirements/supporting/constraints.md"
                        ]
                    && output == OutputFormat::Json
        );

        let artifacts = Cli::parse_from([
            "canon",
            "inspect",
            "artifacts",
            "--run",
            "run-123",
            "--output",
            "json",
        ]);
        assert_command!(
            artifacts.command,
            Command::Inspect { command: InspectCommand::Artifacts { run, output } }
                if run == "run-123" && output == OutputFormat::Json
        );

        let invocations = Cli::parse_from(["canon", "inspect", "invocations", "--run", "run-123"]);
        assert_command!(
            invocations.command,
            Command::Inspect { command: InspectCommand::Invocations { run, output } }
                if run == "run-123" && output == OutputFormat::Markdown
        );

        let evidence = Cli::parse_from(["canon", "inspect", "evidence", "--run", "run-123"]);
        assert_command!(
            evidence.command,
            Command::Inspect { command: InspectCommand::Evidence { run, output } }
                if run == "run-123" && output == OutputFormat::Markdown
        );
    }

    #[test]
    fn skills_update_and_list_parse_correctly() {
        let update = Cli::parse_from(["canon", "skills", "update", "--ai", "copilot"]);
        assert_command!(
            update.command,
            Command::Skills { command: SkillsCommand::Update { ai, output } }
                if ai == AiTarget::Copilot && output == OutputFormat::Text
        );

        let list = Cli::parse_from(["canon", "skills", "list", "--output", "yaml"]);
        assert_command!(
            list.command,
            Command::Skills { command: SkillsCommand::List { output } }
                if output == OutputFormat::Yaml
        );
    }

    #[test]
    fn governance_start_parses_json_flag() {
        let cli = Cli::parse_from(["canon", "governance", "start", "--json"]);

        assert_command!(
            cli.command,
            Command::Governance { command: GovernanceCommand::Start { json } } if json
        );
    }

    #[test]
    fn governance_capabilities_parses_json_flag() {
        let cli = Cli::parse_from(["canon", "governance", "capabilities", "--json"]);

        assert_command!(
            cli.command,
            Command::Governance { command: GovernanceCommand::Capabilities { json } } if json
        );
    }

    #[test]
    fn run_with_dispatches_each_command_variant() {
        let workspace = tempdir().expect("create temp workspace");
        let repo_root = workspace.path().to_path_buf();

        assert_eq!(
            run_with(
                Cli { command: Command::Init { ai: None, output: OutputFormat::Json } },
                repo_root.clone(),
            )
            .expect("init should succeed"),
            0
        );

        assert!(
            run_with(
                Cli {
                    command: Command::Run(Box::new(RunCommand {
                        mode: "not-a-mode".to_string(),
                        system_context: None,
                        risk: "low-impact".to_string(),
                        zone: "green".to_string(),
                        risk_source: None,
                        risk_rationale: None,
                        risk_signals: Vec::new(),
                        zone_source: None,
                        zone_rationale: None,
                        zone_signals: Vec::new(),
                        owner: None,
                        inputs: Vec::new(),
                        inline_inputs: Vec::new(),
                        excluded_paths: Vec::new(),
                        policy_root: None,
                        method_root: None,
                        output: OutputFormat::Json,
                    })),
                },
                repo_root.clone(),
            )
            .is_err()
        );

        assert!(
            run_with(
                Cli { command: Command::Resume { run: "run-missing".to_string() } },
                repo_root.clone(),
            )
            .is_err()
        );

        assert!(
            run_with(
                Cli {
                    command: Command::Status {
                        run: "run-missing".to_string(),
                        output: OutputFormat::Json,
                    },
                },
                repo_root.clone(),
            )
            .is_err()
        );

        assert!(
            run_with(
                Cli {
                    command: Command::Approve(ApproveCommand {
                        run: "run-missing".to_string(),
                        target: None,
                        gate: None,
                        by: None,
                        decision: "approve".to_string(),
                        rationale: "looks good".to_string(),
                    }),
                },
                repo_root.clone(),
            )
            .is_err()
        );

        assert!(
            run_with(
                Cli { command: Command::Verify { run: "run-missing".to_string() } },
                repo_root.clone(),
            )
            .is_err()
        );

        assert_eq!(
            run_with(
                Cli {
                    command: Command::Inspect {
                        command: InspectCommand::Modes { output: OutputFormat::Json },
                    },
                },
                repo_root.clone(),
            )
            .expect("inspect should succeed"),
            0
        );

        assert_eq!(
            run_with(
                Cli {
                    command: Command::Skills {
                        command: SkillsCommand::List { output: OutputFormat::Json },
                    },
                },
                repo_root.clone(),
            )
            .expect("skills list should succeed"),
            0
        );

        assert_eq!(
            run_with(
                Cli {
                    command: Command::Governance {
                        command: GovernanceCommand::Capabilities { json: true },
                    },
                },
                repo_root.clone(),
            )
            .expect("governance capabilities should succeed"),
            0
        );

        assert_eq!(
            run_with(
                Cli {
                    command: Command::List {
                        command: ListCommand::Runs { output: OutputFormat::Json },
                    },
                },
                repo_root.clone(),
            )
            .expect("list runs should succeed"),
            0
        );

        assert!(
            run_with(
                Cli {
                    command: Command::Publish(PublishCommand {
                        run_id: "run-missing".to_string(),
                        to: None,
                    }),
                },
                repo_root,
            )
            .is_err()
        );
    }
}
