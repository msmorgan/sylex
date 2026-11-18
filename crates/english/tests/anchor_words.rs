//! Anchor Words [CR#614.12c] are read from a face's text across its units
//! (`docs/design/lexicon.md` §2), with no list of the words a card prints.

use std::collections::BTreeSet;

use sylex_lexicon::CardScope;
use sylex_lexicon::LexicalReading;
use sylex_syntax::CompiledGrammar;
use sylex_syntax::Reading;
use sylex_vocabulary::LexicalEnvironment;

const FRONTIER_SIEGE: [&str; 3] = [
    "As this enchantment enters, choose Khans or Dragons.",
    "• Khans — At the beginning of each of your main phases, add {G}{G}.",
    "• Dragons — Whenever a creature you control with flying enters, you may have it fight target creature you don't control.",
];

fn anchor_words(
    env: &LexicalEnvironment,
    grammar: &CompiledGrammar,
    card: &str,
    lines: &[&str],
) -> Vec<String> {
    let lines: Vec<String> = lines.iter().map(|line| (*line).to_owned()).collect();
    sylex_english::anchors::anchor_words(
        &lines,
        &CardScope::full_names([card]),
        &env.lexicon,
        grammar,
    )
    .into_iter()
    .collect()
}

fn scope(card: &str, anchors: &[&str]) -> CardScope {
    let mut scope = CardScope::full_names([card]);
    for word in anchors {
        scope.insert_anchor_word(word);
    }
    scope
}

fn readings(
    env: &LexicalEnvironment,
    grammar: &CompiledGrammar,
    text: &str,
    scope: &CardScope,
) -> Vec<Reading> {
    grammar
        .parse_analyzed(&env.lexicon.analyze_source_scoped(text, None, scope))
        .readings()
        .collect()
}

/// The Anchor Word leaves of a Reading, in order.
fn anchors_of(reading: &Reading, found: &mut Vec<String>) {
    match reading {
        Reading::Leaf(LexicalReading::AnchorWord { word }) => found.push(word.clone()),
        Reading::Leaf(_) => {}
        Reading::Node(node) => node
            .children
            .iter()
            .for_each(|child| anchors_of(child, found)),
    }
}

fn productions(reading: &Reading, found: &mut BTreeSet<String>) {
    if let Reading::Node(node) = reading {
        found.insert(node.production.to_string());
        node.children
            .iter()
            .for_each(|child| productions(child, found));
    }
}

/// A face's Anchor Words are the labels of its bulleted lines that one of
/// its lines offers as an Anchor Choice: a card face's name (*Believe*, of
/// *Reason // Believe*), a plural subtype (*Dragons*), an adjective
/// (*Phyrexian*) and an acronym (*NCR*) among them.
#[test]
fn a_face_s_anchor_words_are_the_bulleted_labels_a_line_offers() {
    let env = LexicalEnvironment::load().unwrap();
    let grammar = sylex_english::bind(&env.lexicon, &env.frames).unwrap();
    assert_eq!(
        anchor_words(&env, &grammar, "Frontier Siege", &FRONTIER_SIEGE),
        ["Dragons", "Khans"]
    );
    assert_eq!(
        anchor_words(
            &env,
            &grammar,
            "Phenomenon Investigators",
            &[
                "As this creature enters, choose Believe or Doubt.",
                "• Believe — Whenever a nontoken creature you control dies, create a 2/2 black Horror enchantment creature token.",
                "• Doubt — At the beginning of your end step, you may return a nonland permanent you own to your hand. If you do, draw a card.",
            ]
        ),
        ["Believe", "Doubt"]
    );
    assert_eq!(
        anchor_words(
            &env,
            &grammar,
            "Mirrodin Besieged",
            &[
                "As this enchantment enters, choose Mirran or Phyrexian.",
                "• Mirran — Whenever you cast an artifact spell, create a 1/1 colorless Myr artifact creature token.",
                "• Phyrexian — At the beginning of your end step, draw a card, then discard a card. Then if there are fifteen or more artifact cards in your graveyard, target opponent loses the game.",
            ]
        ),
        ["Mirran", "Phyrexian"]
    );
    assert_eq!(
        anchor_words(
            &env,
            &grammar,
            "Battle of Hoover Dam",
            &[
                "As this enchantment enters, choose NCR or Legion.",
                "• NCR — At the beginning of your end step, return target creature card with mana value 3 or less from your graveyard to the battlefield with a finality counter on it.",
                "• Legion — Whenever a creature you control dies, put two +1/+1 counters on target creature you control.",
            ]
        ),
        ["Legion", "NCR"]
    );
}

