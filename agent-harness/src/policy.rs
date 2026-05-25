use anyhow::{bail, Result};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub struct WorldConfig {
    pub version: String,
    pub world: WorldMeta,
    pub tasks: HashMap<String, TaskConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct WorldMeta {
    pub name: String,
    pub working_dir: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TaskConfig {
    pub description: String,
    pub tools: Vec<String>,
    pub policy: TaskPolicy,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TaskPolicy {
    pub allow_write: bool,
    #[serde(default)]
    pub paths_whitelist: Vec<String>,
    #[serde(default)]
    pub require_approval: Vec<String>,
    #[serde(default = "default_max_steps")]
    pub max_steps: u32,
}

fn default_max_steps() -> u32 {
    20
}

pub struct PolicyEngine {
    pub config: WorldConfig,
    pub active_task: Option<String>,
    pub step_count: u32,
}

impl PolicyEngine {
    pub fn new(config: WorldConfig) -> Self {
        Self {
            config,
            active_task: None,
            step_count: 0,
        }
    }

    pub fn set_task(&mut self, task_name: &str) -> Result<&TaskConfig> {
        if !self.config.tasks.contains_key(task_name) {
            bail!(
                "Unknown task '{}'. Available tasks: {}",
                task_name,
                self.config
                    .tasks
                    .keys()
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
        self.active_task = Some(task_name.to_string());
        self.step_count = 0;
        Ok(self.config.tasks.get(task_name).unwrap())
    }

    pub fn active_task_config(&self) -> Option<&TaskConfig> {
        self.active_task
            .as_ref()
            .and_then(|t| self.config.tasks.get(t))
    }

    /// Validate a tool call and increment the step counter.
    /// Returns Ok(()) if allowed, Err with reason if denied.
    pub fn check(&mut self, tool: &str, path: Option<&str>) -> Result<()> {
        let task = self
            .active_task_config()
            .ok_or_else(|| anyhow::anyhow!("No active task. Call set_task first."))?;

        if self.step_count >= task.policy.max_steps {
            bail!(
                "Step budget exhausted ({}/{}). Task '{}' is complete.",
                self.step_count,
                task.policy.max_steps,
                self.active_task.as_deref().unwrap_or("?")
            );
        }

        if !task.tools.contains(&tool.to_string()) {
            bail!(
                "Tool '{}' is not allowed in task '{}'. Allowed tools: {}",
                tool,
                self.active_task.as_deref().unwrap_or("?"),
                task.tools.join(", ")
            );
        }

        if tool == "write_file" && !task.policy.allow_write {
            bail!(
                "write_file is disabled for task '{}' (allow_write: false)",
                self.active_task.as_deref().unwrap_or("?")
            );
        }

        if (tool == "write_file" || tool == "shell") && !task.policy.paths_whitelist.is_empty() {
            if let Some(p) = path {
                let allowed = task
                    .policy
                    .paths_whitelist
                    .iter()
                    .any(|prefix| p.starts_with(prefix.as_str()));
                if !allowed {
                    bail!(
                        "Path '{}' is outside the whitelist for task '{}'. Allowed prefixes: {}",
                        p,
                        self.active_task.as_deref().unwrap_or("?"),
                        task.policy.paths_whitelist.join(", ")
                    );
                }
            }
        }

        if task.policy.require_approval.contains(&tool.to_string()) {
            tracing::warn!(
                tool = tool,
                task = self.active_task.as_deref().unwrap_or("?"),
                "Tool requires approval — auto-approving in dev mode"
            );
        }

        self.step_count += 1;
        Ok(())
    }

    pub fn working_dir(&self) -> &str {
        &self.config.world.working_dir
    }
}
