//! Another card's name, a card name in the text of a face whose scope does
//! not hold it [CR#201.5], fills only a position that opts in to it by
//! naming `OtherCardName()`. Every other leaf symbol refuses it, so a new
//! position refuses it unless it opts in. `OtherCardName()` admits nothing
//! else: not the face's own name, nor a Proper Name that is no card name.

use sylex_lexicon::CardScope;
use sylex_lexicon::Category;
use sylex_lexicon::Lexeme;
use sylex_lexicon::LexemeId;
use sylex_lexicon::Lexicon;
use sylex_lexicon::NameForm;
use sylex_lexicon::Source;
use sylex_lexicon::SourceKind;
use sylex_syntax::AdmissionError;
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

fn readings(text: &str, scope: &sylex_lexicon::CardScope) -> usize {
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
    let own = CardScope::own_names([LexemeId::new("name:Exile")]);
    let elsewhere = CardScope::own_names([LexemeId::new("name:Excise")]);
    assert_eq!(readings("Exile.", &own), 1);
    assert_eq!(readings("Exile.", &elsewhere), 0);
    assert_eq!(readings("Young Hero.", &elsewhere), 1);
}

#[test]
fn the_opt_in_admits_another_card_s_name_alone() {
    let own = CardScope::own_names([LexemeId::new("name:Exile")]);
    let elsewhere = CardScope::own_names([LexemeId::new("name:Excise")]);
    assert_eq!(readings("Exile!", &elsewhere), 1);
    assert_eq!(readings("Exile!", &own), 0);
    assert_eq!(readings("Young Hero!", &elsewhere), 0);
}

#[test]
fn admission_reads_a_card_name_in_the_scope_it_is_given() {
    let lexicon = lexicon();
    let grammar = grammar(&lexicon);
    let own = CardScope::own_names([LexemeId::new("name:Exile")]);
    let analyzed = lexicon.analyze_source_scoped("Exile.", None, &own);
    let reading = grammar.parse_analyzed(&analyzed).readings().next().unwrap();
    assert_eq!(grammar.admit(&reading, &lexicon, &own), Ok(()));
    assert!(
        grammar
            .admit(&reading, &lexicon, &CardScope::default())
            .is_err()
    );
}

/// Chandra, Pyromaster, whose own text shortens its name to *Chandra*
/// [CR#201.5c], and Marit Lage, the Introduced Name of Dark Depths.
fn scoped_lexicon() -> Lexicon {
    let catalog = Source {
        kind: SourceKind::Catalog,
        path: "tests/card_names.rs".to_owned(),
        owner: "test".to_owned(),
    };
    let chandra = Lexeme::card_name(
        "name:Chandra, Pyromaster",
        "Chandra, Pyromaster",
        Some("Chandra".to_owned()),
        catalog.clone(),
    );
    let depths = Lexeme::card_name("name:Dark Depths", "Dark Depths", None, catalog.clone());
    let mut marit_lage = Lexeme::invariant(
        "introduced_name:Marit Lage",
        "Marit Lage",
        Category::Catalog,
        catalog,
    );
    marit_lage.capitalization = sylex_lexicon::Capitalization::Exact;
    marit_lage.card_scope = [LexemeId::new("name:Dark Depths")].into();
    Lexicon::new([chandra, depths, marit_lage]).unwrap()
}

/// The one Reading of `text` in `scope`.
fn only_reading(
    grammar: &CompiledGrammar,
    lexicon: &Lexicon,
    text: &str,
    scope: &CardScope,
) -> sylex_syntax::Reading {
    let analyzed = lexicon.analyze_source_scoped(text, None, scope);
    let readings: Vec<_> = grammar.parse_analyzed(&analyzed).readings().collect();
    assert_eq!(readings.len(), 1, "{text}");
    readings.into_iter().next().unwrap()
}

#[test]
fn admission_refuses_a_shortened_name_off_its_own_card() {
    let lexicon = scoped_lexicon();
    let grammar = grammar(&lexicon);
    let own = CardScope::own_names([LexemeId::new("name:Chandra, Pyromaster")]);
    let elsewhere = CardScope::own_names([LexemeId::new("name:Dark Depths")]);
    let reading = only_reading(&grammar, &lexicon, "Chandra.", &own);
    assert_eq!(grammar.admit(&reading, &lexicon, &own), Ok(()));
    assert_eq!(
        grammar.admit(&reading, &lexicon, &elsewhere),
        Err(AdmissionError::OutOfScope)
    );
}

#[test]
fn admission_refuses_an_introduced_name_off_the_cards_that_introduce_it() {
    let lexicon = scoped_lexicon();
    let grammar = grammar(&lexicon);
    let depths = CardScope::own_names([LexemeId::new("name:Dark Depths")]);
    let elsewhere = CardScope::own_names([LexemeId::new("name:Chandra, Pyromaster")]);
    let reading = only_reading(&grammar, &lexicon, "Marit Lage.", &depths);
    assert_eq!(grammar.admit(&reading, &lexicon, &depths), Ok(()));
    assert_eq!(
        grammar.admit(&reading, &lexicon, &elsewhere),
        Err(AdmissionError::OutOfScope)
    );
}

/// A scope may hold a card name in its Full form alone: there the Full form
/// is that card's own name, and the Shortened form does not stand.
#[test]
fn a_scope_holding_only_the_full_form_refuses_the_shortened_form() {
    let lexicon = scoped_lexicon();
    let grammar = grammar(&lexicon);
    let chandra = LexemeId::new("name:Chandra, Pyromaster");
    let mut full_only = CardScope::default();
    full_only.insert(chandra, NameForm::Full);
    assert!(full_only.holds(chandra, NameForm::Full));
    assert!(!full_only.holds(chandra, NameForm::Shortened));
    let analyzed = lexicon.analyze_source_scoped("Chandra.", None, &full_only);
    assert_eq!(grammar.parse_analyzed(&analyzed).readings().count(), 0);
    let shortened = only_reading(
        &grammar,
        &lexicon,
        "Chandra.",
        &CardScope::own_names([chandra]),
    );
    assert_eq!(
        grammar.admit(&shortened, &lexicon, &full_only),
        Err(AdmissionError::OutOfScope)
    );
    let full = only_reading(&grammar, &lexicon, "Chandra, Pyromaster.", &full_only);
    assert_eq!(grammar.admit(&full, &lexicon, &full_only), Ok(()));
    let analyzed = lexicon.analyze_source_scoped("Chandra, Pyromaster!", None, &full_only);
    assert_eq!(grammar.parse_analyzed(&analyzed).readings().count(), 0);
}
