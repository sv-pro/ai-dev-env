use anyhow::{bail, Result};
use tokio::process::Command;

/// Run a shell command inside `working_dir`. Returns combined stdout+stderr.
pub async fn shell(command: &str, working_dir: &str) -> Result<String> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .current_dir(working_dir)
        .output()
        .await?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        let code = output.status.code().unwrap_or(-1);
        bail!(
            "Command exited with status {}\nstdout: {}\nstderr: {}",
            code,
            stdout,
            stderr
        );
    }

    let mut out = String::new();
    if !stdout.is_empty() {
        out.push_str(&stdout);
    }
    if !stderr.is_empty() {
        if !out.is_empty() {
            out.push('\n');
        }
        out.push_str(&stderr);
    }
    Ok(out)
}
