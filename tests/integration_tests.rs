/*
 * Copyright (c) 2024-Present, Jeremy Plichta
 * Licensed under the MIT License
 */

//! Integration tests for the tinytown orchestration system.
//!
//! These tests verify the core functionality of tinytown including:
//! - Town initialization and configuration
//! - Agent creation and state management
//! - Message passing through Redis channels
//! - Task assignment and lifecycle management

use std::time::Duration;
use tempfile::TempDir;
use tinytown::message::MessageType;
use tinytown::{
    Agent, AgentId, AgentState, AgentType, Message, Priority, Task, TaskId, TaskState, Town,
};

/// Helper function to create a temporary town for testing.
async fn create_test_town(name: &str) -> Result<(Town, TempDir), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let town = Town::init(temp_dir.path(), name).await?;
    Ok((town, temp_dir))
}

// ============================================================================
// TOWN INITIALIZATION AND CONFIGURATION TESTS
// ============================================================================

/// Test that a town can be initialized with proper directory structure.
#[tokio::test]
async fn test_town_initialization() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let town_path = temp_dir.path();
    let town = Town::init(town_path, "test-town").await?;

    assert!(town_path.join("agents").exists());
    assert!(town_path.join("logs").exists());
    assert!(town_path.join("tasks").exists());
    assert!(town_path.join("tinytown.json").exists());

    let config = town.config();
    assert_eq!(config.name, "test-town");
    assert_eq!(config.root, town_path);

    Ok(())
}

/// Test that a town can be connected to after initialization.
#[tokio::test]
async fn test_town_connect() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let town_path = temp_dir.path();

    let _town1 = Town::init(town_path, "connect-test").await?;
    let town2 = Town::connect(town_path).await?;

    let config = town2.config();
    assert_eq!(config.name, "connect-test");

    Ok(())
}

// ============================================================================
// AGENT CREATION AND STATE MANAGEMENT TESTS
// ============================================================================

/// Test that an agent can be spawned and has correct initial state.
#[tokio::test]
async fn test_agent_spawn() -> Result<(), Box<dyn std::error::Error>> {
    let (town, _temp_dir) = create_test_town("agent-spawn-test").await?;

    let agent_handle = town.spawn_agent("worker-1", "claude").await?;
    let agent_id = agent_handle.id();
    assert_ne!(agent_id, AgentId::supervisor());

    let agent_state = agent_handle.state().await?;
    assert!(agent_state.is_some());

    let agent = agent_state.unwrap();
    assert_eq!(agent.name, "worker-1");
    assert_eq!(agent.model, "claude");
    assert_eq!(agent.agent_type, AgentType::Worker);
    assert_eq!(agent.state, AgentState::Starting);
    assert_eq!(agent.tasks_completed, 0);

    Ok(())
}

/// Test that multiple agents can be spawned independently.
#[tokio::test]
async fn test_multiple_agents_spawn() -> Result<(), Box<dyn std::error::Error>> {
    let (town, _temp_dir) = create_test_town("multi-agent-test").await?;

    let agent1 = town.spawn_agent("worker-1", "claude").await?;
    let agent2 = town.spawn_agent("worker-2", "gemini").await?;
    let agent3 = town.spawn_agent("worker-3", "claude").await?;

    assert_ne!(agent1.id(), agent2.id());
    assert_ne!(agent2.id(), agent3.id());
    assert_ne!(agent1.id(), agent3.id());

    let state1 = agent1.state().await?;
    let state2 = agent2.state().await?;
    let state3 = agent3.state().await?;

    assert!(state1.is_some());
    assert!(state2.is_some());
    assert!(state3.is_some());

    Ok(())
}

