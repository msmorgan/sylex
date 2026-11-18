//! A genitive's possessor is a Noun Phrase under a gate
//! (`HeadFinalPossessor`, `TransparentPossessor`), and each gate is an
//! admission gate (`docs/design/english.md` §6): the grammar without it has
//! more Readings of a real line, and the grammar with it keeps the line read.

use sylex_lexicon::CardScope;
use sylex_syntax::CompiledGrammar;
use sylex_syntax::Grammar;
use sylex_vocabulary::LexicalEnvironment;

/// The number of Readings `text` has on `card` under the grammar, after
/// `edit` rewrites its source.
fn readings(env: &LexicalEnvironment, card: &str, text: &str, edit: (&str, &str)) -> usize {
    let source = sylex_english::GRAMMAR.as_str();
    assert_eq!(source.matches(edit.0).count(), 1, "`{}`", edit.0);
    let grammar = CompiledGrammar::bind(
        &Grammar::from_fug_str(&source.replace(edit.0, edit.1)).unwrap(),
        &env.lexicon,
        &env.frames,
    )
    .unwrap();
    let analyzed = env
        .lexicon
        .analyze_source_scoped(text, None, &CardScope::full_names([card]));
    grammar.parse_analyzed(&analyzed).readings().count()
}

const HEAD_FINAL: &str = "{ head_final: [head], uncoordinated: [head] }";
const TRANSPARENT: &str =
    "TransparentPossessor: Possessor -> NounPhrase { complement_final: [head] }";

#[test]
fn a_possessor_ends_in_its_head_noun() {
    let env = LexicalEnvironment::load().unwrap();
    let line = "When this creature enters, look at the top card of target player's library.";
    let count = |edit| readings(&env, "Merfolk Observer", line, edit);
    assert_eq!(count((HEAD_FINAL, HEAD_FINAL)), 1);
    // *[the top card of target player]'s library*.
    assert!(count((HEAD_FINAL, "{ uncoordinated: [head] }")) > 1);
}

#[test]
fn a_possessor_is_no_coordination_of_noun_phrases() {
    let env = LexicalEnvironment::load().unwrap();
    let line = "Blightning deals 3 damage to target player or planeswalker. That player or that \
                planeswalker's controller discards two cards.";
    let count = |edit| readings(&env, "Blightning", line, edit);
    let gated = count((HEAD_FINAL, HEAD_FINAL));
    assert!(gated >= 1);
    // *[that player or that planeswalker]'s controller*.
    assert!(count((HEAD_FINAL, "{ head_final: [head] }")) > gated);
}

#[test]
fn a_number_transparent_quantity_is_a_possessor() {
    let env = LexicalEnvironment::load().unwrap();
    let line = "• Exile any number of target players' graveyards.";
    let count = |edit| readings(&env, "Thraben Charm", line, edit);
    assert_eq!(count((TRANSPARENT, TRANSPARENT)), 1);
    assert_eq!(
        count((
            TRANSPARENT,
            "TransparentPossessor: Possessor -> NounPhrase { head_final: [head] }"
        )),
        0
    );
}

#[test]
fn a_transparent_possessor_ends_in_its_oblique_head() {
    let env = LexicalEnvironment::load().unwrap();
    let line = "Put any number of target artifact cards from target player's graveyard on top of \
                their library in any order.";
    let count = |edit| readings(&env, "Drafna's Restoration", line, edit);
    let gated = count((TRANSPARENT, TRANSPARENT));
    assert!(gated >= 1);
    // *[any number of target artifact cards from target player]'s graveyard*.
    assert!(count((TRANSPARENT, "TransparentPossessor: Possessor -> NounPhrase")) > gated);
}

#[test]
fn a_possessor_has_no_dependent_outside_its_determiner() {
    let env = LexicalEnvironment::load().unwrap();
    let line = "Whenever this creature becomes blocked by a nonartifact, non-Dragon creature, that \
                creature gets -X/-0 until end of turn, where X is half the creature's power, \
                rounded down.";
    let count = |edit| readings(&env, "Catacomb Dragon", line, edit);
    let gate = "\n    where head.Periphery != External";
    let gated = count((gate, gate));
    assert!(gated >= 1);
    // *[half the creature]'s power*.
    assert!(count((gate, "")) > gated);
}
