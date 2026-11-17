//! `cargo xtask english forestwalk`: one unit's forest surveyed without
//! enumerating it: its settled spine (`ForkForest::settled_spine`), then its
//! forks (`ForkForest::forks`), each option with its exact Derivation count;
//! `--card NAME` alone lists the matching faces' units.
//!
//! `--pick` and `--exclude` narrow the forest by fork options first
//! (`ForkForest::pick`, `ForkForest::exclude`; [`selector`]) and survey what
//! is left; with at most `--limit` Derivations left, one Derivation expands the
//! spine itself and two or more print each Reading's subtrees at the spine's
//! stops (`ForkForest::settled_readings`), while `--assertions` adds the
//! judgment assertions that isolate each remaining Reading among the unit's
//! Readings ([`isolate`]).
//!
//! The survey reports the forest and decides nothing
//! (`docs/design/english.md` §4): options are listed in the order the
//! forks give them, never by count or rank.

use std::io::Write;

use anyhow::Context;
use anyhow::Result;
use clap::Args;
use sylex_english::corpus::UnitKind;
use sylex_english::judgment::Judge;
use sylex_english::judgment::Located;
use sylex_lexicon::LexicalReading;
use sylex_syntax::Answer;
use sylex_syntax::Dependency;
use sylex_syntax::Fork;
use sylex_syntax::ForkForest;
use sylex_syntax::ForkKind;
use sylex_syntax::ForkWord;
use sylex_syntax::NarrowError;
use sylex_syntax::SettledChild;
use sylex_syntax::SettledNode;
use sylex_syntax::SettledRest;
use sylex_syntax::SettledSpine;
use sylex_vocabulary::LexicalEnvironment;

use self::selector::Narrowing;
use self::selector::Selector;
use super::FetchedUnit;
use super::ParseItem;

mod isolate;
mod selector;

#[derive(Debug, Args)]
pub(super) struct ForestwalkArgs {
    /// Text to survey: bare Oracle text or one judgment entry (`+ TEXT`,
    /// `+Category TEXT`), which may state its own card with `<card=NAME>`.
    text: Option<String>,
    /// Survey the text as if it were on the card so named. Nothing is
    /// fetched: the name is an input like the text.
    #[arg(long, value_name = "NAME", conflicts_with = "card")]
    as_card: Option<String>,
    /// Fetch the faces so named (a face's name, or its whole card's) from the
    /// supported corpus: alone, list their units; with `--line` or
    /// `--type-line`, survey that unit (then `NAME` must name one face). Takes
    /// no text.
    #[arg(long, value_name = "NAME")]
    card: Option<String>,
    /// With `--card`: survey the face's Nth rules-text line (1-based).
    #[arg(long, value_name = "N")]
    line: Option<usize>,
    /// With `--card`: survey the face's type line.
    #[arg(long)]
    type_line: bool,
    #[command(flatten)]
    steps: selector::Steps,
    /// With 2 to N Derivations left after narrowing, print each Reading's
    /// subtrees at the spine's stops (or whole Reading when no root Constituent
    /// is settled); with 1 left, the spine is the whole Reading. `--limit 0`
    /// folds a single Derivation's spine to one line, for when only
    /// `--assertions` are wanted.
    #[arg(long, value_name = "N", default_value_t = 1)]
    limit: u64,
    /// Print judgment assertions that isolate each remaining Reading among
    /// the unit's Readings, by greedy set cover. Diagnostic output, not a
    /// judgment to copy unread; needs between 1 and max(`--limit`, 1)
    /// Derivations.
    #[arg(long)]
    assertions: bool,
}

