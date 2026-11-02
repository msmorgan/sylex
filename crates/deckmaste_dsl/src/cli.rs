//! The `.syl` formatter's command line, shared by the `sylfmt` binary and
//! `cargo xtask dsl fmt` so their flags cannot drift.
//!
//! ```text
//! [--check | --stdin] [style flags] [paths...]
//! ```
//!
//! Paths are formatted in place; a directory stands for every `.syl` file
//! under it. `--check` writes nothing and fails listing the files that would
//! change. `--stdin` formats standard input to standard output; a path given
//! with it only names the input in error messages. Every file is formatted in
//! memory first, and nothing is written unless every file formats. A file is
//! formatted only if the result reparses, [means the same](crate::same_meaning)
//! and formats to itself.

use std::io::Read as _;
use std::io::Write as _;
use std::path::Path;
use std::path::PathBuf;
use std::process::ExitCode;

use crate::format::Closers;
use crate::format::Style;
use crate::format::TrailingCommas;
use crate::span::FileId;

/// The flags, for `--help`.
pub const HELP: &str = "\
Formats .syl files in place.

Usage: [--check | --stdin] [style flags] [paths...]

  --check                    Write nothing; fail listing the files that would change
  --stdin                    Format standard input to standard output; a path only
                             names the input in errors
  --width N                  Line width (default 100)
  --closers C                Broken inner closers: hugged (default) or own-line
  --trailing-commas T        never (default), bodies, or own-line (wherever a
                             closer has its own line)
  --record-spaces[=BOOL]     `{ a = b }` for flat records (default false)
  --hug-sole-list[=BOOL]     Hug a call's sole list or record argument: `and([`
                             (default true)
  -h, --help                 Print this help
";

/// A parsed command line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Args {
    pub style: Style,
    pub check: bool,
    pub stdin: bool,
    pub paths: Vec<PathBuf>,
    pub help: bool,
}

/// Parses a command line, without the program name.
///
/// A flag's value follows it (`--width 80`) or is joined by `=`
/// (`--width=80`). The two boolean style flags take a value only with `=`, so
/// a bare `--record-spaces` means `true` and never swallows the next path.
/// `--` ends the flags.
///
/// # Errors
///
/// An unknown flag, a missing or invalid value, `--check` with `--stdin`, or
/// `--stdin` with more than one path.
pub fn parse_args(args: impl IntoIterator<Item = String>) -> Result<Args, String> {
    let mut parsed = Args {
        style: Style::default(),
        check: false,
        stdin: false,
        paths: Vec::new(),
        help: false,
    };
    let mut args = args.into_iter();
    let mut flags_done = false;
    while let Some(arg) = args.next() {
        if flags_done || !arg.starts_with('-') || arg == "-" {
            parsed.paths.push(PathBuf::from(arg));
            continue;
        }
        let (flag, joined) = match arg.split_once('=') {
            Some((flag, value)) => (flag.to_owned(), Some(value.to_owned())),
            None => (arg.clone(), None),
        };
        let mut value = |name: &str| {
            joined
                .clone()
                .or_else(|| args.next())
                .ok_or_else(|| format!("{name} needs a value"))
        };
        let style = &mut parsed.style;
        match flag.as_str() {
            "--" if joined.is_none() => flags_done = true,
            "-h" | "--help" if joined.is_none() => parsed.help = true,
            "--check" if joined.is_none() => parsed.check = true,
            "--stdin" if joined.is_none() => parsed.stdin = true,
            "--width" => {
                let text = value("--width")?;
                style.width = text
                    .parse()
                    .map_err(|_| format!("--width: invalid width {text:?}"))?;
            }
            "--closers" => {
                style.closers = match value("--closers")?.as_str() {
                    "hugged" => Closers::Hugged,
                    "own-line" => Closers::OwnLine,
                    other => {
                        return Err(format!(
                            "--closers: expected hugged or own-line, found {other:?}"
                        ));
                    }
                };
            }
            "--trailing-commas" => {
                style.trailing_commas = match value("--trailing-commas")?.as_str() {
                    "never" => TrailingCommas::Never,
                    "bodies" => TrailingCommas::Bodies,
                    "own-line" => TrailingCommas::OwnLine,
                    other => {
                        return Err(format!(
                            "--trailing-commas: expected never, bodies or own-line, found {other:?}"
                        ));
                    }
                };
            }
            "--record-spaces" => style.record_spaces = boolean(&flag, joined.as_deref())?,
            "--hug-sole-list" => style.hug_sole_list = boolean(&flag, joined.as_deref())?,
            _ => return Err(format!("unknown flag {arg:?}")),
        }
    }
    if parsed.check && parsed.stdin {
        return Err("--check and --stdin cannot be used together".to_owned());
    }
    if parsed.stdin && parsed.paths.len() > 1 {
        return Err("--stdin takes at most one path".to_owned());
    }
    Ok(parsed)
}

