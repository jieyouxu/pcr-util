//! Assumes you have `gh` cli tool and is already authenticated.

#![allow(clippy::enum_variant_names)]

mod config;
mod issue_metadata;
mod logging;

use clap::Parser;
use color_eyre::Result as EResult;
use color_eyre::eyre::eyre;
use config::CommonTriageConfig;
use issue_metadata::IssueMetadata;
use tracing::*;
use tracing_subscriber::filter;

use crate::config::{Config, LogLevel};

mod compiler_tracking_issue;
mod p_high;

fn main() -> EResult<()> {
    let reload_handle = logging::register_global();
    let config = Config::parse();

    match config.common.log_level {
        LogLevel::Trace => reload_handle.modify(|filter| *filter = filter::LevelFilter::TRACE)?,
        LogLevel::Debug => reload_handle.modify(|filter| *filter = filter::LevelFilter::DEBUG)?,
        LogLevel::Info => reload_handle.modify(|filter| *filter = filter::LevelFilter::INFO)?,
    }
    debug!("using config: {:#?}", config);

    if !config.common.repo_path.exists() {
        return Err(eyre!("provided repo path `{}` does not exist!", config.common.repo_path));
    }

    info!("Performing triage: {}", config.cmd.triage_kind());
    info!("\trepo_path:\t\t`{}`", config.common.repo_path);

    match config.cmd {
        config::Cmd::PHighTriage(triage_config) => {
            print_common_triage_config(&triage_config.common);
            p_high::perform_triage(&config.common, &triage_config)?
        }
        config::Cmd::CompilerTrackingIssueTriage(triage_config) => {
            print_common_triage_config(&triage_config.common);
            compiler_tracking_issue::perform_triage(&config.common, &triage_config)?;
            todo!()
        }
        config::Cmd::NoTeamTrackingIssueTriage(triage_config) => {
            print_common_triage_config(&triage_config.common);
            todo!()
        }
    };

    Ok(())
}

fn print_common_triage_config(config: &CommonTriageConfig) {
    info!("\tpersist_path:\t\t`{}`", config.persist_path);
    info!("\tmarkdown_stub_path:\t`{}`", config.markdown_stub_path);
    info!("\tmarkdown_stub_title:\t\"{}\"", config.markdown_stub_title);
}
