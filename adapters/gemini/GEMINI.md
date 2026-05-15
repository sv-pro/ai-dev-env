# AI Dev Env — Gemini CLI Configuration

You are operating inside the AI Dev Env portable agent environment.

Load the following shared context at startup:
- `shared/instructions.md` — operational instructions
- `shared/review-policy.md` — review standards
- `shared/architecture-rules.md` — architecture constraints

## Available Skills

### Branch Delta Review

Perform a structured code review of a branch delta.

When asked to review a branch or diff:

1. Determine the base branch (default: `main`) and feature branch (default: current branch).

2. Collect the diff:
   ```
   git diff <base>...<feature>
   git diff --name-only <base>...<feature>
   ```

3. If a Jira ticket is mentioned, fetch it via the `jira-readonly` MCP tool.

4. Load the prompt template at `skills/branch-delta-review/prompt.md`.

5. Substitute template variables:
   - `{{BASE_BRANCH}}` — base branch name
   - `{{FEATURE_BRANCH}}` — feature branch name
   - `{{BRANCH_DIFF}}` — git diff output
   - `{{CHANGED_FILES}}` — changed file list
   - `{{JIRA_ISSUE}}` — Jira issue content, or empty

6. Execute the review and produce the structured output.

## Operational Principles

- Be precise: make the smallest change that solves the problem
- Be transparent: explain findings clearly
- Be consistent: apply policy uniformly
- Be conservative: prefer well-understood patterns
