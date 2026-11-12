//! `cargo xtask english`: a thin CLI over `sylex_english`'s corpus,
//! stratum profiler and census.

use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use std::path::PathBuf;

use anyhow::Context;
use anyhow::Result;
use clap::Args;
use clap::Subcommand;
use sylex_english::census;
use sylex_english::corpus::Corpus;
use sylex_vocabulary::LexicalEnvironment;

mod forests;

#[derive(Debug, Args)]
pub struct EnglishArgs {
    #[command(subcommand)]
    command: EnglishCommand,
}

#[derive(Debug, Subcommand)]
enum EnglishCommand {
    /// Profile every Corpus Unit into its Complexity Stratum and run the
    /// census: read, known-wrong-only and unread per Stratum.
    Strata(StrataArgs),
    /// Inspect judgments and their cheapest Readings.
    Parse(ParseArgs),
    /// Write every Corpus Unit's Packed Forest, with exact Derivation
    /// counts, to one SQLite database. Questions are SQL over it.
    Forests(forests::ForestsArgs),
}

#[derive(Debug, Args)]
struct StrataArgs {
    /// An Oracle Cards JSON-lines file to read in place of the pinned
    /// snapshot.
    #[arg(long)]
    data: Option<PathBuf>,
    /// Only faces of cards with this name (repeatable).
    #[arg(long = "card")]
    cards: Vec<String>,
    /// Where to write the full report as JSON.
    #[arg(long, default_value = "target/english/strata.json")]
    output: PathBuf,
    /// Skeletons listed per Stratum in the summary.
    #[arg(long, default_value_t = 15)]
    top: usize,
    /// Fail when a unit at or below the completed Stratum is neither read nor
    /// deferred, or when a deferred unit is read.
    #[arg(long)]
    check: bool,
}

pub fn run(args: &EnglishArgs, output: &mut dyn Write) -> Result<()> {
    match &args.command {
        EnglishCommand::Strata(args) => strata(args, output),
        EnglishCommand::Parse(args) => parse(args, output),
        EnglishCommand::Forests(args) => forests::run(args, output),
    }
}

fn strata(args: &StrataArgs, output: &mut dyn Write) -> Result<()> {
    let env = LexicalEnvironment::load().context("loading the lexical environment")?;
    let keep = |face: &sylex_english::corpus::Face| {
        args.cards.is_empty()
            || args
                .cards
                .iter()
                .any(|c| *c == face.card || *c == face.name)
    };
    let corpus = match &args.data {
        Some(data) => {
            let file = File::open(data).with_context(|| format!("opening {}", data.display()))?;
            Corpus::read(BufReader::new(file), keep)?
        }
        None => Corpus::load(keep)?,
    };
    let judgments: Vec<_> = sylex_english::judgment::load_dir(&sylex_english::judgments_dir())?
        .into_iter()
        .filter(|(name, _)| name != "seed.txt")
        .flat_map(|(_, entries)| entries)
        .collect();
    let report = census::run(&corpus, &env, &judgments);
    if let Some(parent) = args.output.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut json = std::io::BufWriter::new(File::create(&args.output)?);
    serde_json::to_writer(&mut json, &report)?;
    json.flush()?;
    write!(output, "{}", report.summary(args.top))?;
    if !corpus.stale_corrections.is_empty() {
        writeln!(
            output,
            "\nstale editorial corrections (no face's Oracle text matches):"
        )?;
        for correction in &corpus.stale_corrections {
            writeln!(output, "  {}", correction.describe())?;
        }
    }
    writeln!(output, "report: {}", args.output.display())?;
    if args.check && !report.regressions.is_empty() {
        anyhow::bail!(
            "{} units at or below the completed stratum are not read",
            report.regressions.len()
        );
    }
    if args.check && !report.not_segmentable.is_empty() {
        anyhow::bail!(
            "{} units have no single product segmentation",
            report.not_segmentable.len()
        );
    }
    if args.check && !corpus.stale_corrections.is_empty() {
        anyhow::bail!(
            "{} editorial corrections match no face; update or remove them in corrections.txt",
            corpus.stale_corrections.len()
        );
    }
    if !corpus.unmatched_deferrals.is_empty() {
        writeln!(output, "\ndeferred lines that match no corpus unit:")?;
        for line in &corpus.unmatched_deferrals {
            writeln!(output, "  {line}")?;
        }
    }
    if args.check && !corpus.unmatched_deferrals.is_empty() {
        anyhow::bail!(
            "{} deferred lines match no corpus unit; update or remove them",
            corpus.unmatched_deferrals.len()
        );
    }
    if args.check && !report.stale_deferrals.is_empty() {
        anyhow::bail!(
            "{} deferred units are read; remove them from deferred.txt",
            report.stale_deferrals.len()
        );
    }
    Ok(())
}