/// Test that agent state can be updated and persisted.
#[tokio::test]
async fn test_agent_state_update() -> Result<(), Box<dyn std::error::Error>> {
    let (town, _temp_dir) = create_test_town("agent-state-test").await?;

    let _agent_handle = town.spawn_agent("worker-1", "claude").await?;

    let mut agent = Agent::new("worker-1", "claude", AgentType::Worker);
    agent.state = AgentState::Idle;
    agent.tasks_completed = 5;

    town.channel().set_agent_state(&agent).await?;

    let retrieved = town.channel().get_agent_state(agent.id).await?;
    assert!(retrieved.is_some());

    let retrieved_agent = retrieved.unwrap();
    assert_eq!(retrieved_agent.state, AgentState::Idle);
    assert_eq!(retrieved_agent.tasks_completed, 5);

    Ok(())
}

// ============================================================================
// MESSAGE PASSING THROUGH REDIS CHANNELS TESTS
// ============================================================================

/// Test that a message can be sent to an agent's inbox.
#[tokio::test]
async fn test_message_send() -> Result<(), Box<dyn std::error::Error>> {
    let (town, _temp_dir) = create_test_town("message-send-test").await?;

    let agent = town.spawn_agent("worker-1", "claude").await?;
    let agent_id = agent.id();

    let msg = Message::new(AgentId::supervisor(), agent_id, MessageType::Ping);

    town.channel().send(&msg).await?;

    let inbox_len = agent.inbox_len().await?;
    assert_eq!(inbox_len, 1);

    Ok(())
}

/// Test that messages can be received from an agent's inbox.
#[tokio::test]
async fn test_message_receive() -> Result<(), Box<dyn std::error::Error>> {
    let (town, _temp_dir) = create_test_town("message-receive-test").await?;

    let agent = town.spawn_agent("worker-1", "claude").await?;
    let agent_id = agent.id();

    let original_msg = Message::new(AgentId::supervisor(), agent_id, MessageType::Ping);
    town.channel().send(&original_msg).await?;

    // Use try_receive instead of blocking receive
    let received = town.channel().try_receive(agent_id).await?;

    assert!(received.is_some());
    let msg = received.unwrap();
    assert_eq!(msg.id, original_msg.id);
    assert_eq!(msg.from, AgentId::supervisor());
    assert_eq!(msg.to, agent_id);

    Ok(())
}

/// Test that message priority affects queue ordering.
#[tokio::test]
async fn test_message_priority() -> Result<(), Box<dyn std::error::Error>> {
    let (town, _temp_dir) = create_test_town("message-priority-test").await?;

    let agent = town.spawn_agent("worker-1", "claude").await?;
    let agent_id = agent.id();

    let low_msg = Message::new(AgentId::supervisor(), agent_id, MessageType::Ping)
        .with_priority(Priority::Low);

    let high_msg = Message::new(AgentId::supervisor(), agent_id, MessageType::Pong)
        .with_priority(Priority::High);

    town.channel().send(&low_msg).await?;
    town.channel().send(&high_msg).await?;

    // High priority messages are pushed to front (lpush), so try_receive gets them first
    let first = town.channel().try_receive(agent_id).await?.unwrap();

    assert_eq!(first.id, high_msg.id);

    Ok(())
}

/// Test that non-blocking message receive works correctly.
#[tokio::test]
async fn test_message_try_receive() -> Result<(), Box<dyn std::error::Error>> {
    let (town, _temp_dir) = create_test_town("message-try-receive-test").await?;

    let agent = town.spawn_agent("worker-1", "claude").await?;
    let agent_id = agent.id();

    let empty = town.channel().try_receive(agent_id).await?;
    assert!(empty.is_none());

    let msg = Message::new(AgentId::supervisor(), agent_id, MessageType::Ping);
    town.channel().send(&msg).await?;

    let received = town.channel().try_receive(agent_id).await?;
    assert!(received.is_some());
    assert_eq!(received.unwrap().id, msg.id);

    Ok(())
}

