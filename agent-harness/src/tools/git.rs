use anyhow::Result;
use tokio::process::Command;

pub async fn git_log(n: u32, working_dir: &str) -> Result<String> {
    let output = Command::new("git")
        .args([
            "log",
            &format!("-{}", n),
            "--oneline",
            "--decorate",
            "--no-color",
        ])
        .current_dir(working_dir)
        .output()
        .await?;

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

pub async fn git_diff(git_ref: &str, working_dir: &str) -> Result<String> {
    let output = Command::new("git")
        .args(["diff", "--no-color", git_ref])
        .current_dir(working_dir)
        .output()
        .await?;

    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    if stdout.is_empty() {
        Ok(format!("No diff against '{}'", git_ref))
    } else {
        Ok(stdout)
    }
}