#[derive(Debug, Args)]
struct ParseArgs {
    /// Text to parse: bare Oracle text or judgment entries (`+ TEXT =>
    /// ASSERTIONS`); quote each input separately. An entry may state its own
    /// card with `<card=NAME>`.
    inputs: Vec<String>,
    /// Parse each input as if it were on the card so named. Nothing is
    /// fetched: the name is an input like the text.
    #[arg(long, value_name = "NAME", conflicts_with = "card")]
    as_card: Option<String>,
    /// Fetch the faces so named (a face's name, or its whole card's) from
    /// the supported corpus and parse their own type lines and rules-text
    /// lines. Takes no text.
    #[arg(long, value_name = "NAME")]
    card: Option<String>,
    /// Maximum number of cheapest Readings to display per input.
    #[arg(long, default_value_t = 8)]
    limit: usize,
}

/// One text `parse` reads.
struct ParseItem {
    /// Printed before the item when it begins a fetched face.
    header: Option<String>,
    entry: sylex_english::judgment::Judgment,
    kind: sylex_english::corpus::UnitKind,
    /// Whether to print the entry's outcome.
    evaluate: bool,
}

/// The texts the inputs state, each as a judgment entry, with `as_card` the
/// card of every entry.
fn stated_items(inputs: &[String], as_card: Option<&str>) -> Result<Vec<ParseItem>> {
    use sylex_english::corpus::UnitKind;
    use sylex_english::judgment;

    inputs
        .iter()
        .enumerate()
        .map(|(index, input)| {
            let input = input.trim();
            anyhow::ensure!(!input.is_empty(), "input {} is empty", index + 1);
            let marked = input.starts_with(['+', '*']);
            let normalized = if marked { input.to_owned() } else { format!("+ {input}") };
            let mut entries = judgment::parse_file(&format!("input {}", index + 1), &normalized)?;
            anyhow::ensure!(
                entries.len() == 1,
                "each input must contain one judgment entry"
            );
            let mut entry = entries.remove(0);
            if let Some(name) = as_card {
                anyhow::ensure!(
                    entry.card.is_none(),
                    "input {} states its card with `<card=…>`; give that or `--as-card`, not both",
                    index + 1
                );
                entry.card = Some(name.to_owned());
            }
            let kind = if entry.category.as_deref() == Some("TypeLine") {
                UnitKind::TypeLine
            } else {
                UnitKind::Line
            };
            Ok(ParseItem {
                header: None,
                entry,
                kind,
                evaluate: marked || input.contains(" => "),
            })
        })
        .collect()
}

