use std::fmt::Write as _;

use color_eyre::Result as EResult;
use itertools::Itertools as _;
use time::Date;

use crate::IssueMetadata;
use crate::config::CommonTriageConfig;

pub(crate) struct ReviewInfo<'a> {
    pub(crate) p_high: &'a [IssueMetadata],
}

impl<'a> ReviewInfo<'a> {
    pub(crate) fn new(p_high: &'a [IssueMetadata]) -> Self {
        Self { p_high }
    }

    pub(crate) fn p_high_no_team(&'a self) -> Vec<&'a IssueMetadata> {
        self.p_high
            .iter()
            .filter(|issue| !issue.labels.iter().any(|label| label.starts_with("T-")))
            .collect()
    }

    /// Partition by ownership: no owner or has owner. Returns `(no owner, has owner)` partition.
    ///
    /// Owner is either WG or assignee.
    pub(crate) fn t_compiler_p_high_partition_by_ownership(
        &'a self,
    ) -> (Vec<&'a IssueMetadata>, Vec<&'a IssueMetadata>) {
        self.p_high
            .iter()
            .filter(|issue| issue.labels.iter().map(String::as_str).contains(&"T-compiler"))
            .partition(|issue| {
                // No WG
                !issue.labels.iter().any(|label| label.starts_with("WG-"))
                    // and no assignee
                    && issue.assignees.is_empty()
            })
    }
}

pub(crate) fn render_markdown_stub<'a>(
    config: &CommonTriageConfig,
    info: ReviewInfo<'a>,
) -> EResult<String> {
    let mut ctx = RenderCtxt::new(config);
    ctx.render_markdown_stub(info)?;
    Ok(ctx.finish())
}

struct RenderCtxt<'c> {
    config: &'c CommonTriageConfig,
    buf: String,
}

impl<'c> RenderCtxt<'c> {
    fn new(config: &'c CommonTriageConfig) -> Self {
        Self { buf: String::new(), config }
    }

    fn finish(self) -> String {
        self.buf
    }

    fn render_comma_sep_inline_code_item<S: AsRef<str>>(&mut self, items: &[S]) -> EResult<()> {
        match items {
            [] => return Ok(()),
            [single] => write!(&mut self.buf, "`{}`", single.as_ref())?,
            [first, rest @ ..] => {
                write!(&mut self.buf, "`{}`", first.as_ref())?;
                for l in rest {
                    write!(&mut self.buf, ", `{}`", l.as_ref())?;
                }
            }
        }
        Ok(())
    }

