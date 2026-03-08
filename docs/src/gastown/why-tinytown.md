# Why Tinytown?

A philosophical guide to choosing simplicity.

## The Problem with Complex Systems

Gastown is impressive engineering. It has:
- Automatic crash recovery
- Git-backed work history  
- Multi-agent coordination
- Visual dashboards
- Sophisticated orchestration

But it also has:
- **317,898 lines of code** to understand
- **50+ concepts** to learn
- **Hours of setup** before your first task
- **Days of learning** before you're productive

## The Tinytown Philosophy

> "Make it work. Make it simple. Stop."

Tinytown takes a different approach:

### 1. You Don't Need Most Features

90% of multi-agent orchestration is:
1. Create agents
2. Assign tasks
3. Wait for completion
4. Check results

Tinytown does exactly this. Nothing more.

### 2. Complexity Compounds

Every feature adds:
- Code to maintain
- Concepts to learn
- Bugs to fix
- Documentation to write

Tinytown has **1,448 lines of code**. You can read the entire codebase in an afternoon.

### 3. Explicit is Better Than Magic

Gastown's Mayor "figures things out" for you:
```bash
gt mayor attach
> Build an authentication system
# Mayor creates convoy, spawns agents, distributes work...
```

Tinytown makes you say what you want:
```rust
architect.assign(Task::new("Design auth system")).await?;
developer.assign(Task::new("Implement auth")).await?;
tester.assign(Task::new("Test auth")).await?;
```

More typing, but you know exactly what's happening.

### 4. Recovery is Your Responsibility

Gastown: Witness patrols, Deacon monitors, Boot watches Deacon...

Tinytown: You write a loop:
```rust
if agent.state == AgentState::Error {
    respawn_and_retry(agent).await?;
}
```

Is this more work? Yes. Is it simpler to understand? Also yes.

## The Tradeoffs

### What You Gain

✅ **Understanding** — You know how it works  
✅ **Speed** — Running in 30 seconds  
✅ **Debuggability** — 1,400 lines to read  
✅ **Control** — You decide everything  
✅ **Simplicity** — 5 concepts total  

### What You Lose

❌ **Automation** — You write recovery logic  
❌ **Scale** — Designed for 1-10 agents  
❌ **History** — No git-backed audit trail  
❌ **Visualization** — No built-in dashboard  
❌ **Federation** — Single machine focus  

## When to Choose What

### Choose Tinytown If:

- You're learning agent orchestration
- You want to ship something today
- You have 1-5 agents
- You prefer explicit over magic
- You value understanding over features

### Choose Gastown If:

- You need 20+ concurrent agents
- You need audit trails
- You need automatic recovery
- You need cross-project coordination
- You have time to learn the system

### Choose Both If:

Start with Tinytown. Learn the patterns. If you outgrow it, Gastown will make more sense because you understand what problems it's solving.

## A Practical Test

Ask yourself:

1. **How many agents do I need?**
   - 1-5: Tinytown
   - 10+: Consider Gastown

2. **How important is automatic recovery?**
   - Nice to have: Tinytown
   - Critical: Gastown

3. **How much time do I have?**
   - Minutes: Tinytown
   - Days/weeks: Either

4. **Do I want to understand the system?**
   - Yes: Tinytown
   - No, just make it work: Gastown (eventually)

## The Honest Answer

Tinytown exists because Gastown is hard to start with.

If you've bounced off Gastown, Tinytown gets you running. You can always graduate to Gastown later—and you'll appreciate its features more because you've felt the pain of not having them.

Start simple. Add complexity only when you need it.

> "Perfection is achieved not when there is nothing more to add, but when there is nothing left to take away."
> — Antoine de Saint-Exupéry