/// The units of the supported faces named `name` (a face's name or its whole
/// card's), each read as the grammar's root, with
/// its face's own name in scope.
fn fetched_items(name: &str) -> Result<Vec<ParseItem>> {
    use sylex_english::corpus::UnitKind;

    let corpus = Corpus::load(|face| face.name == name || face.card == name)
        .context("loading the supported corpus")?;
    anyhow::ensure!(
        !corpus.faces.is_empty(),
        "`--card {name}` names no face of the supported corpus"
    );
    let mut items = Vec::new();
    let mut last_face = None;
    for unit in &corpus.units {
        let face = &corpus.faces[unit.face];
        let header = (last_face != Some(unit.face))
            .then(|| format!("Face: {} (card: {})", face.name, face.card));
        last_face = Some(unit.face);
        let place = match unit.kind {
            UnitKind::TypeLine => "type line".to_owned(),
            UnitKind::Line => format!("line {}", unit.index + 1),
        };
        items.push(ParseItem {
            header,
            entry: sylex_english::judgment::Judgment {
                location: format!("{} {place}", face.name),
                grammatical: true,
                category: None,
                card: Some(face.name.clone()),
                text: unit.text.clone(),
                groups: Vec::new(),
                negated: Vec::new(),
                readings: None,
            },
            kind: unit.kind,
            evaluate: false,
        });
    }
    Ok(items)
}

fn parse(args: &ParseArgs, output: &mut dyn Write) -> Result<()> {
    use sylex_english::judgment::Judge;
    use sylex_english::profile::Profiler;

    let items = match (&args.card, &args.as_card, args.inputs.is_empty()) {
        (Some(name), _, false) => anyhow::bail!(
            "`--card {name}` fetches that card and parses its own text, so it takes no text; \
             to parse text as if it were on a card so named, use `--as-card`"
        ),
        (None, Some(name), true) => anyhow::bail!(
            "`--as-card {name}` parses the text given as if it were on a card so named, and \
             none was given; to fetch that card and parse its own text, use `--card`"
        ),
        (None, None, true) => {
            anyhow::bail!("nothing to parse: give text, or `--card NAME` for a card's own text")
        }
        (Some(name), _, true) => fetched_items(name)?,
        (None, as_card, false) => stated_items(&args.inputs, as_card.as_deref())?,
    };
    let env = LexicalEnvironment::load().context("loading the lexical environment")?;
    if let Some(name) = &args.as_card {
        anyhow::ensure!(
            sylex_english::judgment::is_card_name(&env.lexicon, name),
            "`--as-card {name}` is no card name of the lexicon"
        );
    }
    let grammar = sylex_english::bind(&env.lexicon, &env.frames)?;
    let frame = sylex_english::segments::segment_frame(&grammar)?;
    let judge = Judge {
        grammar: &grammar,
        lexicon: &env.lexicon,
    };
    let profiler = Profiler::new(&env);
    for ParseItem {
        header,
        entry,
        kind,
        evaluate,
    } in items
    {
        if let Some(header) = header {
            writeln!(output, "{header}")?;
        }
        writeln!(output, "{}: {:?}", entry.location, entry.text)?;
        let text = sylex_upstream::oracle_text::strip_reminder_text(&entry.text);
        let scope = match entry.scope(&env.lexicon) {
            Ok(scope) => scope,
            Err(why) => {
                writeln!(output, "Unparsed: {why}")?;
                continue;
            }
        };
        let analyzed = env.lexicon.analyze_source_scoped(&text, None, &scope);
        let profile = profiler.profile(kind, &analyzed);
        writeln!(output, "Stratum: {}", profile.stratum)?;
        writeln!(
            output,
            "Syntactic Families: {}",
            profile
                .families
                .iter()
                .map(|f| f.name)
                .collect::<Vec<_>>()
                .join(", ")
        )?;
        if evaluate {
            writeln!(output, "Outcome: {}", judge.evaluate_scoped(&entry, &scope))?;
        }
        let parse = match judge.parse_scoped(&entry, &scope) {
            Ok(parse) => parse,
            Err(why) => {
                writeln!(output, "Unparsed: {why}")?;
                continue;
            }
        };
        writeln!(output, "Vocabulary gaps: {:?}", parse.vocabulary_gaps)?;
        let derivations = parse.derivation_count();
        writeln!(output, "Packed Forest derivations: {derivations}")?;
        let segmentation = sylex_english::segments::segments(&parse, &frame);
        let segments = match &segmentation {
            sylex_english::segments::Segmentation::Segments(segments) => segments
                .iter()
                .map(|s| format!("{} {:?}", s.derivations, s.text))
                .collect::<Vec<_>>()
                .join(" | "),
            sylex_english::segments::Segmentation::NotSegmentable(why) => {
                format!("not segmentable: {why:?}")
            }
        };
        writeln!(output, "Segments: {segments}")?;
        writeln!(output, "Surplus: {}", segmentation.surplus(derivations))?;
        match parse.cheapest_tier() {
            Some((key, tied)) => writeln!(
                output,
                "Cheapest tier: {tied} at ranking key {}",
                format_key(key)
            )?,
            None => writeln!(output, "Cheapest tier: none")?,
        }
        for (index, (key, reading)) in parse.best_ranked(args.limit).enumerate() {
            parse.admit(&reading, &env.lexicon)?;
            dump_reading(&grammar, &env, key, &reading, index + 1, output)?;
        }
        writeln!(output)?;
    }
    Ok(())
}

