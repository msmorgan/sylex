//! The Segment walk over a Packed Forest: a parse's top-level stretches and
//! their Derivations, over small hand grammars.

use sylex_english::segments::NotSegmentable;
use sylex_english::segments::SegmentFrame;
use sylex_english::segments::Segmentation;
use sylex_lexicon::Category;
use sylex_lexicon::Lexeme;
use sylex_lexicon::Lexicon;
use sylex_lexicon::Source;
use sylex_lexicon::SourceKind;
use sylex_syntax::CompiledGrammar;
use sylex_syntax::FrameCatalog;
use sylex_syntax::Grammar;
use sylex_syntax::LoadError;

fn word(id: &str, lemma: &str, category: Category) -> Lexeme {
    Lexeme::invariant(
        id,
        lemma,
        category,
        Source {
            kind: SourceKind::Core,
            path: "tests/segments.rs".to_owned(),
            owner: "test".to_owned(),
        },
    )
}

/// `cast` is a noun and an adjective, so each `Word` over it has two
/// Derivations; `card` is a noun only.
fn lexicon() -> Lexicon {
    Lexicon::new([
        word("n:cast", "cast", Category::Noun),
        word("a:cast", "cast", Category::Adjective),
        word("n:card", "card", Category::Noun),
    ])
    .unwrap()
}

fn bind(grammar: &str) -> CompiledGrammar {
    CompiledGrammar::bind(
        &Grammar::from_fug_str(grammar).unwrap(),
        &lexicon(),
        &FrameCatalog::default(),
    )
    .unwrap()
}

const OUTER: &[&str] = &["Document", "Sentences"];
const EDGES: &[char] = &['.', ',', ':'];

/// A sentence sequence, and a sentence that holds a bracketed one of its own.
const SEQUENCE: &str = r"
    root Document
    category Document
    category Sentence
    category Sentences
    category Word
    Text: Document -> Sentences
    Single: Sentences -> Sentence
    Sequence: Sentences -> first:Sentence rest:Sentences
    Plain: Sentence -> word:Word '.'
    Nested: Sentence -> word:Word '[':Prefix inner:Sentences ']' '.'
    Noun: Word -> word:Lex(Noun)
    Adjective: Word -> word:Lex(Adjective)
";

fn segments(grammar: &CompiledGrammar, text: &str) -> (u128, Segmentation) {
    let parse = grammar.parse(&lexicon(), text);
    let frame = SegmentFrame::resolve(grammar, OUTER, EDGES).unwrap();
    (
        parse.derivation_count(),
        sylex_english::segments::segments(&parse, &frame),
    )
}

fn counts(segmentation: &Segmentation) -> Vec<(&str, u128)> {
    match segmentation {
        Segmentation::Segments(segments) => segments
            .iter()
            .map(|s| (s.text.as_str(), s.derivations))
            .collect(),
        Segmentation::NotSegmentable(why) => panic!("not segmentable: {why:?}"),
    }
}

#[test]
fn sentences_of_a_sequence_are_segments_whose_counts_multiply() {
    let grammar = bind(SEQUENCE);
    let (derivations, segmentation) = segments(&grammar, "Cast. Card. Cast.");
    assert_eq!(derivations, 4);
    assert_eq!(
        counts(&segmentation),
        [("Cast", 2), ("Card", 1), ("Cast", 2)]
    );
    assert_eq!(segmentation.surplus(derivations), 2);
    let (derivations, segmentation) = segments(&grammar, "Card.");
    assert_eq!((derivations, segmentation.surplus(derivations)), (1, 0));
}

#[test]
fn a_nested_sequence_multiplies_into_its_segment() {
    let grammar = bind(SEQUENCE);
    let (derivations, segmentation) = segments(&grammar, "Card [cast. cast.].");
    assert_eq!(derivations, 4);
    assert_eq!(counts(&segmentation), [("Card [cast. cast.]", 4)]);
    assert_eq!(segmentation.surplus(derivations), 3);
}

#[test]
fn a_parse_without_a_derivation_has_no_segments() {
    let grammar = bind(SEQUENCE);
    let (derivations, segmentation) = segments(&grammar, "Card card.");
    assert_eq!(derivations, 0);
    assert_eq!(segmentation, Segmentation::Segments(Vec::new()));
    assert_eq!(segmentation.surplus(derivations), 0);
}

/// Two outer analyses: two words as two segments, or as one non-outer pair.
#[test]
fn segment_boundaries_that_differ_between_derivations_are_reported() {
    let grammar = bind(
        r"
            root Document
            category Document
            category Pair
            category Sentences
            category Word
            Words: Document -> first:Word second:Word '.'
            Paired: Document -> Pair '.'
            Pair: Pair -> first:Word second:Word
            Noun: Word -> word:Lex(Noun)
        ",
    );
    let (derivations, segmentation) = segments(&grammar, "Card card.");
    assert_eq!(derivations, 2);
    assert_eq!(
        segmentation,
        Segmentation::NotSegmentable(NotSegmentable::Segmentations)
    );
    assert_eq!(segmentation.surplus(derivations), 1);
}

/// One segmentation under two outer Productions: the segment's count is 1,
/// the unit's Derivations 2.
#[test]
fn several_outer_structures_over_one_segmentation_are_reported() {
    let grammar = bind(
        r"
            root Document
            category Document
            category Sentences
            category Word
            Direct: Document -> Word '.'
            Listed: Document -> Sentences
            Single: Sentences -> Word '.'
            Noun: Word -> word:Lex(Noun)
        ",
    );
    let (derivations, segmentation) = segments(&grammar, "Card.");
    assert_eq!(derivations, 2);
    assert_eq!(
        segmentation,
        Segmentation::NotSegmentable(NotSegmentable::NonProduct)
    );
}

#[test]
fn an_outer_category_must_be_declared() {
    let grammar = bind(SEQUENCE);
    assert!(matches!(
        SegmentFrame::resolve(&grammar, &["Document", "Paragraphs"], EDGES),
        Err(LoadError::Invalid(message)) if message.contains("Paragraphs")
    ));
}
