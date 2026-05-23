# Agent Harness

> **Agent Hypervisor Principle:** We do not make agents safe. We make the world they live in safe.

This is a compiled MCP server (stdio transport) that acts as an enforcement layer for AI agents.
Every tool call is validated against the active task's policy before execution.

## First Step — Always

**You must call `set_task` before any other tool.** All other tools are rejected until an active
task is established.

```
set_task(task: "triage")
```

## Available Tasks

| Task | Description | allow_write | max_steps |
|------|-------------|-------------|-----------|
| `triage` | Read-only investigation — search code, read files, inspect git history | false | 15 |
| `refactor` | Read and write source files under `src/` and `tests/`. Write ops require approval | true | 30 |
| `review` | Read-only code review — search, read files, inspect diffs | false | 20 |

## Tools

| Tool | Description |
|------|-------------|
| `set_task` | Activate a named task (bypasses policy — must be called first) |
| `search` | Recursive grep-like file content search |
| `read_file` | Read file contents |
| `write_file` | Write file contents (subject to allow_write and paths_whitelist) |
| `shell` | Run a shell command in the working directory (subject to paths_whitelist) |
| `git_log` | Show last N commits |
| `git_diff` | Show diff against a git ref |

## Policy Enforcement

The harness enforces these rules on every tool call (except `set_task`):

1. **Active task required** — no task, no tools
2. **Step budget** — rejected when step count ≥ max_steps
3. **Tool whitelist** — only tools listed in the task's `tools:` may be called
4. **Write gate** — `write_file` rejected if `allow_write: false`
5. **Path whitelist** — `write_file` and `shell` restricted to `paths_whitelist` paths
6. **Approval flag** — tools in `require_approval` are logged as warnings (auto-approved in dev mode)

## Configuration

Set `HARNESS_CONFIG` to the path of your `harness.yaml`. Defaults to `./harness.yaml`.

Build: `cargo build --release`