fn dump_reading(
    grammar: &sylex_syntax::CompiledGrammar,
    env: &LexicalEnvironment,
    key: sylex_syntax::RankingKey,
    reading: &sylex_syntax::Reading,
    number: usize,
    output: &mut dyn Write,
) -> Result<()> {
    let (text, constituents) = grammar.constituents(reading, &env.lexicon)?;
    writeln!(
        output,
        "Reading {} (ranking key {}): {text:?}",
        number,
        format_key(key)
    )?;
    let mut nodes = Vec::new();
    preorder_nodes(reading, &mut nodes);
    writeln!(
        output,
        "node\tparent\tProduction\tCategory\tspan (bytes)\tsurface\tfunction\thead lemma / lexeme / form\tGap"
    )?;
    for (index, (c, node)) in constituents.iter().zip(nodes).enumerate() {
        let production = node.node().map_or("", |n| n.production.as_str());
        let head = grammar.head_word(node).map_or_else(
            || "-".to_owned(),
            |word| {
                let lemma = env
                    .lexicon
                    .lexemes()
                    .get(&word.lexeme)
                    .map_or("?", |l| l.lemma.as_str());
                format!("{lemma} / {} / {:?}", word.lexeme.as_str(), word.form)
            },
        );
        writeln!(
            output,
            "{index}\t{:?}\t{production}\t{}\t{:?}\t{:?}\t{}\t{head}\t{:?}",
            c.parent,
            c.category,
            c.span,
            &text[c.span.clone()],
            c.function.map_or("", |function| function.as_str()),
            c.gap
        )?;
    }
    Ok(())
}

/// A Ranking Key as `class/association/cost`, its components in comparison
/// order.
fn format_key(key: sylex_syntax::RankingKey) -> String {
    format!(
        "{}/{}/{}",
        key.attachment_class, key.right_association_words, key.production_cost
    )
}

/// A Reading's nodes in preorder, as [`CompiledGrammar::constituents`]
/// lists their Constituents.
fn preorder_nodes<'a>(
    reading: &'a sylex_syntax::Reading,
    out: &mut Vec<&'a sylex_syntax::Reading>,
) {
    if let Some(node) = reading.node() {
        out.push(reading);
        for child in &node.children {
            preorder_nodes(child, out);
        }
    }
}

#[cfg(test)]
mod tests {
    use clap::Parser;

    use super::*;

    #[derive(Parser)]
    struct Cli {
        #[command(flatten)]
        args: EnglishArgs,
    }

    fn dump(inputs: &[&str]) -> String {
        let cli =
            Cli::try_parse_from(std::iter::once("english").chain(inputs.iter().copied())).unwrap();
        let mut output = Vec::new();
        run(&cli.args, &mut output).unwrap();
        String::from_utf8(output).unwrap()
    }

