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

pub fn run() -> CliResult<i32> {
    tracing_subscriber::fmt::try_init().ok();

    let cli = Cli::parse();
    let repo_root = std::env::current_dir()?;
    let service = EngineService::new(repo_root);

    match cli.command {
        Command::Init { ai, output } => {
            commands::init::execute(&service, ai.map(Into::into), output)
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
                &service,
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
        Command::Resume { run } => commands::resume::execute(&service, &run),
        Command::Status { run, output } => commands::status::execute(&service, &run, output),
        Command::Approve(ApproveCommand { run, target, gate, by, decision, rationale }) => {
            commands::approve::execute(&service, &run, target, gate, by, decision, rationale)
        }
        Command::Verify { .. } => commands::verify::execute(),
        Command::Inspect { command } => commands::inspect::execute(&service, command),
        Command::Skills { command } => commands::skills::execute(&service, command),
        Command::List { command } => commands::list::execute(&service, command),
        Command::Publish(cmd) => commands::publish::execute(&service, cmd),
    }
}

#[cfg(test)]
mod tests {
    use clap::Parser;

    use super::{
        AiTarget, ApproveCommand, Cli, Command, InspectCommand, OutputFormat, PublishCommand,
        RunCommand, SkillsCommand,
    };
    use canon_engine::AiTool;

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

        match cli.command {
            Command::Inspect { command: InspectCommand::Modes { output } } => {
                assert_eq!(output, OutputFormat::Markdown);
            }
            other => panic!("unexpected command parsed: {other:?}"),
        }
    }

    #[test]
    fn skills_install_parses_ai_and_default_output() {
        let cli = Cli::parse_from(["canon", "skills", "install", "--ai", "codex"]);

        match cli.command {
            Command::Skills { command: SkillsCommand::Install { ai, output } } => {
                assert_eq!(ai, AiTarget::Codex);
                assert_eq!(output, OutputFormat::Text);
            }
            other => panic!("unexpected command parsed: {other:?}"),
        }
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

        match cli.command {
            Command::Run(run) => {
                let RunCommand {
                    mode,
                    system_context,
                    risk,
                    zone,
                    risk_source,
                    risk_rationale,
                    risk_signals,
                    owner,
                    inputs,
                    inline_inputs,
                    excluded_paths,
                    ..
                } = *run;

                assert_eq!(mode, "requirements");
                assert_eq!(system_context, None);
                assert_eq!(risk, "low-impact");
                assert_eq!(zone, "green");
                assert_eq!(risk_source.as_deref(), Some("inferred-confirmed"));
                assert_eq!(risk_rationale.as_deref(), Some("Production boundary detected"));
                assert_eq!(risk_signals, vec!["Detected boundary keyword"]);
                assert_eq!(owner.as_deref(), Some("Owner <owner@example.com>"));
                assert_eq!(inputs, vec!["idea.md"]);
                assert!(inline_inputs.is_empty());
                assert_eq!(excluded_paths, vec!["target/"]);
            }
            other => panic!("unexpected command parsed: {other:?}"),
        }
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

        match cli.command {
            Command::Run(run) => {
                let RunCommand { inputs, inline_inputs, output, .. } = *run;

                assert_eq!(inputs, vec!["idea.md", "constraints.md", "canon-input/requirements"]);
                assert!(inline_inputs.is_empty());
                assert_eq!(output, OutputFormat::Json);
            }
            other => panic!("unexpected command parsed: {other:?}"),
        }
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

        match cli.command {
            Command::Run(run) => {
                let RunCommand { inputs, inline_inputs, .. } = *run;

                assert!(inputs.is_empty());
                assert_eq!(inline_inputs.len(), 1);
                assert!(inline_inputs[0].contains("Need explicit governance"));
            }
            other => panic!("unexpected command parsed: {other:?}"),
        }
    }

    #[test]
    fn init_status_resume_verify_and_approve_commands_parse_expected_defaults() {
        let init_cli = Cli::parse_from(["canon", "init", "--ai", "claude", "--output", "yaml"]);
        match init_cli.command {
            Command::Init { ai, output } => {
                assert_eq!(ai, Some(AiTarget::Claude));
                assert_eq!(output, OutputFormat::Yaml);
            }
            other => panic!("unexpected init command parsed: {other:?}"),
        }

        let status_cli = Cli::parse_from(["canon", "status", "--run", "run-123"]);
        match status_cli.command {
            Command::Status { run, output } => {
                assert_eq!(run, "run-123");
                assert_eq!(output, OutputFormat::Text);
            }
            other => panic!("unexpected status command parsed: {other:?}"),
        }

        let resume_cli = Cli::parse_from(["canon", "resume", "--run", "run-123"]);
        match resume_cli.command {
            Command::Resume { run } => assert_eq!(run, "run-123"),
            other => panic!("unexpected resume command parsed: {other:?}"),
        }

        let verify_cli = Cli::parse_from(["canon", "verify", "--run", "run-123"]);
        match verify_cli.command {
            Command::Verify { run } => assert_eq!(run, "run-123"),
            other => panic!("unexpected verify command parsed: {other:?}"),
        }

        let publish_cli = Cli::parse_from(["canon", "publish", "run-123", "--to", "docs/public"]);
        match publish_cli.command {
            Command::Publish(PublishCommand { run_id, to }) => {
                assert_eq!(run_id, "run-123");
                assert_eq!(to, Some(std::path::PathBuf::from("docs/public")));
            }
            other => panic!("unexpected publish command parsed: {other:?}"),
        }

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
        match approve_cli.command {
            Command::Approve(ApproveCommand { run, target, gate, by, decision, rationale }) => {
                assert_eq!(run, "run-123");
                assert_eq!(target.as_deref(), Some("gate:risk"));
                assert_eq!(gate, None);
                assert_eq!(by, None);
                assert_eq!(decision, "approve");
                assert_eq!(rationale, "looks good");
            }
            other => panic!("unexpected approve command parsed: {other:?}"),
        }
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
        match classification.command {
            Command::Inspect {
                command: InspectCommand::RiskZone { mode, inputs, inline_inputs, output, .. },
            } => {
                assert_eq!(mode, "discovery");
                assert_eq!(inputs, vec!["canon-input/discovery.md"]);
                assert!(inline_inputs.is_empty());
                assert_eq!(output, OutputFormat::Markdown);
            }
            other => panic!("unexpected risk-zone inspect command parsed: {other:?}"),
        }

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
        match inline_classification.command {
            Command::Inspect {
                command: InspectCommand::RiskZone { mode, inputs, inline_inputs, output, .. },
            } => {
                assert_eq!(mode, "requirements");
                assert!(inputs.is_empty());
                assert_eq!(inline_inputs.len(), 1);
                assert_eq!(output, OutputFormat::Json);
            }
            other => panic!("unexpected inline risk-zone command parsed: {other:?}"),
        }

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
        match clarity.command {
            Command::Inspect { command: InspectCommand::Clarity { mode, inputs, output } } => {
                assert_eq!(mode, "requirements");
                assert_eq!(
                    inputs,
                    vec![
                        "canon-input/requirements.md",
                        "canon-input/requirements/supporting/constraints.md"
                    ]
                );
                assert_eq!(output, OutputFormat::Json);
            }
            other => panic!("unexpected clarity inspect command parsed: {other:?}"),
        }

        let artifacts = Cli::parse_from([
            "canon",
            "inspect",
            "artifacts",
            "--run",
            "run-123",
            "--output",
            "json",
        ]);
        match artifacts.command {
            Command::Inspect { command: InspectCommand::Artifacts { run, output } } => {
                assert_eq!(run, "run-123");
                assert_eq!(output, OutputFormat::Json);
            }
            other => panic!("unexpected artifacts inspect command parsed: {other:?}"),
        }

        let invocations = Cli::parse_from(["canon", "inspect", "invocations", "--run", "run-123"]);
        match invocations.command {
            Command::Inspect { command: InspectCommand::Invocations { run, output } } => {
                assert_eq!(run, "run-123");
                assert_eq!(output, OutputFormat::Markdown);
            }
            other => panic!("unexpected invocations inspect command parsed: {other:?}"),
        }

        let evidence = Cli::parse_from(["canon", "inspect", "evidence", "--run", "run-123"]);
        match evidence.command {
            Command::Inspect { command: InspectCommand::Evidence { run, output } } => {
                assert_eq!(run, "run-123");
                assert_eq!(output, OutputFormat::Markdown);
            }
            other => panic!("unexpected evidence inspect command parsed: {other:?}"),
        }
    }

    #[test]
    fn skills_update_and_list_parse_correctly() {
        let update = Cli::parse_from(["canon", "skills", "update", "--ai", "copilot"]);
        match update.command {
            Command::Skills { command: SkillsCommand::Update { ai, output } } => {
                assert_eq!(ai, AiTarget::Copilot);
                assert_eq!(output, OutputFormat::Text);
            }
            other => panic!("unexpected skills update command parsed: {other:?}"),
        }

        let list = Cli::parse_from(["canon", "skills", "list", "--output", "yaml"]);
        match list.command {
            Command::Skills { command: SkillsCommand::List { output } } => {
                assert_eq!(output, OutputFormat::Yaml);
            }
            other => panic!("unexpected skills list command parsed: {other:?}"),
        }
    }
}
