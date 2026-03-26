# PulseHive DevStudio — Product Specification

> **Version:** 1.0.0-spec
> **Status:** Refined — ready for implementation
> **Created:** March 2026
> **Updated:** March 2026
> **Dependency:** PulseHive SDK v1.0 (`pulsehive` crate with `openai` feature)

---

## 1. Overview

PulseHive DevStudio is a **multi-agent CLI code agent** built on the PulseHive SDK. It takes a task description and a repository path, deploys a team of specialized agents that share consciousness through a PulseDB substrate, and produces working code changes.

```
$ devstudio "Add JWT authentication to the Express API" --repo ./my-express-app

🔍 Explorer analyzing codebase...
  [agent_started] explorer (Llm)
  [tool_call] tree ./my-express-app
  [tool_call] file_read ./my-express-app/src/index.ts
  [agent_completed] explorer → found Express app with 12 routes, no auth middleware

📋 Planner creating implementation plan...
  [agent_started] planner (Llm)
  [agent_completed] planner → 5-step plan: install jsonwebtoken, create middleware, protect routes...

💻 Coder implementing changes...
  [agent_started] coder (Llm)
  [tool_call] file_write ./my-express-app/src/middleware/auth.ts
  [tool_call] shell_exec npm install jsonwebtoken
  [agent_completed] coder → modified 4 files, added 2 new files

🧪 Tester writing and running tests...
  [agent_started] tester (Llm)
  [tool_call] file_write ./my-express-app/tests/auth.test.ts
  [tool_call] shell_exec npm test
  [agent_completed] tester → 6 tests written, 6 passed

✅ Done! 4 agents completed. 6 files modified. All tests pass.
```

### Purpose

DevStudio is the **first vertical product** built on PulseHive, serving as the proof-of-concept that validates the SDK works for real-world multi-agent orchestration. It exercises every core SDK primitive:

| SDK Primitive | DevStudio Usage |
|---------------|-----------------|
| `HiveMind` | Orchestrates the agent team |
| `AgentKind::Sequential` | Explorer → Planner → Coder → Tester pipeline |
| `Tool` trait | File I/O, shell execution, directory listing, search |
| `Lens` | Domain-specific perception (code, architecture, testing) |
| `Experience` | Agents share findings via PulseDB substrate |
| `HiveEvent` | Streaming progress output to terminal |
| `ApprovalHandler` | Safety gates for file writes and shell commands |
| `LlmProvider` | GLM/OpenAI-compatible model via `pulsehive-openai` |

### Relationship to Other Products

- **PulseHive SDK** — DevStudio is a *consumer* of the SDK. It imports `pulsehive` as a dependency.
- **ProjectPulse Desktop** — The project management hub. DevStudio handles *code execution*; ProjectPulse handles *project planning, tickets, sprints*. They're complementary.
- **PulseDB** — DevStudio uses PulseDB (via PulseHive) as the substrate where agents store and perceive experiences.

---

## 2. CLI Interface

### Usage

```
devstudio <TASK> [OPTIONS]

Arguments:
  <TASK>    Natural language description of the task to perform

Options:
  -r, --repo <PATH>        Path to the target repository [default: .]
  -p, --provider <NAME>    LLM provider name [default: "openai"]
  -m, --model <MODEL>      LLM model identifier [default: "glm-4-plus"]
      --base-url <URL>     Override LLM API base URL [default: "https://open.bigmodel.cn/api/paas/v4"]
      --api-key <KEY>      LLM API key [env: DEVSTUDIO_API_KEY]
      --approve-all        Skip approval prompts (auto-approve everything)
      --dry-run            Show plan only, don't execute code changes
      --substrate <PATH>   PulseDB substrate path [default: ".devstudio/substrate.db"]
  -v, --verbose            Show all HiveEvent details
  -h, --help               Print help
  -V, --version            Print version
```

### Examples

```bash
# Basic usage — modify an Express app
devstudio "Add rate limiting to all API endpoints" --repo ./my-api

# Use a specific model
devstudio "Refactor database layer to use Prisma" --repo ./backend --model gpt-4o

# Dry run — see the plan without executing
devstudio "Add WebSocket support" --repo ./chat-app --dry-run

# Auto-approve all file changes (for CI)
devstudio "Fix all ESLint warnings" --repo ./frontend --approve-all

# Use local Ollama
devstudio "Add unit tests" --repo ./lib --base-url http://localhost:11434/v1 --model llama3
```

