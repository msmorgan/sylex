//! A genitive determines no nominal whose *of* Complement names the argument
//! the genitive would name (`GenitiveDependents`, CGEL, Ch. 5, §16.5.3,
//! pp. 475–476): the grammar without it has more Readings of a real line,
//! and a genitive that names something else still combines with its noun's
//! *of* Complement (`distinct_genitive`).

use sylex_lexicon::CardScope;
use sylex_syntax::CompiledGrammar;
use sylex_syntax::Grammar;
use sylex_vocabulary::LexicalEnvironment;

const RULE: &str = "dependents GenitiveDependents = [DeterminerPhrase, NounPhrase]\n    where !(dependent.Case = \
     Genitive & head.GenitiveAlternant = Filled)\n";

/// The number of Readings `text` has on `card`, with and without the rule.
fn readings(env: &LexicalEnvironment, card: &str, text: &str) -> (usize, usize) {
    let source = sylex_english::GRAMMAR.as_str();
    assert_eq!(source.matches(RULE).count(), 1);
    let count = |source: &str| {
        let grammar = CompiledGrammar::bind(
            &Grammar::from_fug_str(source).unwrap(),
            &env.lexicon,
            &env.frames,
        )
        .unwrap();
        let analyzed =
            env.lexicon
                .analyze_source_scoped(text, None, &CardScope::full_names([card]));
        grammar.parse_analyzed(&analyzed).readings().count()
    };
    (count(source), count(&source.replace(RULE, "")))
}

#[test]
fn a_genitive_pronouns_head_shares_no_of_phrase() {
    let env = LexicalEnvironment::load().unwrap();
    // *[your hand or the top] of your library*.
    let line = "Once during each of your turns, you may cast a spell from your hand or the top of \
                your library without paying its mana cost.";
    let (with, without) = readings(&env, "One with the Multiverse", line);
    assert!(with >= 1 && with < without, "{with} {without}");
    // *their [[choice of the top] or [bottom]] of their library*.
    let line = "Target creature's owner puts it on their choice of the top or bottom of their \
                library.";
    let (with, without) = readings(&env, "Misleading Motes", line);
    assert!(with >= 1 && with < without, "{with} {without}");
    // *[its power and the power] of target creature*.
    let line = "Whenever this creature blocks, exchange its power and the power of target creature \
                it's blocking until end of combat.";
    let (with, without) = readings(&env, "Serene Master", line);
    assert!(with >= 1 && with < without, "{with} {without}");
}

#[test]
fn a_distinct_genitive_keeps_its_nouns_of_complement() {
    let env = LexicalEnvironment::load().unwrap();
    let line = "You may reveal this card from your opening hand. If you do, at the beginning of \
                your first main phase of the game, add {G}.";
    let (with, without) = readings(&env, "Chancellor of the Tangle", line);
    assert!(with >= 1 && with == without, "{with} {without}");
    let line = "{T}: Put your choice of a reach, menace, trample, or haste counter on target \
                Dinosaur. Activate only as a sorcery.";
    let (with, without) = readings(&env, "Owen Grady, Raptor Trainer", line);
    assert!(with >= 1 && with == without, "{with} {without}");
}
