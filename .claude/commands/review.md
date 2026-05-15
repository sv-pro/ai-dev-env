Perform a structured branch delta review using the AI Dev Env branch-delta-review skill.

**Arguments:** $ARGUMENTS

## Step 1 — Parse arguments

From the arguments above, extract:
- `--base <branch>` → BASE_BRANCH (default: `main`)
- `--branch <branch>` → FEATURE_BRANCH (default: current git branch)
- `--jira <ticket>` → JIRA_TICKET (optional, e.g. `PROJ-123`)

If `--branch` is not provided, run `git rev-parse --abbrev-ref HEAD` to determine the current branch.

## Step 2 — Collect git data

Run these commands (substituting actual branch names):
```bash
git diff <BASE_BRANCH>...<FEATURE_BRANCH>
git diff --name-only <BASE_BRANCH>...<FEATURE_BRANCH>
```

## Step 3 — Load context

Read the following files:
- `shared/review-policy.md` — severity levels, required checks, output format
- `shared/architecture-rules.md` — layer rules, portability rules (P-1/P-2/P-3), security rules (S-1/S-2/S-3), simplicity rules (K-1/K-2/K-3)

If `--jira <ticket>` was provided and the `jira-readonly` MCP server is available, fetch the issue using its `get_issue` tool and use the result as JIRA_ISSUE context.

## Step 4 — Execute the review

Read `skills/branch-delta-review/prompt.md`. Use it as your review prompt, substituting these placeholders with the collected data:
- `{{BASE_BRANCH}}` → base branch name
- `{{FEATURE_BRANCH}}` → feature branch name
- `{{BRANCH_DIFF}}` → full output of `git diff`
- `{{CHANGED_FILES}}` → output of `git diff --name-only`
- `{{JIRA_ISSUE}}` → Jira issue content, or empty string if not provided

Produce the complete structured review in the format defined in `shared/review-policy.md`.