/// Test that message correlation IDs work for request/response patterns.
#[tokio::test]
async fn test_message_correlation() -> Result<(), Box<dyn std::error::Error>> {
    let (town, _temp_dir) = create_test_town("message-correlation-test").await?;

    let agent = town.spawn_agent("worker-1", "claude").await?;
    let agent_id = agent.id();

    let request = Message::new(AgentId::supervisor(), agent_id, MessageType::StatusRequest);
    let request_id = request.id;

    let response = Message::new(
        agent_id,
        AgentId::supervisor(),
        MessageType::StatusResponse {
            state: "idle".to_string(),
            current_task: None,
        },
    )
    .with_correlation(request_id);

    assert_eq!(response.correlation_id, Some(request_id));

    Ok(())
}

// ============================================================================
// TASK ASSIGNMENT AND LIFECYCLE TESTS
// ============================================================================

/// Test that a task can be created with proper initial state.
#[tokio::test]
async fn test_task_creation() -> Result<(), Box<dyn std::error::Error>> {
    let task = Task::new("Fix the bug in auth.rs");

    assert_eq!(task.description, "Fix the bug in auth.rs");
    assert_eq!(task.state, TaskState::Pending);
    assert!(task.assigned_to.is_none());
    assert!(task.result.is_none());
    assert!(task.completed_at.is_none());

    Ok(())
}

/// Test that a task can be assigned to an agent.
#[tokio::test]
async fn test_task_assignment() -> Result<(), Box<dyn std::error::Error>> {
    let (town, _temp_dir) = create_test_town("task-assignment-test").await?;

    let agent = town.spawn_agent("worker-1", "claude").await?;
    let mut task = Task::new("Implement feature X");
    task.assign(agent.id());

    let task_id = agent.assign(task).await?;

    let stored_task = town.channel().get_task(task_id).await?;
    assert!(stored_task.is_some());

    let stored = stored_task.unwrap();
    assert_eq!(stored.description, "Implement feature X");
    assert_eq!(stored.state, TaskState::Assigned);
    assert_eq!(stored.assigned_to, Some(agent.id()));

    Ok(())
}

/// Test that multiple tasks can be assigned to an agent.
#[tokio::test]
async fn test_multiple_task_assignment() -> Result<(), Box<dyn std::error::Error>> {
    let (town, _temp_dir) = create_test_town("multi-task-test").await?;

    let agent = town.spawn_agent("worker-1", "claude").await?;

    let task1 = Task::new("Task 1");
    let task2 = Task::new("Task 2");
    let task3 = Task::new("Task 3");

    let id1 = agent.assign(task1).await?;
    let id2 = agent.assign(task2).await?;
    let id3 = agent.assign(task3).await?;

    let stored1 = town.channel().get_task(id1).await?;
    let stored2 = town.channel().get_task(id2).await?;
    let stored3 = town.channel().get_task(id3).await?;

    assert!(stored1.is_some());
    assert!(stored2.is_some());
    assert!(stored3.is_some());

    assert_eq!(stored1.unwrap().description, "Task 1");
    assert_eq!(stored2.unwrap().description, "Task 2");
    assert_eq!(stored3.unwrap().description, "Task 3");

    Ok(())
}

/// Test that task state transitions work correctly.
#[tokio::test]
async fn test_task_state_transitions() -> Result<(), Box<dyn std::error::Error>> {
    let (_town, _temp_dir) = create_test_town("task-state-test").await?;

    let mut task = Task::new("Test task");

    assert_eq!(task.state, TaskState::Pending);

    let agent_id = AgentId::new();
    task.assign(agent_id);
    assert_eq!(task.state, TaskState::Assigned);
    assert_eq!(task.assigned_to, Some(agent_id));

    task.start();
    assert_eq!(task.state, TaskState::Running);

    task.complete("Task completed successfully");
    assert_eq!(task.state, TaskState::Completed);
    assert_eq!(task.result, Some("Task completed successfully".to_string()));
    assert!(task.completed_at.is_some());

    Ok(())
}

