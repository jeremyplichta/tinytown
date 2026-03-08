/*
 * Copyright (c) 2024-Present, Jeremy Plichta
 * Licensed under the MIT License
 */

//! Redis-based message passing channels.
//!
//! Uses Redis Lists for reliable message queues and Pub/Sub for broadcasts.

use std::time::Duration;

use redis::aio::ConnectionManager;
use redis::AsyncCommands;
use tracing::{debug, instrument};

use crate::agent::AgentId;
use crate::error::Result;
use crate::message::{Message, Priority};

/// Key prefix for agent inboxes
const INBOX_PREFIX: &str = "mt:inbox:";

/// Key prefix for agent state
const STATE_PREFIX: &str = "mt:agent:";

/// Key prefix for tasks
const TASK_PREFIX: &str = "mt:task:";

/// Pub/sub channel for broadcasts
const BROADCAST_CHANNEL: &str = "mt:broadcast";

/// Redis-based communication channel.
#[derive(Clone)]
pub struct Channel {
    conn: ConnectionManager,
}

impl Channel {
    /// Create a new channel from a Redis connection.
    pub fn new(conn: ConnectionManager) -> Self {
        Self { conn }
    }

    /// Send a message to an agent's inbox.
    #[instrument(skip(self, message), fields(to = %message.to, msg_type = ?message.msg_type))]
    pub async fn send(&self, message: &Message) -> Result<()> {
        let mut conn = self.conn.clone();
        let inbox_key = format!("{}{}", INBOX_PREFIX, message.to);
        let serialized = serde_json::to_string(message)?;
        
        // Use priority queues: high priority goes to front
        match message.priority {
            Priority::Urgent | Priority::High => {
                let _: () = conn.lpush(&inbox_key, &serialized).await?;
            }
            Priority::Normal | Priority::Low => {
                let _: () = conn.rpush(&inbox_key, &serialized).await?;
            }
        }
        
        debug!("Sent message {} to {}", message.id, message.to);
        Ok(())
    }

    /// Receive a message from an agent's inbox (blocking with timeout).
    #[instrument(skip(self))]
    pub async fn receive(&self, agent_id: AgentId, timeout: Duration) -> Result<Option<Message>> {
        let mut conn = self.conn.clone();
        let inbox_key = format!("{}{}", INBOX_PREFIX, agent_id);
        
        let result: Option<String> = conn
            .blpop(&inbox_key, timeout.as_secs_f64())
            .await?;
        
        match result {
            Some(data) => {
                let message: Message = serde_json::from_str(&data)?;
                debug!("Received message {} from inbox", message.id);
                Ok(Some(message))
            }
            None => Ok(None),
        }
    }

    /// Receive a message without blocking.
    pub async fn try_receive(&self, agent_id: AgentId) -> Result<Option<Message>> {
        let mut conn = self.conn.clone();
        let inbox_key = format!("{}{}", INBOX_PREFIX, agent_id);
        
        let result: Option<String> = conn.lpop(&inbox_key, None).await?;
        
        match result {
            Some(data) => {
                let message: Message = serde_json::from_str(&data)?;
                Ok(Some(message))
            }
            None => Ok(None),
        }
    }

    /// Get the number of messages in an agent's inbox.
    pub async fn inbox_len(&self, agent_id: AgentId) -> Result<usize> {
        let mut conn = self.conn.clone();
        let inbox_key = format!("{}{}", INBOX_PREFIX, agent_id);
        let len: usize = conn.llen(&inbox_key).await?;
        Ok(len)
    }

    /// Broadcast a message to all agents.
    pub async fn broadcast(&self, message: &Message) -> Result<()> {
        let mut conn = self.conn.clone();
        let serialized = serde_json::to_string(message)?;
        let _: () = conn.publish(BROADCAST_CHANNEL, &serialized).await?;
        Ok(())
    }

    /// Store agent state in Redis.
    pub async fn set_agent_state(&self, agent: &crate::agent::Agent) -> Result<()> {
        let mut conn = self.conn.clone();
        let key = format!("{}{}", STATE_PREFIX, agent.id);
        let serialized = serde_json::to_string(agent)?;
        let _: () = conn.set(&key, &serialized).await?;
        Ok(())
    }

    /// Get agent state from Redis.
    pub async fn get_agent_state(&self, agent_id: AgentId) -> Result<Option<crate::agent::Agent>> {
        let mut conn = self.conn.clone();
        let key = format!("{}{}", STATE_PREFIX, agent_id);
        let result: Option<String> = conn.get(&key).await?;
        
        match result {
            Some(data) => Ok(Some(serde_json::from_str(&data)?)),
            None => Ok(None),
        }
    }

    /// Store a task in Redis.
    pub async fn set_task(&self, task: &crate::task::Task) -> Result<()> {
        let mut conn = self.conn.clone();
        let key = format!("{}{}", TASK_PREFIX, task.id);
        let serialized = serde_json::to_string(task)?;
        let _: () = conn.set(&key, &serialized).await?;
        Ok(())
    }

    /// Get a task from Redis.
    pub async fn get_task(&self, task_id: crate::task::TaskId) -> Result<Option<crate::task::Task>> {
        let mut conn = self.conn.clone();
        let key = format!("{}{}", TASK_PREFIX, task_id);
        let result: Option<String> = conn.get(&key).await?;
        
        match result {
            Some(data) => Ok(Some(serde_json::from_str(&data)?)),
            None => Ok(None),
        }
    }
}

