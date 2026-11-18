//! A Set Name is read from the text before the noun *expansion*
//! [CR#206.3a..206.3c] (`docs/design/lexicon.md` §2), with no list of the
//! set names a card may cite. Golgothian Sylex's text is the frame, its
//! *Antiquities* replaced by set names of `data/mtgjson/SetList.json` shapes.

use sylex_lexicon::CardScope;
use sylex_lexicon::LexicalReading;
use sylex_syntax::CompiledGrammar;
use sylex_syntax::Reading;
use sylex_vocabulary::LexicalEnvironment;

const SYLEX: &str = "{1}, {T}: Each nontoken permanent with a name originally printed in the NAME expansion is sacrificed by its controller.";

fn scope(card: &str) -> CardScope {
    CardScope::full_names([card])
}

/// For each Reading of `text` in `scope`, whether it holds a Set Name
/// Constituent that spells exactly `name`.
fn set_name_readings(
    env: &LexicalEnvironment,
    grammar: &CompiledGrammar,
    text: &str,
    name: &str,
    scope: &CardScope,
) -> Vec<bool> {
    let parse = grammar.parse_analyzed(&env.lexicon.analyze_source_scoped(text, None, scope));
    assert!(
        parse.vocabulary_gaps.is_empty(),
        "{text}: {:?}",
        parse.vocabulary_gaps
    );
    parse
        .readings()
        .map(|reading| {
            let (realized, constituents) = grammar.constituents(&reading, &env.lexicon).unwrap();
            constituents
                .iter()
                .any(|c| c.category.as_str() == "SetName" && realized[c.span.clone()] == *name)
        })
        .collect()
}

/// Each set name, in another card's text and, where it is spelled like a
/// card's name, in that card's own: every Reading reads it as one Set Name,
/// and there are as many Readings as Golgothian Sylex's own text has, so no
/// Name, subtype or run inside it builds another analysis of it.
#[test]
fn a_set_name_is_one_set_name_in_every_reading() {
    let env = LexicalEnvironment::load().unwrap();
    let grammar = sylex_english::bind(&env.lexicon, &env.frames).unwrap();
    let sylex = set_name_readings(
        &env,
        &grammar,
        &SYLEX.replace("NAME", "Antiquities"),
        "Antiquities",
        &scope("Golgothian Sylex"),
    );
    assert!(!sylex.is_empty() && sylex.iter().all(|held| *held));
    for (name, card) in [
        ("Time Spiral", "Golgothian Sylex"),
        ("Time Spiral", "Time Spiral"),
        ("Urza's Saga", "Golgothian Sylex"),
        ("Urza's Saga", "Urza's Saga"),
        ("Phyrexia: All Will Be One", "Golgothian Sylex"),
        ("The Brothers' War", "Golgothian Sylex"),
        ("Shadows over Innistrad", "Golgothian Sylex"),
        // Its first word is also the plural subtype *Adventure*.
        ("Adventures in the Forgotten Realms", "Golgothian Sylex"),
        ("Warhammer 40,000 Commander", "Golgothian Sylex"),
        ("30th Anniversary Edition", "Golgothian Sylex"),
    ] {
        let held = set_name_readings(
            &env,
            &grammar,
            &SYLEX.replace("NAME", name),
            name,
            &scope(card),
        );
        assert!(
            held.iter().all(|held| *held),
            "{name} in {card}'s text: {held:?}"
        );
        assert_eq!(held.len(), sylex.len(), "{name} in {card}'s text");
    }
}

/// Replaces the first leaf that `matches` with `leaf`.
fn replace_leaf(
    reading: &mut Reading,
    matches: &impl Fn(&LexicalReading) -> bool,
    leaf: &LexicalReading,
) -> bool {
    match reading {
        Reading::Leaf(found) if matches(found) => {
            *found = leaf.clone();
            true
        }
        Reading::Leaf(_) => false,
        Reading::Node(node) => node
            .children
            .iter_mut()
            .any(|child| replace_leaf(child, matches, leaf)),
    }
}

/// A Set Name stands only as the parameter of its noun: admission refuses it
/// as a Name (*named Black Lotus*, Black Lotus's name replaced by a Set Name
/// spelled alike), and admits it where Golgothian Sylex reads it.
#[test]
fn admission_refuses_a_set_name_off_its_noun() {
    let env = LexicalEnvironment::load().unwrap();
    let grammar = sylex_english::bind(&env.lexicon, &env.frames).unwrap();
    let sylex_scope = scope("Golgothian Sylex");
    let sylex = SYLEX.replace("NAME", "Antiquities");
    let parse = grammar.parse_analyzed(&env.lexicon.analyze_source_scoped(
        &sylex,
        None,
        &sylex_scope,
    ));
    let reading = parse.readings().next().unwrap();
    grammar.admit(&reading, &env.lexicon, &sylex_scope).unwrap();

    // Demonic Tutor's search, for a card named Black Lotus.
    let text = "Search your library for a card named Black Lotus, put that card into your hand, then shuffle.";
    let tutor = scope("Demonic Tutor");
    let parse = grammar.parse_analyzed(&env.lexicon.analyze_source_scoped(text, None, &tutor));
    let mut reading = parse
        .readings()
        .next()
        .expect("a Reading of a search for a named card");
    grammar.admit(&reading, &env.lexicon, &tutor).unwrap();
    let name = |found: &LexicalReading| matches!(found, LexicalReading::Word(value) if value.lexeme.as_str() == "name:Black Lotus");
    let set_name = LexicalReading::SetName {
        name: "Black Lotus".into(),
    };
    assert!(replace_leaf(&mut reading, &name, &set_name));
    assert!(grammar.admit(&reading, &env.lexicon, &tutor).is_err());
}