/// Test that task failure state works correctly.
#[tokio::test]
async fn test_task_failure() -> Result<(), Box<dyn std::error::Error>> {
    let mut task = Task::new("Failing task");

    task.assign(AgentId::new());
    task.start();
    task.fail("Connection timeout");

    assert_eq!(task.state, TaskState::Failed);
    assert_eq!(task.result, Some("Connection timeout".to_string()));
    assert!(task.completed_at.is_some());

    Ok(())
}

/// Test that tasks can have tags for filtering.
#[tokio::test]
async fn test_task_tags() -> Result<(), Box<dyn std::error::Error>> {
    let task = Task::new("Implement API endpoint").with_tags(vec!["backend", "api", "urgent"]);

    assert_eq!(task.tags.len(), 3);
    assert!(task.tags.contains(&"backend".to_string()));
    assert!(task.tags.contains(&"api".to_string()));
    assert!(task.tags.contains(&"urgent".to_string()));

    Ok(())
}

/// Test that tasks can have parent tasks for hierarchical organization.
#[tokio::test]
async fn test_task_hierarchy() -> Result<(), Box<dyn std::error::Error>> {
    let parent_id = TaskId::new();
    let child_task = Task::new("Subtask").with_parent(parent_id);

    assert_eq!(child_task.parent_id, Some(parent_id));

    Ok(())
}

/// Test that task state is persisted in Redis.
#[tokio::test]
async fn test_task_persistence() -> Result<(), Box<dyn std::error::Error>> {
    let (town, _temp_dir) = create_test_town("task-persistence-test").await?;

    let mut task = Task::new("Persistent task");
    let agent_id = AgentId::new();
    task.assign(agent_id);

    town.channel().set_task(&task).await?;

    let retrieved = town.channel().get_task(task.id).await?;
    assert!(retrieved.is_some());

    let retrieved_task = retrieved.unwrap();
    assert_eq!(retrieved_task.description, "Persistent task");
    assert_eq!(retrieved_task.state, TaskState::Assigned);
    assert_eq!(retrieved_task.assigned_to, Some(agent_id));

    Ok(())
}

// ============================================================================
// INTEGRATION TESTS - COMBINED WORKFLOWS
// ============================================================================

/// Test a complete workflow: spawn agent, assign task, send messages.
#[tokio::test]
async fn test_complete_workflow() -> Result<(), Box<dyn std::error::Error>> {
    let (town, _temp_dir) = create_test_town("complete-workflow-test").await?;

    let agent = town.spawn_agent("worker-1", "claude").await?;

    let mut task = Task::new("Implement new feature");
    task.assign(agent.id());
    let task_id = agent.assign(task).await?;

    let stored_task = town.channel().get_task(task_id).await?;
    assert!(stored_task.is_some());
    assert_eq!(stored_task.unwrap().state, TaskState::Assigned);

    agent.send(MessageType::StatusRequest).await?;

    // assign() sends a TaskAssign message, and send() sends a StatusRequest message
    let inbox_len = agent.inbox_len().await?;
    assert_eq!(inbox_len, 2);

    Ok(())
}

/// Test agent state transitions through message handling.
#[tokio::test]
async fn test_agent_state_transitions() -> Result<(), Box<dyn std::error::Error>> {
    let (town, _temp_dir) = create_test_town("agent-transitions-test").await?;

    let agent_handle = town.spawn_agent("worker-1", "claude").await?;

    let initial = agent_handle.state().await?;
    assert_eq!(initial.unwrap().state, AgentState::Starting);

    let mut agent = Agent::new("worker-1", "claude", AgentType::Worker);
    agent.id = agent_handle.id();
    agent.state = AgentState::Idle;
    town.channel().set_agent_state(&agent).await?;

    let idle = agent_handle.state().await?;
    assert_eq!(idle.unwrap().state, AgentState::Idle);

    agent.state = AgentState::Working;
    town.channel().set_agent_state(&agent).await?;

    let working = agent_handle.state().await?;
    assert_eq!(working.unwrap().state, AgentState::Working);

    Ok(())
}

