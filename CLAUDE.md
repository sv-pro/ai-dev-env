# AI Dev Env

This is the AI Dev Env repository — a portable operational environment for coding agents.

## Project Architecture

This project follows the layered architecture defined in `shared/architecture-rules.md`:

```
┌─────────────────────────┐
│        Adapters         │  Agent-specific entry points (/adapters/)
├─────────────────────────┤
│         Skills          │  Portable workflow logic (/skills/)
├─────────────────────────┤
│         Shared          │  Context, policy, conventions (/shared/)
├─────────────────────────┤
│          MCP            │  Tool access declarations (/mcp/)
└─────────────────────────┘
```

**Layer rules:**
- Adapters may reference Skills and Shared. They must not contain workflow logic.
- Skills may reference Shared. They must not contain agent-specific code.
- Shared has no dependencies on other layers.
- MCP configs are declarative — no secrets, no embedded logic.

## Shared Context

Read these files before starting any task in this project:

- `shared/instructions.md` — Operational principles
- `shared/review-policy.md` — Review standards and severity levels
- `shared/architecture-rules.md` — Architecture constraints to enforce

## Available Skills

### /review — Branch Delta Review

Reviews a branch diff against the project's shared review policy and architecture rules.

**Usage:**
```
/review
/review --branch feature/my-branch
/review --base main --branch feature/my-branch
/review --jira PROJ-123
```

Implemented as a native Claude Code command in `.claude/commands/review.md`.
Uses the prompt template at `skills/branch-delta-review/prompt.md` and context from `shared/`.

## Working on This Project

When adding or modifying files in this project:

1. **Layer boundaries** — Never put agent-specific syntax in `shared/` or `skills/`. That belongs in `adapters/` only (Rule P-1).
2. **New skill** — Create `skills/<skill-name>/config.yaml`, `prompt.md`, and `README.md`. Register it in each adapter.
3. **New adapter** — Create `adapters/<agent-name>/` with the agent's entry-point file and map skill names to agent commands.
4. **MCP config** — Add to `mcp/<server-name>/config.json`. Use env var references, never literal secrets (Rule S-1).
5. **Prompt templates** — Use `{{UPPER_SNAKE_CASE}}` placeholders for dynamic content (Rule P-2).

Run `/review` before merging any branch to validate changes against the shared review policy.

## MCP Servers

Optional — enhance this environment when available:

- `github-readonly` — Read-only GitHub access
- `jira-readonly` — Read-only Jira access (enables `/review --jira <ticket>`)

See `adapters/claude/mcp-settings.json` for the registration format.
Copy the `mcpServers` block into your `~/.claude/settings.json` to activate.
