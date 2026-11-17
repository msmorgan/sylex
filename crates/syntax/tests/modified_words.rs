//! Bound keyword stems that spell a free keyword lemma require at least one
//! premodifier (`ModifiedOnlyStem`), while other stems stand bare (`Stem`) and
//! both take stacked premodifiers (`ModifiedStem`).

use sylex_lexicon::Binding;
use sylex_lexicon::Category;
use sylex_lexicon::Lexeme;
use sylex_lexicon::Lexicon;
use sylex_lexicon::Source;
use sylex_lexicon::SourceKind;
use sylex_syntax::CompiledGrammar;
use sylex_syntax::FrameCatalog;
use sylex_syntax::Grammar;
use sylex_syntax::frames::ParameterPart;
use sylex_syntax::frames::SlotKind;

fn lexicon() -> Lexicon {
    let catalog = || Source {
        kind: SourceKind::Catalog,
        path: "tests/modified_words.rs".to_owned(),
        owner: "test".to_owned(),
    };
    let mut walk_bound = Lexeme::invariant(
        "keyword:landwalk:bound",
        "walk",
        Category::Keyword,
        catalog(),
    );
    walk_bound.binding = Binding::Suffix;
    Lexicon::new(vec![
        Lexeme::invariant("keyword:landwalk", "landwalk", Category::Keyword, catalog()),
        walk_bound,
        Lexeme::invariant("stem:island", "island", Category::Noun, catalog()),
        Lexeme::invariant("stem:land", "land", Category::Noun, catalog()),
        Lexeme::invariant("adj:nonbasic", "nonbasic", Category::Adjective, catalog()),
        Lexeme::invariant("adj:snow", "snow", Category::Adjective, catalog()),
    ])
    .unwrap()
}

const GRAMMAR: &str = r"
    root KeywordPhrase
    category KeywordPhrase
    category KeywordParameters
    category BoundKeyword
    category TypeModifier
    category Quality
    slot TypeModifier = TypeModifier
    slot Quality = Quality
    Modifier: TypeModifier -> Lex(Adjective)
    FreeKeyword: KeywordPhrase -> KeywordParameters
    BoundKeywordPhrase: KeywordPhrase -> BoundKeyword
    parameters KeywordParameters { head: Keyword, binding: Free }
    parameters BoundKeyword { head: Keyword, binding: Suffix }
";

fn catalog(bare: &[&str], modified_only: &[&str]) -> FrameCatalog {
    let mut frames = FrameCatalog::default();
    frames
        .parameters
        .insert("keyword:landwalk".into(), vec![vec![ParameterPart::Head]]);
    frames.parameters.insert(
        "keyword:landwalk:bound".into(),
        vec![vec![
            ParameterPart::ModifiedWords {
                bare: bare.iter().map(|s| (*s).into()).collect(),
                modified_only: modified_only.iter().map(|s| (*s).into()).collect(),
                modifier: SlotKind::TypeModifier,
                target: SlotKind::Quality,
            },
            ParameterPart::Head,
        ]],
    );
    frames
}

#[test]
fn modified_only_stem_refuses_bare_duplicate_and_admits_stacked_modifiers() {
    let lexicon = lexicon();
    let grammar = CompiledGrammar::bind(
        &Grammar::from_fug_str(GRAMMAR).unwrap(),
        &lexicon,
        &catalog(&["stem:island"], &["stem:land"]),
    )
    .unwrap();

    for text in [
        "landwalk",
        "islandwalk",
        "nonbasic landwalk",
        "nonbasic snow landwalk",
        "nonbasic islandwalk",
    ] {
        let readings: Vec<_> = grammar.parse(&lexicon, text).readings().collect();
        assert_eq!(readings.len(), 1, "{text}: {readings:#?}");
    }
}

#[test]
fn overlapping_or_empty_modified_word_sets_are_rejected() {
    let lexicon = lexicon();
    let parsed = Grammar::from_fug_str(GRAMMAR).unwrap();
    assert!(CompiledGrammar::bind(&parsed, &lexicon, &catalog(&[], &[])).is_err());
    assert!(
        CompiledGrammar::bind(&parsed, &lexicon, &catalog(&["stem:land"], &["stem:land"])).is_err()
    );
}
