# Skill: Branch Delta Review

A portable code review skill that runs identically across Claude Code, Codex, and Gemini CLI.

## Purpose

Perform a structured review of a branch delta (git diff or pull request) against:
- Shared review policy (`shared/review-policy.md`)
- Project architecture rules (`shared/architecture-rules.md`)
- Optional Jira issue context

## Inputs

| Input           | Required | Description                                              |
|-----------------|----------|----------------------------------------------------------|
| `BRANCH_DIFF`   | Yes      | Output of `git diff main...HEAD` or a PR diff            |
| `BASE_BRANCH`   | No       | Base branch name (defaults to `main`)                    |
| `FEATURE_BRANCH`| No       | Feature branch name                                      |
| `JIRA_ISSUE`    | No       | Jira issue content (title, description, acceptance criteria) |
| `CHANGED_FILES` | No       | List of files changed (from `git diff --name-only`)      |

## Outputs

Structured review report following `shared/review-policy.md` output format:

- Summary
- Bug Findings (with severity, file, line)
- Architecture Concerns (with rule references)
- Risky Changes (with risk and mitigation)
- Recommendation (APPROVE / REQUEST_CHANGES / NEEDS_DISCUSSION)

## Usage

### Direct Prompt

Give your agent the following instruction, with actual diff content substituted:

```
Load shared/review-policy.md and shared/architecture-rules.md.
Then perform a branch delta review using skills/branch-delta-review/prompt.md.
The diff is: [paste diff here]
```

### Via Adapter

Each adapter in `/adapters` exposes a pre-configured invocation of this skill.
See the adapter-specific README for agent-native usage syntax.

### Via Demo Script

```bash
./run-demo.sh
```

The demo script runs this skill against the example diff in `examples/demo-repo/`.

## Skill Configuration

See `config.yaml` for configurable parameters.

## Portability

This skill contains no agent-specific syntax. It works with any agent that can:
1. Read files from the filesystem or via MCP filesystem tools
2. Follow a structured prompt template
3. Produce structured text output

Tested with:
- Claude Code (via `adapters/claude/`)
- Codex (via `adapters/codex/`)
- Gemini CLI (via `adapters/gemini/`, future-ready)