### Output

The CLI streams `HiveEvent` events to stderr (progress) and writes a summary to stdout:

- **Agent lifecycle**: `[explorer] started`, `[coder] completed`
- **Tool calls**: `[file_write] src/auth.ts (234 lines)`, `[shell_exec] npm test`
- **Approval prompts**: Interactive y/n for file writes and shell commands
- **Summary**: Files modified, tests run, agent outcomes

---

## 3. Architecture

```
pulsehive-devstudio/
├── Cargo.toml              ← deps: pulsehive (openai), clap, colored
├── src/
│   ├── main.rs             ← CLI parsing, HiveMind setup, event streaming
│   ├── agents/
│   │   ├── mod.rs          ← Agent factory functions
│   │   ├── explorer.rs     ← Codebase exploration agent
│   │   ├── planner.rs      ← Implementation planning agent
│   │   ├── coder.rs        ← Code implementation agent
│   │   └── tester.rs       ← Test writing and execution agent
│   ├── tools/
│   │   ├── mod.rs          ← Tool registration
│   │   ├── file_read.rs    ← Read file contents
│   │   ├── file_write.rs   ← Write/create files (requires approval)
│   │   ├── shell_exec.rs   ← Execute shell commands (requires approval)
│   │   ├── tree.rs         ← Directory listing
│   │   └── search.rs       ← Content search (grep/ripgrep)
│   ├── approval.rs         ← Interactive CLI approval handler
│   └── workflow.rs         ← Sequential pipeline assembly
└── tests/
    └── integration_test.rs ← End-to-end test with mock LLM
```

### Dependencies

```toml
[dependencies]
pulsehive = { version = "1.0", features = ["openai"] }
clap = { version = "4", features = ["derive"] }
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
colored = "3"
futures = "0.3"
serde_json = "1"
```

### Data Flow

```
User task + repo path
        │
        ▼
  ┌─────────────┐
  │  HiveMind    │
  │  .builder()  │──── PulseDB substrate (.devstudio/substrate.db)
  │  .build()    │──── OpenAI-compatible provider (GLM)
  └──────┬──────┘
         │
         ▼
  AgentKind::Sequential
  ┌──────────────────────────────────────────────────────────┐
  │                                                          │
  │  1. Explorer ──────────────────────────────────────────  │
  │     Lens: ["code", "architecture"]                       │
  │     Tools: file_read, tree, search                       │
  │     Output: Experiences describing repo structure        │
  │         │                                                │
  │         │ (perceives via substrate)                       │
  │         ▼                                                │
  │  2. Planner ──────────────────────────────────────────   │
  │     Lens: ["architecture", "requirements"]               │
  │     Tools: file_read                                     │
  │     Output: Experience containing step-by-step plan      │
  │         │                                                │
  │         │ (perceives via substrate)                       │
  │         ▼                                                │
  │  3. Coder ────────────────────────────────────────────   │
  │     Lens: ["code", "implementation"]                     │
  │     Tools: file_read, file_write, shell_exec             │
  │     Output: Modified files + experiences                 │
  │         │                                                │
  │         │ (perceives via substrate)                       │
  │         ▼                                                │
  │  4. Tester ───────────────────────────────────────────   │
  │     Lens: ["testing", "quality"]                         │
  │     Tools: file_read, file_write, shell_exec             │
  │     Output: Tests written + test results                 │
  │                                                          │
  └──────────────────────────────────────────────────────────┘
         │
         ▼
  Summary output (files changed, tests passed)
```

---

## 4. Agent Definitions

### Explorer

**Purpose**: Understand the codebase structure, dependencies, and current state relevant to the task.

**System Prompt**:
```
You are a codebase exploration agent. Your job is to thoroughly understand the
repository structure, technology stack, dependencies, and existing code patterns
before any changes are made.

Given the task: "{task}"
Repository: {repo_path}

Use the `tree` tool to see the directory structure.
Use the `search` tool to find code related to the task.
Use the `file_read` tool to read key files (package.json, Cargo.toml, README, main entry points).

Provide a comprehensive summary of:
1. Technology stack and framework
2. Project structure (key directories and files)
3. Existing patterns relevant to the task
4. Dependencies that may be needed
5. Files that will likely need modification
```

**Lens**: `Lens::new(["code", "architecture"])` — attention_budget: 50
**Tools**: `file_read`, `tree`, `search`
**Approval**: None required (read-only operations)

