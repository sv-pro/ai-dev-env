# Demo Repository

This example demonstrates the branch-delta-review skill running against a real
(simulated) branch delta. It shows the same review output you'd get from
Claude Code, Codex, or Gemini CLI using the AI Dev Env environment.

## Contents

```
examples/demo-repo/
  src/
    auth.js          # Sample source file with intentional issues
    user-service.js  # Sample source file
  reviews/
    sample-review.md # Sample review output (what the agent produces)
  branch.diff        # The diff being reviewed
  README.md          # This file
```

## Running the Demo

From the repository root:

```bash
./run-demo.sh
```

The script:
1. Loads the sample diff from `examples/demo-repo/branch.diff`
2. Populates the `branch-delta-review` prompt template
3. Prints the populated prompt (so you can paste it into any agent)
4. Shows the expected output from `examples/demo-repo/reviews/sample-review.md`

## What the Demo Shows

The sample diff introduces two intentional issues:

1. **A bug**: Missing input validation in `auth.js` (SQL injection vector)
2. **An architecture violation**: Direct database access from a controller,
   bypassing the service layer (violates architecture-rules.md layer boundaries)

The sample review in `reviews/sample-review.md` shows how the agent catches
both issues and formats the output per `shared/review-policy.md`.

## Trying It With a Real Agent

### Claude Code
```bash
cp adapters/claude/CLAUDE.md ./CLAUDE.md
claude
# Then type: /review
```

### Codex
```bash
cp -r adapters/codex/.codex ./
codex
# Then say: "review this branch"
```

### Manual (any agent)
```bash
# Generate the populated prompt
./run-demo.sh --prompt-only

# Copy the output and paste into your agent of choice
```
