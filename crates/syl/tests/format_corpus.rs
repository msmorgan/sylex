//! The formatter over every `.syl` file under `plugins_v3/`: in every style,
//! formatting preserves the tree and is idempotent.

use std::path::Path;
use std::path::PathBuf;

use sylex_syl::Closers;
use sylex_syl::FileId;
use sylex_syl::Style;
use sylex_syl::TrailingCommas;
use sylex_syl::format;
use sylex_syl::parse;
use sylex_syl::same_meaning;

fn collect(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).expect("readable directory") {
        let path = entry.expect("directory entry").path();
        if path.is_dir() {
            collect(&path, out);
        } else if path.extension().is_some_and(|ext| ext == "syl") {
            out.push(path);
        }
    }
}

/// Every combination of the style options, at the default width, plus the
/// default style at a narrow and a wide width.
fn styles() -> Vec<Style> {
    let mut styles = Vec::new();
    for closers in [Closers::Hugged, Closers::OwnLine] {
        for trailing_commas in [
            TrailingCommas::Never,
            TrailingCommas::Bodies,
            TrailingCommas::OwnLine,
        ] {
            for record_spaces in [false, true] {
                for hug_first_list in [false, true] {
                    styles.push(Style {
                        closers,
                        trailing_commas,
                        record_spaces,
                        hug_first_list,
                        ..Style::default()
                    });
                }
            }
        }
    }
    for width in [40, 60, 140] {
        styles.push(Style {
            width,
            ..Style::default()
        });
    }
    styles
}

/// What is wrong with formatting `file` in `style`, if anything.
fn check(file: &sylex_syl::File, style: &Style) -> Option<String> {
    let once = format(file, style);
    let reparsed = match parse(FileId(0), &once) {
        Ok(reparsed) => reparsed,
        Err(error) => {
            return Some(format!(
                "output does not parse: {}",
                error.render("formatted", &once)
            ));
        }
    };
    if !same_meaning(&reparsed, file) {
        return Some("tree changed".to_owned());
    }
    (format(&reparsed, style) != once).then(|| "not idempotent".to_owned())
}

#[test]
fn formatting_the_corpus_preserves_trees_and_is_idempotent() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins_v3");
    let mut paths = Vec::new();
    collect(&root, &mut paths);
    paths.sort();
    assert!(!paths.is_empty(), "no .syl files under {}", root.display());
    let sources: Vec<_> = paths
        .iter()
        .map(|path| {
            let source = std::fs::read_to_string(path).expect("readable .syl file");
            let file = parse(FileId(0), &source).expect("corpus file parses");
            (path, file)
        })
        .collect();
    // One thread per style.
    let failures: Vec<String> = std::thread::scope(|scope| {
        let workers: Vec<_> = styles()
            .into_iter()
            .map(|style| {
                let sources = &sources;
                scope.spawn(move || {
                    let mut failures = Vec::new();
                    for (path, file) in sources {
                        if let Some(failure) = check(file, &style) {
                            failures.push(format!("{} in {style:?}: {failure}", path.display()));
                        }
                    }
                    failures
                })
            })
            .collect();
        workers
            .into_iter()
            .flat_map(|worker| worker.join().expect("worker finishes"))
            .collect()
    });
    assert!(
        failures.is_empty(),
        "{} failures:\n{}",
        failures.len(),
        failures[..failures.len().min(30)].join("\n")
    );
}
