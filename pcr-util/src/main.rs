//! Assumes you have `gh` cli tool and is already authenticated.
//!
//! P-high T-compiler issues can be generated via `gh` cli:
//!
//! ```shell
//! $ gh issue list \
//!     --label 'T-compiler' --label 'P-high' \
//!     --limit=100 \
//!     --json='assignees,author,createdAt,labels,number,title,updatedAt,url' \
//!     > p-high-t-compiler-issues.json
//! ```

mod config;
mod issue_metadata;
mod logging;
mod markdown_stub;

use std::fs;
use std::process::Command;

use clap::Parser;
use color_eyre::Result as EResult;
use color_eyre::eyre::{Context, eyre};
use issue_metadata::IssueMetadata;
use tracing::*;
use tracing_subscriber::filter;

use crate::config::{Conf, LogLevel};
use crate::issue_metadata::IssueMetadataRepr;

fn main() -> EResult<()> {
    let reload_handle = logging::register_global();
    let config = Conf::parse();

    match config.log_level {
        LogLevel::Trace => reload_handle.modify(|filter| *filter = filter::LevelFilter::TRACE)?,
        LogLevel::Debug => reload_handle.modify(|filter| *filter = filter::LevelFilter::DEBUG)?,
        LogLevel::Info => reload_handle.modify(|filter| *filter = filter::LevelFilter::INFO)?,
    }
    info!("using config: {:#?}", config);

    info!("downloading P-high T-compiler issues via `gh` cli tool");

    if !config.repo_path.exists() {
        return Err(eyre!("provided repo path `{}` does not exist!", config.repo_path));
    }

    let mut cmd = Command::new("gh");
    cmd.current_dir(&config.repo_path);
    cmd.args(["issue", "list"]);
    cmd.args(["--label", "T-compiler", "--label", "P-high"]);
    cmd.args(["--limit", "100"]);
    cmd.args(["--json", "assignees,author,createdAt,labels,number,title,updatedAt,url"]);
    let res = cmd.output().wrap_err("failed to obtain JSON response via `gh` cli")?;
    if !res.status.success() {
        return Err(eyre!("`gh` cli command failed: {}", String::from_utf8_lossy(&res.stderr)));
    }

    let issues: Vec<IssueMetadataRepr> = serde_json::from_slice(&res.stdout)
        .wrap_err("failed to deserialize JSON response as issue metadata")?;

    let issues = simplify_repr(issues);
    info!("P-high T-compiler issues count: {}", issues.len());

    info!("writing issue metadata json to `{}`", config.persist_path);
    let buf = serde_json::to_vec_pretty(&issues)?;
    fs::write(&config.persist_path, &buf)
        .wrap_err_with(|| format!("failed to write response to `{}`", config.persist_path))?;

    info!("writing markdown stub to `{}`", config.markdown_stub_path);
    let stub = markdown_stub::render_markdown_stub(&config, &issues)
        .wrap_err("failed to render markdown stub")?;

    fs::write(&config.markdown_stub_path, &stub).wrap_err_with(|| {
        format!("failed to write markdown stub to `{}`", config.markdown_stub_path)
    })?;

    Ok(())
}

fn simplify_repr(issues: Vec<IssueMetadataRepr>) -> Vec<IssueMetadata> {
    issues
        .into_iter()
        .map(
            |IssueMetadataRepr {
                 assignees,
                 author,
                 created_at,
                 labels,
                 number,
                 title,
                 updated_at,
                 url,
             }| IssueMetadata {
                assignees: assignees.into_iter().map(|a| a.login).collect(),
                author: author.login,
                labels: labels.into_iter().map(|l| l.name).collect(),
                number,
                title,
                created_at,
                updated_at,
                url,
            },
        )
        .collect()
}
