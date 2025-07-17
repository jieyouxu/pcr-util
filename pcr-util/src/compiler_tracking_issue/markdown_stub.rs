use std::fmt::Write as _;

use color_eyre::Result as EResult;
use time::Date;

use crate::IssueMetadata;
use crate::config::CommonTriageConfig;

pub(crate) fn render_markdown_stub(
    config: &CommonTriageConfig,
    issues: &[IssueMetadata],
) -> EResult<String> {
    let mut ctx = RenderCtxt::new(config);
    ctx.render_markdown_stub(issues)?;
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

    fn render_markdown_stub(&mut self, issues: &[IssueMetadata]) -> EResult<()> {
        self.render_document_header(&self.config.markdown_stub_title)?;

        self.render_section(issues)?;

        Ok(())
    }

    fn render_document_header(&mut self, title: &str) -> EResult<()> {
        writeln!(&mut self.buf, "<!-- stubs generated with pcr-util -->")?;
        writeln!(&mut self.buf, "# {title}\n")?;

        let datetime = time::OffsetDateTime::now_utc();
        writeln!(
            &mut self.buf,
            "*Issues snapshot collected on {}*",
            datetime.format(&time::format_description::well_known::Rfc3339).unwrap()
        )?;
        writeln!(&mut self.buf)?;
        Ok(())
    }

    fn render_section(&mut self, issues: &[IssueMetadata]) -> EResult<()> {
        writeln!(&mut self.buf, "## T-compiler-only tracking issues\n")?;
        self.render_issues(issues)?;
        Ok(())
    }

    fn render_issues(&mut self, issues: &[IssueMetadata]) -> EResult<()> {
        issues.iter().try_for_each(|issue| self.render_issue(issue))?;
        Ok(())
    }

    fn render_issue(&mut self, issue: &IssueMetadata) -> EResult<()> {
        writeln!(&mut self.buf, "### #{}: {}", issue.number, issue.title)?;

        writeln!(&mut self.buf, "| Kind | Value |")?;
        writeln!(&mut self.buf, "| - | - |")?;

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
        writeln!(&mut self.buf, "| Link | <{url}> |")?;
        Ok(())
    }

    fn render_creation_date(&mut self, date: Date) -> EResult<()> {
        writeln!(&mut self.buf, "| Creation date | {date} |")?;
        Ok(())
    }

    fn render_labels(&mut self, labels: &[String]) -> EResult<()> {
        write!(&mut self.buf, "| Labels | ")?;
        self.render_comma_sep_inline_code_item(labels)?;
        writeln!(&mut self.buf, "|")?;
        Ok(())
    }

    fn render_author(&mut self, author: &str) -> EResult<()> {
        writeln!(&mut self.buf, "| Author | `{author}` |")?;
        Ok(())
    }

    fn render_wg(&mut self, labels: &[String]) -> EResult<()> {
        write!(&mut self.buf, "| Working groups | ")?;
        let wg_labels = labels.iter().filter(|l| l.starts_with("WG-")).collect::<Vec<_>>();
        self.render_comma_sep_inline_code_item(wg_labels.as_slice())?;
        writeln!(&mut self.buf, "|")?;
        Ok(())
    }

    fn render_assignees(&mut self, assignees: &[String]) -> EResult<()> {
        write!(&mut self.buf, "| Assignees | ")?;
        self.render_comma_sep_inline_code_item(assignees)?;
        writeln!(&mut self.buf, "|")?;
        Ok(())
    }
}
