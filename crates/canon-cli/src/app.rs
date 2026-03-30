use std::path::PathBuf;

use canon_engine::EngineService;
use clap::{Parser, Subcommand, ValueEnum};

use crate::commands;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum OutputFormat {
    Text,
    Json,
    Yaml,
    Markdown,
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Text => "text",
            Self::Json => "json",
            Self::Yaml => "yaml",
            Self::Markdown => "markdown",
        };
        write!(f, "{value}")
    }
}

impl Default for OutputFormat {
    fn default() -> Self {
        Self::Text
    }
}

#[derive(Debug, Subcommand, Clone)]
pub enum InspectCommand {
    Modes {
        #[arg(long, default_value_t)]
        output: OutputFormat,
    },
    Methods {
        #[arg(long, default_value_t)]
        output: OutputFormat,
    },
    Policies {
        #[arg(long, default_value_t)]
        output: OutputFormat,
    },
    Artifacts {
        #[arg(long)]
        run: String,
        #[arg(long, default_value_t)]
        output: OutputFormat,
    },
    Invocations {
        #[arg(long)]
        run: String,
        #[arg(long, default_value_t)]
        output: OutputFormat,
    },
    Evidence {
        #[arg(long)]
        run: String,
        #[arg(long, default_value_t)]
        output: OutputFormat,
    },
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Init {
        #[arg(long, default_value_t)]
        output: OutputFormat,
    },
    Run {
        #[arg(long)]
        mode: String,
        #[arg(long)]
        risk: String,
        #[arg(long)]
        zone: String,
        #[arg(long)]
        owner: String,
        #[arg(long = "input")]
        inputs: Vec<String>,
        #[arg(long = "exclude")]
        excluded_paths: Vec<String>,
        #[arg(long)]
        policy_root: Option<String>,
        #[arg(long)]
        method_root: Option<String>,
        #[arg(long, default_value_t)]
        output: OutputFormat,
    },
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
    Approve {
        #[arg(long)]
        run: String,
        #[arg(
            long,
            help = "Approval target in the form gate:<gate-kind> or invocation:<request-id>"
        )]
        target: Option<String>,
        #[arg(long, hide = true)]
        gate: Option<String>,
        #[arg(long)]
        by: String,
        #[arg(long)]
        decision: String,
        #[arg(long)]
        rationale: String,
    },
    Verify {
        #[arg(long)]
        run: String,
    },
    Inspect {
        #[command(subcommand)]
        command: InspectCommand,
    },
}

#[derive(Debug, Parser)]
#[command(name = "canon")]
#[command(about = "A governed method engine for AI-assisted software engineering.")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

pub fn run() -> Result<i32, Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::try_init().ok();

    let cli = Cli::parse();
    let repo_root = std::env::current_dir()?;
    let service = EngineService::new(PathBuf::from(repo_root));

    match cli.command {
        Command::Init { output } => commands::init::execute(&service, output),
        Command::Run {
            mode,
            risk,
            zone,
            owner,
            inputs,
            excluded_paths,
            policy_root,
            method_root,
            output,
        } => commands::run::execute(
            &service,
            mode,
            risk,
            zone,
            owner,
            inputs,
            excluded_paths,
            policy_root,
            method_root,
            output,
        ),
        Command::Resume { run } => commands::resume::execute(&service, &run),
        Command::Status { run, output } => commands::status::execute(&service, &run, output),
        Command::Approve { run, target, gate, by, decision, rationale } => {
            commands::approve::execute(&service, &run, target, gate, by, decision, rationale)
        }
        Command::Verify { .. } => commands::verify::execute(),
        Command::Inspect { command } => commands::inspect::execute(&service, command),
    }
}
