# GitHub Read-Only MCP Server

This MCP server provides read-only access to GitHub repositories for use during
branch delta reviews and code analysis tasks.

## Setup

1. Create a GitHub Personal Access Token with `read:repo` scope at
   https://github.com/settings/tokens

2. Export the token as an environment variable:
   ```bash
   export GITHUB_TOKEN=ghp_your_token_here
   ```

3. Register this MCP server with your agent (see adapter-specific instructions).

## Available Tools

| Tool                    | Description                              |
|-------------------------|------------------------------------------|
| `get_file_contents`     | Read a file from a repository            |
| `search_repositories`   | Search GitHub repositories               |
| `get_issue`             | Get details of a specific issue          |
| `list_issues`           | List issues in a repository              |
| `get_pull_request`      | Get details of a pull request            |
| `list_pull_requests`    | List pull requests in a repository       |
| `get_pull_request_diff` | Get the diff for a pull request          |
| `get_pull_request_files`| List files changed in a pull request     |
| `list_commits`          | List commits in a repository             |
| `get_commit`            | Get details of a specific commit         |
| `search_code`           | Search code across GitHub                |

## Security Notes

- This configuration is read-only. Write tools are explicitly denied.
- Never commit your `GITHUB_TOKEN` to the repository.
- Use the minimum scope required (`read:repo` for private, `public_repo` for public).
