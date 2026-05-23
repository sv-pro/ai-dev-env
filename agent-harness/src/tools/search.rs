use anyhow::Result;
use std::path::Path;
use tokio::fs;

/// Recursive grep-like search: finds lines matching `query` in files under `root`.
pub async fn search(query: &str, root: &str) -> Result<String> {
    let mut results = Vec::new();
    search_dir(query, Path::new(root), &mut results).await?;
    if results.is_empty() {
        Ok(format!("No matches for '{}' under '{}'", query, root))
    } else {
        Ok(results.join("\n"))
    }
}

async fn search_dir(query: &str, dir: &Path, results: &mut Vec<String>) -> Result<()> {
    let mut entries = fs::read_dir(dir).await?;
    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        let meta = entry.metadata().await?;
        if meta.is_dir() {
            // Skip hidden dirs and common noise
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if name_str.starts_with('.') || name_str == "target" || name_str == "node_modules" {
                continue;
            }
            Box::pin(search_dir(query, &path, results)).await?;
        } else if meta.is_file() {
            if let Ok(contents) = fs::read_to_string(&path).await {
                for (lineno, line) in contents.lines().enumerate() {
                    if line.contains(query) {
                        results.push(format!(
                            "{}:{}: {}",
                            path.display(),
                            lineno + 1,
                            line.trim()
                        ));
                    }
                }
            }
        }
    }
    Ok(())
}
