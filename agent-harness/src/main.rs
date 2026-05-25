mod policy;
mod registry;
mod server;
mod tools;

use anyhow::{Context, Result};
use policy::{PolicyEngine, WorldConfig};
use rmcp::ServiceExt;
use server::HarnessServer;
use tracing_subscriber::{EnvFilter, fmt};

#[tokio::main]
async fn main() -> Result<()> {
    fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();

    let config_path = std::env::var("HARNESS_CONFIG").unwrap_or_else(|_| "harness.yaml".into());

    let raw = std::fs::read_to_string(&config_path)
        .with_context(|| format!("Failed to read config: {}", config_path))?;

    let config: WorldConfig =
        serde_yaml::from_str(&raw).with_context(|| "Failed to parse harness.yaml")?;

    tracing::info!(
        world = config.world.name.as_str(),
        tasks = config.tasks.len(),
        "Agent harness starting"
    );

    let engine = PolicyEngine::new(config);
    let harness = HarnessServer::new(engine);

    let transport = rmcp::transport::io::stdio();
    let running = harness
        .serve(transport)
        .await
        .context("Failed to start MCP server")?;

    running.waiting().await.context("MCP server exited with error")?;

    Ok(())
}
