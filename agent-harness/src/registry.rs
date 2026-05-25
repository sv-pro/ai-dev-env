use rmcp::model::Tool;
use serde_json::{json, Map, Value};

/// Build the list of MCP tool descriptors.
pub fn all_tools() -> Vec<Tool> {
    vec![
        tool(
            "set_task",
            "Activate a named task. Must be called before any other tool.",
            json!({
                "type": "object",
                "properties": {
                    "task": { "type": "string", "description": "Task name (e.g. triage, refactor, review)" }
                },
                "required": ["task"]
            }),
        ),
        tool(
            "search",
            "Recursively search file contents for a query string.",
            json!({
                "type": "object",
                "properties": {
                    "query": { "type": "string", "description": "String to search for" },
                    "path":  { "type": "string", "description": "Directory to search (default: working_dir)" }
                },
                "required": ["query"]
            }),
        ),
        tool(
            "read_file",
            "Read the contents of a file.",
            json!({
                "type": "object",
                "properties": {
                    "path": { "type": "string", "description": "File path to read" }
                },
                "required": ["path"]
            }),
        ),
        tool(
            "write_file",
            "Write content to a file (subject to policy).",
            json!({
                "type": "object",
                "properties": {
                    "path":    { "type": "string", "description": "File path to write" },
                    "content": { "type": "string", "description": "Content to write" }
                },
                "required": ["path", "content"]
            }),
        ),
        tool(
            "shell",
            "Run a shell command in the working directory (subject to policy).",
            json!({
                "type": "object",
                "properties": {
                    "command": { "type": "string", "description": "Shell command to run" },
                    "path":    { "type": "string", "description": "Working directory override" }
                },
                "required": ["command"]
            }),
        ),
        tool(
            "git_log",
            "Show the last N git commits.",
            json!({
                "type": "object",
                "properties": {
                    "n": { "type": "integer", "description": "Number of commits (default: 10)" }
                },
                "required": []
            }),
        ),
        tool(
            "git_diff",
            "Show git diff against a ref (branch, commit, or HEAD).",
            json!({
                "type": "object",
                "properties": {
                    "ref": { "type": "string", "description": "Git ref to diff against (default: HEAD)" }
                },
                "required": []
            }),
        ),
    ]
}

fn tool(name: &'static str, description: &'static str, schema: serde_json::Value) -> Tool {
    let map: Map<String, Value> = schema
        .as_object()
        .cloned()
        .unwrap_or_default();
    Tool::new(name, description, map)
}
