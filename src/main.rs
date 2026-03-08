/*
 * Copyright (c) 2024-Present, Jeremy Plichta
 * Licensed under the MIT License
 */

//! Tinytown CLI - Simple multi-agent orchestration.

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use tracing::info;
use tracing_subscriber::EnvFilter;

use tinytown::{Result, Task, Town, plan};

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

        /// Model to use (uses default_model from config if not specified)
        #[arg(short, long)]
        model: Option<String>,
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

    /// Start the conductor (interactive orchestration mode)
    Conductor,

    /// Plan tasks without starting agents (edit tasks.toml)
    Plan {
        /// Initialize a new tasks.toml file
        #[arg(short, long)]
        init: bool,
    },

    /// Sync tasks.toml with Redis
    Sync {
        /// Direction: 'push' (file→Redis) or 'pull' (Redis→file)
        #[arg(default_value = "push")]
        direction: String,
    },
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
    tracing_subscriber::fmt().with_env_filter(filter).init();

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
            let model = model.unwrap_or_else(|| town.config().default_model.clone());
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
            tokio::signal::ctrl_c()
                .await
                .expect("Failed to listen for ctrl-c");
            info!("👋 Shutting down...");
        }

        Commands::Stop => {
            info!("👋 Town stopped (Redis will be cleaned up)");
        }

        Commands::Conductor => {
            let town = Town::connect(&cli.town).await?;
            let config = town.config();

            // Build conductor context with current state
            let agents = town.list_agents().await;
            let mut agent_status = String::new();
            for agent in &agents {
                let inbox = town.channel().inbox_len(agent.id).await.unwrap_or(0);
                agent_status.push_str(&format!(
                    "  - {} ({:?}) - {} messages pending\n",
                    agent.name, agent.state, inbox
                ));
            }
            if agent_status.is_empty() {
                agent_status = "  (no agents spawned yet)\n".to_string();
            }

            let context = format!(
                r#"# Tinytown Conductor

You are the **conductor** of Tinytown "{name}" - like the train conductor guiding the miniature train through Tiny Town, Colorado, you coordinate AI agents working on this project.

## Current Town State

**Town:** {name}
**Location:** {root}
**Agents ({agent_count}):**
{agent_status}

## Your Capabilities

You have access to the `tt` CLI tool. Run these commands in your shell to orchestrate:

### Spawn agents
```bash
tt spawn <name>                    # Create agent with default model
tt spawn <name> --model <model>    # Create with specific model (claude, auggie, codex)
```

### Assign tasks
```bash
tt assign <agent> "<task description>"
```

### Check status
```bash
tt status    # Overview of town and agents
tt list      # List all agents
```

### Plan tasks (for complex work)
```bash
tt plan --init              # Create tasks.toml for planning
tt plan                     # View planned tasks
tt sync push                # Send plan to Redis
```

## Your Role

1. **Understand** what the user wants to accomplish
2. **Break down** complex requests into discrete tasks
3. **Spawn** appropriate agents for different roles (backend, frontend, tester, reviewer, etc.)
4. **Assign** tasks to agents with clear, actionable descriptions
5. **Monitor** progress with `tt status`
6. **Coordinate** handoffs between agents
7. **Help** resolve blockers when agents are stuck

## Guidelines

- Be proactive: spawn agents and assign tasks without waiting to be told exactly how
- Be specific: task descriptions should be clear and actionable
- Be efficient: parallelize independent work across multiple agents
- Be helpful: if an agent has pending messages, check what they need

## Example Workflow

User: "Build a user authentication system"

You might:
1. `tt spawn architect` - for API design
2. `tt spawn backend` - for implementation
3. `tt spawn tester` - for tests
4. `tt assign architect "Design REST API for user auth: signup, login, logout, password reset. Output OpenAPI spec."`
5. Monitor with `tt status`, then assign implementation to backend
6. `tt assign tester "Write integration tests for auth API endpoints"`

Now, help the user orchestrate their project!
"#,
                name = config.name,
                root = cli.town.display(),
                agent_count = agents.len(),
                agent_status = agent_status,
            );

            // Write context to a temp file for the model
            let context_file = cli.town.join(".conductor_context.md");
            std::fs::write(&context_file, &context)?;

            // Get the model command
            let model = &config.default_model;
            let model_config = config.models.get(model);

            info!("🚂 Starting conductor with {} model...", model);
            info!("   Context: {}", context_file.display());
            info!("");

            // Get the command for the model
            let command = if let Some(m) = model_config {
                m.command.clone()
            } else {
                model.clone() // Fallback to model name as command
            };

            // Launch the model with the context
            // Claude CLI: cat context | claude
            // Most AI CLIs accept input from stdin or can read files
            let shell_cmd = format!(
                "cat '{}' && echo '' && echo '---' && echo '' && {}",
                context_file.display(),
                command
            );

            info!("   Running: {}", command);
            info!("");

            // Execute the AI model interactively
            let status = std::process::Command::new("sh")
                .arg("-c")
                .arg(&shell_cmd)
                .current_dir(&cli.town)
                .stdin(std::process::Stdio::inherit())
                .stdout(std::process::Stdio::inherit())
                .stderr(std::process::Stdio::inherit())
                .status()?;

            if !status.success() {
                info!("❌ Conductor exited with error");
            } else {
                info!("👋 Conductor signing off!");
            }

            // Cleanup context file
            let _ = std::fs::remove_file(&context_file);
        }

        Commands::Plan { init } => {
            if init {
                plan::init_tasks_file(&cli.town)?;
                info!("📝 Created tasks.toml - edit it to plan your work!");
            } else {
                // Open tasks.toml in editor
                let tasks_file = cli.town.join("tasks.toml");
                if !tasks_file.exists() {
                    info!("No tasks.toml found. Run 'tt plan --init' first.");
                } else {
                    let tasks = plan::load_tasks_file(&cli.town)?;
                    info!("📋 Tasks in plan ({}):", tasks_file.display());
                    for task in &tasks.tasks {
                        let status_icon = match task.status.as_str() {
                            "pending" => "⏳",
                            "assigned" => "📌",
                            "running" => "🔄",
                            "completed" => "✅",
                            "failed" => "❌",
                            _ => "❓",
                        };
                        let agent = task.agent.as_deref().unwrap_or("unassigned");
                        info!(
                            "  {} [{}] {} - {}",
                            status_icon, agent, task.id, task.description
                        );
                    }
                }
            }
        }

        Commands::Sync { direction } => {
            let town = Town::connect(&cli.town).await?;
            match direction.as_str() {
                "push" => {
                    let count = plan::push_tasks_to_redis(&cli.town, town.channel()).await?;
                    info!("⬆️  Pushed {} tasks from tasks.toml to Redis", count);
                }
                "pull" => {
                    let count = plan::pull_tasks_from_redis(&cli.town, town.channel()).await?;
                    info!("⬇️  Pulled {} tasks from Redis to tasks.toml", count);
                }
                _ => {
                    info!("Usage: tt sync [push|pull]");
                    info!("  push - Send tasks.toml to Redis");
                    info!("  pull - Save Redis tasks to tasks.toml");
                }
            }
        }
    }

    Ok(())
}
