/*
 * Copyright (c) 2024-Present, Jeremy Plichta
 * Licensed under the MIT License
 */

//! Message types for inter-agent communication.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::agent::AgentId;

/// Unique identifier for a message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MessageId(Uuid);

impl MessageId {
    /// Create a new random message ID.
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for MessageId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for MessageId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Message priority levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    /// Low priority - processed when idle
    Low,
    /// Normal priority - standard processing
    #[default]
    Normal,
    /// High priority - processed before normal
    High,
    /// Urgent - interrupt current work
    Urgent,
}

/// Message types for agent communication.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MessageType {
    /// Task assignment from supervisor to worker
    TaskAssign { task_id: String },
    /// Task completion notification  
    TaskDone { task_id: String, result: String },
    /// Task failure notification
    TaskFailed { task_id: String, error: String },
    /// Status request
    StatusRequest,
    /// Status response
    StatusResponse {
        state: String,
        current_task: Option<String>,
    },
    /// Heartbeat ping
    Ping,
    /// Heartbeat pong
    Pong,
    /// Shutdown request
    Shutdown,
    /// Custom message with arbitrary payload
    Custom { kind: String, payload: String },
}

/// A message passed between agents via Redis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Unique message identifier
    pub id: MessageId,
    /// Sender agent ID  
    pub from: AgentId,
    /// Recipient agent ID
    pub to: AgentId,
    /// Message type and payload
    #[serde(flatten)]
    pub msg_type: MessageType,
    /// Priority level
    pub priority: Priority,
    /// Timestamp when created
    pub created_at: DateTime<Utc>,
    /// Optional correlation ID for request/response
    pub correlation_id: Option<MessageId>,
}

impl Message {
    /// Create a new message.
    #[must_use]
    pub fn new(from: AgentId, to: AgentId, msg_type: MessageType) -> Self {
        Self {
            id: MessageId::new(),
            from,
            to,
            msg_type,
            priority: Priority::Normal,
            created_at: Utc::now(),
            correlation_id: None,
        }
    }

    /// Set message priority.
    #[must_use]
    pub fn with_priority(mut self, priority: Priority) -> Self {
        self.priority = priority;
        self
    }

    /// Set correlation ID for request/response tracking.
    #[must_use]
    pub fn with_correlation(mut self, id: MessageId) -> Self {
        self.correlation_id = Some(id);
        self
    }
}
