//! `cargo xtask english forestwalk`: one unit's forest surveyed without
//! enumerating it: its settled spine (`ForkForest::settled_spine`), then its
//! forks (`ForkForest::forks`), each option with its exact Derivation count.
//!
//! The survey reports the forest and decides nothing
//! (`docs/decisions/english.md` §4): options are listed in the order the
//! forks give them, never by count or rank.

use std::io::Write;

use anyhow::Context;
use anyhow::Result;
use clap::Args;
use sylex_english::corpus::UnitKind;
use sylex_lexicon::LexicalReading;
use sylex_syntax::Answer;
use sylex_syntax::Dependency;
use sylex_syntax::Fork;
use sylex_syntax::ForkKind;
use sylex_syntax::ForkWord;
use sylex_syntax::SettledChild;
use sylex_syntax::SettledNode;
use sylex_syntax::SettledRest;
use sylex_syntax::SettledSpine;
use sylex_vocabulary::LexicalEnvironment;

use super::FetchedUnit;
use super::ParseItem;

#[derive(Debug, Args)]
pub(super) struct ForestwalkArgs {
    /// Text to survey: bare Oracle text or one judgment entry (`+ TEXT`,
    /// `+Category TEXT`), which may state its own card with `<card=NAME>`.
    text: Option<String>,
    /// Survey the text as if it were on the card so named. Nothing is
    /// fetched: the name is an input like the text.
    #[arg(long, value_name = "NAME", conflicts_with = "card")]
    as_card: Option<String>,
    /// Fetch the face so named (a face's name, or its whole card's when the
    /// card has one face) from the supported corpus and survey one of its
    /// units, picked by `--line` or `--type-line`. Takes no text.
    #[arg(long, value_name = "NAME")]
    card: Option<String>,
    /// With `--card`: the face's Nth rules-text line (1-based), as `parse`
    /// labels it.
    #[arg(long, value_name = "N")]
    line: Option<usize>,
    /// With `--card`: the face's type line.
    #[arg(long)]
    type_line: bool,
}

