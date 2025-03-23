mod cmd;
mod markdown_stub;

use std::fs;

use color_eyre::eyre::Context;
use tracing::*;

use self::markdown_stub::ReviewInfo;
use crate::EResult;
use crate::config::{CommonConf, PHighTriage};
use crate::issue_metadata::{self, IssueMetadataRepr};

pub(crate) fn perform_triage(config: &CommonConf, triage_config: &PHighTriage) -> EResult<()> {
    let p_high = {
        let _sp = span!(Level::INFO, "collecting P-high issues").entered();
        let p_high = cmd::p_high_cmd(&config.repo_path)?;
        let p_high: Vec<IssueMetadataRepr> = serde_json::from_slice(&p_high)
            .wrap_err("failed to deserialize JSON response as issue metadata")?;
        let mut p_high = issue_metadata::simplify_repr(p_high);

        // Intentionally sort by oldest to newest.
        p_high.sort_by(|a, b| a.number.cmp(&b.number));
        p_high
    };

    info!("P-high issues count: {}", p_high.len());
    info!("writing P-high issue metadata json to `{}`", triage_config.common.persist_path);
    let json = serde_json::to_vec_pretty(&p_high)?;
    fs::write(&triage_config.common.persist_path, &json).wrap_err_with(|| {
        format!("failed to write response to `{}`", triage_config.common.persist_path)
    })?;

    let review_info = ReviewInfo::new(&p_high);
    info!("writing markdown stub to `{}`", triage_config.common.markdown_stub_path);

    let stub = markdown_stub::render_markdown_stub(&triage_config.common, review_info)
        .wrap_err("failed to render markdown stub")?;

    fs::write(&triage_config.common.markdown_stub_path, &stub).wrap_err_with(|| {
        format!("failed to write markdown stub to `{}`", triage_config.common.markdown_stub_path)
    })?;

    Ok(())
}
