//! The `head_final` and `complement_final` child gates of a Production. A
//! genitive's possessor must end in its head noun (Merfolk Observer: *the top
//! card of target player's library*, where *target player* is the possessor,
//! never *the top card of target player* with its trailing unselected
//! Preposition Phrase modifier). A number-transparent quantity is a possessor
//! too, ending in the head of the Complement it ends in (Thraben Charm: *any
//! number of target players' graveyards*): it passes `complement_final` but not
//! `head_final`. A phrase ending in a `modifier` or `adjunct` child passes
//! neither.

use sylex_lexicon::Category;
use sylex_lexicon::Countability;
use sylex_lexicon::Lexeme;
use sylex_lexicon::Lexicon;
use sylex_lexicon::Source;
use sylex_lexicon::SourceKind;
use sylex_syntax::CompiledGrammar;
use sylex_syntax::FrameCatalog;
use sylex_syntax::Grammar;

fn lexicon() -> Lexicon {
    let core = || Source {
        kind: SourceKind::Core,
        path: "tests/final_gates.rs".to_owned(),
        owner: "test".to_owned(),
    };
    Lexicon::new([
        Lexeme::invariant("a:target", "target", Category::Adverb, core()),
        Lexeme::invariant("a:card", "card", Category::Adverb, core()),
        Lexeme::invariant("a:item", "item", Category::Adverb, core()),
        Lexeme::invariant("a:number", "number", Category::Adverb, core()),
        Lexeme::invariant("p:of", "of", Category::Preposition, core()),
        Lexeme::noun("n:player", "player", vec![Countability::Count], core()),
    ])
    .unwrap()
}

/// *Tail* ends the phrase in an unselected dependent (a `modifier`, as in
/// *card of player*), *Adjoined* in an `adjunct` and *Quantity* in a
/// `complement` that ends in its head. Each sentence takes its phrase under
/// one gate, or none.
const GRAMMAR: &str = r#"
    root Sentence
    category NounPhrase
    category PrepositionPhrase
    category Sentence
    word Target = "a:target"
    word Card = "a:card"
    word Item = "a:item"
    word Number = "a:number"
    Bare: NounPhrase -> Lex(Noun)
    Targeted: NounPhrase -> Word(Target) head:NounPhrase
    Tail: NounPhrase -> head:Word(Card) modifier:PrepositionPhrase
    Adjoined: NounPhrase -> head:Word(Item) adjunct:PrepositionPhrase
    Quantity: NounPhrase -> head:Word(Number) complement:PrepositionPhrase
    Of: PrepositionPhrase -> head:Lex(Preposition) complement:NounPhrase
    Possessor: Sentence -> possessor:NounPhrase ';' { head_final: [possessor] }
    Quantified: Sentence -> possessor:NounPhrase '.' { complement_final: [possessor] }
    Ungated: Sentence -> phrase:NounPhrase '!'
"#;

/// Whether the text parses.
fn accepted(text: &str) -> bool {
    let lexicon = lexicon();
    let grammar = CompiledGrammar::bind(
        &Grammar::from_fug_str(GRAMMAR).unwrap(),
        &lexicon,
        &FrameCatalog::default(),
    )
    .unwrap();
    grammar.parse(&lexicon, text).readings().next().is_some()
}

#[test]
fn a_possessor_ends_in_its_head() {
    assert!(accepted("player;"));
    // Merfolk Observer: *target player's library*.
    assert!(accepted("target player;"));
}

#[test]
fn a_trailing_dependent_ends_no_possessor() {
    assert!(accepted("card of target player!"));
    assert!(!accepted("card of target player;"));
    assert!(accepted("item of target player!"));
    assert!(!accepted("item of target player;"));
}

#[test]
fn a_quantity_ends_in_the_head_of_its_last_complement() {
    // Thraben Charm: *any number of target players' graveyards*.
    assert!(accepted("number of target player!"));
    assert!(!accepted("number of target player;"));
    assert!(accepted("number of target player."));
    assert!(accepted("number of player."));
}

#[test]
fn a_head_final_phrase_passes_both_gates() {
    assert!(accepted("player."));
    assert!(accepted("target player."));
}

#[test]
fn a_modifier_or_adjunct_ends_no_complement_final_phrase() {
    assert!(!accepted("card of target player."));
    assert!(!accepted("item of target player."));
}

#[test]
fn a_complement_of_a_trailing_dependent_is_no_head_line() {
    // *card of number of player*: the quantity inside the modifier ends in a
    // Complement, but the whole phrase ends in a modifier.
    assert!(!accepted("card of number of player."));
}
