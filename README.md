# pcr-util

**P**-high t-**C**ompiler pre-triage **R**eview util.

(Can't be bothered to come up with a better name, close enough.)

(Which is already outdated, since it's now also useful for other kinds of triages.)

## Usage

> [!TIP]
>
> Assumes `gh` cli util is available in PATH and is already authenticated.

```text
Usage: pcr-util [OPTIONS] --repo-path <REPO_PATH> <COMMAND>

Commands:
  p-high-triage
  compiler-tracking-issue-triage
  no-team-tracking-issue-triage
  help                            Print this message or the help of the given subcommand(s)

Options:
      --repo-path <REPO_PATH>  Path to a `rust-lang/rust` checkout
      --log-level <LOG_LEVEL>  Default log level [default: info] [possible values: info, debug, trace]
  -h, --help                   Print help
```

### Example: T-compiler-only tracking issue triage

```
Usage: pcr-util --repo-path <REPO_PATH> compiler-tracking-issue-triage --persist-path <PERSIST_PATH> --markdown-stub-title <MARKDOWN_STUB_TITLE> --markdown-stub-path <MARKDOWN_STUB_PATH>

Options:
      --persist-path <PERSIST_PATH>
          Where to store the deserialized JSON response
      --markdown-stub-title <MARKDOWN_STUB_TITLE>
          Markdown stub document title
      --markdown-stub-path <MARKDOWN_STUB_PATH>
          Where to output a Markdown issue review document stub
  -h, --help
          Print help
```

```bash
$ pcr-util \
    --repo-path="../../rust/" \
    compiler-tracking-issue-triage \
    --persist-path="../scratch/compiler-only-tracking-issues.json" \
    --markdown-stub-title="2025Q1 T-compiler-only Tracking Issues Triage" \
    --markdown-stub-path="../scratch/compiler-only-tracking-issues-triage.md"
```

You can copy the generated markdown stub into something like HackMD, e.g.:

![Screenshot 2024-11-11 003939](https://github.com/user-attachments/assets/beac98f6-e47b-4359-b972-a476afa73162)