pub(super) fn run(args: &ForestwalkArgs, output: &mut dyn Write) -> Result<()> {
    use sylex_english::judgment::Judge;

    let item = unit(args)?;
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
    let forest = parse.fork_forest()?;
    let total = forest.total();
    writeln!(output, "Derivations: {total}")?;
    let tokens = parse.tokens();
    writeln!(output, "Settled spine:")?;
    match forest.settled_spine() {
        SettledSpine::Root(root) => spine_lines(&root, "  ".to_owned(), 1, tokens, output)?,
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
    Ok(())
}

/// The one unit the arguments name, as a judgment entry.
fn unit(args: &ForestwalkArgs) -> Result<ParseItem> {
    let picks = args.line.is_some() || args.type_line;
    match (&args.card, &args.text) {
        (Some(name), Some(_)) => anyhow::bail!(
            "`--card {name}` fetches that card and surveys its own text, so it takes no text; \
             to survey text as if it were on a card so named, use `--as-card`"
        ),
        (Some(name), None) => fetched_unit(name, args.line, args.type_line),
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
            Ok(item)
        }
        (None, None) => match &args.as_card {
            Some(name) => anyhow::bail!(
                "`--as-card {name}` surveys the text given as if it were on a card so named, \
                 and none was given; to fetch that card and survey its own text, use `--card`"
            ),
            None => anyhow::bail!(
                "nothing to survey: give text, or `--card NAME` with `--line N` or \
                 `--type-line` for a unit of a card's own text"
            ),
        },
    }
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
    // Faces of two cards that share a name list a unit of one text once.
    let listing = || {
        let mut seen = std::collections::HashSet::new();
        units
            .iter()
            .map(|u| format!("  {}: {:?}", u.label(), u.text))
            .filter(|line| seen.insert(line.clone()))
            .collect::<Vec<_>>()
            .join("\n")
    };
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
            "give exactly one of `--line N` and `--type-line` to pick a unit of {}:\n{}",
            faces[0],
            listing()
        ),
    };
    let Some(first) = picked.first() else {
        let missing = line.map_or_else(|| "type line".to_owned(), |n| format!("line {n}"));
        anyhow::bail!("{} has no {missing}; its units:\n{}", faces[0], listing());
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

/// A settled node's lines: its pass-through chain folded into one line,
/// then its children's lines one level deeper. `line` is the line so far:
/// its indent and the name the node bears in its mother.
fn spine_lines(
    node: &SettledNode,
    mut line: String,
    depth: usize,
    tokens: &[char],
    output: &mut dyn Write,
) -> Result<()> {
    let mut node = node;
    loop {
        let surface = span_text(&node.span, tokens);
        match &node.rest {
            SettledRest::Alternatives(n) => {
                line = format!("{line}{} {surface:?}: {n} alternatives", node.category);
                break;
            }
            SettledRest::Splits { production } => {
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
                        writeln!(output, "{line} {surface:?}")?;
                        for (name, child) in children {
                            let mut line = format!("{:indent$}", "", indent = 2 * (depth + 1));
                            if !name.as_str().is_empty() {
                                line = format!("{line}{name}: ");
                            }
                            match child {
                                SettledChild::Node(child) => {
                                    spine_lines(child, line, depth + 1, tokens, output)?;
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
/// then each option with its Derivations.
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
        let mut line = format!(
            "    {:>width$}  {}",
            option.derivations,
            answer(&option.answer)
        );
        for alias in &option.aliases {
            line = format!("{line} = {}", answer(alias));
        }
        writeln!(output, "{line}")?;
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
mod tests {
    use clap::Parser;

    use super::super::EnglishArgs;
    use super::super::run;

    #[derive(Parser)]
    struct Cli {
        #[command(flatten)]
        args: EnglishArgs,
    }

    fn survey(inputs: &[&str]) -> Result<String, String> {
        let cli = Cli::try_parse_from(
            ["english", "forestwalk"]
                .into_iter()
                .chain(inputs.iter().copied()),
        )
        .map_err(|e| e.to_string())?;
        let mut output = Vec::new();
        run(&cli.args, &mut output).map_err(|e| e.to_string())?;
        Ok(String::from_utf8(output).unwrap())
    }

    /// *Draugr Necromancer*'s type line has one Reading: the spine is its
    /// whole tree, pass-through chains folded, and there is no fork.
    #[test]
    fn a_unit_with_one_reading_is_its_whole_spine() {
        let output = survey(&["--card", "Draugr Necromancer", "--type-line"]).unwrap();
        assert_eq!(
            output,
            r##"Draugr Necromancer type line: "Snow Creature — Zombie Cleric"
Derivations: 1
Settled spine:
  Document [TextTypeLine] > TypeLine [SupertypePrefix] "Snow Creature — Zombie Cleric"
    modifier: "Snow"#1 supertype:snow Invariant
    head: TypeLine [TypesWithSubtypes] "Creature — Zombie Cleric"
      head: CardTypes [SingleCardType] > "Creature"#1 type:creature Singular {number=Singular}
      punctuation
      subtypes: Subtypes [MultipleSubtypes] "Zombie Cleric"
        first: "Zombie"#1 subtype:Zombie Singular {number=Singular}
        rest: Subtypes [SingleSubtype] > "Cleric"#1 subtype:Cleric Singular {number=Singular}
Forks: 0
"##
        );
    }

    /// *Crook of Condemnation*'s first line: "from a graveyard" modifies
    /// "card" or adjoins the Verb Phrase, so the spine stops at the
    /// Sentence and one dependency fork has two options.
    #[test]
    fn a_dependency_fork_lists_its_options_with_their_derivations() {
        let output = survey(&["--card", "Crook of Condemnation", "--line", "1"]).unwrap();
        assert_eq!(
            output,
            r##"Crook of Condemnation line 1: "{1}, {T}: Exile target card from a graveyard."
Derivations: 2
Settled spine:
  Document [TextAbility] > Ability [ActivatedAbility] "{1}, {T}: Exile target card from a graveyard."
    cost: ActivationCost [ActivationPayment] > Cost [SymbolCost] > SymbolCosts [CostSequence] "{1}, {T}"
      first: Symbols [SingleSymbol] > "{1}"#1 symbol:{1} Invariant
      punctuation
      rest: SymbolCosts [SingleSymbolCost] > Symbols [SingleSymbol] > "{T}"#1 symbol:{T} Invariant
    punctuation
    effect: Effect [SentenceEffect] > Sentences [SingleSentence] > Sentence [Declarative] "Exile target card from a graveyard.": its children differ between Derivations
Forks: 1
  dependency "from"#1
    1  adjunct of VerbPhrase "Exile"#1
    1  modifier of Nominal "card"#1
"##
        );
    }

    /// Stated text is read as given, its card stated by `--as-card` or by
    /// the entry, and nothing is fetched; a Category prefix is the root.
    #[test]
    fn stated_text_is_surveyed_with_its_stated_card() {
        let text = "Jace draws two cards.";
        let as_card = survey(&["--as-card", "Jace, Cunning Castaway", text]).unwrap();
        let entry = survey(&[&format!("<card=Jace, Cunning Castaway> {text}")]).unwrap();
        assert_eq!(as_card, entry);
        assert!(
            as_card.contains(
                "subject: NounPhrase [ProperName] > Name [NamedParameter] > \"Jace\"#1 \
                 name:Jace, Cunning Castaway Invariant"
            ),
            "{as_card}"
        );
        let bare = survey(&[text]).unwrap();
        assert!(bare.contains("Derivations: 0\n"), "{bare}");
        let phrase = survey(&["+NounPhrase target creature"]).unwrap();
        assert!(
            phrase.contains("Settled spine:\n  NounPhrase [Determined] \"target creature\"\n"),
            "{phrase}"
        );
    }

    /// *Lightning Bolt* is also a face of *Emeritus of Conflict // Lightning
    /// Bolt*; the faces' units have one text, so the name picks one unit.
    #[test]
    fn a_face_name_two_cards_share_picks_their_one_text() {
        let output = survey(&["--card", "Lightning Bolt", "--line", "1"]).unwrap();
        assert!(
            output.starts_with(
                "Lightning Bolt line 1: \"Lightning Bolt deals 3 damage to any target.\"\n\
                 Derivations: 1\n"
            ),
            "{output}"
        );
    }

    #[test]
    fn each_bad_unit_choice_names_the_way_out() {
        let fail = |inputs: &[&str]| survey(inputs).unwrap_err();
        let units = "  Lightning Bolt type line: \"Instant\"\n  \
                     Lightning Bolt line 1: \"Lightning Bolt deals 3 damage to any target.\"";
        assert_eq!(
            fail(&["--card", "Lightning Bolt"]),
            format!(
                "give exactly one of `--line N` and `--type-line` to pick a unit of \
                 Lightning Bolt:\n{units}"
            )
        );
        assert_eq!(
            fail(&["--card", "Lightning Bolt", "--line", "1", "--type-line"]),
            fail(&["--card", "Lightning Bolt"])
        );
        assert_eq!(
            fail(&["--card", "Lightning Bolt", "--line", "2"]),
            format!("Lightning Bolt has no line 2; its units:\n{units}")
        );
        assert_eq!(
            fail(&[
                "--card",
                "Delver of Secrets // Insectile Aberration",
                "--type-line"
            ]),
            "`--card Delver of Secrets // Insectile Aberration` names faces of more than one \
             name (Delver of Secrets; Insectile Aberration); give one face's name"
        );
        assert!(fail(&["--line", "1", "Flying"]).contains("stated text is surveyed as given"));
        assert!(fail(&["--type-line", "Flying"]).contains("stated text is surveyed as given"));
        assert!(
            fail(&["--card", "Lightning Bolt", "--line", "1", "Flying"])
                .contains("use `--as-card`")
        );
        assert!(fail(&["--as-card", "Lightning Bolt"]).contains("use `--card`"));
        assert!(fail(&[]).contains("nothing to survey"));
        assert!(fail(&["Flying => NounPhrase \"Flying\""]).contains("checks no assertions"));
    }
}
