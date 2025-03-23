use std::process::Command;

use camino::Utf8Path;
use color_eyre::Result as EResult;
use color_eyre::eyre::{Context, eyre};
use tracing::*;

pub(crate) fn p_high_cmd(repo_path: &Utf8Path) -> EResult<Vec<u8>> {
    info!("Downloading P-high issues via `gh` cli");
    let mut cmd = Command::new("gh");
    cmd.current_dir(repo_path);
    cmd.args(["issue", "list"]);
    cmd.args(["--label", "P-high"]);
    cmd.args(["--limit", "100"]);
    cmd.args(["--json", "assignees,author,createdAt,labels,number,title,updatedAt,url"]);
    let res = cmd.output().wrap_err("failed to obtain JSON response via `gh` cli")?;
    if !res.status.success() {
        return Err(eyre!("`gh` cli command failed: {}", String::from_utf8_lossy(&res.stderr)));
    }
    Ok(res.stdout)
}
