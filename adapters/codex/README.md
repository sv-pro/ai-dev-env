# Codex Adapter

This adapter configures Codex to operate as a runtime within the AI Dev Env
portable environment.

## How It Works

Codex reads `.codex/instructions.md` from the project root on startup. This adapter
provides that file, loading shared environment context and mapping the
branch-delta-review skill to Codex's workflow system.

## Setup

1. Copy the `.codex` directory to your project root:
   ```bash
   cp -r adapters/codex/.codex ./
   ```

2. (Optional) Set environment variables for MCP integration:
   ```bash
   export GITHUB_TOKEN=ghp_your_token_here
   export JIRA_URL=https://yourorg.atlassian.net
   export JIRA_EMAIL=you@yourorg.com
   export JIRA_API_TOKEN=your_token_here
   ```

3. Start Codex:
   ```bash
   codex
   ```

## Available Workflows

Once Codex is running with this adapter:

| Workflow                  | Description                            |
|---------------------------|----------------------------------------|
| `review`                  | Review the current branch against main |
| `review --branch <name>`  | Review a specific branch               |
| `review --jira <ticket>`  | Review with Jira issue context         |

## Skill Mapping

| AI Dev Env Skill          | Codex Equivalent          |
|---------------------------|---------------------------|
| `branch-delta-review`     | `review` workflow         |

## MCP Configuration

See `.codex/mcp.json` for the Codex MCP server configuration format.
