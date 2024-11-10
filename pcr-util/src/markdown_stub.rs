use std::fmt::Write as _;

use color_eyre::Result as EResult;
use time::Date;

use crate::IssueMetadata;
use crate::config::Conf;

pub(crate) fn render_markdown_stub(config: &Conf, issues: &Vec<IssueMetadata>) -> EResult<String> {
    let mut ctx = RenderCtxt::new(config);
    ctx.render_markdown_stub(issues)?;
    Ok(ctx.finish())
}

struct RenderCtxt<'c> {
    config: &'c Conf,
    buf: String,
}

impl<'c> RenderCtxt<'c> {
    fn new(config: &'c Conf) -> Self {
        Self { buf: String::new(), config }
    }

    fn finish(self) -> String {
        self.buf
    }

    fn render_comma_sep_inline_code_item(&mut self, items: &[String]) -> EResult<()> {
        match items {
            [] => return Ok(()),
            [single] => write!(&mut self.buf, "`{}`", single)?,
            [first, rest @ ..] => {
                write!(&mut self.buf, "`{}`", first)?;
                for l in rest {
                    write!(&mut self.buf, ", `{}`", l)?;
                }
            }
        }
        Ok(())
    }

    fn render_markdown_stub(&mut self, issues: &Vec<IssueMetadata>) -> EResult<()> {
        self.render_document_heading(&self.config.markdown_stub_title)?;
        self.render_issues(issues)?;
        Ok(())
    }

    fn render_document_heading(&mut self, title: &str) -> EResult<()> {
        writeln!(&mut self.buf, "<!-- generated with pcr-util -->")?;
        writeln!(&mut self.buf, "# {}\n", title)?;
        Ok(())
    }

    fn render_issues(&mut self, issues: &Vec<IssueMetadata>) -> EResult<()> {
        issues.iter().try_for_each(|issue| self.render_issue(issue))?;
        Ok(())
    }

    fn render_issue(&mut self, issue: &IssueMetadata) -> EResult<()> {
        writeln!(&mut self.buf, "## #{}: {}", issue.number, issue.title)?;
        self.render_issue_link(&issue.url)?;
        self.render_creation_date(issue.created_at.date())?;
        self.render_labels(&issue.labels)?;
        self.render_author(&issue.author)?;
        self.render_assignees(&issue.assignees)?;
        writeln!(&mut self.buf, "\n**TODO**\n\n")?;
        Ok(())
    }

    fn render_issue_link(&mut self, url: &str) -> EResult<()> {
        writeln!(&mut self.buf, "Link: <{}>", url)?;
        Ok(())
    }

    fn render_creation_date(&mut self, date: Date) -> EResult<()> {
        writeln!(&mut self.buf, "Creation date: {}", date)?;
        Ok(())
    }

    fn render_labels(&mut self, labels: &[String]) -> EResult<()> {
        write!(&mut self.buf, "Labels: ")?;
        self.render_comma_sep_inline_code_item(labels)?;
        write!(&mut self.buf, "\n")?;
        Ok(())
    }

    fn render_author(&mut self, author: &str) -> EResult<()> {
        writeln!(&mut self.buf, "Author: `{author}`")?;
        Ok(())
    }

    fn render_assignees(&mut self, assignees: &[String]) -> EResult<()> {
        write!(&mut self.buf, "Assignees: ")?;
        self.render_comma_sep_inline_code_item(assignees)?;
        write!(&mut self.buf, "\n")?;
        Ok(())
    }
}
