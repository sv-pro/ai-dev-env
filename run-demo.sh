#!/usr/bin/env bash
#
# run-demo.sh — AI Dev Env Demo Runner
#
# Demonstrates the branch-delta-review skill by:
# 1. Loading the sample diff from examples/demo-repo/branch.diff
# 2. Populating the branch-delta-review prompt template
# 3. Printing the populated prompt (paste into any agent)
# 4. Showing the expected review output
#
# Usage:
#   ./run-demo.sh                # Full demo (prompt + expected output)
#   ./run-demo.sh --prompt-only  # Print only the populated prompt
#   ./run-demo.sh --output-only  # Print only the expected review output
#

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

PROMPT_TEMPLATE="$REPO_ROOT/skills/branch-delta-review/prompt.md"
DIFF_FILE="$REPO_ROOT/examples/demo-repo/branch.diff"
REVIEW_OUTPUT="$REPO_ROOT/examples/demo-repo/reviews/sample-review.md"
REVIEW_POLICY="$REPO_ROOT/shared/review-policy.md"
ARCH_RULES="$REPO_ROOT/shared/architecture-rules.md"

PROMPT_ONLY=false
OUTPUT_ONLY=false

for arg in "$@"; do
  case "$arg" in
    --prompt-only) PROMPT_ONLY=true ;;
    --output-only) OUTPUT_ONLY=true ;;
    --help|-h)
      echo "Usage: $0 [--prompt-only | --output-only]"
      exit 0
      ;;
    *)
      echo "Unknown argument: $arg"
      echo "Usage: $0 [--prompt-only | --output-only]"
      exit 1
      ;;
  esac
done

# ── Verify required files exist ──────────────────────────────────────────────

missing=0
for f in "$PROMPT_TEMPLATE" "$DIFF_FILE" "$REVIEW_OUTPUT" "$REVIEW_POLICY" "$ARCH_RULES"; do
  if [[ ! -f "$f" ]]; then
    echo "ERROR: Required file not found: $f" >&2
    missing=1
  fi
done
if [[ "$missing" -eq 1 ]]; then
  echo "Run this script from the repository root: ./run-demo.sh" >&2
  exit 1
fi

# ── Load inputs ──────────────────────────────────────────────────────────────

BRANCH_DIFF=$(cat "$DIFF_FILE")
BASE_BRANCH="main"
FEATURE_BRANCH="feature/add-auth"
CHANGED_FILES="src/auth.js
src/user-controller.js"
JIRA_ISSUE=""

export _BRANCH_DIFF="$BRANCH_DIFF"
export _BASE_BRANCH="$BASE_BRANCH"
export _FEATURE_BRANCH="$FEATURE_BRANCH"
export _CHANGED_FILES="$CHANGED_FILES"
export _JIRA_ISSUE="$JIRA_ISSUE"

# ── Populate the prompt template ─────────────────────────────────────────────

# Use Python for safe multi-line string substitution
POPULATED_PROMPT=$(python3 - <<'PYEOF'
import sys

with open("skills/branch-delta-review/prompt.md") as f:
    template = f.read()

import os
replacements = {
    "{{BASE_BRANCH}}":    os.environ.get("_BASE_BRANCH", ""),
    "{{FEATURE_BRANCH}}": os.environ.get("_FEATURE_BRANCH", ""),
    "{{CHANGED_FILES}}":  os.environ.get("_CHANGED_FILES", ""),
    "{{JIRA_ISSUE}}":     os.environ.get("_JIRA_ISSUE", ""),
    "{{BRANCH_DIFF}}":    os.environ.get("_BRANCH_DIFF", ""),
}

for key, value in replacements.items():
    template = template.replace(key, value)

print(template, end="")
PYEOF
)

# ── Output ───────────────────────────────────────────────────────────────────

hr() {
  printf '%0.s─' {1..72}
  echo
}

if [[ "$OUTPUT_ONLY" == false ]]; then
  echo ""
  echo "╔══════════════════════════════════════════════════════════════════════╗"
  echo "║              AI Dev Env — Branch Delta Review Demo                  ║"
  echo "╚══════════════════════════════════════════════════════════════════════╝"
  echo ""
  echo "This demo shows the branch-delta-review skill in action."
  echo "The same prompt and output work across Claude Code, Codex, and Gemini CLI."
  echo ""
  hr
  echo "STEP 1: Populated Prompt (paste this into your agent)"
  hr
  echo ""
  echo "$POPULATED_PROMPT"
  echo ""
fi

if [[ "$PROMPT_ONLY" == false ]]; then
  hr
  echo "STEP 2: Expected Review Output"
  echo "(This is what the agent should produce)"
  hr
  echo ""
  cat "$REVIEW_OUTPUT"
  echo ""
fi

if [[ "$OUTPUT_ONLY" == false && "$PROMPT_ONLY" == false ]]; then
  hr
  echo ""
  echo "To use with a real agent:"
  echo ""
  echo "  Claude Code:  cp adapters/claude/CLAUDE.md ./CLAUDE.md && claude"
  echo "  Codex:        cp -r adapters/codex/.codex ./ && codex"
  echo "  Gemini CLI:   cp adapters/gemini/GEMINI.md ./GEMINI.md && gemini"
  echo ""
  echo "  Manual:       ./run-demo.sh --prompt-only | pbcopy   (then paste into agent)"
  echo ""
fi
