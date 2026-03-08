/*
 * Copyright (c) 2024-Present, Jeremy Plichta
 * Licensed under the MIT License
 */

//! Tinytown CLI - Simple multi-agent orchestration.

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use tracing::info;
use tracing_subscriber::EnvFilter;

use tinytown::{Result, Task, Town};

#[derive(Parser)]
#[command(name = "tt")]
#[command(author, version, about = "Tinytown - Simple multi-agent orchestration using Redis", long_about = None)]
struct Cli {
    /// Town directory (defaults to current directory)
    #[arg(short, long, global = true, default_value = ".")]
    town: PathBuf,

    /// Enable verbose logging
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new town
    Init {
        /// Town name
        #[arg(short, long)]
        name: Option<String>,
    },

    /// Spawn a new agent
    Spawn {
        /// Agent name
        name: String,

        /// Model to use (default: claude)
        #[arg(short, long, default_value = "claude")]
        model: String,
    },

    /// List all agents
    List,

    /// Assign a task to an agent
    Assign {
        /// Agent name
        agent: String,

        /// Task description
        task: String,
    },

    /// Show town status
    Status,

    /// Start the town (Redis server)
    Start,

    /// Stop the town
    Stop,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Setup logging
    let filter = if cli.verbose {
        EnvFilter::new("debug")
    } else {
        EnvFilter::new("info")
    };
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .init();

    match cli.command {
        Commands::Init { name } => {
            let name = name.unwrap_or_else(|| {
                cli.town
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("tinytown")
                    .to_string()
            });

            let town = Town::init(&cli.town, &name).await?;
            info!("✨ Initialized town '{}' at {}", name, cli.town.display());
            info!("📡 Redis running with Unix socket for fast message passing");
            info!("🚀 Run 'tt spawn <name>' to create agents");

            // Keep town alive briefly to show it's working
            drop(town);
        }

        Commands::Spawn { name, model } => {
            let town = Town::connect(&cli.town).await?;
            let agent = town.spawn_agent(&name, &model).await?;
            info!("🤖 Spawned agent '{}' using model '{}'", name, model);
            info!("   ID: {}", agent.id());
        }

        Commands::List => {
            let town = Town::connect(&cli.town).await?;
            let agents = town.list_agents().await;

            if agents.is_empty() {
                info!("No agents. Run 'tt spawn <name>' to create one.");
            } else {
                info!("Agents:");
                for agent in agents {
                    info!("  {} ({}) - {:?}", agent.name, agent.id, agent.state);
                }
            }
        }

        Commands::Assign { agent, task } => {
            let town = Town::connect(&cli.town).await?;
            let handle = town.agent(&agent).await?;
            let task = Task::new(&task);
            let task_id = handle.assign(task).await?;
            info!("📋 Assigned task {} to agent '{}'", task_id, agent);
        }

        Commands::Status => {
            let town = Town::connect(&cli.town).await?;
            let config = town.config();

            info!("🏘️  Town: {}", config.name);
            info!("📂 Root: {}", town.root().display());
            info!("📡 Redis: {}", config.redis_url());

            let agents = town.list_agents().await;
            info!("🤖 Agents: {}", agents.len());

            for agent in agents {
                let inbox_len = town.channel().inbox_len(agent.id).await.unwrap_or(0);
                info!(
                    "   {} ({:?}) - {} messages pending",
                    agent.name, agent.state, inbox_len
                );
            }
        }

        Commands::Start => {
            let _town = Town::connect(&cli.town).await?;
            info!("🚀 Town started");
            // Keep running until Ctrl+C
            tokio::signal::ctrl_c().await.expect("Failed to listen for ctrl-c");
            info!("👋 Shutting down...");
        }

        Commands::Stop => {
            info!("👋 Town stopped (Redis will be cleaned up)");
        }
    }

    Ok(())
}

