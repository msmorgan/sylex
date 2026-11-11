//! A name is number-neutral (`docs/decisions/lexicon.md` §6): the agreement
//! its text shows fixes its Number in the Reading, and where the text shows
//! none it stays unvalued, in one Reading.

use sylex_lexicon::CardScope;
use sylex_lexicon::LexicalReading;
use sylex_syntax::Reading;
use sylex_vocabulary::LexicalEnvironment;
use sylex_vocabulary::card_name_id;

/// The Number of each Constituent from the root down to the first leaf of
/// `card`'s name, innermost last.
fn numbers_above_name(reading: &Reading, card: &str) -> Vec<(String, Option<String>)> {
    fn walk(
        reading: &Reading,
        name: dpsi::Ident,
        path: &mut Vec<(String, Option<String>)>,
    ) -> bool {
        match reading {
            Reading::Leaf(LexicalReading::Word(value)) => value.lexeme == name,
            Reading::Leaf(_) => false,
            Reading::Node(node) => {
                path.push((
                    node.category.to_string(),
                    node.features.get("Number").map(ToString::to_string),
                ));
                if node.children.iter().any(|child| walk(child, name, path)) {
                    return true;
                }
                path.pop();
                false
            }
        }
    }
    let mut path = Vec::new();
    assert!(walk(reading, card_name_id(card), &mut path), "{card}");
    path
}

/// The Readings of `text` on `card`'s face.
fn readings(env: &LexicalEnvironment, card: &str, text: &str) -> Vec<Reading> {
    let grammar = sylex_english::bind(&env.lexicon, &env.frames).unwrap();
    let scope = CardScope::own_names([card_name_id(card)]);
    let parse = grammar.parse_analyzed(&env.lexicon.analyze_source_scoped(text, None, &scope));
    parse.readings().collect()
}

/// The Number of the innermost Clause above the name.
fn clause_number(path: &[(String, Option<String>)]) -> Option<String> {
    path.iter()
        .rev()
        .find(|(category, _)| category == "Clause")
        .and_then(|(_, number)| number.clone())
}

#[test]
fn subject_verb_agreement_fixes_a_name_s_number() {
    let env = LexicalEnvironment::load().unwrap();
    for (card, text, number) in [
        (
            "The Notary Hobbits",
            "When The Notary Hobbits enter, draw a card.",
            "Plural",
        ),
        (
            "Mizzium Mortars",
            "Mizzium Mortars deals 4 damage to target creature you don't control.",
            "Singular",
        ),
    ] {
        let found = readings(&env, card, text);
        assert!(!found.is_empty(), "{text}");
        for reading in &found {
            let path = numbers_above_name(reading, card);
            assert_eq!(clause_number(&path).as_deref(), Some(number), "{text}");
        }
    }
}

/// From Falcon and Redwing: no agreement shows the Number of the name after
/// *on*, so the name's Noun Phrase leaves it unvalued, in one Reading.
#[test]
fn a_name_whose_text_shows_no_number_leaves_it_unvalued() {
    let env = LexicalEnvironment::load().unwrap();
    let card = "Falcon and Redwing";
    let found = readings(&env, card, "Put a +1/+1 counter on Falcon and Redwing.");
    assert_eq!(found.len(), 1);
    let path = numbers_above_name(&found[0], card);
    let noun_phrase = path
        .iter()
        .rev()
        .find(|(category, _)| category == "NounPhrase")
        .unwrap();
    assert_eq!(noun_phrase.1, None);
}