/// Test task lifecycle with agent interaction.
#[tokio::test]
async fn test_task_lifecycle_with_agent() -> Result<(), Box<dyn std::error::Error>> {
    let (town, _temp_dir) = create_test_town("task-lifecycle-test").await?;

    let agent = town.spawn_agent("worker-1", "claude").await?;

    let mut task = Task::new("Complete task");
    assert_eq!(task.state, TaskState::Pending);

    task.assign(agent.id());
    assert_eq!(task.state, TaskState::Assigned);

    town.channel().set_task(&task).await?;

    task.start();
    town.channel().set_task(&task).await?;

    let running = town.channel().get_task(task.id).await?;
    assert_eq!(running.unwrap().state, TaskState::Running);

    task.complete("Successfully completed");
    town.channel().set_task(&task).await?;

    let completed = town.channel().get_task(task.id).await?;
    let completed_task = completed.unwrap();
    assert_eq!(completed_task.state, TaskState::Completed);
    assert_eq!(
        completed_task.result,
        Some("Successfully completed".to_string())
    );

    Ok(())
}

/// Test message inbox behavior with multiple messages.
#[tokio::test]
async fn test_message_inbox_ordering() -> Result<(), Box<dyn std::error::Error>> {
    let (town, _temp_dir) = create_test_town("inbox-ordering-test").await?;

    let agent = town.spawn_agent("worker-1", "claude").await?;
    let agent_id = agent.id();

    for i in 0..3 {
        let msg = Message::new(
            AgentId::supervisor(),
            agent_id,
            MessageType::Custom {
                kind: "test".to_string(),
                payload: format!("message-{}", i),
            },
        );
        town.channel().send(&msg).await?;
    }

    let inbox_len = agent.inbox_len().await?;
    assert_eq!(inbox_len, 3);

    let _msg1 = town.channel().try_receive(agent_id).await?.unwrap();
    let _msg2 = town.channel().try_receive(agent_id).await?.unwrap();
    let _msg3 = town.channel().try_receive(agent_id).await?.unwrap();

    let final_len = agent.inbox_len().await?;
    assert_eq!(final_len, 0);

    Ok(())
}

/// Test that agent wait functionality works (with timeout).
#[tokio::test]
async fn test_agent_wait_timeout() -> Result<(), Box<dyn std::error::Error>> {
    let (town, _temp_dir) = create_test_town("agent-wait-test").await?;

    let agent = town.spawn_agent("worker-1", "claude").await?;

    let mut agent_state = Agent::new("worker-1", "claude", AgentType::Worker);
    agent_state.id = agent.id();
    agent_state.state = AgentState::Idle;
    town.channel().set_agent_state(&agent_state).await?;

    let start = std::time::Instant::now();
    agent.wait().await?;
    let elapsed = start.elapsed();

    assert!(elapsed < Duration::from_secs(1));

    Ok(())
}

// ============================================================================
// ERROR HANDLING TESTS
// ============================================================================

/// Test that invalid task retrieval returns None.
#[tokio::test]
async fn test_task_not_found() -> Result<(), Box<dyn std::error::Error>> {
    let (town, _temp_dir) = create_test_town("task-not-found-test").await?;

    let fake_id = TaskId::new();
    let result = town.channel().get_task(fake_id).await?;

    assert!(result.is_none());

    Ok(())
}

/// Test that invalid agent retrieval returns None.
#[tokio::test]
async fn test_agent_state_not_found() -> Result<(), Box<dyn std::error::Error>> {
    let (town, _temp_dir) = create_test_town("agent-state-not-found-test").await?;

    let fake_id = AgentId::new();
    let result = town.channel().get_agent_state(fake_id).await?;

    assert!(result.is_none());

    Ok(())
}

