mod agents;
mod approval;
mod provider_fix;
mod tools;
mod workflow;

use approval::CliApprovalHandler;
use clap::Parser;
use colored::Colorize;
use futures::StreamExt;
use pulsehive::prelude::*;
use pulsehive::{HiveMind, Task};
use provider_fix::FixedProvider;
use pulsehive_openai::OpenAIConfig;
use std::path::PathBuf;
use workflow::{build_dry_run_pipeline, build_pipeline};

#[derive(Parser)]
#[command(
    name = "devstudio",
    about = "Multi-agent CLI code agent powered by PulseHive",
    version
)]
struct Cli {
    /// Natural language description of the task to perform
    task: String,

    /// Path to the target repository
    #[arg(short, long, default_value = ".")]
    repo: PathBuf,

    /// LLM provider name
    #[arg(short, long, default_value = "openai")]
    provider: String,

    /// LLM model identifier
    #[arg(short, long, default_value = "glm-4-plus")]
    model: String,

    /// Override LLM API base URL
    #[arg(long, default_value = "https://open.bigmodel.cn/api/paas/v4")]
    base_url: String,

    /// LLM API key
    #[arg(long, env = "DEVSTUDIO_API_KEY")]
    api_key: String,

    /// Skip approval prompts (auto-approve everything)
    #[arg(long)]
    approve_all: bool,

    /// Show plan only, don't execute code changes
    #[arg(long)]
    dry_run: bool,

    /// PulseDB substrate path
    #[arg(long, default_value = ".devstudio/substrate.db")]
    substrate: PathBuf,

    /// Show all HiveEvent details
    #[arg(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Resolve repo path
    let repo_path = cli.repo.canonicalize().unwrap_or_else(|_| cli.repo.clone());

    eprintln!(
        "\n{}",
        format!(
            "  DevStudio — {}",
            if cli.dry_run {
                "Dry Run (plan only)"
            } else {
                "Full Pipeline"
            }
        )
        .bold()
    );
    eprintln!("  {} {}", "Task:".dimmed(), cli.task);
    eprintln!("  {} {}", "Repo:".dimmed(), repo_path.display());
    eprintln!("  {} {}", "Model:".dimmed(), cli.model);
    eprintln!();

    // Ensure substrate directory exists
    let substrate_path = repo_path.join(&cli.substrate);
    if let Some(parent) = substrate_path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }

    // Set up LLM provider (custom impl with correct tool_call wire format)
    let provider = FixedProvider::new(
        OpenAIConfig::new(&cli.api_key, &cli.model).with_base_url(cli.base_url.clone()),
    );

    // Build HiveMind
    let hive = HiveMind::builder()
        .substrate_path(&substrate_path)
        .llm_provider(&cli.provider, provider)
        .approval_handler(CliApprovalHandler::new(cli.approve_all))
        .build()?;

    // Build pipeline
    let pipeline = if cli.dry_run {
        build_dry_run_pipeline(&cli.task, &repo_path, &cli.provider, &cli.model)
    } else {
        build_pipeline(&cli.task, &repo_path, &cli.provider, &cli.model)
    };

    // Deploy and stream events
    let mut stream = hive
        .deploy(vec![pipeline], vec![Task::new(&cli.task)])
        .await?;

    let mut agents_started = 0u32;
    let mut agents_completed = 0u32;

    while let Some(event) = stream.next().await {
        match &event {
            HiveEvent::AgentStarted { name, kind, .. } => {
                agents_started += 1;
                let icon = match name.as_str() {
                    "explorer" => "🔍",
                    "planner" => "📋",
                    "coder" => "💻",
                    "tester" => "🧪",
                    _ => "🔄",
                };
                eprintln!(
                    "  {} {} {}",
                    icon,
                    name.bold(),
                    format!("started ({:?})", kind).dimmed()
                );
            }
            HiveEvent::ToolCallStarted { tool_name, .. } => {
                eprintln!("    {} {}", "🔧".dimmed(), tool_name);
            }
            HiveEvent::ToolCallCompleted {
                tool_name,
                duration_ms,
                ..
            } => {
                if cli.verbose {
                    eprintln!(
                        "    {} {} ({}ms)",
                        "✓".green(),
                        tool_name,
                        duration_ms
                    );
                }
            }
            HiveEvent::AgentCompleted { outcome, .. } => {
                agents_completed += 1;
                match outcome {
                    AgentOutcome::Complete { response } => {
                        let truncated = if response.len() > 120 {
                            format!("{}...", &response[..120])
                        } else {
                            response.clone()
                        };
                        eprintln!("  {} {}", "✅".green(), truncated.dimmed());
                    }
                    AgentOutcome::Error { error } => {
                        eprintln!("  {} {}", "❌".red(), error);
                    }
                    AgentOutcome::MaxIterationsReached => {
                        eprintln!("  ⚠️  {}", "Max iterations reached".yellow());
                    }
                }

                // Break when the top-level pipeline completes
                // The pipeline itself is an agent, so we get N child agents + 1 pipeline
                if agents_completed >= agents_started {
                    break;
                }
            }
            HiveEvent::LlmCallStarted {
                model,
                message_count,
                ..
            } => {
                if cli.verbose {
                    eprintln!(
                        "    {} LLM call: {} ({} messages)",
                        "→".dimmed(),
                        model,
                        message_count
                    );
                }
            }
            HiveEvent::LlmCallCompleted {
                duration_ms, ..
            } => {
                if cli.verbose {
                    eprintln!("    {} LLM completed ({}ms)", "←".dimmed(), duration_ms);
                }
            }
            HiveEvent::SubstratePerceived {
                agent_id,
                experience_count,
                insight_count,
                ..
            } => {
                if cli.verbose {
                    eprintln!(
                        "    {} {} perceived {} experiences, {} insights",
                        "👁".dimmed(),
                        agent_id,
                        experience_count,
                        insight_count
                    );
                }
            }
            _ => {
                if cli.verbose {
                    eprintln!("    {:?}", event);
                }
            }
        }
    }

    eprintln!(
        "\n  {} Done! {} agents completed.\n",
        "✅".green(),
        agents_completed
    );

    hive.shutdown();
    std::process::exit(0);
}
