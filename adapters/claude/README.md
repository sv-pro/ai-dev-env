# Claude Code Adapter

This adapter configures Claude Code to operate as a runtime within the AI Dev Env
portable environment.

## How It Works

Claude Code reads `CLAUDE.md` from the project root on startup. This adapter
provides a `CLAUDE.md` that loads the shared environment context and exposes
the branch-delta-review skill as a Claude slash command.

## Setup

1. Copy `CLAUDE.md` to your project root:
   ```bash
   cp adapters/claude/CLAUDE.md ./CLAUDE.md
   ```

2. (Optional) Register MCP servers in your Claude settings:
   ```bash
   # Add the GitHub read-only MCP server
   claude mcp add github-readonly npx @modelcontextprotocol/server-github \
     --env GITHUB_PERSONAL_ACCESS_TOKEN=$GITHUB_TOKEN
   ```

3. Start Claude Code:
   ```bash
   claude
   ```

## Available Commands

Once Claude Code is running with this adapter:

| Command                              | Description                            |
|--------------------------------------|----------------------------------------|
| `/review`                            | Review the current branch against main |
| `/review --branch feature/my-branch` | Review a specific branch               |
| `/review --jira PROJ-123`            | Review with Jira issue context         |

## Skill Mapping

| AI Dev Env Skill          | Claude Code Equivalent |
|---------------------------|------------------------|
| `branch-delta-review`     | `/review` command      |

## MCP Configuration

See `mcp-settings.json` for the Claude Code MCP server registration format.
Copy relevant sections to your Claude settings file (`~/.claude/settings.json`).