### Planner

**Purpose**: Create a detailed implementation plan based on the Explorer's findings.

**System Prompt**:
```
You are an implementation planning agent. Based on the codebase understanding
from the exploration phase (available in your context), create a detailed
step-by-step implementation plan.

Task: "{task}"

Your plan should include:
1. Files to create (with purpose)
2. Files to modify (with specific changes)
3. Dependencies to install
4. Configuration changes needed
5. Order of operations (what to do first)
6. Potential risks or edge cases

Be specific — the coder agent will follow your plan exactly.
Do NOT write code. Only describe what needs to be done.
```

**Lens**: `Lens::new(["architecture", "requirements"])` — attention_budget: 100
**Tools**: `file_read` (to verify details from exploration)
**Approval**: None required

### Coder

**Purpose**: Implement the plan by writing code, modifying files, and installing dependencies.

**System Prompt**:
```
You are a code implementation agent. Follow the implementation plan from the
planning phase (available in your context) and write the actual code.

Task: "{task}"

Rules:
- Follow the plan step by step
- Write production-quality code with proper error handling
- Follow existing code conventions in the repository
- Use file_write to create or modify files
- Use shell_exec for package installations (npm install, pip install, etc.)
- Do NOT run tests — the tester agent handles that
- After each file modification, briefly note what you changed and why
```

**Lens**: `Lens::new(["code", "implementation"])` — attention_budget: 100
**Tools**: `file_read`, `file_write`, `shell_exec`
**Approval**: `file_write` and `shell_exec` require approval (unless `--approve-all`)

### Tester

**Purpose**: Write tests for the changes and run them to verify correctness.

**System Prompt**:
```
You are a testing agent. Based on the code changes made by the coder agent
(available in your context), write appropriate tests and run them.

Task: "{task}"

Rules:
- Read the modified/created files to understand what was implemented
- Write tests that cover the main functionality
- Follow the project's existing test conventions (framework, directory, naming)
- Use shell_exec to run the test suite
- Report test results clearly: passed, failed, errors
- If tests fail, describe what went wrong (but don't fix the code)
```

**Lens**: `Lens::new(["testing", "quality"])` — attention_budget: 100
**Tools**: `file_read`, `file_write`, `shell_exec`
**Approval**: `file_write` and `shell_exec` require approval

---

## 5. Tool Implementations

Each tool implements `pulsehive_core::tool::Tool`. Parameters follow JSON Schema.

### file_read

```rust
fn name(&self) -> &str { "file_read" }
fn description(&self) -> &str { "Read the contents of a file at the given path" }
fn parameters(&self) -> Value {
    json!({
        "type": "object",
        "properties": {
            "path": { "type": "string", "description": "File path relative to repo root" }
        },
        "required": ["path"]
    })
}
// execute: reads file, returns contents as ToolResult::text
```

### file_write

```rust
fn name(&self) -> &str { "file_write" }
fn description(&self) -> &str { "Write content to a file, creating it if it doesn't exist" }
fn parameters(&self) -> Value {
    json!({
        "type": "object",
        "properties": {
            "path": { "type": "string", "description": "File path relative to repo root" },
            "content": { "type": "string", "description": "Complete file content to write" }
        },
        "required": ["path", "content"]
    })
}
fn requires_approval(&self) -> bool { true }
// execute: writes file, returns confirmation
```

### shell_exec

```rust
fn name(&self) -> &str { "shell_exec" }
fn description(&self) -> &str { "Execute a shell command in the repository directory" }
fn parameters(&self) -> Value {
    json!({
        "type": "object",
        "properties": {
            "command": { "type": "string", "description": "Shell command to execute" },
            "cwd": { "type": "string", "description": "Working directory (defaults to repo root)" }
        },
        "required": ["command"]
    })
}
fn requires_approval(&self) -> bool { true }
// execute: runs command, returns stdout + stderr as ToolResult::text
// Timeout: 60 seconds default
```

### tree

```rust
fn name(&self) -> &str { "tree" }
fn description(&self) -> &str { "List directory structure" }
fn parameters(&self) -> Value {
    json!({
        "type": "object",
        "properties": {
            "path": { "type": "string", "description": "Directory path (defaults to repo root)" },
            "max_depth": { "type": "integer", "description": "Maximum depth (default: 3)" }
        }
    })
}
// execute: walks directory, returns tree-formatted string
// Respects .gitignore, excludes node_modules/, .git/, target/, etc.
```

