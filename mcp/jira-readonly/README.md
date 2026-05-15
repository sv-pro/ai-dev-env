# Jira Read-Only MCP Server

This MCP server provides read-only access to Jira issues for use during
branch delta reviews. It allows the agent to load issue context (title,
description, acceptance criteria) when a Jira ticket number is provided.

## Setup

1. Create an Atlassian API token at
   https://id.atlassian.com/manage-profile/security/api-tokens

2. Export the required environment variables:
   ```bash
   export JIRA_URL=https://yourorg.atlassian.net
   export JIRA_EMAIL=you@yourorg.com
   export JIRA_API_TOKEN=your_token_here
   ```

3. Register this MCP server with your agent (see adapter-specific instructions).

## Available Tools

| Tool                | Description                                    |
|---------------------|------------------------------------------------|
| `get_issue`         | Fetch a Jira issue by key (e.g., PROJ-123)     |
| `search_issues`     | Search Jira issues using JQL                   |
| `get_project`       | Get project details                            |
| `list_projects`     | List available projects                        |
| `get_sprint`        | Get sprint details                             |
| `get_issue_comments`| Get comments on a Jira issue                   |

## Usage in Reviews

When running a branch delta review with a Jira ticket:

```
Review branch feature/PROJ-123-add-auth against main.
Load the Jira issue PROJ-123 for context.
```

The agent will fetch the issue details and use them to validate that the
implementation matches the requirements.

## Security Notes

- This configuration is read-only. Write tools are explicitly denied.
- Never commit your `JIRA_API_TOKEN` to the repository.
- Jira integration is optional — reviews work fine without it.
