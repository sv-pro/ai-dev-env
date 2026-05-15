# Gemini CLI Adapter (Future-Ready)

This adapter configures Gemini CLI to operate as a runtime within the AI Dev Env
portable environment.

## Status

**Future-ready.** This adapter is prepared for Gemini CLI once it supports:
- File-based instruction loading (e.g., `GEMINI.md`)
- MCP server registration
- Structured workflow invocation

The adapter files are complete and will work as Gemini CLI's configuration
capabilities mature.

## How It Works

Gemini CLI reads `GEMINI.md` from the project root on startup (once supported).
This adapter provides that file, loading shared environment context and mapping
the branch-delta-review skill to Gemini's interface.

## Setup (When Available)

1. Copy `GEMINI.md` to your project root:
   ```bash
   cp adapters/gemini/GEMINI.md ./GEMINI.md
   ```

2. (Optional) Register MCP servers via Gemini CLI settings:
   ```bash
   # Refer to Gemini CLI documentation for MCP registration syntax
   gemini mcp add github-readonly ...
   ```

3. Start Gemini CLI:
   ```bash
   gemini
   ```

## Skill Mapping

| AI Dev Env Skill          | Gemini CLI Equivalent     |
|---------------------------|---------------------------|
| `branch-delta-review`     | "review" prompt           |

## Manual Usage (Available Now)

Even without native adapter support, you can use Gemini CLI manually:

```bash
# Get the diff
git diff main...HEAD > /tmp/branch.diff

# Start Gemini CLI and paste the following prompt:
gemini
```

Then paste the contents of `skills/branch-delta-review/prompt.md` with the diff substituted in.

## MCP Configuration

See `mcp-settings.json` for the expected Gemini MCP configuration format.
