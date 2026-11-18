use clap::Parser;
use sylex_english::judgment::Judge;
use sylex_english::judgment::Judgment;
use sylex_english::judgment::Located;
use sylex_english::judgment::Outcome;
use sylex_syntax::Answer;
use sylex_syntax::ForkForest;
use sylex_syntax::ForkKind;
use sylex_syntax::Parse;
use sylex_vocabulary::LexicalEnvironment;

use self::oracle::Oracle;
use super::super::EnglishArgs;
use super::super::run;
use super::selector::Narrowing;

mod oracle;

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
/// whole tree, pass-through chains folded without repeating spans on
/// expanded mothers, and there is no fork.
#[test]
fn a_unit_with_one_reading_is_its_whole_spine() {
    let output = survey(&["--card", "Draugr Necromancer", "--type-line"]).unwrap();
    assert_eq!(
        output,
        r##"Draugr Necromancer type line: "Snow Creature — Zombie Cleric"
Derivations: 1
Settled spine:
  Document [TextTypeLine] > TypeLine [SupertypePrefix]
    modifier: "Snow"#1 supertype:snow Invariant
    head: TypeLine [TypesWithSubtypes]
      head: CardTypes [SingleCardType] > "Creature"#1 type:creature Singular {number=Singular}
      subtypes: Subtypes [MultipleSubtypes]
        first: "Zombie"#1 subtype:Zombie Singular {number=Singular}
        rest: Subtypes [SingleSubtype] > "Cleric"#1 subtype:Cleric Singular {number=Singular}
Forks: 0
"##
    );
}

