//! Another card's name stands only after *named* or *Partner with*
//! (`docs/decisions/lexicon.md` §6), so it never reads as a Subject however its Number is declared. With
//! every card name number-neutral, as a parse that reads a name's Number off
//! its agreement needs, *Exile*, *Remove* and *Turn* (of *Turn // Burn*) are
//! other cards' names that would otherwise read *[Name] targets …* here.

use std::collections::BTreeSet;

use sylex_english::corpus::card_name_lexeme;
use sylex_lexicon::Lexicon;
use sylex_vocabulary::LexicalEnvironment;

const UNITS: [(&str, &str); 6] = [
    (
        "Excise",
        "Exile target attacking creature unless its controller pays {X}.",
    ),
    (
        "Soul Nova",
        "Exile target attacking creature and all Equipment attached to it.",
    ),
    (
        "Unified Strike",
        "Exile target attacking creature if its power is less than or equal to the number of Soldiers on the battlefield.",
    ),
    (
        "Mystifying Maze",
        "{4}, {T}: Exile target attacking creature an opponent controls. At the beginning of the next end step, return it to the battlefield tapped under its owner's control.",
    ),
    (
        "Reconnaissance",
        "{0}: Remove target attacking creature you control from combat and untap it.",
    ),
    (
        "Kaust, Eyes of the Glade",
        "{T}: Turn target face-down attacking creature you control face up.",
    ),
];

/// The lexicon with every card name number-neutral and, unless
/// `card_names` holds, no longer marked a card name, its Shortened Name
/// dropped with the mark that scopes it to its own card.
fn neutral(env: &LexicalEnvironment, card_names: bool) -> Lexicon {
    let lexemes = env.lexicon.lexemes().values().cloned().map(|mut lexeme| {
        if lexeme.card_name {
            lexeme.properties.features.remove("Number");
            lexeme.card_name = card_names;
            if !card_names {
                lexeme.forms[0].surfaces = None;
                lexeme.card_scoped_surfaces.clear();
            }
        }
        lexeme
    });
    Lexicon::new(lexemes).unwrap()
}

/// For each unit, how many of its Readings hold a name.
fn named_readings(lexicon: &Lexicon, env: &LexicalEnvironment) -> Vec<(&'static str, usize)> {
    let grammar = sylex_english::bind(lexicon, &env.frames).unwrap();
    UNITS
        .iter()
        .map(|(card, text)| {
            let scope = BTreeSet::from([card_name_lexeme(card)]);
            let parse = grammar.parse_analyzed(&lexicon.analyze_source_scoped(text, None, &scope));
            let readings: Vec<_> = parse.readings().collect();
            assert!(!readings.is_empty(), "{card}: no Reading");
            let named = readings
                .iter()
                .filter(|reading| {
                    let (_, constituents) = grammar.constituents(reading, lexicon).unwrap();
                    constituents
                        .iter()
                        .any(|c| matches!(c.category.as_str(), "Name" | "NameComplement"))
                })
                .count();
            (*card, named)
        })
        .collect()
}

#[test]
fn another_card_s_name_is_no_subject_even_when_number_neutral() {
    let env = LexicalEnvironment::load().unwrap();
    let named = named_readings(&neutral(&env, true), &env);
    assert!(named.iter().all(|(_, n)| *n == 0), "{named:?}");
}

/// The control: the same lexicon with card names unmarked reads each unit
/// with a name as its Subject, so the test above is not vacuous.
#[test]
fn without_the_card_name_feature_each_unit_reads_a_name_as_subject() {
    let env = LexicalEnvironment::load().unwrap();
    let named = named_readings(&neutral(&env, false), &env);
    assert!(named.iter().all(|(_, n)| *n > 0), "{named:?}");
}
