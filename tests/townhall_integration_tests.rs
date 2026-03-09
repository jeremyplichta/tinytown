/*
 * Copyright (c) 2024-Present, Jeremy Plichta
 * Licensed under the MIT License
 */

//! Integration tests for the townhall daemon REST API (Issue #15).
//!
//! These tests verify the townhall REST API endpoints including:
//! - Health endpoints (/healthz, /v1/status)
//! - Agent management (/v1/agents)
//! - Task assignment and backlog (/v1/tasks, /v1/backlog)
//! - Messaging (/v1/messages)
//! - Recovery operations (/v1/recover, /v1/reclaim)
//!
//! Test infrastructure includes:
//! - `TownhallTestServer`: Wrapper for testing townhall with a real Redis backend
//! - `TestTownhall`: Test fixture providing full E2E testing capabilities
//! - Helper functions for common test scenarios

use tempfile::TempDir;
use tinytown::town::AgentHandle;
use tinytown::{Task, Town};

// ============================================================================
// TEST FIXTURES AND HELPERS
// ============================================================================

/// Test server wrapper that manages a townhall instance for testing.
/// Includes the underlying Town (with Redis) and provides HTTP client access.
pub struct TownhallTestServer {
    /// The underlying town with Redis connection
    pub town: Town,
    /// Temp directory for the town (cleaned up on drop)
    pub temp_dir: TempDir,
    /// Base URL for the townhall REST API (when server is running)
    pub base_url: Option<String>,
}

impl TownhallTestServer {
    /// Create a new test server with a fresh town and Redis instance.
    /// Uses Unix socket mode for test isolation.
    pub async fn new(name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Force Unix socket mode for test isolation
        unsafe {
            std::env::set_var("TT_USE_SOCKET", "1");
        }

        let temp_dir = TempDir::new()?;
        let town = Town::init(temp_dir.path(), name).await?;

        Ok(Self {
            town,
            temp_dir,
            base_url: None,
        })
    }

    /// Get the town's channel for direct Redis operations
    pub fn channel(&self) -> &tinytown::Channel {
        self.town.channel()
    }

    /// Get the town's config
    pub fn config(&self) -> &tinytown::Config {
        self.town.config()
    }

    /// Create a test agent in the town
    pub async fn spawn_test_agent(&self, name: &str) -> Result<AgentHandle, tinytown::Error> {
        self.town.spawn_agent(name, "test-cli").await
    }

    /// Add a task to the backlog
    pub async fn add_backlog_task(
        &self,
        description: &str,
    ) -> Result<tinytown::TaskId, tinytown::Error> {
        let task = Task::new(description);
        let task_id = task.id;
        self.channel().set_task(&task).await?;
        self.channel().backlog_push(task_id).await?;
        Ok(task_id)
    }
}

impl Drop for TownhallTestServer {
    fn drop(&mut self) {
        // Clean up Redis when test ends
        let pid_file = self.temp_dir.path().join("redis.pid");
        if let Ok(pid_str) = std::fs::read_to_string(&pid_file)
            && let Ok(pid) = pid_str.trim().parse::<i32>()
        {
            // SAFETY: This kills our test Redis process, which is safe to do.
            unsafe {
                libc::kill(pid, libc::SIGKILL);
            }
        }
    }
}

// ============================================================================
// EXPECTED API RESPONSE TYPES (for deserializing townhall responses)
// ============================================================================

