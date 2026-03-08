# Tutorial: Error Handling & Recovery

Things go wrong. Agents crash, tasks fail, Redis restarts. Here's how to handle it.

## Agent State Checks

Always check agent state before assuming success:

```rust
use tinytown::{AgentState, Result};

async fn check_agent_health(handle: &AgentHandle) -> Result<bool> {
    if let Some(agent) = handle.state().await? {
        match agent.state {
            AgentState::Idle => Ok(true),        // Ready for work
            AgentState::Working => Ok(true),     // Busy but healthy
            AgentState::Error => Ok(false),      // Something went wrong
            AgentState::Stopped => Ok(false),    // Agent terminated
            _ => Ok(true),
        }
    } else {
        Ok(false)  // Agent not found
    }
}
```

## Task State Validation

Check if tasks completed successfully:

```rust
use tinytown::{TaskState, Result};

async fn get_task_result(channel: &Channel, task_id: TaskId) -> Result<String> {
    let task = channel.get_task(task_id).await?
        .ok_or_else(|| Error::TaskNotFound(task_id.to_string()))?;
    
    match task.state {
        TaskState::Completed => {
            Ok(task.result.unwrap_or_else(|| "No result".to_string()))
        }
        TaskState::Failed => {
            Err(Error::AssignmentFailed(
                task.result.unwrap_or_else(|| "Unknown error".to_string())
            ))
        }
        TaskState::Cancelled => {
            Err(Error::AssignmentFailed("Task was cancelled".to_string()))
        }
        _ => {
            Err(Error::AssignmentFailed("Task not complete".to_string()))
        }
    }
}
```

## Timeout Handling

Don't wait forever:

```rust
use std::time::Duration;
use tokio::time::timeout;

async fn wait_with_timeout(
    handle: &AgentHandle,
    max_wait: Duration,
) -> Result<()> {
    match timeout(max_wait, handle.wait()).await {
        Ok(result) => result,
        Err(_) => Err(Error::Timeout(format!(
            "Agent {} did not complete in {:?}",
            handle.id(),
            max_wait
        ))),
    }
}

// Usage
let result = wait_with_timeout(&agent, Duration::from_secs(300)).await;
match result {
    Ok(_) => println!("✅ Completed"),
    Err(Error::Timeout(msg)) => {
        println!("⏰ Timeout: {}", msg);
        // Consider: cancel task, reassign, or alert
    }
    Err(e) => println!("❌ Error: {}", e),
}
```

## Respawning Failed Agents

If an agent dies, spawn a new one:

```rust
async fn ensure_agent(
    town: &Town,
    name: &str,
    model: &str,
) -> Result<AgentHandle> {
    match town.agent(name).await {
        Ok(handle) => {
            // Check if healthy
            if let Some(agent) = handle.state().await? {
                if !agent.state.is_terminal() {
                    return Ok(handle);
                }
            }
            // Respawn if unhealthy
            println!("🔄 Respawning {} (was unhealthy)", name);
            town.spawn_agent(name, model).await
        }
        Err(_) => {
            // Doesn't exist, create new
            println!("🆕 Creating new agent: {}", name);
            town.spawn_agent(name, model).await
        }
    }
}
```

## Graceful Shutdown

Handle Ctrl+C and other signals:

```rust
use tokio::signal;

async fn run_with_graceful_shutdown(town: Town) -> Result<()> {
    let shutdown = async {
        signal::ctrl_c().await.expect("Failed to listen for ctrl-c");
        println!("\n⚡ Shutdown signal received");
    };
    
    let work = async {
        // Your agent coordination logic here
        loop {
            // ... do work ...
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    };
    
    tokio::select! {
        _ = shutdown => {
            println!("🛑 Shutting down gracefully...");
            // Cleanup: send shutdown messages to agents
            for agent in town.list_agents().await {
                let handle = town.agent(&agent.name).await?;
                handle.send(MessageType::Shutdown).await.ok();
            }
        }
        _ = work => {}
    }
    
    Ok(())
}
```

## Redis Connection Recovery

The Channel uses ConnectionManager which auto-reconnects:

```rust
// ConnectionManager handles reconnection automatically
// But you should still handle errors gracefully:

async fn send_with_retry(channel: &Channel, msg: &Message) -> Result<()> {
    for attempt in 1..=3 {
        match channel.send(msg).await {
            Ok(_) => return Ok(()),
            Err(e) if attempt < 3 => {
                println!("⚠️  Send failed (attempt {}): {}", attempt, e);
                tokio::time::sleep(Duration::from_millis(100 * attempt as u64)).await;
            }
            Err(e) => return Err(e),
        }
    }
    unreachable!()
}
```

## Recovery Checklist

When things go wrong:

1. **Check Redis** — Is `redis-server` running?
   ```bash
   redis-cli -s ./redis.sock PING
   ```

2. **Check agent state** — What state is it in?
   ```bash
   tt list
   tt status
   ```

3. **Check inbox** — Are messages stuck?
   ```bash
   redis-cli -s ./redis.sock LLEN mt:inbox:<agent-id>
   ```

4. **Check task** — What happened to the task?
   ```bash
   redis-cli -s ./redis.sock GET mt:task:<task-id>
   ```

5. **Check logs** — Look in `logs/` directory

## Comparison with Gastown Recovery

| Feature | Tinytown | Gastown |
|---------|----------|---------|
| Auto-recovery | Manual (you write it) | Witness patrol |
| State persistence | Redis | Git-backed beads |
| Crash detection | Check agent state | Boot/Deacon monitors |
| Work resumption | Reassign tasks | Hook-based (automatic) |

Tinytown puts you in control. Gastown automates more but is more complex. Choose based on your reliability requirements.

