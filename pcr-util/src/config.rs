use camino::Utf8PathBuf;
use clap::{Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Debug, Parser)]
pub struct Config {
    #[command(flatten)]
    pub common: CommonConfig,

    #[command(subcommand)]
    pub cmd: Cmd,
}

#[derive(Debug, Args)]
pub struct CommonConfig {
    /// Path to a `rust-lang/rust` checkout.
    #[clap(long)]
    pub repo_path: Utf8PathBuf,

    /// Default log level.
    #[clap(long, value_enum, default_value_t = LogLevel::Info)]
    pub log_level: LogLevel,
}

#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
#[derive(clap::ValueEnum)]
#[serde(rename_all = "kebab-case")]
pub enum LogLevel {
    #[default]
    Info,
    Debug,
    Trace,
}

#[derive(Debug, Subcommand)]
pub enum Cmd {
    PHighTriage(PHighTriage),
    CompilerTrackingIssueTriage(CompilerTrackingIssueTriage),
    NoTeamTrackingIssueTriage(NoTeamTrackingIssueTriage),
}

impl Cmd {
    pub fn triage_kind(&self) -> &'static str {
        match self {
            Cmd::PHighTriage(_) => "P-high triage",
            Cmd::CompilerTrackingIssueTriage(_) => "T-compiler tracking issue triage",
            Cmd::NoTeamTrackingIssueTriage(_) => "No-team tracking issue triage",
        }
    }
}

#[derive(Debug, Args)]
pub struct CommonTriageConfig {
    /// Where to store the deserialized JSON response.
    #[clap(long)]
    pub persist_path: Utf8PathBuf,

    /// Markdown stub document title.
    #[clap(long)]
    pub markdown_stub_title: String,
    /// Where to output a Markdown issue review document stub.
    #[clap(long)]
    pub markdown_stub_path: Utf8PathBuf,
}

#[derive(Debug, Parser)]
pub struct PHighTriage {
    #[command(flatten)]
    pub common: CommonTriageConfig,
}

#[derive(Debug, Parser)]
pub struct CompilerTrackingIssueTriage {
    #[command(flatten)]
    pub common: CommonTriageConfig,
}

#[derive(Debug, Parser)]
pub struct NoTeamTrackingIssueTriage {
    #[command(flatten)]
    pub common: CommonTriageConfig,
}