    #[test]
    fn parse_dumps_structure_and_evaluates_all_inputs_with_a_display_limit() {
        let output = dump(&[
            "parse",
            "--limit",
            "1",
            "Destroy target creature.",
            "+NounPhrase target creature => NounPhrase \"target creature\" head creature",
            "*PrepositionPhrase target creature",
            "+ Destroy target creature. => NounPhrase \"target creature\" as Subject of destroy",
            "+ <card=Seedborn Muse> Untap all permanents you control.",
        ]);
        assert_eq!(output.matches("Stratum:").count(), 5);
        assert_eq!(output.matches("Syntactic Families:").count(), 5);
        assert_eq!(output.matches("Packed Forest derivations:").count(), 5);
        assert_eq!(output.matches("Segments:").count(), 5);
        assert_eq!(output.matches("Surplus:").count(), 5);
        assert_eq!(output.matches("Cheapest tier: ").count(), 5);
        assert!(
            output.contains("Segments: 1 \"Destroy target creature\"\nSurplus: 0"),
            "{output}"
        );
        assert_eq!(output.matches("Outcome:").count(), 4);
        assert!(output.contains("Outcome: holds"));
        assert!(output.contains("Outcome: no Reading satisfies"));
        assert!(output.contains("Reading 1 (ranking key"));
        assert!(!output.contains("Reading 2 (ranking key"));
        assert!(output.contains("Production\tCategory\tspan (bytes)"));
        assert!(output.contains("\"target creature\"\tobject\tcreature /"));
        assert!(
            output.contains("destroy / action:destroy / Plain"),
            "{output}"
        );
    }

    #[test]
    fn parse_reports_gaps_unknown_categories_and_unknown_cards_without_aborting_other_inputs() {
        let output = dump(&[
            "parse",
            "--limit",
            "0",
            "+ Destroy target zzzunknownword.",
            "+MissingCategory target creature",
            "+ <card=Lightning Blot> Draw a card.",
            "*PrepositionPhrase target creature",
        ]);
        assert!(output.contains("Outcome: vocabulary gaps"));
        assert!(output.contains("Unparsed: the grammar has no Category `MissingCategory`"));
        assert!(output.contains(
            "Unparsed: input 3:1: `<card=Lightning Blot>` states a name that is no card name of \
             the lexicon"
        ));
        assert!(output.contains("Outcome: holds"));
        assert!(!output.contains("Reading 1 (ranking key"));
    }

    #[test]
    fn parse_preserves_card_scope_and_evaluates_beyond_the_display_limit() {
        let output = dump(&[
            "parse",
            "--limit",
            "0",
            "+ <card=Jace, Cunning Castaway> Jace draws two cards. => NounPhrase \"Jace\" as Subject of draw",
            "* Jace draws two cards.",
            "Destroy target creature. => NounPhrase \"target creature\" as Object of destroy",
        ]);
        assert_eq!(output.matches("Outcome: holds").count(), 3, "{output}");
        assert!(!output.contains("Reading 1 (ranking key"));
    }

    /// *Jace, Cunning Castaway* shortens its own name: `--as-card` puts the
    /// name in scope as `<card=…>` does, and without it *Jace* is no word.
    #[test]
    fn as_card_parses_the_text_as_if_on_the_card_so_named() {
        let text = "Jace draws two cards.";
        let stated = dump(&["parse", "--as-card", "Jace, Cunning Castaway", text]);
        let entry = dump(&["parse", &format!("<card=Jace, Cunning Castaway> {text}")]);
        assert_eq!(stated, entry);
        assert!(stated.contains("Packed Forest derivations: 1"), "{stated}");
        assert!(stated.contains("name:Jace, Cunning Castaway / Invariant"));
        let bare = dump(&["parse", text]);
        assert!(bare.contains("Packed Forest derivations: 0"), "{bare}");
    }