/// Test that message receive timeout works correctly.
#[tokio::test]
async fn test_message_receive_timeout() -> Result<(), Box<dyn std::error::Error>> {
    let (town, _temp_dir) = create_test_town("message-timeout-test").await?;

    let agent = town.spawn_agent("worker-1", "claude").await?;

    let start = std::time::Instant::now();
    let result = town
        .channel()
        .receive(agent.id(), Duration::from_millis(100))
        .await?;
    let elapsed = start.elapsed();

    assert!(result.is_none());
    assert!(elapsed >= Duration::from_millis(100));

    Ok(())
}

// ============================================================================
// EDGE CASES AND STRESS TESTS
// ============================================================================

/// Test that task state is terminal when completed.
#[tokio::test]
async fn test_task_terminal_states() -> Result<(), Box<dyn std::error::Error>> {
    let mut task1 = Task::new("Task 1");
    task1.complete("Done");
    assert!(task1.state.is_terminal());

    let mut task2 = Task::new("Task 2");
    task2.fail("Error");
    assert!(task2.state.is_terminal());

    let task3 = Task::new("Task 3");
    assert!(!task3.state.is_terminal());

    Ok(())
}

/// Test that agent state can be checked for work acceptance.
#[tokio::test]
async fn test_agent_can_accept_work() -> Result<(), Box<dyn std::error::Error>> {
    assert!(AgentState::Idle.can_accept_work());
    assert!(!AgentState::Working.can_accept_work());
    assert!(!AgentState::Paused.can_accept_work());
    assert!(!AgentState::Starting.can_accept_work());

    Ok(())
}

/// Test that agent state is terminal when stopped or errored.
#[tokio::test]
async fn test_agent_terminal_states() -> Result<(), Box<dyn std::error::Error>> {
    assert!(AgentState::Stopped.is_terminal());
    assert!(AgentState::Error.is_terminal());
    assert!(!AgentState::Idle.is_terminal());
    assert!(!AgentState::Working.is_terminal());

    Ok(())
}

/// Test creating many agents in sequence.
#[tokio::test]
async fn test_many_agents() -> Result<(), Box<dyn std::error::Error>> {
    let (town, _temp_dir) = create_test_town("many-agents-test").await?;

    let mut agent_ids = Vec::new();

    for i in 0..10 {
        let agent = town.spawn_agent(&format!("worker-{}", i), "claude").await?;
        agent_ids.push(agent.id());
    }

    let unique_count = agent_ids
        .iter()
        .collect::<std::collections::HashSet<_>>()
        .len();
    assert_eq!(unique_count, 10);

    Ok(())
}

/// Test creating many tasks in sequence.
#[tokio::test]
async fn test_many_tasks() -> Result<(), Box<dyn std::error::Error>> {
    let (town, _temp_dir) = create_test_town("many-tasks-test").await?;

    let agent = town.spawn_agent("worker-1", "claude").await?;

    let mut task_ids = Vec::new();

    for i in 0..20 {
        let task = Task::new(format!("Task {}", i));
        let task_id = agent.assign(task).await?;
        task_ids.push(task_id);
    }

    let unique_count = task_ids
        .iter()
        .collect::<std::collections::HashSet<_>>()
        .len();
    assert_eq!(unique_count, 20);

    Ok(())
}

/// Test sending many messages in sequence.
#[tokio::test]
async fn test_many_messages() -> Result<(), Box<dyn std::error::Error>> {
    let (town, _temp_dir) = create_test_town("many-messages-test").await?;

    let agent = town.spawn_agent("worker-1", "claude").await?;
    let agent_id = agent.id();

    for i in 0..50 {
        let msg = Message::new(
            AgentId::supervisor(),
            agent_id,
            MessageType::Custom {
                kind: "test".to_string(),
                payload: format!("msg-{}", i),
            },
        );
        town.channel().send(&msg).await?;
    }

    let inbox_len = agent.inbox_len().await?;
    assert_eq!(inbox_len, 50);

    Ok(())
}

// ============================================================================
// TASK PLANNING DSL TESTS
// ============================================================================

