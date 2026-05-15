# AI Dev Env — Claude Code Configuration

This file configures Claude Code to operate within the AI Dev Env portable environment.
Place this file at the root of your project as `CLAUDE.md`.

## Environment

You are operating inside the AI Dev Env portable agent environment.

Load the following shared context at startup:
- `shared/instructions.md` — operational instructions
- `shared/review-policy.md` — review standards
- `shared/architecture-rules.md` — architecture constraints

## Available Skills

### /review — Branch Delta Review

Perform a structured code review of a branch delta.

**Usage:**
```
/review
/review --branch feature/my-branch
/review --base main --branch feature/my-branch
/review --jira PROJ-123
```

**What it does:**
1. Runs `git diff {{BASE}}...{{BRANCH}}` to get the delta
2. Loads `shared/review-policy.md` and `shared/architecture-rules.md`
3. Loads the Jira issue if `--jira` is provided
4. Executes the prompt in `skills/branch-delta-review/prompt.md`
5. Outputs a structured review: Summary, Bug Findings, Architecture Concerns, Risky Changes, Recommendation

**Implementation:**

When the user types `/review`, execute the following steps:

1. Determine base and feature branches:
   - If `--base` is provided, use it; otherwise use `main`
   - If `--branch` is provided, use it; otherwise use the current git branch

2. Run:
   ```bash
   git diff {{BASE}}...{{BRANCH}}
   git diff --name-only {{BASE}}...{{BRANCH}}
   ```

3. If `--jira TICKET` is provided and Jira MCP is available, fetch the issue:
   - Use the `get_issue` tool from the `jira-readonly` MCP server

4. Load the prompt template from `skills/branch-delta-review/prompt.md`

5. Substitute the following template variables:
   - `{{BASE_BRANCH}}` → base branch name
   - `{{FEATURE_BRANCH}}` → feature branch name
   - `{{BRANCH_DIFF}}` → output of git diff
   - `{{CHANGED_FILES}}` → output of git diff --name-only
   - `{{JIRA_ISSUE}}` → Jira issue content, or empty string

6. Execute the populated prompt and produce the structured review output.

## MCP Servers

When available, the following MCP servers are registered:

- `github-readonly` — Read-only GitHub access (see `mcp/github-readonly/`)
- `jira-readonly` — Read-only Jira access (see `mcp/jira-readonly/`)

## Operational Principles

Follow all principles in `shared/instructions.md`:
- Be precise: make the smallest change that solves the problem
- Be transparent: explain findings clearly
- Be consistent: apply policy uniformly
- Be conservative: prefer well-understood patterns