/// *Crook of Condemnation*'s first line: "from a graveyard" modifies
/// "card" or adjoins the Verb Phrase, so the settled activation cost folds
/// to one line, the spine stops at the Sentence, and one dependency fork
/// has two options.
#[test]
fn a_dependency_fork_lists_its_options_with_their_derivations() {
    let output = survey(&["--card", "Crook of Condemnation", "--line", "1"]).unwrap();
    assert_eq!(
        output,
        r##"Crook of Condemnation line 1: "{1}, {T}: Exile target card from a graveyard."
Derivations: 2
Settled spine:
  Document [TextAbility] > Ability [ActivatedAbility]
    cost: ActivationCost "{1}, {T}"
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
        phrase.contains("Settled spine:\n  NounPhrase [Determined]\n"),
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

/// `--card` with neither `--line` nor `--type-line` lists the units of every
/// matching face, deduplicating identical units across cards that share a face
/// name (*Lightning Bolt*).
#[test]
fn card_without_a_line_lists_its_units() {
    assert_eq!(
        survey(&["--card", "Lightning Bolt"]).unwrap(),
        "Lightning Bolt type line: \"Instant\"\n\
         Lightning Bolt line 1: \"Lightning Bolt deals 3 damage to any target.\"\n"
    );
    let both = survey(&["--card", "Delver of Secrets // Insectile Aberration"]).unwrap();
    assert!(
        both.starts_with(
            "Delver of Secrets type line: \"Creature — Human Wizard\"\nDelver of Secrets line 1: "
        ),
        "{both}"
    );
    assert!(
        both.ends_with("Insectile Aberration type line: \"Creature — Human Insect\"\nInsectile Aberration line 1: \"Flying\"\n"),
        "{both}"
    );
}

#[test]
fn each_bad_unit_choice_names_the_way_out() {
    let fail = |inputs: &[&str]| survey(inputs).unwrap_err();
    let units = "  Lightning Bolt type line: \"Instant\"\n  \
                 Lightning Bolt line 1: \"Lightning Bolt deals 3 damage to any target.\"";
    assert_eq!(
        fail(&["--card", "Lightning Bolt", "--line", "1", "--type-line"]),
        format!(
            "give `--line N` or `--type-line`, not both, to pick a unit of \
             Lightning Bolt:\n{units}"
        )
    );
    assert!(
        fail(&["--card", "Lightning Bolt", "--assertions"])
            .contains("pass `--line N` or `--type-line`")
    );
    assert!(
        fail(&[
            "--card",
            "Lightning Bolt",
            "--pick",
            "dependency \"deals\"#1 root"
        ])
        .contains("pass `--line N` or `--type-line`")
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
        fail(&["--card", "Lightning Bolt", "--line", "1", "Flying"]).contains("use `--as-card`")
    );
    assert!(fail(&["--as-card", "Lightning Bolt"]).contains("use `--card`"));
    assert!(fail(&[]).contains("nothing to survey"));
    assert!(fail(&["Flying => NounPhrase \"Flying\""]).contains("checks no assertions"));
}

/// A unit of a card, parsed, for the tests that count its Readings.
fn with_unit<R>(
    card: &str,
    line: usize,
    check: impl FnOnce(&Judge<'_>, &Judgment, &Parse<'_>) -> R,
) -> R {
    let item = super::fetched_unit(card, Some(line), false).unwrap();
    let env = LexicalEnvironment::load().unwrap();
    let grammar = sylex_english::bind(&env.lexicon, &env.frames).unwrap();
    let judge = Judge {
        grammar: &grammar,
        lexicon: &env.lexicon,
    };
    let scope = item.entry.scope(&env.lexicon).unwrap();
    let parse = judge.parse_scoped(&item.entry, &scope).unwrap();
    check(&judge, &item.entry, &parse)
}

/// One narrowing step: how it narrows, the selector as the survey spells
/// it, the question and answer it names, and the Derivations left by the
/// survey's own option counts.
struct Step {
    narrowing: Narrowing,
    selector: String,
    kind: ForkKind,
    /// The question's word start.
    start: usize,
    answer: Answer,
    expected: u128,
}

/// Narrows `forest` to one Derivation by its first fork each time, picking
/// one option (the next in turn) and then excluding the first option of the
/// next fork, and returns the steps with the forest left after each.
fn plan<'p>(forest: &ForkForest<'p>) -> Vec<(Step, ForkForest<'p>)> {
    let mut steps = Vec::new();
    let mut current = forest.clone();
    while current.total() > 1 {
        let forks = current.forks();
        let fork = forks
            .first()
            .expect("a forest of several Derivations has a fork");
        let n = steps.len();
        let (narrowing, option) = if n % 2 == 0 {
            (Narrowing::Pick, &fork.options[(n / 2) % fork.options.len()])
        } else {
            (Narrowing::Exclude, &fork.options[0])
        };
        let (narrowed, expected) = match narrowing {
            Narrowing::Pick => (
                current.pick(fork.kind, &fork.word, &option.answer),
                option.derivations,
            ),
            Narrowing::Exclude => (
                current.exclude(fork.kind, &fork.word, &option.answer),
                current.total() - option.derivations,
            ),
        };
        let narrowed = narrowed.unwrap();
        assert_eq!(narrowed.total(), expected, "the survey's option count");
        steps.push((
            Step {
                narrowing,
                selector: format!(
                    "{} {}",
                    super::fork_name(fork.kind, &fork.word),
                    super::answer(&option.answer)
                ),
                kind: fork.kind,
                start: fork.word.start,
                answer: option.answer.clone(),
                expected,
            },
            narrowed.clone(),
        ));
        current = narrowed;
    }
    steps
}

/// The survey of a unit of a card narrowed by `steps` in order.
fn narrow_unit(card: &str, line: &str, steps: &[&Step], extra: &[&str]) -> Result<String, String> {
    let mut args: Vec<&str> = vec!["--card", card, "--line", line];
    for step in steps {
        args.push(step.narrowing.flag());
        args.push(&step.selector);
    }
    args.extend(extra);
    survey(&args)
}

/// The Derivations each narrowing step of the output says are left.
fn step_counts(output: &str) -> Vec<u128> {
    output
        .lines()
        .filter(|l| l.starts_with("  --pick ") || l.starts_with("  --exclude "))
        .map(|l| {
            let (_, tail) = l.rsplit_once(": ").unwrap();
            tail.strip_suffix(" Derivations")
                .or_else(|| tail.strip_suffix(" Derivation"))
                .unwrap()
                .parse()
                .unwrap()
        })
        .collect()
}

/// *Draugr Necromancer*'s second line (3,312 Derivations): picks and
/// exclusions in turn narrow it to one Reading, and after each step the
/// count printed is the number of the unit's enumerated Readings that give
/// every answer picked and none excluded so far, each Reading's answers read
/// from the Reading itself.
#[test]
fn picks_and_exclusions_narrow_to_the_enumerated_readings_that_agree() {
    with_unit("Draugr Necromancer", 2, |_, _, parse| {
        let forest = parse.fork_forest().unwrap();
        let oracle = Oracle::new(parse, forest.words());
        let readings: Vec<_> = parse.readings().collect();
        assert_eq!(readings.len(), 3312);
        let answers: Vec<_> = readings.iter().map(|r| oracle.answers(r)).collect();
        let mut agreeing: Vec<usize> = (0..readings.len()).collect();
        let steps = plan(&forest);
        assert!(steps.len() >= 4, "{} steps", steps.len());
        for (step, narrowed) in &steps {
            agreeing.retain(|&r| {
                let gives = answers[r]
                    .get(&(step.start, step.kind))
                    .is_some_and(|a| a.names(&step.answer));
                gives == (step.narrowing == Narrowing::Pick)
            });
            assert_eq!(
                narrowed.total(),
                agreeing.len() as u128,
                "after `{} {}`",
                step.narrowing.flag(),
                step.selector
            );
        }
        assert_eq!(agreeing.len(), 1);
        let taken: Vec<&Step> = steps.iter().map(|(s, _)| s).collect();
        let output = narrow_unit("Draugr Necromancer", "2", &taken, &[]).unwrap();
        assert_eq!(
            step_counts(&output),
            taken.iter().map(|s| s.expected).collect::<Vec<_>>()
        );
        assert!(output.contains("\nDerivations: 1\n"), "{output}");
        assert!(output.ends_with("Forks: 0\n"), "{output}");
    });
}

/// *Remove Enchantments*' only line, a forest of thousands of Derivations,
/// narrows to one Derivation without enumerating, and each count the command
/// prints is the Derivations the survey's option counts leave.
#[test]
fn a_large_forest_narrows_by_the_surveys_option_counts() {
    with_unit("Remove Enchantments", 1, |_, _, parse| {
        let forest = parse.fork_forest().unwrap();
        let steps = plan(&forest);
        assert!(steps.len() >= 4, "{} steps", steps.len());
        let taken: Vec<&Step> = steps.iter().map(|(s, _)| s).collect();
        let output = narrow_unit("Remove Enchantments", "1", &taken, &[]).unwrap();
        assert_eq!(
            step_counts(&output),
            taken.iter().map(|s| s.expected).collect::<Vec<_>>()
        );
        assert!(output.contains("\nDerivations: 1\n"), "{output}");
        assert!(!output.contains("Readings:"), "{output}");
    });
}

/// The assertions an output prints for its one Reading.
fn printed_assertions(output: &str) -> Vec<String> {
    let mut lines = output
        .lines()
        .skip_while(|l| !l.starts_with("Assertions isolating "));
    assert!(
        lines
            .next()
            .unwrap()
            .contains("not a judgment to copy unread")
    );
    lines
        .take_while(|l| l.starts_with("  "))
        .map(|l| l.trim().to_owned())
        .filter(|l| !l.starts_with("not isolated") && !l.starts_with("(none"))
        .collect()
}

/// The printed assertions of a unit narrowed to one Reading, written as a
/// judgment entry, hold under `Judge::evaluate` and are satisfied by that
/// Reading only among the unit's Readings.
fn check_isolation(card: &str, line: usize, stated: usize) {
    with_unit(card, line, |judge, unit, parse| {
        let forest = parse.fork_forest().unwrap();
        let steps = plan(&forest);
        assert!(steps.len() >= stated, "{} steps", steps.len());
        let narrowed = steps.last().unwrap().1.readings(1).unwrap();
        let [reading] = &narrowed[..] else {
            panic!("{} Readings left", narrowed.len());
        };
        let taken: Vec<&Step> = steps.iter().map(|(s, _)| s).collect();
        let output = narrow_unit(card, &line.to_string(), &taken, &["--assertions"]).unwrap();
        let assertions = printed_assertions(&output);
        assert!(!assertions.is_empty(), "{output}");
        let written = format!(
            "+ <card={}> {} => {}",
            unit.card.as_deref().unwrap(),
            unit.text,
            assertions.join("; ")
        );
        let entry = sylex_english::judgment::parse_file("isolation", &written)
            .unwrap()
            .remove(0);
        assert_eq!(judge.evaluate(&entry), Outcome::Holds, "{written}");
        let endorsed: Vec<_> = parse
            .readings()
            .filter(|r| {
                let (text, constituents) = judge.grammar.constituents(r, judge.lexicon).unwrap();
                judge.endorses(&entry, &Located { text, constituents })
            })
            .collect();
        assert_eq!(endorsed, std::slice::from_ref(reading), "{written}");
    });
}

#[test]
fn the_printed_assertions_isolate_a_reading_of_a_small_unit() {
    check_isolation("Crook of Condemnation", 1, 1);
}

#[test]
fn the_printed_assertions_isolate_a_reading_of_a_large_unit() {
    check_isolation("Draugr Necromancer", 2, 4);
}

/// With `--limit` at the Derivations left, each Reading prints only its
/// subtrees at the stops of `Settled spine:`, each labeled with the function
/// its stop bears on the spine.
#[test]
fn a_limit_prints_each_remaining_reading_at_the_spines_stops() {
    let output = survey(&[
        "--card",
        "Crook of Condemnation",
        "--line",
        "1",
        "--limit",
        "2",
    ])
    .unwrap();
    assert!(output.contains("\nForks: 1\n"), "{output}");
    let readings = output.split("\nReadings: 2\n").nth(1).expect(&output);
    let (first, second) = readings.split_once("\nReading 2:\n").unwrap();
    assert!(
        first.starts_with("Reading 1:\n  effect: Sentence [Declarative] > "),
        "{output}"
    );
    assert!(
        second.starts_with("  effect: Sentence [Declarative] > "),
        "{output}"
    );
    assert!(!first.contains("ActivationCost"), "{output}");
    assert!(!second.contains("ActivationCost"), "{output}");
    let modifier = "modifier: PrepositionPhrase [Valence:PrepositionPhrase/NP@NominalAndPredicate]";
    let adjunct = "adjunct: PrepositionPhrase [Valence:PrepositionPhrase/NP@NominalAndPredicate]";
    assert_ne!(
        first.contains(modifier),
        second.contains(modifier),
        "{output}"
    );
    assert_ne!(
        first.contains(adjunct),
        second.contains(adjunct),
        "{output}"
    );
    // Without a selector or a limit the output has no Readings.
    let default = survey(&["--card", "Crook of Condemnation", "--line", "1"]).unwrap();
    assert!(!default.contains("Readings:"), "{default}");
    // With two stops on the spine, each Reading prints both stops in spine
    // order.
    let two_stops = survey(&[
        "Exile target card from a graveyard. Exile target creature from a graveyard.",
        "--limit",
        "4",
    ])
    .unwrap();
    assert!(two_stops.contains("\nDerivations: 4\n"), "{two_stops}");
    let (spine_part, two_stop_readings) =
        two_stops.split_once("\nReadings: 4\n").expect(&two_stops);
    assert!(
        spine_part
            .contains("    first: Sentence [Declarative] \"Exile target card from a graveyard.\":"),
        "{spine_part}"
    );
    assert!(
        spine_part.contains("    rest: Sentences [SingleSentence] > Sentence [Declarative] \"Exile target creature from a graveyard.\":"),
        "{spine_part}"
    );
    for header in [
        "Reading 1:\n",
        "\nReading 2:\n",
        "\nReading 3:\n",
        "\nReading 4:\n",
    ] {
        let (_, body) = two_stop_readings
            .split_once(header)
            .expect(two_stop_readings);
        let section = body.split("\nReading ").next().unwrap();
        let first_at = section
            .find("  first: Sentence [Declarative] > ")
            .expect(section);
        let rest_at = section
            .find("  rest: Sentence [Declarative] > ")
            .expect(section);
        assert!(first_at < rest_at, "{section}");
    }
    // When no root Constituent is settled, each Reading prints its whole tree.
    let unsettled_root = survey(&["+Clause you cast a spell", "--limit", "2"]).unwrap();
    assert!(
        unsettled_root.contains("  (none: 2 root Constituents have Derivations)\n"),
        "{unsettled_root}"
    );
    let (_, roots_readings) = unsettled_root
        .split_once("\nReadings: 2\n")
        .expect(&unsettled_root);
    let (r1, r2) = roots_readings.split_once("\nReading 2:\n").unwrap();
    assert!(r1.starts_with("Reading 1:\n  Clause [SubjectPredicate]\n"));
    assert!(r2.starts_with("  Clause [SubjectPredicate]\n"));
    assert_ne!(r1.contains("Present"), r2.contains("Present"));
}

/// On *Remove Enchantments* narrowed to two Derivations in its final
/// conjunct, `Settled spine:` folds the settled first sentence and earlier
/// conjuncts to single lines, and `--limit 2` prints only the stopped
/// `Nominal` under each Reading.
#[test]
fn settled_siblings_fold_as_a_large_forest_narrows_and_under_limit() {
    let output = survey(&[
        "--card",
        "Remove Enchantments",
        "--line",
        "1",
        "--pick",
        "dependency \"and\"#2 object of VerbPhrase \"Return\"#1",
        "--pick",
        "dependency \"and\"#3 object of VerbPhrase \"destroy\"#1",
        "--pick",
        "lexical \"attached\"#1 action:attach PastParticiple {finiteness=Nonfinite}",
        "--pick",
        "dependency \"attached\"#1 modifier of Nominal \"Auras\"#1",
        "--pick",
        "dependency \"control\"#2 modifier of Nominal \"permanents\"#1",
        "--pick",
        "lexical \"attached\"#2 action:attach PastParticiple {finiteness=Nonfinite}",
        "--pick",
        "dependency \"attached\"#2 modifier of Nominal \"Auras\"#2",
        "--pick",
        "dependency \"attacking\"#1 modifier of Nominal \"creatures\"#1",
        "--pick",
        "dependency \"control\"#3 modifier of Nominal \"creatures\"#1",
        "--pick",
        "dependency \"control\"#5 modifier of Nominal \"permanents\"#2",
        "--pick",
        "lexical \"attached\"#4 action:attach PastParticiple {finiteness=Nonfinite}",
        "--pick",
        "dependency \"attached\"#4 modifier of Nominal \"Auras\"#4",
        "--pick",
        "dependency \"attacking\"#2 modifier of Nominal \"creatures\"#2",
        "--exclude",
        "dependency \"control\"#6 modifier of NounPhrase \"and\"#3",
        "--limit",
        "2",
    ])
    .unwrap();
    assert!(output.contains("\nDerivations: 2\n"), "{output}");
    let (before_readings, readings) = output.split_once("\nReadings: 2\n").expect(&output);
    let (_, spine) = before_readings
        .split_once("\nSettled spine:\n")
        .expect(before_readings);
    assert!(
        spine.contains("    first: Sentence \"Return to your hand "),
        "{spine}"
    );
    assert!(!spine.contains("\"Return\"#1"), "{spine}");
    assert!(
        spine.contains("first: NounPhrase \"all other Auras attached to permanents you control\""),
        "{spine}"
    );
    assert!(
        spine.contains(
            "head: Nominal \"other Auras attached to attacking creatures your opponents \
             control\": 2 alternatives"
        ),
        "{spine}"
    );
    let (first, second) = readings.split_once("\nReading 2:\n").unwrap();
    assert!(
        first.starts_with("Reading 1:\n  head: Nominal ["),
        "{readings}"
    );
    assert!(second.starts_with("  head: Nominal ["), "{readings}");
    assert!(!readings.contains("\"destroy\"#1"), "{readings}");
    assert!(first.contains("\"Auras\"#4"), "{first}");
    assert!(second.contains("\"Auras\"#4"), "{second}");
}

/// `--limit 0` folds closed subtrees with or without `--assertions` (folding a
/// single remaining Reading's root to one line) and still permits
/// `--assertions` when one Derivation remains.
#[test]
fn limit_zero_folds_a_single_reading_and_still_isolates_assertions() {
    let pick = "dependency \"from\"#1 modifier of Nominal \"card\"#1";
    let folded_spine = "Derivations: 1\n\
                        Settled spine:\n  \
                        Document \"{1}, {T}: Exile target card from a graveyard.\"\n\
                        Forks: 0\n";
    let one_without = survey(&[
        "--card",
        "Crook of Condemnation",
        "--line",
        "1",
        "--pick",
        pick,
        "--limit",
        "0",
    ])
    .unwrap();
    assert!(one_without.ends_with(folded_spine), "{one_without}");
    assert!(!one_without.contains("\"Exile\"#1"), "{one_without}");
    assert!(!one_without.contains("Readings:"), "{one_without}");
    assert!(!one_without.contains("Assertions"), "{one_without}");

    let two_without = survey(&[
        "--card",
        "Crook of Condemnation",
        "--line",
        "1",
        "--limit",
        "0",
    ])
    .unwrap();
    assert!(
        two_without.contains("    cost: ActivationCost \"{1}, {T}\"\n"),
        "{two_without}"
    );
    assert!(!two_without.contains("Readings:"), "{two_without}");
    assert!(!two_without.contains("Assertions"), "{two_without}");

    let output = survey(&[
        "--card",
        "Crook of Condemnation",
        "--line",
        "1",
        "--pick",
        pick,
        "--limit",
        "0",
        "--assertions",
    ])
    .unwrap();
    assert!(output.contains(folded_spine), "{output}");
    assert!(!output.contains("\"Exile\"#1"), "{output}");
    assert!(!printed_assertions(&output).is_empty(), "{output}");
    let two = survey(&[
        "--card",
        "Crook of Condemnation",
        "--line",
        "1",
        "--limit",
        "0",
        "--assertions",
    ])
    .unwrap_err();
    assert!(
        two.contains("`--assertions` with `--limit 0` needs 1 Derivation, and the slice has 2"),
        "{two}"
    );
}

/// *Remove Enchantments*' `lexical "attached"#1` fork has the alias
/// `dependency "attached"#1`: each option prints the alias's answer on its
/// own line under it, a whole selector that `--pick` narrows by.
#[test]
fn an_aliased_fork_prints_each_alias_answer_on_its_own_selector_line() {
    let card = ["--card", "Remove Enchantments", "--line", "1"];
    let output = survey(&card).unwrap();
    let lines: Vec<&str> = output.lines().collect();
    let header = lines
        .iter()
        .position(|l| l.starts_with("  lexical \"attached\"#1"))
        .unwrap_or_else(|| panic!("no attached fork: {output}"));
    let aliases = ["dependency \"attached\"#1 "];
    assert!(
        lines[header].ends_with(", aliases: dependency \"attached\"#1"),
        "{output}"
    );
    let mut options = 0;
    let mut at = header + 1;
    while at < lines.len() && lines[at].starts_with("    ") {
        let option = lines[at];
        assert!(!option.contains(" = "), "{output}");
        let column = option.find(char::is_alphabetic).unwrap();
        let count = option[..column].trim();
        let indent = " ".repeat(column + 2);
        for (offset, alias) in aliases.iter().enumerate() {
            let line = lines[at + 1 + offset];
            let selector = line
                .strip_prefix(indent.as_str())
                .unwrap_or_else(|| panic!("alias line `{line}` is not indented to {column}+2"));
            assert!(selector.starts_with(alias), "{output}");
            let picked = survey(&[card[0], card[1], card[2], card[3], "--pick", selector]).unwrap();
            assert!(
                picked.contains(&format!("Derivations: {count}\n")),
                "`{selector}` should leave {count} Derivations:\n{picked}"
            );
        }
        options += 1;
        at += 1 + aliases.len();
    }
    assert!(options >= 2, "{output}");
}

#[test]
fn a_selector_that_names_no_fork_or_option_is_an_error_naming_it() {
    let crook = |selectors: &[&str]| {
        let mut args = vec!["--card", "Crook of Condemnation", "--line", "1"];
        args.extend(selectors);
        survey(&args).unwrap_err()
    };
    let word = "dependency \"from\"#1";
    let card = format!("{word} modifier of Nominal \"card\"#1");
    let verb = format!("{word} adjunct of VerbPhrase \"Exile\"#1");
    let no_word = crook(&["--pick", "dependency \"zebra\"#1 root"]);
    assert!(
        no_word.contains("`--pick dependency \"zebra\"#1 root`"),
        "{no_word}"
    );
    assert!(no_word.contains("no word of the unit"), "{no_word}");
    let no_occurrence = crook(&["--exclude", "dependency \"from\"#2 root"]);
    assert!(
        no_occurrence.contains("\"from\" occurs 1 times"),
        "{no_occurrence}"
    );
    let no_fork = crook(&["--pick", "lexical \"from\"#1 prep:from Invariant"]);
    assert!(
        no_fork.contains("lexical \"from\"#1 is no fork or alias of the unit"),
        "{no_fork}"
    );
    assert!(no_fork.contains("dependency \"from\"#1"), "{no_fork}");
    let no_option = crook(&["--pick", &format!("{word} root")]);
    assert!(
        no_option.contains("has no option `root`; its options:"),
        "{no_option}"
    );
    assert!(
        no_option.contains(&format!("  {}", &card[word.len() + 1..])),
        "{no_option}"
    );
    let removed = crook(&["--pick", &card, "--pick", &verb]);
    assert!(removed.contains(&format!("`--pick {verb}`")), "{removed}");
    assert!(removed.contains("an earlier step removed it"), "{removed}");
    let leaves_none = crook(&["--exclude", &card, "--exclude", &verb]);
    assert!(
        leaves_none.contains(&format!("`--exclude {verb}` leaves no Derivation")),
        "{leaves_none}"
    );
    let too_many = crook(&["--assertions"]);
    assert!(
        too_many.contains("between 1 and `--limit` (1) Derivations, and the slice has 2"),
        "{too_many}"
    );
}
