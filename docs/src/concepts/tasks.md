# Tasks

A **Task** is a unit of work that can be assigned to an agent.

## Task Properties

| Property | Type | Description |
|----------|------|-------------|
| `id` | UUID | Unique identifier |
| `description` | String | What needs to be done |
| `state` | Enum | Current lifecycle state |
| `assigned_to` | Option | Agent working on this |
| `created_at` | DateTime | When created |
| `updated_at` | DateTime | Last modification |
| `completed_at` | Option | When finished |
| `result` | Option | Output or error message |
| `parent_id` | Option | For hierarchical tasks |
| `tags` | Vec | Labels for filtering |

## Task States

```
┌─────────┐
│ Pending │ ── Created, waiting for assignment
└────┬────┘
     │ assign()
     ▼
┌──────────┐
│ Assigned │ ── Given to an agent
└────┬─────┘
     │ start()
     ▼
┌─────────┐
│ Running │ ── Agent is working on it
└────┬────┘
     │
     ├─── complete() ──► ┌───────────┐
     │                   │ Completed │ ✓
     │                   └───────────┘
     │
     ├─── fail() ──────► ┌────────┐
     │                   │ Failed │ ✗
     │                   └────────┘
     │
     └─── cancel() ────► ┌───────────┐
                         │ Cancelled │ ⊘
                         └───────────┘
```

## Creating Tasks

### CLI

```bash
tt assign worker-1 "Implement user authentication"
```

### Rust API

```rust
use tinytown::Task;

// Simple task
let task = Task::new("Implement user authentication");

// With tags
let task = Task::new("Fix login bug")
    .with_tags(["bug", "auth", "urgent"]);

// With parent (for subtasks)
let parent = Task::new("Build auth system");
let subtask = Task::new("Implement password hashing")
    .with_parent(parent.id);
```

## Task Lifecycle Methods

```rust
let mut task = Task::new("Build the API");

// Assign to an agent
task.assign(agent_id);
assert_eq!(task.state, TaskState::Assigned);

// Mark as started
task.start();
assert_eq!(task.state, TaskState::Running);

// Complete with result
task.complete("API implemented at /api/v1/users");
assert_eq!(task.state, TaskState::Completed);
assert!(task.completed_at.is_some());

// Or fail with error
task.fail("Could not connect to database");
assert_eq!(task.state, TaskState::Failed);
```

## Checking Task State

```rust
// Is the task finished?
if task.state.is_terminal() {
    match task.state {
        TaskState::Completed => println!("Done: {}", task.result.unwrap()),
        TaskState::Failed => println!("Error: {}", task.result.unwrap()),
        TaskState::Cancelled => println!("Cancelled"),
        _ => unreachable!(),
    }
}
```

## Task Storage in Redis

Tasks are stored as JSON:

```
tt:task:<uuid>  →  JSON serialized Task struct
```

You can inspect tasks directly:
```bash
redis-cli -s ./redis.sock GET "tt:task:550e8400-e29b-41d4-a716-446655440000"
```

## Hierarchical Tasks

Create parent-child relationships for complex work:

```rust
// Epic
let epic = Task::new("User Management System");
channel.set_task(&epic).await?;

// Features under the epic
let signup = Task::new("User signup flow").with_parent(epic.id);
let login = Task::new("User login flow").with_parent(epic.id);
let profile = Task::new("User profile page").with_parent(epic.id);
```

## Task Tags

Use tags to categorize and filter:

```rust
let task = Task::new("Fix XSS vulnerability")
    .with_tags(["security", "bug", "P0"]);
```

## Comparison with Gastown Beads

| Feature | Tinytown Tasks | Gastown Beads |
|---------|----------------|---------------|
| Storage | Redis (JSON) | Dolt SQL |
| Hierarchy | Optional parent_id | Full graph |
| Metadata | Tags array | Full schema |
| Persistence | Redis persistence | Git-backed |
| Complexity | Simple | Complex |

Tinytown tasks are intentionally simpler. If you need Gastown's bead features (dependency graphs, git versioning, SQL queries), consider using Gastown or building on top of Tinytown.

