//! The formatter's shared command line (`deckmaste_dsl::cli`), the `sylfmt`
//! binary that runs it, and `same_meaning`, its guard.

use std::io::Write as _;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::Output;
use std::process::Stdio;

use deckmaste_dsl::Closers;
use deckmaste_dsl::FileId;
use deckmaste_dsl::Style;
use deckmaste_dsl::TrailingCommas;
use deckmaste_dsl::cli::Args;
use deckmaste_dsl::cli::format_paths;
use deckmaste_dsl::cli::format_source;
use deckmaste_dsl::cli::parse_args;
use deckmaste_dsl::parse;
use deckmaste_dsl::same_meaning;

fn args(args: &[&str]) -> Result<Args, String> {
    parse_args(args.iter().map(|&a| a.to_owned()))
}

// Flags.

#[test]
fn no_flags_is_the_default_style() {
    let parsed = args(&[]).unwrap();
    assert_eq!(parsed.style, Style::default());
    assert!(!parsed.check && !parsed.stdin && !parsed.help);
    assert_eq!(parsed.paths, Vec::<PathBuf>::new());
}

#[test]
fn style_flags_set_every_option() {
    let parsed = args(&[
        "--width",
        "80",
        "--closers",
        "own-line",
        "--trailing-commas=own-line",
        "--record-spaces",
        "--hug-first-list=false",
    ])
    .unwrap();
    assert_eq!(
        parsed.style,
        Style {
            width: 80,
            closers: Closers::OwnLine,
            trailing_commas: TrailingCommas::OwnLine,
            record_spaces: true,
            hug_first_list: false,
        }
    );
    let parsed = args(&[
        "--width=120",
        "--closers=hugged",
        "--trailing-commas",
        "bodies",
        "--record-spaces=false",
        "--hug-first-list=true",
    ])
    .unwrap();
    assert_eq!(
        parsed.style,
        Style {
            width: 120,
            trailing_commas: TrailingCommas::Bodies,
            ..Style::default()
        }
    );
    assert_eq!(
        args(&["--trailing-commas", "never"]).unwrap().style,
        Style::default()
    );
}

#[test]
fn a_bare_bool_flag_does_not_take_the_next_path() {
    let parsed = args(&["--record-spaces", "cards", "--hug-first-list", "more"]).unwrap();
    assert!(parsed.style.record_spaces);
    assert!(parsed.style.hug_first_list);
    assert_eq!(
        parsed.paths,
        [PathBuf::from("cards"), PathBuf::from("more")]
    );
}

#[test]
fn modes_paths_and_the_end_of_flags() {
    let parsed = args(&["--check", "a.syl", "--", "--width"]).unwrap();
    assert!(parsed.check);
    assert_eq!(
        parsed.paths,
        [PathBuf::from("a.syl"), PathBuf::from("--width")]
    );
    let parsed = args(&["--stdin", "name.syl"]).unwrap();
    assert!(parsed.stdin);
    assert!(args(&["-h"]).unwrap().help);
    assert!(args(&["--help"]).unwrap().help);
}

#[test]
fn bad_command_lines_are_rejected() {
    for (line, message) in [
        (&["--check", "--stdin"][..], "cannot be used together"),
        (&["--stdin", "a", "b"], "at most one path"),
        (&["--width"], "--width needs a value"),
        (&["--width", "wide"], "invalid width"),
        (&["--closers", "loose"], "expected hugged or own-line"),
        (
            &["--trailing-commas=always"],
            "expected never, bodies or own-line",
        ),
        (&["--record-spaces=yes"], "expected true or false"),
        (&["--check=true"], "unknown flag"),
        (&["--frobnicate"], "unknown flag"),
    ] {
        let error = args(line).unwrap_err();
        assert!(error.contains(message), "{line:?}: {error}");
    }
}

// Formatting sources and files.

#[test]
fn format_source_formats_and_reports_parse_errors() {
    let style = Style::default();
    assert_eq!(
        format_source("a.syl", "card {name = \"A\",}", &style).unwrap(),
        "card {\n  name = \"A\"\n}\n"
    );
    assert_eq!(
        format_source("a.syl", "card {\n  name = (\n}", &style).unwrap_err(),
        "a.syl:2:10: expected a value, found '('"
    );
}

