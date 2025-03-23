mod cmd;
mod markdown_stub;

use std::fs;

use color_eyre::eyre::Context;
use tracing::*;

use crate::EResult;
use crate::config::{CommonConfig, CompilerTrackingIssueTriage};
use crate::issue_metadata::{self, IssueMetadataRepr};

pub(crate) fn perform_triage(
    config: &CommonConfig,
    triage_config: &CompilerTrackingIssueTriage,
) -> EResult<()> {
    let compiler_tracking_issues = {
        let _sp = span!(Level::INFO, "Collecting T-compiler-only tracking issues").entered();
        let compiler_tracking_issues = cmd::compiler_tracking_issue_cmd(&config.repo_path)?;
        let compiler_tracking_issues: Vec<IssueMetadataRepr> =
            serde_json::from_slice(&compiler_tracking_issues)
                .wrap_err("failed to deserialize JSON response as issue metadata")?;
        let mut compiler_tracking_issues = issue_metadata::simplify_repr(compiler_tracking_issues);

        // Intentionally sort by oldest to newest.
        compiler_tracking_issues.sort_by(|a, b| a.number.cmp(&b.number));
        compiler_tracking_issues
    };

    info!("T-compiler-only tracking issues count: {}", compiler_tracking_issues.len());
    info!(
        "Writing T-compiler-only tracking issues metadata json to `{}`",
        triage_config.common.persist_path
    );
    let json = serde_json::to_vec_pretty(&compiler_tracking_issues)?;
    fs::write(&triage_config.common.persist_path, &json).wrap_err_with(|| {
        format!("failed to write response to `{}`", triage_config.common.persist_path)
    })?;

    info!("Writing markdown stub to `{}`", triage_config.common.markdown_stub_path);

    let stub =
        markdown_stub::render_markdown_stub(&triage_config.common, &compiler_tracking_issues)
            .wrap_err("failed to render markdown stub")?;

    fs::write(&triage_config.common.markdown_stub_path, &stub).wrap_err_with(|| {
        format!("failed to write markdown stub to `{}`", triage_config.common.markdown_stub_path)
    })?;

    Ok(())
}