pub(super) fn run(args: &ForestwalkArgs, output: &mut dyn Write) -> Result<()> {
    let selectors = args.steps.selectors()?;
    let item = match target(args, &selectors)? {
        Target::List(listing) => {
            writeln!(output, "{listing}")?;
            return Ok(());
        }
        Target::Survey(item) => *item,
    };
    let env = LexicalEnvironment::load().context("loading the lexical environment")?;
    if let Some(name) = &args.as_card {
        anyhow::ensure!(
            sylex_english::judgment::is_card_name(&env.lexicon, name),
            "`--as-card {name}` is no card name of the lexicon"
        );
    }
    let grammar = sylex_english::bind(&env.lexicon, &env.frames)?;
    let judge = Judge {
        grammar: &grammar,
        lexicon: &env.lexicon,
    };
    let entry = &item.entry;
    let scope = entry.scope(&env.lexicon).map_err(anyhow::Error::msg)?;
    let parse = judge
        .parse_scoped(entry, &scope)
        .map_err(anyhow::Error::msg)?;
    writeln!(output, "{}: {:?}", entry.location, entry.text)?;
    if !parse.vocabulary_gaps.is_empty() {
        writeln!(output, "Vocabulary gaps: {:?}", parse.vocabulary_gaps)?;
    }
    let surveyed = parse.fork_forest()?;
    let forest = if selectors.is_empty() {
        surveyed
    } else {
        narrow(&surveyed, &selectors, output)?
    };
    let total = forest.total();
    let limit = u128::from(args.limit);
    let assertion_limit = limit.max(1);
    if args.assertions && !(1..=assertion_limit).contains(&total) {
        if limit == 0 {
            anyhow::bail!(
                "`--assertions` with `--limit 0` needs 1 Derivation, and the slice has {total}: \
                 narrow it with `--pick` and `--exclude`, or raise `--limit`"
            );
        }
        anyhow::bail!(
            "`--assertions` needs between 1 and `--limit` ({limit}) Derivations, and the slice \
             has {total}: narrow it with `--pick` and `--exclude`, or raise `--limit`"
        );
    }
    writeln!(output, "Derivations: {total}")?;
    let tokens = parse.tokens();
    let spine = forest.settled_spine();
    let fold_closed = total > 1 || limit == 0;
    writeln!(output, "Settled spine:")?;
    match &spine {
        SettledSpine::Root(root) => {
            spine_lines(root, "  ".to_owned(), 1, fold_closed, tokens, output)?;
        }
        SettledSpine::Roots(0) => writeln!(output, "  (no Derivation)")?,
        SettledSpine::Roots(n) => {
            writeln!(output, "  (none: {n} root Constituents have Derivations)")?;
        }
    }
    let forks = forest.forks();
    writeln!(output, "Forks: {}", forks.len())?;
    for fork in &forks {
        fork_lines(fork, total, output)?;
    }
    let show_readings = total >= 2 && total <= limit;
    let remaining = if show_readings {
        forest.settled_readings(limit)
    } else if args.assertions {
        forest.settled_readings(assertion_limit)
    } else {
        None
    };
    let mut found = match (&remaining, args.assertions) {
        (_, false) => Vec::new().into_iter(),
        (Some(readings), true) => {
            isolations(&judge, &parse, readings.iter().map(|(reading, _)| reading))?.into_iter()
        }
        (None, true) => unreachable!(
            "`--assertions` is refused unless 1 to max(`--limit`, 1) Derivations are left, and \
             then the forest's Readings are taken"
        ),
    };
    if let Some(readings) = &remaining {
        if show_readings {
            writeln!(output, "Readings: {total}")?;
        }
        for (index, (_, node)) in readings.iter().enumerate() {
            let label = if show_readings {
                writeln!(output, "Reading {}:", index + 1)?;
                reading_lines(&spine, node, tokens, output)?;
                format!("Reading {}", index + 1)
            } else {
                "the Reading".to_owned()
            };
            if let Some((isolation, readings)) = found.next() {
                isolation_lines(&label, &isolation, readings, output)?;
            }
        }
    }
    Ok(())
}

/// The forest narrowed by each selector in turn, printing the Derivations
/// left after each step.
fn narrow<'p>(
    surveyed: &ForkForest<'p>,
    selectors: &[Selector],
    output: &mut dyn Write,
) -> Result<ForkForest<'p>> {
    let questions = selector::questions(&surveyed.forks())?;
    let steps = selectors
        .iter()
        .map(|selector| selector.resolve(surveyed.words(), &questions))
        .collect::<Result<Vec<_>>>()?;
    writeln!(output, "Narrowing from {}:", derivations(surveyed.total()))?;
    let mut forest = surveyed.clone();
    for step in &steps {
        let flag = step.narrowing.flag();
        let narrowed = match step.narrowing {
            Narrowing::Pick => forest.pick(step.kind, &step.word, &step.answer),
            Narrowing::Exclude => forest.exclude(step.kind, &step.word, &step.answer),
        }
        .map_err(|why| match why {
            NarrowError::NoAnswer { .. } => anyhow::anyhow!(
                "`{flag} {}`: no Derivation left gives that option; an earlier step removed it",
                step.text
            ),
            NarrowError::NoWord { .. } => anyhow::anyhow!("`{flag} {}`: {why}", step.text),
        })?;
        anyhow::ensure!(
            narrowed.total() > 0,
            "`{flag} {}` leaves no Derivation",
            step.text
        );
        writeln!(
            output,
            "  {flag} {}: {}",
            step.printed,
            derivations(narrowed.total())
        )?;
        forest = narrowed;
    }
    Ok(forest)
}

