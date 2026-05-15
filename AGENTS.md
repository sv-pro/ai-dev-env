# AI Dev Env — Codex Runtime Instructions

This repository is using AI Dev Env on itself. Treat Codex as the active
runtime and use the shared portable context as the durable source of policy.

Before starting work, load these files when they exist:
- `shared/instructions.md`
- `shared/review-policy.md`
- `shared/architecture-rules.md`

For branch or diff reviews, use the `branch-delta-review` workflow:

1. Determine the base branch, defaulting to `main`, and the feature branch,
   defaulting to the current branch.
2. Gather the review inputs:
   ```bash
   git diff <base>...<feature>
   git diff --name-only <base>...<feature>
   ```
3. If a Jira ticket is provided and Jira MCP is available, fetch the issue as
   additional context.
4. Load `skills/branch-delta-review/prompt.md`.
5. Substitute the prompt placeholders with the collected inputs.
6. Produce the review in the format required by `shared/review-policy.md`.

Follow the repository's portability rules:
- Keep shared files agent-neutral.
- Keep agent-specific instructions in adapter files or root runtime entry
  points such as this file.
- Do not commit secrets. Reference credentials through environment variables.
- Prefer small, local-first changes that preserve the layered structure.
