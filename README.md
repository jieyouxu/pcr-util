# pcr-util

**P**-high t-**C**ompiler pre-triage **R**eview util.

## Usage

- Assumes `gh` cli util is available in PATH and is already authenticated.

Assuming `rust-lang/rust` checkout and `pcr-util` repo checkout is under same
folder:

```text
repos/
    rust/
    pcr-util/
```

```bash
$ cargo run -- --repo-path='../../rust'
```

By default, `pcr-util` will just output to
`pcr-util/pcr-util/issue-review-stub.md` for markdown stub, unless otherwise
specified.

```text
Usage: pcr-util.exe [OPTIONS] --repo-path <REPO_PATH>

Options:
      --repo-path <REPO_PATH>
          Path to a `rust-lang/rust` checkout
      --config-path <CONFIG_PATH>
          Path to config file [default: config.toml]
      --persist-path <PERSIST_PATH>
          Where to store the deserialized JSON response [default: p-high-t-compiler-issues.json]
      --markdown-stub-title <MARKDOWN_STUB_TITLE>
          Markdown stub document title [default: "2024 Q3 T-compiler P-high Issue Review"]
      --markdown-stub-path <MARKDOWN_STUB_PATH>
          Where to output a Markdown issue review document stub [default: issue-review-stub.md]
      --log-level <LOG_LEVEL>
          Default log level [default: info] [possible values: info, debug, trace]
  -h, --help
          Print help
```