/// A count of Derivations, in words.
fn derivations(n: u128) -> String {
    format!("{n} Derivation{}", if n == 1 { "" } else { "s" })
}

/// The assertions isolating each of `narrowed` among the unit's Readings,
/// with how many Readings the unit has.
fn isolations<'r>(
    judge: &Judge<'_>,
    parse: &sylex_syntax::Parse<'_>,
    narrowed: impl Iterator<Item = &'r sylex_syntax::Reading>,
) -> Result<Vec<(isolate::Isolation, usize)>> {
    let located: Vec<(sylex_syntax::Reading, Located)> = parse
        .readings()
        .map(|reading| {
            let (text, constituents) = judge.grammar.constituents(&reading, judge.lexicon)?;
            Ok((reading, Located { text, constituents }))
        })
        .collect::<Result<_>>()?;
    narrowed
        .map(|reading| {
            let at = located
                .iter()
                .position(|(r, _)| r == reading)
                .context("a Reading of the narrowed forest is not among the unit's Readings")?;
            let others: Vec<&Located> = located
                .iter()
                .enumerate()
                .filter(|(index, _)| *index != at)
                .map(|(_, (_, l))| l)
                .collect();
            Ok((
                isolate::isolate(judge, &located[at].1, &others)?,
                located.len(),
            ))
        })
        .collect()
}

/// A Reading's isolating assertions, marked as diagnostic output.
fn isolation_lines(
    label: &str,
    isolation: &isolate::Isolation,
    readings: usize,
    output: &mut dyn Write,
) -> Result<()> {
    writeln!(
        output,
        "Assertions isolating {label} among the unit's {readings} Readings (diagnostic output \
         of a greedy cover, not a judgment to copy unread):"
    )?;
    if isolation.assertions.is_empty() && isolation.unseparated == 0 {
        writeln!(output, "  (none: no other Reading to tell it from)")?;
    }
    for assertion in &isolation.assertions {
        writeln!(output, "  {assertion}")?;
    }
    if isolation.unseparated > 0 {
        writeln!(
            output,
            "  not isolated: {} other Readings satisfy every assertion of these forms that \
             holds on it",
            isolation.unseparated
        )?;
    }
    Ok(())
}

enum Target {
    List(String),
    Survey(Box<ParseItem>),
}

/// What the arguments ask for: a card's unit listing, or one unit to survey.
fn target(args: &ForestwalkArgs, selectors: &[Selector]) -> Result<Target> {
    let picks = args.line.is_some() || args.type_line;
    match (&args.card, &args.text) {
        (Some(name), Some(_)) => anyhow::bail!(
            "`--card {name}` fetches that card and surveys its own text, so it takes no text; \
             to survey text as if it were on a card so named, use `--as-card`"
        ),
        (Some(name), None) if !picks => {
            anyhow::ensure!(
                selectors.is_empty() && !args.assertions,
                "`--pick`, `--exclude` and `--assertions` survey one unit; pass `--line N` or \
                 `--type-line`"
            );
            let units = super::fetched_units(name)?;
            Ok(Target::List(format_units(&units, "")))
        }
        (Some(name), None) => Ok(Target::Survey(Box::new(fetched_unit(
            name,
            args.line,
            args.type_line,
        )?))),
        (None, Some(text)) => {
            anyhow::ensure!(
                !picks,
                "`--line` and `--type-line` pick a unit of a card fetched with `--card`; \
                 stated text is surveyed as given"
            );
            let mut items =
                super::stated_items(std::slice::from_ref(text), args.as_card.as_deref())?;
            let item = items.remove(0);
            let entry = &item.entry;
            anyhow::ensure!(
                entry.groups.is_empty() && entry.negated.is_empty() && entry.readings.is_none(),
                "the survey checks no assertions; give the entry without them, or use `parse`"
            );
            Ok(Target::Survey(Box::new(item)))
        }
        (None, None) => match &args.as_card {
            Some(name) => anyhow::bail!(
                "`--as-card {name}` surveys the text given as if it were on a card so named, \
                 and none was given; to fetch that card and survey its own text, use `--card`"
            ),
            None => anyhow::bail!(
                "nothing to survey: give text, or `--card NAME` (alone to list its units, or \
                 with `--line N` or `--type-line` to survey one)"
            ),
        },
    }
}