/// Standard RFC 7807 error response format
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ApiError {
    pub r#type: String,
    pub title: String,
    pub status: u16,
    pub detail: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// Health check response
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct HealthResponse {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// Town status response
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct TownStatusResponse {
    pub name: String,
    pub agent_count: usize,
    pub backlog_count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redis_connected: Option<bool>,
}

/// Agent list response
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct AgentListResponse {
    pub agents: Vec<AgentInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// Agent info in list response
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct AgentInfo {
    pub id: String,
    pub name: String,
    pub state: String,
    pub cli: String,
}

/// Backlog task entry
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct BacklogEntry {
    pub id: String,
    pub description: String,
    #[serde(default)]
    pub tags: Vec<String>,
}

/// Backlog list response
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct BacklogListResponse {
    pub tasks: Vec<BacklogEntry>,
    pub total: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// Message send request
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct SendMessageRequest {
    pub to: String,
    pub message: String,
    #[serde(default)]
    pub kind: String, // "task" | "query" | "info" | "ack"
    #[serde(default)]
    pub urgent: bool,
}

/// Message send response
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct SendMessageResponse {
    pub message_id: String,
    pub delivered: bool,
}

// ============================================================================
// PLACEHOLDER TESTS - These will test townhall when it's implemented
// ============================================================================

/// Test that the test infrastructure itself works correctly.
#[tokio::test]
async fn test_townhall_test_server_creation() -> Result<(), Box<dyn std::error::Error>> {
    let server = TownhallTestServer::new("townhall-infra-test").await?;

    // Verify town was created
    assert_eq!(server.config().name, "townhall-infra-test");

    // Verify we can spawn agents through the test server
    let agent = server.spawn_test_agent("test-worker").await?;
    let state = agent.state().await?;
    assert!(state.is_some());

    Ok(())
}

/// Test that backlog operations work through the test server.
#[tokio::test]
async fn test_townhall_test_server_backlog() -> Result<(), Box<dyn std::error::Error>> {
    let server = TownhallTestServer::new("townhall-backlog-infra-test").await?;

    // Add tasks to backlog
    let task1_id = server.add_backlog_task("Task 1 for testing").await?;
    let task2_id = server.add_backlog_task("Task 2 for testing").await?;

    // Verify backlog has the tasks
    let backlog = server.channel().backlog_list().await?;
    assert_eq!(backlog.len(), 2);
    assert_eq!(backlog[0], task1_id);
    assert_eq!(backlog[1], task2_id);

    Ok(())
}

/// Test agent spawn and list through test infrastructure.
#[tokio::test]
async fn test_townhall_test_server_agents() -> Result<(), Box<dyn std::error::Error>> {
    let server = TownhallTestServer::new("townhall-agents-infra-test").await?;

    // Spawn multiple agents
    let _agent1 = server.spawn_test_agent("worker-1").await?;
    let _agent2 = server.spawn_test_agent("worker-2").await?;
    let _agent3 = server.spawn_test_agent("reviewer").await?;

    // List agents
    let agents = server.town.list_agents().await;
    assert_eq!(agents.len(), 3);

    // Verify agent names
    let names: Vec<&str> = agents.iter().map(|a| a.name.as_str()).collect();
    assert!(names.contains(&"worker-1"));
    assert!(names.contains(&"worker-2"));
    assert!(names.contains(&"reviewer"));

    Ok(())
}

// ============================================================================
// TOWNHALL REST API TESTS
// ============================================================================

// Import townhall router creation - note: this requires the bin to expose create_router
// For now, we test via the services layer which is what townhall uses

/// Test GET /healthz equivalent via service layer
#[tokio::test]
async fn test_services_status() -> Result<(), Box<dyn std::error::Error>> {
    let server = TownhallTestServer::new("townhall-status-test").await?;

    // Test AgentService::status (what /v1/status uses)
    let status = tinytown::AgentService::status(&server.town).await?;
    assert_eq!(status.name, "townhall-status-test");
    assert_eq!(status.agent_count, 0);

    Ok(())
}

/// Test agent spawn via service layer (what POST /v1/agents uses)
#[tokio::test]
async fn test_services_spawn_agent() -> Result<(), Box<dyn std::error::Error>> {
    let server = TownhallTestServer::new("townhall-spawn-test").await?;

    let result =
        tinytown::AgentService::spawn(&server.town, "test-worker", Some("test-cli")).await?;
    assert_eq!(result.name, "test-worker");
    assert_eq!(result.cli, "test-cli");

    // Verify agent exists
    let agents = tinytown::AgentService::list(&server.town).await?;
    assert_eq!(agents.len(), 1);
    assert_eq!(agents[0].name, "test-worker");

    Ok(())
}

/// Test backlog operations via service layer (what /v1/backlog uses)
#[tokio::test]
async fn test_services_backlog() -> Result<(), Box<dyn std::error::Error>> {
    let server = TownhallTestServer::new("townhall-backlog-test").await?;

    // Add to backlog
    let result = tinytown::BacklogService::add(
        server.channel(),
        "Test task",
        Some(vec!["test".to_string()]),
    )
    .await?;
    assert_eq!(result.description, "Test task");

    // List backlog
    let items = tinytown::BacklogService::list(server.channel()).await?;
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].description, "Test task");
    assert_eq!(items[0].tags, vec!["test"]);

    Ok(())
}

/// Test task assignment via service layer (what POST /v1/tasks/assign uses)
#[tokio::test]
async fn test_services_assign_task() -> Result<(), Box<dyn std::error::Error>> {
    let server = TownhallTestServer::new("townhall-assign-test").await?;

    // First spawn an agent
    let _agent = server.spawn_test_agent("worker").await?;

    // Assign a task
    let result = tinytown::TaskService::assign(&server.town, "worker", "Do something").await?;
    assert_eq!(result.agent_name, "worker");

    // Verify task is pending
    let pending = tinytown::TaskService::list_pending(&server.town).await?;
    assert_eq!(pending.len(), 1);
    assert_eq!(pending[0].description, "Do something");

    Ok(())
}

/// Test message sending via service layer (what POST /v1/messages/send uses)
#[tokio::test]
async fn test_services_send_message() -> Result<(), Box<dyn std::error::Error>> {
    let server = TownhallTestServer::new("townhall-message-test").await?;

    // Spawn an agent
    let _agent = server.spawn_test_agent("receiver").await?;

    // Send a message
    let result = tinytown::MessageService::send(
        &server.town,
        "receiver",
        "Hello!",
        tinytown::app::services::messages::MessageKind::Task,
        false,
    )
    .await?;
    assert!(!result.urgent);

    // Check inbox
    let inbox = tinytown::MessageService::get_inbox(&server.town, "receiver").await?;
    assert_eq!(inbox.total_messages, 1);

    Ok(())
}