    /// `--card` fetches the card's own units. *Delver of Secrets* is a
    /// double-faced card: by its whole card's name both faces are read, each
    /// under its own name, and by a face's name that face alone.
    #[test]
    fn card_fetches_the_faces_so_named_and_parses_their_own_units() {
        let both = dump(&[
            "parse",
            "--limit",
            "0",
            "--card",
            "Delver of Secrets // Insectile Aberration",
        ]);
        let headers: Vec<&str> = both.lines().filter(|l| l.starts_with("Face: ")).collect();
        assert_eq!(
            headers,
            [
                "Face: Delver of Secrets (card: Delver of Secrets // Insectile Aberration)",
                "Face: Insectile Aberration (card: Delver of Secrets // Insectile Aberration)",
            ]
        );
        assert!(both.contains("Delver of Secrets type line: \"Creature — Human Wizard\""));
        assert!(
            both.contains("Insectile Aberration line 1: \"Flying\""),
            "{both}"
        );
        assert!(!both.contains("Unparsed"), "{both}");
        let back = dump(&["parse", "--limit", "0", "--card", "Insectile Aberration"]);
        assert_eq!(back.matches("Face: ").count(), 1);
        assert_eq!(
            back.matches("Packed Forest derivations:").count(),
            2,
            "{back}"
        );
    }

    /// *Gisela, the Broken Blade* melds into *Brisela, Voice of Nightmares*,
    /// another card's name, which *into* takes after *meld*; her text also
    /// shortens her own name.
    #[test]
    fn a_meld_card_s_own_text_reads_its_meld_result_as_a_name_complement() {
        let output = dump(&[
            "parse",
            "--limit",
            "1",
            "--card",
            "Gisela, the Broken Blade",
        ]);
        assert!(
            output.contains(
                "OtherCardNameComplement\tNameComplement\t148..176\t\"Brisela, Voice of \
                 Nightmares\"\tcomplement"
            ),
            "{output}"
        );
        assert!(output.contains("name:Gisela, the Broken Blade / Invariant"));
        assert!(!output.contains("derivations: 0"), "{output}");
    }

    #[test]
    fn card_and_as_card_each_say_which_flag_was_meant() {
        let fail = |inputs: &[&str]| {
            let cli = Cli::try_parse_from(std::iter::once("english").chain(inputs.iter().copied()))
                .unwrap();
            run(&cli.args, &mut Vec::new()).unwrap_err().to_string()
        };
        let with_text = fail(&["parse", "--card", "Seedborn Muse", "Flying"]);
        assert!(with_text.contains("use `--as-card`"), "{with_text}");
        let without_text = fail(&["parse", "--as-card", "Seedborn Muse"]);
        assert!(without_text.contains("use `--card`"), "{without_text}");
        assert!(fail(&["parse"]).contains("nothing to parse"));
        assert!(fail(&["parse", "--card", "Lightning Blot"]).contains("names no face"));
        assert_eq!(
            fail(&["parse", "--as-card", "Lightning Blot", "Draw a card."]),
            "`--as-card Lightning Blot` is no card name of the lexicon"
        );
        let twice = fail(&[
            "parse",
            "--as-card",
            "Seedborn Muse",
            "<card=Seedborn Muse> Flying",
        ]);
        assert!(twice.contains("not both"), "{twice}");
        assert!(
            Cli::try_parse_from(["english", "parse", "--card", "A", "--as-card", "B"]).is_err()
        );
    }

    #[test]
    fn parse_rejects_bad_limits_and_malformed_entries() {
        assert!(Cli::try_parse_from(["english", "parse", "--limit", "no", "text"]).is_err());
        for input in [
            "",
            "+ <card=Unclosed text",
            "+ Destroy target creature. => nonsense",
            "+ Flying\n+ Haste",
        ] {
            let args = ParseArgs {
                inputs: vec![input.to_owned()],
                as_card: None,
                card: None,
                limit: 8,
            };
            assert!(parse(&args, &mut Vec::new()).is_err(), "{input}");
        }
    }
}