/// A line that offers two words no bulleted line is labeled with offers no
/// Anchor Words (Roots of Life chooses a land type [CR#607.2d]), and bulleted
/// labels no line offers are none either (Cult of Skaro's flavor-labeled
/// modes [CR#207.2d]; Frontier Siege's own lines, one kind at a time).
#[test]
fn a_choice_or_a_label_alone_is_no_anchor_word() {
    let env = LexicalEnvironment::load().unwrap();
    let grammar = sylex_english::bind(&env.lexicon, &env.frames).unwrap();
    let none: [&str; 0] = [];
    assert_eq!(
        anchor_words(
            &env,
            &grammar,
            "Roots of Life",
            &[
                "As this enchantment enters, choose Island or Swamp.",
                "Whenever a land of the chosen type an opponent controls becomes tapped, you gain 1 life.",
            ]
        ),
        none
    );
    assert_eq!(
        anchor_words(
            &env,
            &grammar,
            "Cult of Skaro",
            &[
                "Whenever Cult of Skaro attacks, choose one at random —",
                "• Thay — Put a +1/+1 counter on each artifact creature you control.",
                "• Caan — Draw two cards.",
                "• Sec — Create a 3/3 black Dalek artifact creature token with menace.",
                "• Jast — Each opponent loses 4 life.",
            ]
        ),
        none
    );
    assert_eq!(
        anchor_words(&env, &grammar, "Frontier Siege", &FRONTIER_SIEGE[..1]),
        none
    );
    assert_eq!(
        anchor_words(&env, &grammar, "Frontier Siege", &FRONTIER_SIEGE[1..]),
        none
    );
    // One of the two words labels a line: the choice is not between them.
    assert_eq!(
        anchor_words(&env, &grammar, "Frontier Siege", &FRONTIER_SIEGE[..2]),
        none
    );
}

/// With its Anchor Words in scope each line of Frontier Siege has one
/// Reading: the offer is an Anchor Choice of the two words, never the
/// subtype *Dragons*, and each bulleted line is an ability under its word.
#[test]
fn the_offer_and_each_bulleted_ability_read_their_anchor_words() {
    let env = LexicalEnvironment::load().unwrap();
    let grammar = sylex_english::bind(&env.lexicon, &env.frames).unwrap();
    let scope = scope("Frontier Siege", &["Khans", "Dragons"]);
    for (line, words, production) in [
        (
            FRONTIER_SIEGE[0],
            vec!["Khans", "Dragons"],
            "AnchorWordChoice",
        ),
        (FRONTIER_SIEGE[1], vec!["Khans"], "AnchoredAbility"),
        (FRONTIER_SIEGE[2], vec!["Dragons"], "AnchoredAbility"),
    ] {
        let found = readings(&env, &grammar, line, &scope);
        let [reading] = &found[..] else {
            panic!("{line}: {} Readings", found.len());
        };
        let mut anchors = Vec::new();
        anchors_of(reading, &mut anchors);
        assert_eq!(anchors, words, "{line}");
        let mut used = BTreeSet::new();
        productions(reading, &mut used);
        assert!(used.contains(production), "{line}: {used:?}");
    }
}

/// Admission refuses an Anchor Word the scope does not hold, and admits it
/// where the scope does: the parse and admission take the one scope
/// (`docs/design/english.md` §7). Off its face the same label is a Flavor
/// Word heading a mode, which admission refuses where the scope holds the
/// Anchor Word.
#[test]
fn admission_takes_an_anchor_word_only_in_a_scope_that_holds_it() {
    let env = LexicalEnvironment::load().unwrap();
    let grammar = sylex_english::bind(&env.lexicon, &env.frames).unwrap();
    let held = scope("Frontier Siege", &["Khans", "Dragons"]);
    let other = scope("Frontier Siege", &["Dragons"]);
    let bare = scope("Frontier Siege", &[]);
    for line in &FRONTIER_SIEGE[..2] {
        let found = readings(&env, &grammar, line, &held);
        let [reading] = &found[..] else {
            panic!("{line}: {} Readings", found.len());
        };
        assert_eq!(
            grammar.admit(reading, &env.lexicon, &held),
            Ok(()),
            "{line}"
        );
        for scope in [&other, &bare] {
            assert!(
                grammar.admit(reading, &env.lexicon, scope).is_err(),
                "{line}"
            );
        }
    }
    // Without the word in scope the offer has an unknown word and no
    // Reading, and the bulleted line is a flavor-labeled mode.
    assert_eq!(readings(&env, &grammar, FRONTIER_SIEGE[0], &bare), []);
    let found = readings(&env, &grammar, FRONTIER_SIEGE[1], &bare);
    let [mode] = &found[..] else {
        panic!("{} Readings", found.len());
    };
    let mut anchors = Vec::new();
    anchors_of(mode, &mut anchors);
    assert_eq!(anchors, [] as [String; 0]);
    assert_eq!(grammar.admit(mode, &env.lexicon, &bare), Ok(()));
    assert!(grammar.admit(mode, &env.lexicon, &held).is_err());
}
