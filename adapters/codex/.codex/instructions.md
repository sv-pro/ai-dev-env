# AI Dev Env — Codex Configuration

You are operating inside the AI Dev Env portable agent environment.

Load the following shared context at startup:
- `shared/instructions.md` — operational instructions
- `shared/review-policy.md` — review standards
- `shared/architecture-rules.md` — architecture constraints

## Available Workflows

### review — Branch Delta Review

Perform a structured code review of a branch delta.

Usage examples the user may provide:
- "review this branch"
- "review feature/my-branch against main"
- "review this PR, Jira ticket is PROJ-123"

When asked to perform a review:

1. Determine the base branch (default: `main`) and feature branch (default: current branch).

2. Run the following shell commands to gather inputs:
   ```
   git diff <base>...<feature>
   git diff --name-only <base>...<feature>
   ```

3. If a Jira ticket is mentioned, fetch it using the `jira-readonly` MCP tool `get_issue`.

4. Load the prompt template at `skills/branch-delta-review/prompt.md`.

5. Substitute template variables:
   - `{{BASE_BRANCH}}` — base branch name
   - `{{FEATURE_BRANCH}}` — feature branch name
   - `{{BRANCH_DIFF}}` — git diff output
   - `{{CHANGED_FILES}}` — git diff --name-only output
   - `{{JIRA_ISSUE}}` — Jira issue text, or empty

6. Execute the review and output the structured report:
   - Summary
   - Bug Findings
   - Architecture Concerns
   - Risky Changes
   - Recommendation

## Operational Principles

Follow all principles in `shared/instructions.md`:
- Be precise: make the smallest change that solves the problem
- Be transparent: explain findings clearly
- Be consistent: apply policy uniformly
- Be conservative: prefer well-understood patterns
