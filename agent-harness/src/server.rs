use crate::{
    policy::PolicyEngine,
    registry::all_tools,
    tools::{git, search, shell},
};
use rmcp::{
    ServerHandler,
    model::{
        CallToolRequestParam, CallToolResult, Content, ErrorData, Implementation,
        ListToolsResult, PaginatedRequestParam, ProtocolVersion, ServerCapabilities, ServerInfo,
    },
    service::RequestContext,
    RoleServer,
};
use serde_json::{Map, Value};
use std::sync::Arc;
use tokio::{fs, sync::Mutex};

#[derive(Clone)]
pub struct HarnessServer {
    policy: Arc<Mutex<PolicyEngine>>,
}

impl HarnessServer {
    pub fn new(policy: PolicyEngine) -> Self {
        Self {
            policy: Arc::new(Mutex::new(policy)),
        }
    }

    async fn dispatch(&self, name: &str, args: &Map<String, Value>) -> Result<String, ErrorData> {
        match name {
            "set_task" => {
                let task = str_arg(args, "task")?;
                let mut engine = self.policy.lock().await;
                match engine.set_task(&task) {
                    Ok(cfg) => Ok(format!(
                        "Task '{}' activated. {}\nStep budget: 0/{}",
                        task, cfg.description, cfg.policy.max_steps
                    )),
                    Err(e) => Err(ErrorData::invalid_request(e.to_string(), None)),
                }
            }

            "search" => {
                let query = str_arg(args, "query")?;
                let path = {
                    let engine = self.policy.lock().await;
                    args.get("path")
                        .and_then(|v| v.as_str())
                        .unwrap_or(engine.working_dir())
                        .to_string()
                };
                enforce(&self.policy, "search", None).await?;
                search::search(&query, &path)
                    .await
                    .map_err(|e| ErrorData::internal_error(e.to_string(), None))
            }

            "read_file" => {
                let path = str_arg(args, "path")?;
                enforce(&self.policy, "read_file", None).await?;
                fs::read_to_string(&path)
                    .await
                    .map_err(|e| ErrorData::internal_error(e.to_string(), None))
            }

            "write_file" => {
                let path = str_arg(args, "path")?;
                let content = str_arg(args, "content")?;
                enforce(&self.policy, "write_file", Some(&path.clone())).await?;
                fs::write(&path, &content)
                    .await
                    .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
                Ok(format!("Written {} bytes to {}", content.len(), path))
            }

            "shell" => {
                let command = str_arg(args, "command")?;
                let working_dir = {
                    let engine = self.policy.lock().await;
                    args.get("path")
                        .and_then(|v| v.as_str())
                        .unwrap_or(engine.working_dir())
                        .to_string()
                };
                enforce(&self.policy, "shell", Some(&command.clone())).await?;
                shell::shell(&command, &working_dir)
                    .await
                    .map_err(|e| ErrorData::internal_error(e.to_string(), None))
            }

            "git_log" => {
                let n = args.get("n").and_then(|v| v.as_u64()).unwrap_or(10) as u32;
                let working_dir = self.policy.lock().await.working_dir().to_string();
                enforce(&self.policy, "git_log", None).await?;
                git::git_log(n, &working_dir)
                    .await
                    .map_err(|e| ErrorData::internal_error(e.to_string(), None))
            }

            "git_diff" => {
                let git_ref = args
                    .get("ref")
                    .and_then(|v| v.as_str())
                    .unwrap_or("HEAD")
                    .to_string();
                let working_dir = self.policy.lock().await.working_dir().to_string();
                enforce(&self.policy, "git_diff", None).await?;
                git::git_diff(&git_ref, &working_dir)
                    .await
                    .map_err(|e| ErrorData::internal_error(e.to_string(), None))
            }

            other => Err(ErrorData::invalid_request(
                format!("Unknown tool: {}", other),
                None,
            )),
        }
    }
}

impl ServerHandler for HarnessServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::default(),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation {
                name: "agent-harness".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
            },
            instructions: Some(
                "Call set_task before any other tool. \
                 See CLAUDE.md for available tasks and policy rules."
                    .to_string(),
            ),
        }
    }

    async fn list_tools(
        &self,
        _request: PaginatedRequestParam,
        _ctx: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, ErrorData> {
        Ok(ListToolsResult {
            tools: all_tools(),
            next_cursor: None,
        })
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParam,
        _ctx: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, ErrorData> {
        let args = request.arguments.unwrap_or_default();
        match self.dispatch(&request.name, &args).await {
            Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
                "Policy error: {}",
                e.message
            ))])),
        }
    }
}

// ── helpers ──────────────────────────────────────────────────────────────────

async fn enforce(
    policy: &Arc<Mutex<PolicyEngine>>,
    tool: &str,
    path: Option<&str>,
) -> Result<(), ErrorData> {
    let mut engine = policy.lock().await;
    engine
        .check(tool, path)
        .map_err(|e| ErrorData::invalid_request(e.to_string(), None))
}

fn str_arg(args: &Map<String, Value>, key: &str) -> Result<String, ErrorData> {
    args.get(key)
        .and_then(|v| v.as_str())
        .map(str::to_string)
        .ok_or_else(|| ErrorData::invalid_params(format!("Missing required argument: {}", key), None))
}
