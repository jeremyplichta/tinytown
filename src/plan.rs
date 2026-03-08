/*
 * Copyright (c) 2024-Present, Jeremy Plichta
 * Licensed under the MIT License
 */

//! Task planning DSL - Simple TOML-based task definitions.
//!
//! Tasks are defined in `tasks.toml` and can be synced to/from Redis.

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::{Channel, Error, Result, Task};

/// Tasks file structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TasksFile {
    /// Optional metadata
    #[serde(default)]
    pub meta: TasksMeta,

    /// List of tasks
    #[serde(default)]
    pub tasks: Vec<TaskEntry>,
}

/// Metadata for the tasks file
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TasksMeta {
    /// Description of this task plan
    #[serde(default)]
    pub description: String,

    /// Default agent for unassigned tasks
    #[serde(default)]
    pub default_agent: Option<String>,
}

/// A single task entry in the DSL
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskEntry {
    /// Task ID (auto-generated if not provided)
    #[serde(default = "generate_short_id")]
    pub id: String,

    /// Task description
    pub description: String,

    /// Assigned agent (optional)
    #[serde(default)]
    pub agent: Option<String>,

    /// Task status: pending, assigned, running, completed, failed
    #[serde(default = "default_status")]
    pub status: String,

    /// Optional tags
    #[serde(default)]
    pub tags: Vec<String>,

    /// Optional parent task ID
    #[serde(default)]
    pub parent: Option<String>,
}

fn generate_short_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    format!("task-{}", ts % 100000)
}

fn default_status() -> String {
    "pending".to_string()
}

/// Initialize a new tasks.toml file
pub fn init_tasks_file(town_path: &Path) -> Result<()> {
    let tasks_file = town_path.join("tasks.toml");

    let template = TasksFile {
        meta: TasksMeta {
            description: "Task plan for this project".to_string(),
            default_agent: None,
        },
        tasks: vec![TaskEntry {
            id: "example-1".to_string(),
            description: "Example task - replace with your own".to_string(),
            agent: None,
            status: "pending".to_string(),
            tags: vec!["example".to_string()],
            parent: None,
        }],
    };

    let content =
        toml::to_string_pretty(&template).map_err(|e| Error::Io(std::io::Error::other(e)))?;

    std::fs::write(&tasks_file, content)?;
    Ok(())
}

/// Load tasks from tasks.toml
pub fn load_tasks_file(town_path: &Path) -> Result<TasksFile> {
    let tasks_file = town_path.join("tasks.toml");
    let content = std::fs::read_to_string(&tasks_file)?;
    let tasks: TasksFile =
        toml::from_str(&content).map_err(|e| Error::Io(std::io::Error::other(e)))?;
    Ok(tasks)
}

/// Save tasks to tasks.toml
pub fn save_tasks_file(town_path: &Path, tasks: &TasksFile) -> Result<()> {
    let tasks_file = town_path.join("tasks.toml");
    let content = toml::to_string_pretty(tasks).map_err(|e| Error::Io(std::io::Error::other(e)))?;
    std::fs::write(&tasks_file, content)?;
    Ok(())
}

/// Push tasks from tasks.toml to Redis
pub async fn push_tasks_to_redis(town_path: &Path, channel: &Channel) -> Result<usize> {
    let tasks_file = load_tasks_file(town_path)?;
    let mut count = 0;

    for entry in &tasks_file.tasks {
        let mut task = Task::new(&entry.description);
        // Include entry ID in tags for tracking
        let mut all_tags: Vec<String> = entry.tags.clone();
        all_tags.push(format!("plan:{}", entry.id));
        task = task.with_tags(all_tags);

        // Set state based on status
        match entry.status.as_str() {
            "completed" => task.complete("Completed via sync"),
            "failed" => task.fail("Failed via sync"),
            _ => {}
        }

        channel.set_task(&task).await?;
        count += 1;
    }

    Ok(count)
}

/// Pull tasks from Redis to tasks.toml (placeholder for now)
pub async fn pull_tasks_from_redis(town_path: &Path, _channel: &Channel) -> Result<usize> {
    let tasks_file = town_path.join("tasks.toml");
    if !tasks_file.exists() {
        init_tasks_file(town_path)?;
    }
    // TODO: Implement full Redis scan for mt:task:* keys
    Ok(0)
}
