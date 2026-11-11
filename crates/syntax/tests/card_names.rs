//! Another card's name, a card name in the text of a face whose scope does
//! not hold it [CR#201.5], fills only a position that opts in to it by
//! naming `OtherCardName()`. Every other leaf symbol refuses it, so a new
//! position refuses it unless it opts in. `OtherCardName()` admits nothing
//! else: not the face's own name, nor a Proper Name that is no card name.

use std::collections::BTreeSet;

use sylex_lexicon::Category;
use sylex_lexicon::Lexeme;
use sylex_lexicon::LexemeId;
use sylex_lexicon::Lexicon;
use sylex_lexicon::Source;
use sylex_lexicon::SourceKind;
use sylex_syntax::CompiledGrammar;
use sylex_syntax::FrameCatalog;
use sylex_syntax::Grammar;

fn lexicon() -> Lexicon {
    let catalog = Source {
        kind: SourceKind::Catalog,
        path: "tests/card_names.rs".to_owned(),
        owner: "test".to_owned(),
    };
    // From Excise's *Exile target attacking creature*: *Exile* is also a
    // card's name.
    let card = Lexeme::card_name("name:Exile", "Exile", None, catalog.clone());
    let token = Lexeme::invariant("token:Young Hero", "Young Hero", Category::Catalog, catalog);
    Lexicon::new([card, token]).unwrap()
}

/// `.` ends a position that does not opt in, `!` one that admits only
/// another card's name.
fn grammar(lexicon: &Lexicon) -> CompiledGrammar {
    let text = r"
        root Sentence
        category Sentence
        category Name
        NameLeaf: Name -> Lex(Catalog)
        Plain: Sentence -> Name '.'
        OptedIn: Sentence -> OtherCardName() '!'
    ";
    CompiledGrammar::bind(
        &Grammar::from_fug_str(text).unwrap(),
        lexicon,
        &FrameCatalog::default(),
    )
    .unwrap()
}

fn readings(text: &str, scope: &BTreeSet<LexemeId>) -> usize {
    let lexicon = lexicon();
    let grammar = grammar(&lexicon);
    let analyzed = lexicon.analyze_source_scoped(text, None, scope);
    let parse = grammar.parse_analyzed(&analyzed);
    let found: Vec<_> = parse.readings().collect();
    for reading in &found {
        assert_eq!(parse.admit(reading, &lexicon), Ok(()), "{text}");
        assert_eq!(grammar.admit(reading, &lexicon, scope), Ok(()), "{text}");
    }
    found.len()
}

#[test]
fn a_position_without_the_opt_in_refuses_another_card_s_name() {
    let own = BTreeSet::from([LexemeId::new("name:Exile")]);
    let elsewhere = BTreeSet::from([LexemeId::new("name:Excise")]);
    assert_eq!(readings("Exile.", &own), 1);
    assert_eq!(readings("Exile.", &elsewhere), 0);
    assert_eq!(readings("Young Hero.", &elsewhere), 1);
}

#[test]
fn the_opt_in_admits_another_card_s_name_alone() {
    let own = BTreeSet::from([LexemeId::new("name:Exile")]);
    let elsewhere = BTreeSet::from([LexemeId::new("name:Excise")]);
    assert_eq!(readings("Exile!", &elsewhere), 1);
    assert_eq!(readings("Exile!", &own), 0);
    assert_eq!(readings("Young Hero!", &elsewhere), 0);
}

#[test]
fn admission_reads_a_card_name_in_the_scope_it_is_given() {
    let lexicon = lexicon();
    let grammar = grammar(&lexicon);
    let own = BTreeSet::from([LexemeId::new("name:Exile")]);
    let analyzed = lexicon.analyze_source_scoped("Exile.", None, &own);
    let reading = grammar.parse_analyzed(&analyzed).readings().next().unwrap();
    assert_eq!(grammar.admit(&reading, &lexicon, &own), Ok(()));
    assert!(grammar.admit(&reading, &lexicon, &BTreeSet::new()).is_err());
}
