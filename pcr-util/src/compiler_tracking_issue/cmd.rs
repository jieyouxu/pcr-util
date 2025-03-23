use std::process::Command;

use camino::Utf8Path;
use color_eyre::Result as EResult;
use color_eyre::eyre::{Context, eyre};
use tracing::*;

pub(crate) fn compiler_tracking_issue_cmd(repo_path: &Utf8Path) -> EResult<Vec<u8>> {
    info!("Downloading T-compiler-only tracking issues via `gh` cli");
    let mut cmd = Command::new("gh");
    cmd.current_dir(repo_path);
    cmd.args(["issue", "list"]);

    let search_query = "\
        is:issue \
        state:open \
        sort:updated-asc \
        label:T-compiler label:C-tracking-issue \
        -label:T-bootstrap \
        -label:T-cargo \
        -label:T-core \
        -label:T-dev-tools \
        -label:T-infra \
        -label:T-lang \
        -label:T-libs \
        -label:T-libs-api \
        -label:T-opsem \
        -label:T-release \
        -label:T-rustdoc \
        -label:T-rustdoc-frontend \
        -label:T-rustfmt \
        -label:T-rust-analyzer \
        -label:T-style \
        -label:T-types \
    ";

    cmd.args(["--search", search_query]);
    cmd.args(["--limit", "200"]);
    cmd.args(["--json", "assignees,author,createdAt,labels,number,title,updatedAt,url"]);
    let res = cmd.output().wrap_err("failed to obtain JSON response via `gh` cli")?;
    if !res.status.success() {
        return Err(eyre!("`gh` cli command failed: {}", String::from_utf8_lossy(&res.stderr)));
    }
    Ok(res.stdout)
}
