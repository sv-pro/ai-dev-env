# AI Dev Env

> A portable operational environment for coding agents.

## What Is This?

**AI Dev Env** is a portability layer that lets you run the same workflows, context, and review policies across different coding agent runtimes — Claude Code, Codex, Gemini CLI, and others — without rebuilding your operational infrastructure each time.

This is **NOT**:
- a new foundation model
- a new IDE
- a new coding agent

This **IS**:
- a portability layer
- operational workflow infrastructure
- shared agent context system
- reusable review flows

## The Problem

Vendor ecosystems are fragmenting:
- Claude Skills
- Codex workflows
- Gemini context systems
- Cursor rules
- custom prompts per tool

But [MCP (Model Context Protocol)](https://modelcontextprotocol.io/) is becoming a shared substrate.

**The hypothesis:** MCP can evolve from a tool transport layer into a portable operational context layer.

## The Solution

Externalize workflows, memory, context, and conventions so that different coding agents become interchangeable runtimes.

```
same workflow + same context + same review policy = interchangeable agents
```

Think of it like:
- **devcontainers** — but for agent environments
- **shared CI workflows** — but for review and coding tasks
- **Dockerized dev environments** — but for operational agent context

## Repository Structure

```
/shared
  instructions.md        # Universal agent instructions
  review-policy.md       # Shared review policy
  architecture-rules.md  # Project architecture conventions

/skills
  branch-delta-review/   # Portable branch review skill
    README.md            # Skill documentation
    prompt.md            # Reusable prompt template
    config.yaml          # Skill configuration

/mcp
  github-readonly/       # MCP config for GitHub access
  jira-readonly/         # MCP config for Jira access

/adapters
  claude/                # Claude Code adapter
  codex/                 # Codex adapter
  gemini/                # Gemini CLI adapter (future-ready)

/examples
  demo-repo/             # Runnable demo with sample output
```

## Quick Start

### Run the Demo

```bash
# Clone the repo
git clone https://github.com/sv-pro/ai-dev-env
cd ai-dev-env

# Run the demo review flow
./run-demo.sh
```

### Use With Claude Code

Point Claude Code at the Claude adapter:

```bash
cp adapters/claude/CLAUDE.md ./CLAUDE.md
# Claude Code reads CLAUDE.md automatically on startup
claude
```

### Use With Codex

```bash
# Codex reads .codex/instructions.md
cp -r adapters/codex/.codex ./
codex
```

### Use With Gemini CLI

```bash
# Gemini reads GEMINI.md
cp adapters/gemini/GEMINI.md ./GEMINI.md
gemini
```

## The Branch Delta Review Workflow

The primary demonstration workflow is **branch-delta-review**: a standardized code review skill that works identically across all supported agents.

**Input:**
- git diff / branch delta
- optional Jira issue context
- shared architecture conventions

**Output:**
- bug findings
- architecture concerns
- risky changes
- review summary

See [`skills/branch-delta-review/README.md`](skills/branch-delta-review/README.md) for full documentation.

## Design Principles

### Treat Agents as Interchangeable Runtimes

The durable layer is your operational workflows, shared context, and reusable skills — **not** the vendor-specific agent.

### AI Aikido

Use vendor ecosystems against vendor lock-in. Don't fight Claude Skills or Codex environments — virtualize them behind portable operational workflows.

### Keep It Simple

- Lightweight and local-first
- No giant orchestration systems
- No distributed runtimes
- No complicated governance engines
- Reproducible and understandable

## License

MIT