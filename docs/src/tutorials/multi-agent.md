# Tutorial: Multi-Agent Coordination

Let's coordinate multiple agents working together on a feature.

## The Scenario

We'll build a system where:
1. **Architect** designs the API
2. **Developer** implements it
3. **Tester** writes tests
4. **Reviewer** reviews everything

## Setup

```bash
mkdir multi-agent-demo && cd multi-agent-demo
tt init --name multi-demo
```

## Spawning the Team

```bash
tt spawn architect --model claude
tt spawn developer --model auggie
tt spawn tester --model codex
tt spawn reviewer --model claude
```

Or in code:

```rust
use tinytown::{Town, Task, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let town = Town::connect(".").await?;
    
    // Spawn all agents
    let architect = town.spawn_agent("architect", "claude").await?;
    let developer = town.spawn_agent("developer", "auggie").await?;
    let tester = town.spawn_agent("tester", "codex").await?;
    let reviewer = town.spawn_agent("reviewer", "claude").await?;
    
    println!("🏗️  Team assembled!");
    Ok(())
}
```

## Sequential Pipeline

The simplest pattern: each agent waits for the previous one.

```rust
use tinytown::{Town, Task, AgentState, Result};
use std::time::Duration;

async fn wait_for_idle(handle: &tinytown::AgentHandle) -> Result<()> {
    loop {
        if let Some(agent) = handle.state().await? {
            if matches!(agent.state, AgentState::Idle | AgentState::Stopped) {
                return Ok(());
            }
        }
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let town = Town::connect(".").await?;
    
    let architect = town.spawn_agent("architect", "claude").await?;
    let developer = town.spawn_agent("developer", "auggie").await?;
    let tester = town.spawn_agent("tester", "codex").await?;
    
    // Step 1: Design
    println!("📐 Phase 1: Architecture");
    architect.assign(Task::new(
        "Design a REST API for user authentication with JWT tokens. \
         Output: API spec with endpoints, request/response formats."
    )).await?;
    wait_for_idle(&architect).await?;
    
    // Step 2: Implement
    println!("💻 Phase 2: Implementation");
    developer.assign(Task::new(
        "Implement the auth API from the architect's design. \
         Use the endpoints and formats specified."
    )).await?;
    wait_for_idle(&developer).await?;
    
    // Step 3: Test
    println!("🧪 Phase 3: Testing");
    tester.assign(Task::new(
        "Write comprehensive tests for the auth API. \
         Cover success cases, error handling, and edge cases."
    )).await?;
    wait_for_idle(&tester).await?;
    
    println!("✅ Pipeline complete!");
    Ok(())
}
```

## Parallel Execution

When tasks are independent, run them in parallel:

```rust
use tokio::join;

#[tokio::main]
async fn main() -> Result<()> {
    let town = Town::connect(".").await?;
    
    let frontend = town.spawn_agent("frontend", "claude").await?;
    let backend = town.spawn_agent("backend", "auggie").await?;
    let docs = town.spawn_agent("docs", "codex").await?;
    
    // Assign all at once
    frontend.assign(Task::new("Build the login UI")).await?;
    backend.assign(Task::new("Build the auth API")).await?;
    docs.assign(Task::new("Write API documentation")).await?;
    
    // Wait for all in parallel
    let (r1, r2, r3) = join!(
        wait_for_idle(&frontend),
        wait_for_idle(&backend),
        wait_for_idle(&docs)
    );
    
    r1?; r2?; r3?;
    println!("✅ All agents completed!");
    Ok(())
}
```

## Fan-Out / Fan-In

Split work across agents, then aggregate:

```rust
#[tokio::main]
async fn main() -> Result<()> {
    let town = Town::connect(".").await?;
    
    // Fan-out: Create workers
    let mut workers = vec![];
    for i in 1..=3 {
        let worker = town.spawn_agent(&format!("worker-{}", i), "claude").await?;
        workers.push(worker);
    }
    
    // Assign chunks of work
    let tasks = vec![
        "Implement module A",
        "Implement module B", 
        "Implement module C",
    ];
    
    for (worker, task) in workers.iter().zip(tasks.iter()) {
        worker.assign(Task::new(*task)).await?;
    }
    
    // Wait for all
    for worker in &workers {
        wait_for_idle(worker).await?;
    }
    
    // Fan-in: Aggregate results
    let reviewer = town.spawn_agent("reviewer", "claude").await?;
    reviewer.assign(Task::new(
        "Review modules A, B, and C for consistency and integration issues"
    )).await?;
    wait_for_idle(&reviewer).await?;
    
    println!("✅ Fan-out/fan-in complete!");
    Ok(())
}
```

## Agent-to-Agent Communication

Agents can send messages directly:

```rust
use tinytown::{Message, MessageType, AgentId};

// Worker notifies reviewer when done
let msg = Message::new(
    worker_id,
    reviewer_id,
    MessageType::Custom {
        kind: "ready_for_review".into(),
        payload: r#"{"pr_url": "https://github.com/..."}"#.into()
    }
);
town.channel().send(&msg).await?;
```

## Comparison with Gastown

| Pattern | Tinytown | Gastown |
|---------|----------|---------|
| Sequential | `wait_for_idle()` loop | Convoy + Beads events |
| Parallel | `tokio::join!` | Mayor distributes |
| Fan-out/in | Manual coordination | Convoy tracking |
| Messaging | Direct `channel.send()` | Mail protocol |

Tinytown is more explicit—you write the coordination logic. Gastown abstracts it with Convoys and the Mayor. Choose based on your needs.

## Next Steps

- [Task Pipelines](./pipelines.md) — Build complex workflows
- [Error Handling](./recovery.md) — Handle failures gracefully