### search

```rust
fn name(&self) -> &str { "search" }
fn description(&self) -> &str { "Search for a pattern in files (like grep)" }
fn parameters(&self) -> Value {
    json!({
        "type": "object",
        "properties": {
            "pattern": { "type": "string", "description": "Search pattern (regex)" },
            "path": { "type": "string", "description": "Directory to search (default: repo root)" },
            "file_pattern": { "type": "string", "description": "File glob filter (e.g., '*.ts')" }
        },
        "required": ["pattern"]
    })
}
// execute: searches files, returns matching lines with file paths
// Max 50 results to avoid context overflow
```

---

## 6. Approval Handler

Interactive CLI approval for dangerous operations:

```
┌─────────────────────────────────────────────┐
│ 📝 file_write: src/middleware/auth.ts       │
│                                             │
│ + import jwt from 'jsonwebtoken';           │
│ + export function authMiddleware(req, ...) {│
│ +   const token = req.headers.auth...       │
│ + }                                         │
│                                             │
│ [y] Approve  [n] Deny  [a] Approve all      │
└─────────────────────────────────────────────┘
```

Implementation:
```rust
struct CliApprovalHandler { approve_all: AtomicBool }

impl ApprovalHandler for CliApprovalHandler {
    async fn request_approval(&self, action: &PendingAction) -> ApprovalResult {
        if self.approve_all.load(Ordering::Relaxed) {
            return ApprovalResult::Approved;
        }
        // Print action details, read stdin for y/n/a
    }
}
```

---

## 7. Workflow Assembly

```rust
fn build_workflow(task: &str, repo: &str, tools: Vec<Arc<dyn Tool>>) -> AgentDefinition {
    let config = LlmConfig::new("openai", "glm-4-plus");

    AgentDefinition {
        name: "devstudio-pipeline".into(),
        kind: AgentKind::Sequential(vec![
            build_explorer(task, repo, &config, &tools),
            build_planner(task, &config, &tools),
            build_coder(task, &config, &tools),
            build_tester(task, &config, &tools),
        ]),
    }
}
```

---

## 8. Success Criteria

The proof-of-concept is successful if:

1. **Explorer** correctly identifies the repo structure and relevant files
2. **Planner** produces a coherent plan that references real files from exploration
3. **Coder** writes syntactically correct file modifications
4. **Tester** writes tests that actually run and produce pass/fail results
5. **Shared consciousness**: Later agents perceive earlier agents' experiences (the Coder's response should reference the Planner's plan, not re-invent it)
6. **Event streaming**: The terminal shows real-time progress events
7. **Approval**: File writes and shell commands prompt for approval (unless `--approve-all`)
8. **Substrate persistence**: Re-running DevStudio on the same repo perceives previous runs

### Test Scenarios

1. **Express.js auth**: "Add JWT authentication to the Express API" on a simple Express app
2. **Python Flask endpoint**: "Add a /health endpoint" to a Flask app
3. **React component**: "Add a dark mode toggle" to a React app
4. **Bug fix**: "Fix the failing test in tests/auth.test.ts" on a repo with a known bug

---

## 9. Non-Goals (v1)

- No web UI (ProjectPulse handles that)
- No sprint/kanban management (ProjectPulse handles that)
- No git operations (user manages git)
- No deployment (user deploys manually)
- No parallel agent execution (Sequential only for v1)
- No custom agent configuration (hardcoded agent prompts)
- No code review agent (future addition)

---

## 10. Future Enhancements (Post-PoC)

- **Parallel exploration**: Multiple Explorer agents scanning different parts of a large codebase
- **Code review agent**: Reviews Coder's output before Tester runs
- **Git integration**: Auto-commit, branch creation, PR submission
- **ProjectPulse integration**: Read ticket context from ProjectPulse, report progress back
- **Custom agent prompts**: User-configurable system prompts per agent
- **Multi-language support**: Detect and adapt to Python, Go, Java, etc. (v1 focuses on JS/TS)
- **EmbeddingProvider**: Code-specific embeddings for better perception (vs generic all-MiniLM-L6-v2)

---

*This specification is the product definition for implementation. The PulseHive SDK (v1.0) provides all primitives needed — DevStudio only implements Tools, Agents, and the CLI wrapper.*
