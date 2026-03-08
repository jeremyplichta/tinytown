# Tutorial: Single Agent Workflow

Let's build a complete workflow with one agent doing a coding task.

## What We'll Build

A simple system that:
1. Creates a coding task
2. Assigns it to an agent
3. Waits for completion
4. Reports the result

## Setup

```bash
mkdir single-agent-demo && cd single-agent-demo
tt init --name demo
```

## The Code

Create `main.rs`:

```rust
use tinytown::{Town, Task, Result};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    // Connect to town
    let town = Town::connect(".").await?;
    
    // Create an agent
    let agent = town.spawn_agent("coder", "claude").await?;
    println!("🤖 Spawned agent: {}", agent.id());
    
    // Create a task
    let task = Task::new(
        "Create a Rust function that calculates fibonacci numbers recursively"
    );
    println!("📋 Created task: {}", task.id);
    
    // Assign to agent
    let task_id = agent.assign(task).await?;
    println!("✅ Assigned task {} to coder", task_id);
    
    // Check status periodically
    loop {
        tokio::time::sleep(Duration::from_secs(5)).await;
        
        if let Some(state) = agent.state().await? {
            println!("   Agent state: {:?}", state.state);
            
            match state.state {
                tinytown::AgentState::Idle => {
                    println!("🎉 Agent completed work!");
                    break;
                }
                tinytown::AgentState::Error => {
                    println!("❌ Agent encountered error");
                    break;
                }
                _ => continue,
            }
        }
    }
    
    // Get task result
    if let Some(task) = town.channel().get_task(task_id).await? {
        println!("\n📊 Task result:");
        println!("   State: {:?}", task.state);
        if let Some(result) = task.result {
            println!("   Output: {}", result);
        }
    }
    
    Ok(())
}
```

## Running It

```bash
# In terminal 1: Keep the town running
tt start

# In terminal 2: Run your code
cargo run
```

## What Happens

1. **Town connects** to the existing town (and its Redis)
2. **Agent spawns** with state `Starting` → `Idle`
3. **Task creates** with state `Pending`
4. **Assignment** sends a `TaskAssign` message to agent's inbox
5. **Agent receives** the message (in a real setup, Claude would process it)
6. **Polling** checks agent state every 5 seconds
7. **Completion** when agent returns to `Idle`

## The Message Flow

```
Your Code                     Redis                        Agent
    │                           │                            │
    │  spawn_agent()            │                            │
    │ ─────────────────────────►│                            │
    │                           │  SET mt:agent:xxx          │
    │                           │ ───────────────────────────│
    │                           │                            │
    │  assign(task)             │                            │
    │ ─────────────────────────►│                            │
    │                           │  SET mt:task:yyy           │
    │                           │  RPUSH mt:inbox:xxx        │
    │                           │ ───────────────────────────│
    │                           │                            │
    │                           │  BLPOP mt:inbox:xxx        │
    │                           │◄───────────────────────────│
    │                           │                            │
    │  state()                  │                            │
    │ ─────────────────────────►│                            │
    │                           │  GET mt:agent:xxx          │
    │◄───────────────────────── │                            │
```

## Simulating the Agent

In a real workflow, Claude (or another AI) receives the task. For testing, you can simulate completion:

```bash
# In redis-cli
redis-cli -s ./redis.sock

# Get the inbox message
LPOP mt:inbox:550e8400-e29b-41d4-a716-446655440000

# Update agent state to idle
# (In practice, the agent process does this)
```

## Key Takeaways

1. **Towns manage Redis** — You don't need to start it manually
2. **Agents are stateful** — Their state persists in Redis
3. **Tasks are tracked** — Full lifecycle from pending to complete
4. **Messages are reliable** — Redis lists ensure delivery

## Next Steps

- [Multi-Agent Coordination](./multi-agent.md) — Coordinate multiple agents
- [Task Pipelines](./pipelines.md) — Chain tasks together