/// Test that tasks.toml can be initialized.
#[tokio::test]
async fn test_plan_init_tasks_file() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;

    // Initialize tasks file
    tinytown::plan::init_tasks_file(temp_dir.path())?;

    // Check file exists
    let tasks_file = temp_dir.path().join("tasks.toml");
    assert!(tasks_file.exists());

    // Load and verify structure
    let tasks = tinytown::plan::load_tasks_file(temp_dir.path())?;
    assert_eq!(tasks.meta.description, "Task plan for this project");
    assert_eq!(tasks.tasks.len(), 1);
    assert_eq!(tasks.tasks[0].id, "example-1");
    assert_eq!(tasks.tasks[0].status, "pending");

    Ok(())
}

/// Test loading and saving tasks file.
#[tokio::test]
async fn test_plan_load_save_tasks_file() -> Result<(), Box<dyn std::error::Error>> {
    use tinytown::plan::{TaskEntry, TasksFile, TasksMeta};

    let temp_dir = TempDir::new()?;

    // Create a custom tasks file
    let tasks = TasksFile {
        meta: TasksMeta {
            description: "Test plan".to_string(),
            default_agent: Some("developer".to_string()),
        },
        tasks: vec![
            TaskEntry {
                id: "task-1".to_string(),
                description: "Build the API".to_string(),
                agent: Some("backend".to_string()),
                status: "pending".to_string(),
                tags: vec!["api".to_string(), "backend".to_string()],
                parent: None,
            },
            TaskEntry {
                id: "task-2".to_string(),
                description: "Write tests".to_string(),
                agent: Some("tester".to_string()),
                status: "pending".to_string(),
                tags: vec!["tests".to_string()],
                parent: Some("task-1".to_string()),
            },
        ],
    };

    // Save
    tinytown::plan::save_tasks_file(temp_dir.path(), &tasks)?;

    // Load back
    let loaded = tinytown::plan::load_tasks_file(temp_dir.path())?;

    assert_eq!(loaded.meta.description, "Test plan");
    assert_eq!(loaded.meta.default_agent, Some("developer".to_string()));
    assert_eq!(loaded.tasks.len(), 2);
    assert_eq!(loaded.tasks[0].id, "task-1");
    assert_eq!(loaded.tasks[0].agent, Some("backend".to_string()));
    assert_eq!(loaded.tasks[1].parent, Some("task-1".to_string()));

    Ok(())
}

/// Test pushing tasks from file to Redis.
#[tokio::test]
async fn test_plan_push_to_redis() -> Result<(), Box<dyn std::error::Error>> {
    let (town, temp_dir) = create_test_town("plan-push-test").await?;

    // Initialize and modify tasks file
    tinytown::plan::init_tasks_file(temp_dir.path())?;

    let tasks = tinytown::plan::TasksFile {
        meta: tinytown::plan::TasksMeta {
            description: "Push test".to_string(),
            default_agent: None,
        },
        tasks: vec![tinytown::plan::TaskEntry {
            id: "push-task-1".to_string(),
            description: "Task to push".to_string(),
            agent: None,
            status: "pending".to_string(),
            tags: vec!["test".to_string()],
            parent: None,
        }],
    };
    tinytown::plan::save_tasks_file(temp_dir.path(), &tasks)?;

    // Push to Redis
    let count = tinytown::plan::push_tasks_to_redis(temp_dir.path(), town.channel()).await?;
    assert_eq!(count, 1);

    Ok(())
}

/// Test that default_model is used when spawning without --model.
#[tokio::test]
async fn test_default_model_config() -> Result<(), Box<dyn std::error::Error>> {
    let (town, _temp_dir) = create_test_town("default-model-test").await?;

    // Config should have a default_model
    let config = town.config();
    assert!(!config.default_model.is_empty());
    assert_eq!(config.default_model, "claude"); // Default is claude

    // Models should include built-in presets
    assert!(config.models.contains_key("claude"));
    assert!(config.models.contains_key("auggie"));
    assert!(config.models.contains_key("codex"));

    Ok(())
}