/// Faces of two cards that share a name list a unit of one text once, and
/// name their card when two homonymous units differ.
fn format_units(units: &[FetchedUnit], indent: &str) -> String {
    let mut seen = std::collections::HashSet::new();
    units
        .iter()
        .map(|u| {
            let differs = units.iter().any(|o| {
                o.face == u.face && o.kind == u.kind && o.index == u.index && o.text != u.text
            });
            if differs {
                format!("{indent}{} (card: {}): {:?}", u.label(), u.card, u.text)
            } else {
                format!("{indent}{}: {:?}", u.label(), u.text)
            }
        })
        .filter(|line| seen.insert(line.clone()))
        .collect::<Vec<_>>()
        .join("\n")
}

/// The unit of the face named `name` that `line` or `type_line` picks.
fn fetched_unit(name: &str, line: Option<usize>, type_line: bool) -> Result<ParseItem> {
    let units = super::fetched_units(name)?;
    let mut faces: Vec<&str> = units.iter().map(|u| u.face.as_str()).collect();
    faces.sort_unstable();
    faces.dedup();
    anyhow::ensure!(
        faces.len() == 1,
        "`--card {name}` names faces of more than one name ({}); give one face's name",
        faces.join("; ")
    );
    let picked: Vec<&FetchedUnit> = match (line, type_line) {
        (Some(n), false) => units
            .iter()
            .filter(|u| u.kind == UnitKind::Line && u.index + 1 == n)
            .collect(),
        (None, true) => units
            .iter()
            .filter(|u| u.kind == UnitKind::TypeLine)
            .collect(),
        (Some(_), true) | (None, false) => anyhow::bail!(
            "give `--line N` or `--type-line`, not both, to pick a unit of {}:\n{}",
            faces[0],
            format_units(&units, "  ")
        ),
    };
    let Some(first) = picked.first() else {
        let missing = line.map_or_else(|| "type line".to_owned(), |n| format!("line {n}"));
        anyhow::bail!(
            "{} has no {missing}; its units:\n{}",
            faces[0],
            format_units(&units, "  ")
        );
    };
    if picked.iter().any(|u| u.text != first.text) {
        let texts: Vec<String> = picked
            .iter()
            .map(|u| format!("  {} (card: {}): {:?}", u.label(), u.card, u.text))
            .collect();
        anyhow::bail!(
            "faces of more than one card are named {} and their picked units differ:\n{}",
            faces[0],
            texts.join("\n")
        );
    }
    Ok(first.item(None))
}

/// Whether every Derivation builds `node` and all its descendants alike.
fn is_closed(node: &SettledNode) -> bool {
    match &node.rest {
        SettledRest::Alternatives(_) | SettledRest::Splits { .. } => false,
        SettledRest::Children { children, .. } => children.iter().all(|(_, child)| match child {
            SettledChild::Node(child) => is_closed(child),
            SettledChild::Leaf { .. } | SettledChild::Punct => true,
        }),
    }
}

/// A Reading's differing subtrees: the whole Reading when no root
/// Constituent is settled, or the Reading's subtree at each stop of `spine`,
/// in spine order, carrying the function label its stop bears on `spine`.
fn reading_lines(
    spine: &SettledSpine,
    reading: &SettledNode,
    tokens: &[char],
    output: &mut dyn Write,
) -> Result<()> {
    match spine {
        SettledSpine::Roots(_) => spine_lines(reading, "  ".to_owned(), 1, false, tokens, output),
        SettledSpine::Root(spine_root) => stop_lines(spine_root, reading, "", tokens, output),
    }
}

