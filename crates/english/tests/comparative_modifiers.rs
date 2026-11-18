//! A comparative determinative's degree modifier is one of
//! `ComparativeDegreeModifier` (*no more than two spells*, Fires of
//! Invention), never any other determinative, and the comparative bound of
//! a scalar change's endpoint (*reduce your life total to less than 1*, Ali
//! from Cairo) is a Preposition's Amount Complement.

use sylex_lexicon::CardScope;
use sylex_syntax::CompiledGrammar;
use sylex_syntax::Grammar;
use sylex_vocabulary::LexicalEnvironment;

fn readings(env: &LexicalEnvironment, grammar: &CompiledGrammar, card: &str, text: &str) -> usize {
    let analyzed = env
        .lexicon
        .analyze_source_scoped(text, None, &CardScope::full_names([card]));
    grammar.parse_analyzed(&analyzed).readings().count()
}

fn grammar(env: &LexicalEnvironment) -> CompiledGrammar {
    CompiledGrammar::bind(
        &Grammar::from_fug_str(sylex_english::GRAMMAR.as_str()).unwrap(),
        &env.lexicon,
        &env.frames,
    )
    .unwrap()
}

const FIRES: &str =
    "You can cast spells only during your turn and you can cast no more than two spells each turn.";

#[test]
fn no_modifies_a_comparative_determinative() {
    let env = LexicalEnvironment::load().unwrap();
    let grammar = grammar(&env);
    assert!(readings(&env, &grammar, "Fires of Invention", FIRES) >= 1);
}

#[test]
fn a_distributive_determinative_modifies_no_comparative() {
    let env = LexicalEnvironment::load().unwrap();
    let grammar = grammar(&env);
    let each = FIRES.replace("no more than", "each more than");
    assert_eq!(readings(&env, &grammar, "Fires of Invention", &each), 0);
}

#[test]
fn a_scalar_changes_endpoint_is_a_comparative_bound() {
    let env = LexicalEnvironment::load().unwrap();
    let grammar = grammar(&env);
    let line = "Damage that would reduce your life total to less than 1 reduces it to 1 instead.";
    assert_eq!(readings(&env, &grammar, "Ali from Cairo", line), 1);
}

#[test]
fn a_degree_modifier_agrees_in_countability_and_stands_once() {
    let env = LexicalEnvironment::load().unwrap();
    let grammar = grammar(&env);
    let with = |modifier: &str| {
        let line = FIRES.replace("no more than", &format!("{modifier} more than"));
        readings(&env, &grammar, "Fires of Invention", &line)
    };
    assert!(with("many") >= 1);
    // *Much* is mass, and count *more* determines *spells*.
    assert_eq!(with("much"), 0);
    assert_eq!(with("no no"), 0);
}
