//! `cargo xtask dsl fmt --stdin`, the mode a `jj fix` tool runs.

use std::io::Write as _;
use std::process::Command;
use std::process::Stdio;

fn run_stdin(args: &[&str], input: &str) -> std::process::Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_cargo-xtask"))
        .args(["dsl", "fmt", "--stdin"])
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("cargo-xtask starts");
    child
        .stdin
        .take()
        .expect("piped stdin")
        .write_all(input.as_bytes())
        .expect("stdin accepts the input");
    child.wait_with_output().expect("cargo-xtask finishes")
}

#[test]
fn stdin_is_formatted_to_stdout() {
    let output = run_stdin(&["--record-spaces"], "card {a = {b = 1},}");
    assert!(output.status.success(), "{output:?}");
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "card {\n  a = { b = 1 }\n}\n"
    );
}

#[test]
fn a_stdin_parse_error_fails_with_the_named_path_and_no_output() {
    let output = run_stdin(&["cards/Bad.syl"], "card {\n  a = (\n}");
    assert!(!output.status.success(), "{output:?}");
    assert_eq!(output.stdout, b"");
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("cards/Bad.syl:2:7: expected a value, found '('"),
        "{stderr}"
    );
}

#[test]
fn formatting_leaves_every_spelling_as_written() {
    let input = "template t(p: Predicate, xs: List(Predicate)): Predicate {\n  \
                 spelling = \"non~$p< and $xs#<and>> %VerbPhrase: for $p@Ordinal\",\n  \
                 body = $p\n}\n";
    let output = run_stdin(&[], input);
    assert!(output.status.success(), "{output:?}");
    assert_eq!(String::from_utf8(output.stdout).unwrap(), input);
}