fn stop_lines(
    spine: &SettledNode,
    reading: &SettledNode,
    label: &str,
    tokens: &[char],
    output: &mut dyn Write,
) -> Result<()> {
    match &spine.rest {
        SettledRest::Alternatives(_) | SettledRest::Splits { .. } => {
            let prefix = if label.is_empty() { "  ".to_owned() } else { format!("  {label}: ") };
            spine_lines(reading, prefix, 1, false, tokens, output)
        }
        SettledRest::Children {
            production: spine_prod,
            children: spine_children,
        } => {
            let SettledRest::Children {
                production: reading_prod,
                children: reading_children,
            } = &reading.rest
            else {
                panic!("a Reading's node is built by one Production and split");
            };
            assert_eq!(
                (
                    spine.category,
                    &spine.span,
                    spine_prod,
                    spine_children.len()
                ),
                (
                    reading.category,
                    &reading.span,
                    reading_prod,
                    reading_children.len()
                ),
                "a Reading's tree agrees with the settled spine above every stop"
            );
            let pass_through = matches!(&spine_children[..], [(_, SettledChild::Node(_))]);
            for (index, ((name, spine_child), (_, reading_child))) in
                spine_children.iter().zip(reading_children).enumerate()
            {
                match (spine_child, reading_child) {
                    (SettledChild::Node(s), SettledChild::Node(r)) => {
                        if !is_closed(s) {
                            let child_label = if pass_through { label } else { name.as_str() };
                            stop_lines(s, r, child_label, tokens, output)?;
                        }
                    }
                    (SettledChild::Leaf { .. }, SettledChild::Leaf { .. })
                    | (SettledChild::Punct, SettledChild::Punct) => {}
                    (SettledChild::Node(_), SettledChild::Leaf { .. } | SettledChild::Punct)
                    | (SettledChild::Leaf { .. }, SettledChild::Node(_) | SettledChild::Punct)
                    | (SettledChild::Punct, SettledChild::Node(_) | SettledChild::Leaf { .. }) => {
                        panic!(
                            "child {index} of {} [{spine_prod}] at {:?} differs in kind between \
                         the settled spine and a Reading",
                            spine.category, spine.span
                        )
                    }
                }
            }
            Ok(())
        }
    }
}

/// A settled node's lines: a closed subtree as one `Category "surface"` line
/// when `fold_closed` is set; otherwise its pass-through chain folded into
/// one line without repeating `"surface"`, then its children's lines one
/// level deeper. `line` is the line so far: its indent and the name the node
/// bears in its mother.
fn spine_lines(
    node: &SettledNode,
    mut line: String,
    depth: usize,
    fold_closed: bool,
    tokens: &[char],
    output: &mut dyn Write,
) -> Result<()> {
    if fold_closed && is_closed(node) {
        let surface = span_text(&node.span, tokens);
        writeln!(output, "{line}{} {surface:?}", node.category)?;
        return Ok(());
    }
    let mut node = node;
    loop {
        match &node.rest {
            SettledRest::Alternatives(n) => {
                let surface = span_text(&node.span, tokens);
                line = format!("{line}{} {surface:?}: {n} alternatives", node.category);
                break;
            }
            SettledRest::Splits { production } => {
                let surface = span_text(&node.span, tokens);
                line = format!(
                    "{line}{} [{production}] {surface:?}: its children differ between Derivations",
                    node.category
                );
                break;
            }
            SettledRest::Children {
                production,
                children,
            } => {
                line = format!("{line}{} [{production}]", node.category);
                match &children[..] {
                    [(_, SettledChild::Node(child))] => {
                        line.push_str(" > ");
                        node = child;
                    }
                    [(_, leaf @ SettledChild::Leaf { .. })] => {
                        line = format!("{line} > {}", child_text(leaf, tokens));
                        break;
                    }
                    _ => {
                        writeln!(output, "{line}")?;
                        for (name, child) in children {
                            let mut line = format!("{:indent$}", "", indent = 2 * (depth + 1));
                            if !name.as_str().is_empty() {
                                line = format!("{line}{name}: ");
                            }
                            match child {
                                SettledChild::Node(child) => {
                                    spine_lines(
                                        child,
                                        line,
                                        depth + 1,
                                        fold_closed,
                                        tokens,
                                        output,
                                    )?;
                                }
                                SettledChild::Leaf { .. } | SettledChild::Punct => {
                                    writeln!(output, "{line}{}", child_text(child, tokens))?;
                                }
                            }
                        }
                        return Ok(());
                    }
                }
            }
        }
    }
    writeln!(output, "{line}")?;
    Ok(())
}

/// The text of a span, without the space before it.
fn span_text(span: &std::ops::Range<usize>, tokens: &[char]) -> String {
    tokens[span.clone()]
        .iter()
        .collect::<String>()
        .trim()
        .to_owned()
}

