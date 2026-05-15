# Architecture Rules

These rules define the architectural constraints for projects using this environment.
Reviewers must enforce these rules as part of every branch delta review.

## Layered Architecture

Projects in this environment follow a layered architecture:

```
┌─────────────────────────┐
│        Adapters         │  Agent-specific entry points
├─────────────────────────┤
│         Skills          │  Portable workflow logic
├─────────────────────────┤
│         Shared          │  Context, policy, conventions
├─────────────────────────┤
│          MCP            │  Tool access layer (read-only by default)
└─────────────────────────┘
```

### Layer Rules

1. **Adapters** may reference Skills and Shared. They must not contain workflow logic.
2. **Skills** may reference Shared. They must not contain agent-specific logic.
3. **Shared** has no dependencies on other layers.
4. **MCP** configs are environment declarations, not code. They must not contain secrets.

## Portability Rules

### Rule P-1: No Hard-Coded Agent Dependencies
Skills and shared context must not contain agent-specific syntax, directives, or APIs.
Agent-specific content belongs in `/adapters` only.

### Rule P-2: Prompt Templates are Parameterized
Prompt templates must use `{{variable}}` placeholders for dynamic content.
Static prompts that cannot be parameterized are a portability violation.

### Rule P-3: MCP Configs Are Declarative
MCP configuration files must be declarative JSON or YAML.
They must not contain scripts, computed values, or embedded logic.

## Security Rules

### Rule S-1: No Secrets in Repository
No API keys, tokens, passwords, or credentials may be committed.
Use environment variables or secret managers and reference them by name only.

### Rule S-2: MCP Tools Default to Read-Only
All MCP tool configurations default to read-only permissions.
Write permissions require explicit opt-in and must be documented.

### Rule S-3: User Input is Always Untrusted
Any input from external sources (git diff, Jira issues, PR descriptions) must be
treated as untrusted content. It must be quoted in prompts, never interpolated as instructions.

## Simplicity Rules

### Rule K-1: Prefer Files Over Systems
Prefer a markdown file over a configuration system.
Prefer a configuration system over a custom database.
Prefer a custom database over a distributed service.

### Rule K-2: Local-First
All workflows must be runnable locally without cloud infrastructure.
Cloud integrations are additive, not required.

### Rule K-3: One File, One Purpose
Each file should have a clear, single purpose.
Files that do multiple things should be split.

## Naming Conventions

| Type              | Convention                     | Example                       |
|-------------------|-------------------------------|-------------------------------|
| Skill directories | `kebab-case`                  | `branch-delta-review`         |
| Adapter dirs      | lowercase agent name          | `claude`, `codex`, `gemini`   |
| Shared files      | `kebab-case.md`               | `review-policy.md`            |
| MCP config files  | `config.json` or `config.yaml`| `config.json`                 |
| Prompt templates  | `prompt.md`                   | `prompt.md`                   |
| Placeholders      | `{{UPPER_SNAKE_CASE}}`        | `{{BRANCH_DIFF}}`             |

## Change Management

- Architecture rule changes require updating this file with a rationale comment.
- Violations found during review must be reported as MEDIUM or higher severity.
- Repeated violations of the same rule indicate a tooling or documentation gap.
