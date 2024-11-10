use camino::Utf8PathBuf;
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Parser)]
pub struct Conf {
    /// Path to a `rust-lang/rust` checkout.
    #[clap(long)]
    pub repo_path: Utf8PathBuf,

    /// Path to config file.
    #[clap(long, default_value_t = Utf8PathBuf::from("config.toml"))]
    pub config_path: Utf8PathBuf,

    /// Where to store the deserialized JSON response.
    #[clap(long, default_value_t = Utf8PathBuf::from("p-high-t-compiler-issues.json"))]
    pub persist_path: Utf8PathBuf,

    /// Markdown stub document title.
    #[clap(long, default_value_t = String::from("2024 Q3 T-compiler P-high Issue Review"))]
    pub markdown_stub_title: String,
    /// Where to output a Markdown issue review document stub.
    #[clap(long, default_value_t = Utf8PathBuf::from("issue-review-stub.md"))]
    pub markdown_stub_path: Utf8PathBuf,

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