/// A fresh, empty scratch directory for one test.
fn scratch(name: &str) -> PathBuf {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join("cli")
        .join(name);
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn check_lists_changed_files_and_writes_nothing() {
    let dir = scratch("check");
    let sub = dir.join("sub");
    std::fs::create_dir(&sub).unwrap();
    let tidy = dir.join("tidy.syl");
    let messy = sub.join("messy.syl");
    let other = sub.join("notes.txt");
    std::fs::write(&tidy, "relation r {}\n").unwrap();
    std::fs::write(&messy, "relation   r {}").unwrap();
    std::fs::write(&other, "relation   r {}").unwrap();
    let style = Style::default();

    let changed = format_paths(std::slice::from_ref(&dir), &style, true).unwrap();
    assert_eq!(changed, vec![messy.clone()]);
    assert_eq!(std::fs::read_to_string(&messy).unwrap(), "relation   r {}");

    let changed = format_paths(std::slice::from_ref(&dir), &style, false).unwrap();
    assert_eq!(changed, vec![messy.clone()]);
    assert_eq!(std::fs::read_to_string(&messy).unwrap(), "relation r {}\n");
    assert_eq!(std::fs::read_to_string(&other).unwrap(), "relation   r {}");
    assert_eq!(
        format_paths(&[dir], &style, true).unwrap(),
        Vec::<PathBuf>::new()
    );
}

#[test]
fn a_parse_error_writes_no_file() {
    let dir = scratch("parse-error");
    let messy = dir.join("a.syl");
    let broken = dir.join("b.syl");
    std::fs::write(&messy, "relation   r {}").unwrap();
    std::fs::write(&broken, "relation r {").unwrap();
    let error = format_paths(&[dir], &Style::default(), false).unwrap_err();
    assert!(error.contains("b.syl:1:13: expected"), "{error}");
    assert!(
        error.contains("1 file failed to format; nothing written"),
        "{error}"
    );
    assert_eq!(std::fs::read_to_string(&messy).unwrap(), "relation   r {}");
}

// The guard.

#[test]
fn same_meaning_ignores_spans_and_leading_blank_lines_only() {
    let file = |source: &str| parse(FileId(0), source).unwrap();
    let base = file("# doc\nrelation r { a = [x,\n\ny] }\n");
    assert!(same_meaning(
        &base,
        &file("\n\n# doc\nrelation   r {\n\n  a = [\n\n    x,\n\n    y]}")
    ));
    // A comment's text, a comment's position and an interior blank line all
    // count.
    assert!(!same_meaning(
        &base,
        &file("# doc!\nrelation r { a = [x,\n\ny] }")
    ));
    assert!(!same_meaning(
        &base,
        &file("relation r { # doc\n a = [x,\n\ny] }")
    ));
    assert!(!same_meaning(
        &base,
        &file("# doc\nrelation r { a = [x, y] }")
    ));
    assert!(!same_meaning(
        &base,
        &file("# doc\nrelation r { a = [x,\n\nz] }")
    ));
}

// The `sylfmt` binary.

fn sylfmt(args: &[&str], input: &str) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_sylfmt"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("sylfmt starts");
    child
        .stdin
        .take()
        .expect("piped stdin")
        .write_all(input.as_bytes())
        .expect("stdin accepts the input");
    child.wait_with_output().expect("sylfmt finishes")
}

#[test]
fn sylfmt_formats_stdin_to_stdout() {
    let output = sylfmt(&["--stdin", "--record-spaces"], "card {a = {b = 1},}");
    assert!(output.status.success(), "{output:?}");
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "card {\n  a = { b = 1 }\n}\n"
    );
}

#[test]
fn sylfmt_reports_a_stdin_parse_error_with_the_named_path() {
    let output = sylfmt(&["--stdin", "cards/Bad.syl"], "card {\n  a = (\n}");
    assert!(!output.status.success(), "{output:?}");
    assert_eq!(output.stdout, b"");
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("cards/Bad.syl:2:7: expected a value, found '('"),
        "{stderr}"
    );
}

#[test]
fn sylfmt_checks_paths_and_needs_one() {
    let dir = scratch("sylfmt-check");
    let messy = dir.join("a.syl");
    std::fs::write(&messy, "relation   r {}").unwrap();
    let path = messy.to_str().unwrap();
    let output = sylfmt(&["--check", path], "");
    assert!(!output.status.success(), "{output:?}");
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        format!("{path}\n")
    );
    assert!(
        String::from_utf8(output.stderr)
            .unwrap()
            .contains("1 file would change")
    );

    let output = sylfmt(&[path], "");
    assert!(output.status.success(), "{output:?}");
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "1 file changed\n"
    );
    assert!(sylfmt(&["--check", path], "").status.success());

    let output = sylfmt(&[], "");
    assert!(!output.status.success(), "{output:?}");
    assert!(
        String::from_utf8(output.stderr)
            .unwrap()
            .contains("no paths given")
    );
    let output = sylfmt(&["--help"], "");
    assert!(output.status.success(), "{output:?}");
    assert!(
        String::from_utf8(output.stdout)
            .unwrap()
            .contains("--trailing-commas")
    );
}
