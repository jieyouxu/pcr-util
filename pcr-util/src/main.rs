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

mod cmd;
mod config;
mod issue_metadata;
mod logging;
mod markdown_stub;

use std::fs;

use clap::Parser;
use color_eyre::Result as EResult;
use color_eyre::eyre::{Context, eyre};
use issue_metadata::IssueMetadata;
use markdown_stub::ReviewInfo;
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

    if !config.repo_path.exists() {
        return Err(eyre!("provided repo path `{}` does not exist!", config.repo_path));
    }

    let p_high = p_high(&config).wrap_err("failed to collect P-high issues")?;
    let review_info = ReviewInfo::new(&p_high);
    info!("writing markdown stub to `{}`", config.markdown_stub_path);

    let stub = markdown_stub::render_markdown_stub(&config, review_info)
        .wrap_err("failed to render markdown stub")?;

    fs::write(&config.markdown_stub_path, &stub).wrap_err_with(|| {
        format!("failed to write markdown stub to `{}`", config.markdown_stub_path)
    })?;

    Ok(())
}

/// P-high
fn p_high(config: &Conf) -> EResult<Vec<IssueMetadata>> {
    let _sp = span!(Level::INFO, "collecting P-high issues").entered();

    let p_high = cmd::p_high_cmd(&config.repo_path)?;
    let p_high: Vec<IssueMetadataRepr> = serde_json::from_slice(&p_high)
        .wrap_err("failed to deserialize JSON response as issue metadata")?;
    let p_high = issue_metadata::simplify_repr(p_high);
    info!("P-high issues count: {}", p_high.len());
    info!("writing P-high issue metadata json to `{}`", config.persist_path);
    let json = serde_json::to_vec_pretty(&p_high)?;
    fs::write(&config.persist_path, &json)
        .wrap_err_with(|| format!("failed to write response to `{}`", config.persist_path))?;
    Ok(p_high)
}
