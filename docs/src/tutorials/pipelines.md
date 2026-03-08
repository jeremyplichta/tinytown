# Tutorial: Task Pipelines

Build structured workflows with task dependencies and hierarchies.

## What We'll Build

A code review pipeline:
1. Developer writes code
2. Linter checks style
3. Tester writes tests
4. Reviewer approves
5. Merger deploys

## Pipeline with Parent Tasks

```rust
use tinytown::{Town, Task, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let town = Town::connect(".").await?;
    let channel = town.channel();
    
    // Create the epic (parent task)
    let epic = Task::new("Implement user profile feature")
        .with_tags(["epic", "q1-2024"]);
    channel.set_task(&epic).await?;
    
    // Create subtasks
    let design = Task::new("Design profile API schema")
        .with_parent(epic.id)
        .with_tags(["design"]);
    
    let implement = Task::new("Implement profile endpoints")
        .with_parent(epic.id)
        .with_tags(["backend"]);
    
    let test = Task::new("Write profile API tests")
        .with_parent(epic.id)
        .with_tags(["testing"]);
    
    let review = Task::new("Review profile implementation")
        .with_parent(epic.id)
        .with_tags(["review"]);
    
    // Store all tasks
    for task in [&design, &implement, &test, &review] {
        channel.set_task(task).await?;
    }
    
    println!("📋 Created pipeline with {} subtasks", 4);
    println!("   Epic: {}", epic.id);
    
    Ok(())
}
```

## Stateful Pipeline Manager

A reusable pipeline structure:

```rust
use tinytown::{Town, Task, TaskId, TaskState, AgentHandle, Result};
use std::collections::HashMap;

struct Pipeline {
    name: String,
    stages: Vec<Stage>,
    current_stage: usize,
}

struct Stage {
    name: String,
    task: Task,
    agent: String,
    completed: bool,
}

impl Pipeline {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            stages: vec![],
            current_stage: 0,
        }
    }
    
    fn add_stage(&mut self, name: &str, description: &str, agent: &str) {
        self.stages.push(Stage {
            name: name.to_string(),
            task: Task::new(description),
            agent: agent.to_string(),
            completed: false,
        });
    }
    
    async fn run(&mut self, town: &Town) -> Result<()> {
        println!("🚀 Starting pipeline: {}", self.name);
        
        for i in 0..self.stages.len() {
            let stage = &self.stages[i];
            println!("\n📍 Stage {}: {}", i + 1, stage.name);
            
            // Get or spawn agent
            let agent = match town.agent(&stage.agent).await {
                Ok(a) => a,
                Err(_) => town.spawn_agent(&stage.agent, "claude").await?,
            };
            
            // Assign task
            let task = Task::new(&stage.task.description);
            agent.assign(task).await?;
            
            // Wait for completion
            wait_for_idle(&agent).await?;
            
            println!("   ✅ Stage {} complete", stage.name);
        }
        
        println!("\n🎉 Pipeline '{}' completed!", self.name);
        Ok(())
    }
}

// Usage
#[tokio::main]
async fn main() -> Result<()> {
    let town = Town::connect(".").await?;
    
    let mut pipeline = Pipeline::new("Feature Development");
    pipeline.add_stage("Design", "Design the feature architecture", "architect");
    pipeline.add_stage("Implement", "Implement the feature", "developer");
    pipeline.add_stage("Test", "Write tests for the feature", "tester");
    pipeline.add_stage("Review", "Review the implementation", "reviewer");
    
    pipeline.run(&town).await?;
    
    Ok(())
}
```

## Conditional Pipelines

Branch based on results:

```rust
async fn conditional_pipeline(town: &Town) -> Result<()> {
    let linter = town.spawn_agent("linter", "claude").await?;
    let developer = town.spawn_agent("developer", "claude").await?;
    
    // First: lint check
    linter.assign(Task::new("Run linting on src/")).await?;
    wait_for_idle(&linter).await?;
    
    // Check lint result
    let lint_passed = check_lint_result(town).await?;
    
    if lint_passed {
        // Continue to tests
        developer.assign(Task::new("Run test suite")).await?;
    } else {
        // Fix lint errors first
        developer.assign(Task::new("Fix linting errors in src/")).await?;
        wait_for_idle(&developer).await?;
        
        // Re-run linter
        linter.assign(Task::new("Re-run linting on src/")).await?;
    }
    
    Ok(())
}
```

## Retry Logic

Handle transient failures:

```rust
async fn with_retry<F, Fut, T>(mut f: F, max_attempts: u32) -> Result<T>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T>>,
{
    let mut attempts = 0;
    loop {
        attempts += 1;
        match f().await {
            Ok(result) => return Ok(result),
            Err(e) if attempts < max_attempts => {
                println!("⚠️  Attempt {} failed: {}, retrying...", attempts, e);
                tokio::time::sleep(Duration::from_secs(2)).await;
            }
            Err(e) => return Err(e),
        }
    }
}

// Usage
with_retry(|| async {
    let agent = town.spawn_agent("worker", "claude").await?;
    agent.assign(Task::new("Flaky operation")).await?;
    wait_for_idle(&agent).await
}, 3).await?;
```

## Best Practices

1. **Use parent tasks** for grouping related work
2. **Tag tasks** for easy filtering and reporting
3. **Keep stages small** — easier to retry and debug
4. **Log stage transitions** — helps troubleshooting
5. **Handle failures gracefully** — don't crash the whole pipeline

## Next Steps

- [Error Handling & Recovery](./recovery.md)
- [Coming from Gastown: Convoy Mapping](../gastown/concepts.md)