    fn render_markdown_stub<'a>(&mut self, info: ReviewInfo<'a>) -> EResult<()> {
        self.render_document_header(&self.config.markdown_stub_title)?;

        let no_team = info.p_high_no_team();
        let (no_owner, has_owner) = info.t_compiler_p_high_partition_by_ownership();

        self.render_no_team(no_team.as_slice())?;
        self.render_no_owner(no_owner.as_slice())?;
        self.render_has_owner(has_owner.as_slice())?;

        Ok(())
    }

    fn render_document_header(&mut self, title: &str) -> EResult<()> {
        writeln!(&mut self.buf, "<!-- stubs generated with pcr-util -->\n")?;
        writeln!(&mut self.buf, "# {title}\n")?;

        let datetime = time::OffsetDateTime::now_utc();
        writeln!(
            &mut self.buf,
            "*Issues snapshot collected on {}*\n\n",
            datetime.format(&time::format_description::well_known::Rfc3339).unwrap()
        )?;

        Ok(())
    }

    fn render_no_team(&mut self, no_team: &[&IssueMetadata]) -> EResult<()> {
        const NO_TEAM_URL: &str = "is:open is:issue label:P-high -label:T-cargo -label:T-community -label:T-compiler -label:T-core -label:T-crates-io -label:T-dev-tools -label:T-docs-rs -label:T-infra -label:T-libs -label:T-libs-api -label:T-release -label:T-release -label:T-rustdoc -label:T-style -label:T-types -label:T-lang -label:T-leadership-council";
        writeln!(&mut self.buf, "## P-high missing team label\n")?;
        writeln!(&mut self.buf, "[P-high issues without team label]({NO_TEAM_URL})\n\n")?;

        if no_team.is_empty() {
            writeln!(&mut self.buf, "**Did not find P-high issues without a team label**")?;
        } else {
            self.render_issues(no_team)?;
        }

        write!(&mut self.buf, "\n\n")?;
        Ok(())
    }

    fn render_no_owner(&mut self, no_owner: &[&IssueMetadata]) -> EResult<()> {
        const NO_OWNER_URL: &str = "https://github.com/rust-lang/rust/issues?q=is%3Aissue%20is%3Aopen%20label%3AT-compiler%20label%3AP-high%20-label%3Awg-debugging%20-label%3AWG-embedded%20-label%3AWG-diagnostics%20-label%3AWG-async%20-label%3AWG-incr-comp%20no%3Aassignee%20sort%3Acreated-asc%20-label%3AI-types-nominated%20-label%3AI-lang-nominated%20-label%3AI-compiler-nominated%20-label%3AT-types%20-label%3AWG-llvm";
        writeln!(
            &mut self.buf,
            "## P-high T-compiler issues missing owner (no WG and no assignee)\n"
        )?;
        writeln!(&mut self.buf, "[P-high issues with no owner]({NO_OWNER_URL})\n\n")?;
        self.render_issues(no_owner)?;
        write!(&mut self.buf, "\n\n")?;
        Ok(())
    }

    fn render_has_owner(&mut self, has_owner: &[&IssueMetadata]) -> EResult<()> {
        writeln!(&mut self.buf, "## P-high T-compiler issues with owner (WG or assignee)\n")?;
        self.render_issues(has_owner)?;
        write!(&mut self.buf, "\n\n")?;
        Ok(())
    }

    fn render_issues(&mut self, issues: &[&IssueMetadata]) -> EResult<()> {
        issues.iter().try_for_each(|issue| self.render_issue(issue))?;
        Ok(())
    }

    fn render_issue(&mut self, issue: &IssueMetadata) -> EResult<()> {
        writeln!(&mut self.buf, "### #{}: {}", issue.number, issue.title)?;
        self.render_issue_link(&issue.url)?;
        self.render_creation_date(issue.created_at.date())?;
        self.render_labels(&issue.labels)?;
        self.render_author(&issue.author)?;
        self.render_wg(&issue.labels)?;
        self.render_assignees(&issue.assignees)?;
        writeln!(&mut self.buf, "\n**TODO**\n\n")?;
        Ok(())
    }

    fn render_issue_link(&mut self, url: &str) -> EResult<()> {
        writeln!(&mut self.buf, "Link: <{url}>")?;
        Ok(())
    }

    fn render_creation_date(&mut self, date: Date) -> EResult<()> {
        writeln!(&mut self.buf, "Creation date: {date}")?;
        Ok(())
    }

    fn render_labels(&mut self, labels: &[String]) -> EResult<()> {
        write!(&mut self.buf, "Labels: ")?;
        self.render_comma_sep_inline_code_item(labels)?;
        writeln!(&mut self.buf)?;
        Ok(())
    }

    fn render_author(&mut self, author: &str) -> EResult<()> {
        writeln!(&mut self.buf, "Author: `{author}`")?;
        Ok(())
    }

    fn render_wg(&mut self, labels: &[String]) -> EResult<()> {
        write!(&mut self.buf, "Working groups: ")?;
        let wg_labels = labels.iter().filter(|l| l.starts_with("WG-")).collect::<Vec<_>>();
        self.render_comma_sep_inline_code_item(wg_labels.as_slice())?;
        writeln!(&mut self.buf)?;
        Ok(())
    }

    fn render_assignees(&mut self, assignees: &[String]) -> EResult<()> {
        write!(&mut self.buf, "Assignees: ")?;
        self.render_comma_sep_inline_code_item(assignees)?;
        writeln!(&mut self.buf)?;
        Ok(())
    }
}
