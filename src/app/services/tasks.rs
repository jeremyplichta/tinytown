/*
 * Copyright (c) 2024-Present, Jeremy Plichta
 * Licensed under the MIT License
 */

//! Task assignment service.
//!
//! Provides operations for assigning tasks to agents.

use crate::agent::AgentId;
use crate::channel::Channel;
use crate::error::Result;
use crate::message::MessageType;
use crate::task::{Task, TaskId};
use crate::town::Town;

/// Result of an assign operation.
#[derive(Debug, Clone)]
pub struct AssignResult {
    pub task_id: TaskId,
    pub agent_id: AgentId,
    pub agent_name: String,
}

/// Information about a pending task.
#[derive(Debug, Clone)]
pub struct PendingTask {
    pub task_id: TaskId,
    pub description: String,
    pub agent_id: AgentId,
    pub agent_name: String,
}

/// Result of completing a task.
#[derive(Debug, Clone)]
pub struct CompleteResult {
    pub task: Task,
    pub result: String,
    pub cleared_current_task: bool,
    pub tasks_completed: Option<u64>,
}

/// Service for task-related operations.
pub struct TaskService;

impl TaskService {
    /// Assign a task to an agent.
    pub async fn assign(town: &Town, agent_name: &str, description: &str) -> Result<AssignResult> {
        let handle = town.agent(agent_name).await?;

        // Create a persisted Task record for tracking
        let mut task_record = Task::new(description);
        task_record.assign(handle.id());
        let task_id = handle.assign(task_record).await?;

        Ok(AssignResult {
            task_id,
            agent_id: handle.id(),
            agent_name: agent_name.to_string(),
        })
    }

    /// List pending tasks across all agents.
    pub async fn list_pending(town: &Town) -> Result<Vec<PendingTask>> {
        let agents = town.list_agents().await;
        let channel = town.channel();
        let mut pending = Vec::new();

        for agent in agents {
            let messages = channel.peek_inbox(agent.id, 100).await.unwrap_or_default();

            for msg in messages {
                match &msg.msg_type {
                    MessageType::TaskAssign { task_id } => {
                        if let Ok(tid) = task_id.parse::<TaskId>()
                            && let Ok(Some(task)) = channel.get_task(tid).await
                        {
                            pending.push(PendingTask {
                                task_id: tid,
                                description: task.description,
                                agent_id: agent.id,
                                agent_name: agent.name.clone(),
                            });
                        }
                    }
                    MessageType::Task { description } => {
                        // Generate a temporary ID for non-persisted tasks
                        pending.push(PendingTask {
                            task_id: TaskId::new(),
                            description: description.clone(),
                            agent_id: agent.id,
                            agent_name: agent.name.clone(),
                        });
                    }
                    _ => {}
                }
            }
        }

        Ok(pending)
    }

    /// Get a task by ID.
    ///
    /// Reserved for future use (e.g., task status endpoint).
    #[allow(dead_code)]
    pub async fn get(channel: &Channel, task_id: TaskId) -> Result<Option<Task>> {
        channel.get_task(task_id).await
    }

    /// Record the task an agent is currently working on.
    pub async fn set_current_for_agent(
        channel: &Channel,
        agent_id: AgentId,
        task_id: TaskId,
    ) -> Result<()> {
        if let Some(mut agent) = channel.get_agent_state(agent_id).await? {
            agent.current_task = Some(task_id);
            agent.last_heartbeat = chrono::Utc::now();
            channel.set_agent_state(&agent).await?;
        }
        Ok(())
    }

    /// Resolve the current task for an agent.
    ///
    /// Prefer the explicit `current_task` pointer, but fall back to a single
    /// non-terminal assigned task when older state does not have the pointer set yet.
    pub async fn current_for_agent(channel: &Channel, agent_id: AgentId) -> Result<Option<Task>> {
        if let Some(agent) = channel.get_agent_state(agent_id).await?
            && let Some(task_id) = agent.current_task
        {
            return channel.get_task(task_id).await;
        }

        let mut assigned = channel
            .list_tasks()
            .await?
            .into_iter()
            .filter(|task| task.assigned_to == Some(agent_id) && !task.state.is_terminal());

        let current = assigned.next();
        if assigned.next().is_some() {
            Ok(None)
        } else {
            Ok(current)
        }
    }

    /// Complete a persisted task and clear the agent's tracked current task when it matches.
    pub async fn complete(
        channel: &Channel,
        task_id: TaskId,
        result: Option<String>,
    ) -> Result<Option<CompleteResult>> {
        let Some(mut task) = channel.get_task(task_id).await? else {
            return Ok(None);
        };

        let was_terminal = task.state.is_terminal();
        let result_msg = result.unwrap_or_else(|| "Completed".to_string());
        task.complete(&result_msg);
        channel.set_task(&task).await?;

        let mut cleared_current_task = false;
        let mut tasks_completed = None;

        if let Some(agent_id) = task.assigned_to
            && let Some(mut agent) = channel.get_agent_state(agent_id).await?
        {
            if agent.current_task == Some(task_id) {
                agent.current_task = None;
                cleared_current_task = true;
            }
            agent.last_heartbeat = chrono::Utc::now();
            channel.set_agent_state(&agent).await?;

            if !was_terminal {
                tasks_completed = Some(channel.increment_agent_tasks_completed(agent_id).await?);
            }
        }

        Ok(Some(CompleteResult {
            task,
            result: result_msg,
            cleared_current_task,
            tasks_completed,
        }))
    }
}