fn boolean(flag: &str, value: Option<&str>) -> Result<bool, String> {
    match value {
        None | Some("true") => Ok(true),
        Some("false") => Ok(false),
        Some(other) => Err(format!("{flag}: expected true or false, found {other:?}")),
    }
}

/// Runs the command line `args` (without the program name). Without paths it
/// formats `default_paths`, and fails if there are none.
#[must_use]
pub fn main(args: impl IntoIterator<Item = String>, default_paths: &[PathBuf]) -> ExitCode {
    match run(args, default_paths) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

fn run(args: impl IntoIterator<Item = String>, default_paths: &[PathBuf]) -> Result<(), String> {
    let args = parse_args(args)?;
    if args.help {
        print!("{HELP}");
        return Ok(());
    }
    if args.stdin {
        let name = args
            .paths
            .first()
            .map_or_else(|| "<stdin>".to_owned(), |p| p.display().to_string());
        let mut source = String::new();
        std::io::stdin()
            .read_to_string(&mut source)
            .map_err(|e| format!("reading standard input: {e}"))?;
        let formatted = format_source(&name, &source, &args.style)?;
        return std::io::stdout()
            .write_all(formatted.as_bytes())
            .map_err(|e| format!("writing standard output: {e}"));
    }
    let paths = if args.paths.is_empty() { default_paths } else { &args.paths };
    if paths.is_empty() {
        return Err(format!("no paths given\n\n{HELP}"));
    }
    let changed = format_paths(paths, &args.style, args.check)?;
    if args.check {
        for path in &changed {
            println!("{}", path.display());
        }
        if !changed.is_empty() {
            return Err(format!("{} would change", files(changed.len())));
        }
    } else {
        println!("{} changed", files(changed.len()));
    }
    Ok(())
}

/// `n` files, as `1 file` or `n files`.
fn files(n: usize) -> String {
    if n == 1 { "1 file".to_owned() } else { format!("{n} files") }
}

/// Formats every `.syl` file named by `paths` (a directory stands for the
/// files under it), returning the files whose text changes, sorted. Unless
/// `check`, writes those files, but only once every file has formatted.
///
/// # Errors
///
/// Every file that cannot be read or formatted, one per line; nothing is
/// written then. Or the first write that fails.
pub fn format_paths(paths: &[PathBuf], style: &Style, check: bool) -> Result<Vec<PathBuf>, String> {
    let mut files_found = Vec::new();
    for path in paths {
        if path.is_dir() {
            collect(path, &mut files_found)?;
        } else {
            files_found.push(path.clone());
        }
    }
    files_found.sort();
    files_found.dedup();
    let mut errors = Vec::new();
    let mut changed = Vec::new();
    for path in files_found {
        let name = path.display().to_string();
        let result = std::fs::read_to_string(&path)
            .map_err(|e| format!("{name}: {e}"))
            .and_then(|source| Ok((format_source(&name, &source, style)?, source)));
        match result {
            Ok((formatted, source)) if formatted != source => changed.push((path, formatted)),
            Ok(_) => {}
            Err(error) => errors.push(error),
        }
    }
    if !errors.is_empty() {
        return Err(format!(
            "{}\n{} failed to format; nothing written",
            errors.join("\n"),
            files(errors.len())
        ));
    }
    if !check {
        for (path, formatted) in &changed {
            std::fs::write(path, formatted)
                .map_err(|e| format!("writing {}: {e}", path.display()))?;
        }
    }
    Ok(changed.into_iter().map(|(path, _)| path).collect())
}

/// Formats one file's source.
///
/// # Errors
///
/// The rendered parse error, or a report that the printer's output fails to
/// reparse, changes the file's meaning or does not format to itself.
pub fn format_source(name: &str, source: &str, style: &Style) -> Result<String, String> {
    let file = crate::parse(FileId(0), source).map_err(|e| e.render(name, source))?;
    let formatted = crate::format(&file, style);
    // Guard against a printer defect before anything is written.
    let reparsed = crate::parse(FileId(0), &formatted).map_err(|e| {
        format!(
            "{name}: formatter output does not parse: {}",
            e.render(name, &formatted)
        )
    })?;
    if !crate::same_meaning(&file, &reparsed) {
        return Err(format!(
            "{name}: formatting would change the file's meaning"
        ));
    }
    if crate::format(&reparsed, style) != formatted {
        return Err(format!("{name}: formatter output is not stable"));
    }
    Ok(formatted)
}

fn collect(dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), String> {
    let entries = std::fs::read_dir(dir).map_err(|e| format!("reading {}: {e}", dir.display()))?;
    for entry in entries {
        let path = entry
            .map_err(|e| format!("reading {}: {e}", dir.display()))?
            .path();
        if path.is_dir() {
            collect(&path, out)?;
        } else if path.extension().is_some_and(|ext| ext == "syl") {
            out.push(path);
        }
    }
    Ok(())
}
