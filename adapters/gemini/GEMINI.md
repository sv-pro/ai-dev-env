# AI Dev Env — Gemini CLI Configuration

This file configures the Gemini CLI to operate within the AI Dev Env portable environment.
This configuration is optimized for the Gemini CLI's specific capabilities and safety mandates.

## Environment

You are operating inside the AI Dev Env portable agent environment.

At the start of any session or when requested, load the following shared context:
- `shared/instructions.md` — operational instructions
- `shared/review-policy.md` — review standards
- `shared/architecture-rules.md` — architecture constraints

## Available Skills

### Branch Delta Review

Perform a structured code review of a branch delta using the project's review policy and architecture rules.

**Workflow:**

1. **Information Gathering**:
   - Determine the base branch (default: `main`) and feature branch (default: current HEAD).
   - Retrieve the diff and file list using `run_shell_command`:
     ```bash
     git diff {{BASE_BRANCH}}...{{FEATURE_BRANCH}}
     git diff --name-only {{BASE_BRANCH}}...{{FEATURE_BRANCH}}
     ```
   - If a Jira issue is referenced, use the `jira-readonly` MCP server to fetch issue details.

2. **Prompt Assembly**:
   - Read the template at `skills/branch-delta-review/prompt.md`.
   - Substitute placeholders:
     - `{{BASE_BRANCH}}`: The base branch name.
     - `{{FEATURE_BRANCH}}`: The branch being reviewed.
     - `{{BRANCH_DIFF}}`: The full output of the `git diff` command.
     - `{{CHANGED_FILES}}`: The output of `git diff --name-only`.
     - `{{JIRA_ISSUE}}`: Content from Jira, or "No Jira issue provided."

3. **Review Execution**:
   - Analyze the collected context following the steps in the prompt template.
   - Adhere strictly to `shared/review-policy.md` for severity and structure.
   - Adhere strictly to `shared/architecture-rules.md` for architectural validation.

4. **Output Generation**:
   - Output the final review report in the Markdown format specified in `shared/review-policy.md`.

## Operational Principles

Follow these principles, which align the project's goals with Gemini CLI's core mandates:

- **Be Precise (Standard R-1)**: Make the smallest change that solves the problem. Minimize context bloat.
- **Be Transparent (Standard R-2)**: Clearly explain findings. Use `grep_search` and `read_file` to provide evidence for your conclusions.
- **Be Consistent (Standard R-3)**: Apply the shared policies in `shared/` uniformly.
- **Be Conservative (Standard R-4)**: Prefer well-understood patterns. Do not bypass the type system or existing conventions.

## Tool Usage Guidelines

- **Context Retrieval**: Use `grep_search` to find rule references and `glob` to locate relevant files across layers.
- **Security**: Never use write-capable tools unless explicitly authorized. Protect secrets and credentials.
- **Efficiency**: Batch file reads and searches to minimize turns and context usage.

## MCP Servers

The following MCP servers are configured in `mcp-settings.json` and available for read-only operations:
- `github-readonly`: Repository history and PR data.
- `jira-readonly`: Issue tracking and requirements.