/// A leaf as its word and lexical reading, with its own surface when it
/// reaches past its word's; punctuation as such.
fn child_text(child: &SettledChild, tokens: &[char]) -> String {
    match child {
        SettledChild::Leaf {
            word,
            span,
            reading,
        } => {
            let surface = span_text(span, tokens);
            if surface == word.surface {
                format!("{} {}", word_name(word), lexical(reading))
            } else {
                format!("{} {surface:?} {}", word_name(word), lexical(reading))
            }
        }
        SettledChild::Punct => "punctuation".to_owned(),
        SettledChild::Node(node) => panic!(
            "a node child ({} at {:?}) has lines of its own",
            node.category, node.span
        ),
    }
}

/// A fork's lines: its name, whether it is conditional and its aliases,
/// then each option with its Derivations and answer, and under it one line
/// per alias in the header's order: the alias's name and its answer, a whole
/// selector.
fn fork_lines(fork: &Fork, total: u128, output: &mut dyn Write) -> Result<()> {
    let mut line = format!("  {}", fork_name(fork.kind, &fork.word));
    if fork.conditional {
        line = format!(
            "{line}, conditional: answered in {} of {total} Derivations",
            fork.context
        );
    }
    if !fork.aliases.is_empty() {
        let aliases: Vec<String> = fork
            .aliases
            .iter()
            .map(|(kind, word)| fork_name(*kind, word))
            .collect();
        line = format!("{line}, aliases: {}", aliases.join(", "));
    }
    writeln!(output, "{line}")?;
    let width = fork
        .options
        .iter()
        .map(|o| o.derivations.to_string().len())
        .max()
        .expect("a fork has options");
    for option in &fork.options {
        writeln!(
            output,
            "    {:>width$}  {}",
            option.derivations,
            answer(&option.answer)
        )?;
        for ((kind, word), alias) in fork.aliases.iter().zip(&option.aliases) {
            writeln!(
                output,
                "    {:width$}    {} {}",
                "",
                fork_name(*kind, word),
                answer(alias)
            )?;
        }
    }
    Ok(())
}

/// A fork's name: its kind and word.
fn fork_name(kind: ForkKind, word: &ForkWord) -> String {
    let kind = match kind {
        ForkKind::Lexical => "lexical",
        ForkKind::Dependency => "dependency",
        ForkKind::Frame => "frame",
    };
    format!("{kind} {}", word_name(word))
}

/// A word as `"surface"#N`: its surface and 1-based occurrence, the
/// occurrence written for a surface that occurs once too.
fn word_name(word: &ForkWord) -> String {
    format!("{:?}#{}", word.surface, word.occurrence)
}

/// An option's answer by its grammatical content.
fn answer(answer: &Answer) -> String {
    match answer {
        Answer::Lexical { reading } => lexical(reading),
        Answer::Dependency(Dependency::Root) => "root".to_owned(),
        Answer::Dependency(Dependency::Dependent {
            governor,
            function,
            governor_category,
        }) => match governor {
            Some(word) => format!("{function} of {governor_category} {}", word_name(word)),
            None => format!("{function} of {governor_category} (no head word)"),
        },
        Answer::Frame(productions) => {
            let ids: Vec<String> = productions.iter().map(ToString::to_string).collect();
            format!("frames [{}]", ids.join(", "))
        }
    }
}

/// A lexical reading: its Lexeme, Word Form and the features it sets.
fn lexical(reading: &LexicalReading) -> String {
    match reading {
        LexicalReading::Word(value) => {
            let features =
                serde_json::to_value(&value.features).expect("a feature bundle serializes");
            let serde_json::Value::Object(features) = features else {
                panic!("a feature bundle serializes as an object: {features}");
            };
            let set: Vec<String> = features
                .iter()
                .filter(|(_, v)| !v.is_null())
                .map(|(k, v)| match v {
                    serde_json::Value::String(s) => format!("{k}={s}"),
                    other => format!("{k}={other}"),
                })
                .collect();
            if set.is_empty() {
                format!("{} {:?}", value.lexeme.as_str(), value.form)
            } else {
                format!(
                    "{} {:?} {{{}}}",
                    value.lexeme.as_str(),
                    value.form,
                    set.join(", ")
                )
            }
        }
        LexicalReading::FlavorWord { label } => format!("flavor word {label:?}"),
        LexicalReading::Numeral {
            value, notation, ..
        } => format!("numeral {value} {notation:?}"),
    }
}

#[cfg(test)]
mod tests;
