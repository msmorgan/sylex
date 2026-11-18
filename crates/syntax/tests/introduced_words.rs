//! Words a text introduces and uses: a Production that `offers` reads
//! whatever open word stands at each lexical child, any other Production
//! uses a word only where something introduces it, and a choice offered
//! stands only where a use discharges it, in the same Reading or on a line
//! after the text. The grammar here is a miniature of a vote: its offer
//! (*votes for sickness or psychosis*) and its mention (*If psychosis gets
//! more votes*, Bite of the Black Rose), with a name beside them.

use std::collections::BTreeSet;

use sylex_lexicon::CardScope;
use sylex_lexicon::Category;
use sylex_lexicon::Lexeme;
use sylex_lexicon::LexicalReading;
use sylex_lexicon::Lexicon;
use sylex_lexicon::Source;
use sylex_lexicon::SourceKind;
use sylex_syntax::AdmissionError;
use sylex_syntax::CompiledGrammar;
use sylex_syntax::FrameCatalog;
use sylex_syntax::Grammar;
use sylex_syntax::Parse;
use sylex_syntax::Reading;

fn lexicon() -> Lexicon {
    let core = |id: &str, lemma: &str, category| {
        let source = Source {
            kind: SourceKind::Core,
            path: "tests/introduced_words.rs".to_owned(),
            owner: "test".to_owned(),
        };
        let mut lexeme = Lexeme::invariant(id, lemma, category, source);
        lexeme.capitalization = sylex_lexicon::Capitalization::Exact;
        lexeme
    };
    Lexicon::new([
        core("v:offer", "offer", Category::Adverb),
        core("v:counts", "counts", Category::Adverb),
        core("v:named", "named", Category::Adverb),
        core("v:attacks", "attacks", Category::Adverb),
        core("c:or", "or", Category::Coordinator),
    ])
    .unwrap()
}

const GRAMMAR: &str = r#"
    root Text
    category Text
    category Sentence
    category Choice
    category Mention
    category Name
    word Offer = "v:offer"
    word Counts = "v:counts"
    word Named = "v:named"
    word Attacks = "v:attacks"
    word Or = "c:or"
    One: Text -> Sentence
    More: Text -> first:Sentence rest:Text
    Offering: Sentence -> Word(Offer) complement:Choice '.'
    Counting: Sentence -> subject:Mention Word(Counts) '.'
    Naming: Sentence -> Word(Named) complement:IntroducedName() '.'
    Attack: Sentence -> subject:Name Word(Attacks) '.'
    Pair: Choice -> first:Lex(OptionWord) coordinator:Word(Or) last:Lex(OptionWord) { offers }
    Cited: Mention -> Lex(OptionWord)
    Repeated: Name -> RepeatedName()
"#;

fn bind(text: &str, lexicon: &Lexicon) -> Result<CompiledGrammar, String> {
    CompiledGrammar::bind(
        &Grammar::from_fug_str(text).unwrap(),
        lexicon,
        &FrameCatalog::default(),
    )
    .map_err(|error| error.to_string())
}

fn option(word: &str) -> LexicalReading {
    LexicalReading::VoteOption {
        word: word.to_owned(),
    }
}

fn parse<'g>(
    grammar: &'g CompiledGrammar,
    lexicon: &Lexicon,
    text: &str,
    scope: &CardScope,
) -> Parse<'g> {
    grammar.parse_analyzed(&lexicon.analyze_source_scoped(text, None, scope))
}

/// The number of Readings of `text` in `scope`, each one admitted apart
/// from its parse and realized as the text.
fn readings(text: &str, scope: &CardScope) -> usize {
    let lexicon = lexicon();
    let grammar = bind(GRAMMAR, &lexicon).unwrap();
    let parse = parse(&grammar, &lexicon, text, scope);
    let found: Vec<Reading> = parse.readings().collect();
    for reading in &found {
        assert_eq!(grammar.admit(reading, &lexicon, scope), Ok(()), "{text}");
        assert_eq!(grammar.realize(reading, &lexicon).unwrap(), text);
    }
    found.len()
}

const OFFER: &str = "offer sickness or psychosis.";
const MENTION: &str = "psychosis counts.";

#[test]
fn an_offer_reads_any_word_and_stands_where_the_reading_uses_one() {
    let scope = CardScope::default();
    // No inventory holds either word, and nothing puts one in scope.
    assert_eq!(
        readings("offer sickness or psychosis. psychosis counts.", &scope),
        1
    );
    // The use may stand before the offer in one text.
    assert_eq!(
        readings("psychosis counts. offer sickness or psychosis.", &scope),
        1
    );
    // An offer no use discharges is no Reading, and neither is a use of a
    // word nothing introduces.
    assert_eq!(readings(OFFER, &scope), 0);
    assert_eq!(readings(MENTION, &scope), 0);
    assert_eq!(
        readings("offer sickness or psychosis. plague counts.", &scope),
        0
    );
    assert_eq!(
        readings(
            "offer sickness or psychosis. plague counts. offer plague or war.",
            &scope
        ),
        0
    );
    // Each of two offers is discharged by a use of one of its own words.
    assert_eq!(
        readings(
            "offer sickness or psychosis. plague counts. offer plague or war. sickness counts.",
            &scope
        ),
        1
    );
    // A word an offer reads is no vocabulary gap; a word nothing introduces
    // is one.
    let lexicon = lexicon();
    let grammar = bind(GRAMMAR, &lexicon).unwrap();
    assert_eq!(parse(&grammar, &lexicon, OFFER, &scope).vocabulary_gaps, []);
    assert_eq!(
        parse(&grammar, &lexicon, MENTION, &scope).vocabulary_gaps,
        vec![0..9]
    );
}

#[test]
fn a_scope_introduces_words_for_a_use_and_discharges_an_offer() {
    // A line before the text introduces the word.
    let mut introduced = CardScope::default();
    introduced.introduce(option("psychosis"));
    assert_eq!(readings(MENTION, &introduced), 1);
    assert_eq!(readings(OFFER, &introduced), 0);
    // A line after the text uses it.
    let mut discharged = CardScope::default();
    discharged.discharge(option("psychosis"));
    assert_eq!(readings(OFFER, &discharged), 1);
    assert_eq!(readings(MENTION, &discharged), 0);
    // A stated word is both.
    let mut stated = CardScope::default();
    stated.insert_vote_option("psychosis");
    assert_eq!(readings(OFFER, &stated), 1);
    assert_eq!(readings(MENTION, &stated), 1);
    // A Reading built apart from a parse is held to the same.
    let lexicon = lexicon();
    let grammar = bind(GRAMMAR, &lexicon).unwrap();
    let bare = CardScope::default();
    let offer: Vec<Reading> = parse(&grammar, &lexicon, OFFER, &stated)
        .readings()
        .collect();
    assert_eq!(
        grammar.admit(&offer[0], &lexicon, &bare),
        Err(AdmissionError::Undischarged)
    );
    assert_eq!(
        grammar.admit(&offer[0], &lexicon, &introduced),
        Err(AdmissionError::Undischarged)
    );
    let mention: Vec<Reading> = parse(&grammar, &lexicon, MENTION, &stated)
        .readings()
        .collect();
    for scope in [&bare, &discharged] {
        assert_eq!(
            grammar.admit(&mention[0], &lexicon, scope),
            Err(AdmissionError::NotIntroduced)
        );
    }
}

/// Two lines read in order, each parsed once: the first says what it
/// introduces before anything discharges it, and takes the words the
/// second uses without being parsed again.
#[test]
fn a_parse_keeps_an_offer_for_the_lines_after_it_to_discharge() {
    let lexicon = lexicon();
    let grammar = bind(GRAMMAR, &lexicon).unwrap();
    let mut scope = CardScope::default();
    let mut first = parse(&grammar, &lexicon, OFFER, &scope);
    assert_eq!(first.derivation_count(), 0);
    let words = BTreeSet::from([option("sickness"), option("psychosis")]);
    assert_eq!(first.may_introduce(), words);
    assert_eq!(first.introduced(), BTreeSet::new());
    for word in first.may_introduce() {
        scope.introduce(word);
    }
    let second = parse(&grammar, &lexicon, MENTION, &scope);
    assert_eq!(second.derivation_count(), 1);
    assert_eq!(second.uses(), BTreeSet::from([option("psychosis")]));
    first.discharge(second.uses());
    assert_eq!(first.derivation_count(), 1);
    assert_eq!(first.introduced(), words);
    // The line alone in the scope it is left with has the same Readings,
    // and admission takes that scope.
    let alone = parse(&grammar, &lexicon, OFFER, first.scope());
    let together: Vec<Reading> = first.readings().collect();
    assert_eq!(alone.readings().collect::<Vec<_>>(), together);
    assert_eq!(grammar.admit(&together[0], &lexicon, first.scope()), Ok(()));
    // A use of another word discharges nothing.
    let mut other = parse(&grammar, &lexicon, OFFER, &CardScope::default());
    other.discharge([option("plague")]);
    assert_eq!(other.derivation_count(), 0);
}

/// A name a line before the text introduces is named again there, as one
/// the text introduces is (Kari Zev, Skyship Raider's *Ragavan*, were its
/// second sentence a line of its own).
#[test]
fn a_name_a_line_before_introduces_is_named_again() {
    let lexicon = lexicon();
    let grammar = bind(GRAMMAR, &lexicon).unwrap();
    let mut scope = CardScope::default();
    assert_eq!(readings("Ragavan attacks.", &scope), 0);
    let first = parse(&grammar, &lexicon, "named Ragavan.", &scope);
    let name = LexicalReading::IntroducedName {
        name: "Ragavan".to_owned(),
    };
    assert_eq!(first.may_introduce(), BTreeSet::from([name.clone()]));
    // A name is offered in no choice: it stands with no use.
    assert_eq!(first.derivation_count(), 1);
    scope.introduce(name);
    assert_eq!(readings("Ragavan attacks.", &scope), 1);
    assert_eq!(readings("Angelo attacks.", &scope), 0);
}

#[test]
fn a_production_that_offers_has_only_lexical_children() {
    let lexicon = lexicon();
    let nested = GRAMMAR.replace(
        "Cited: Mention -> Lex(OptionWord)",
        "Cited: Mention -> Lex(OptionWord)\n    Nested: Choice -> first:Mention coordinator:Word(Or) \
         last:Mention { offers }",
    );
    let error = bind(&nested, &lexicon).unwrap_err();
    assert!(
        error.contains("a Production that offers words has only lexical children"),
        "{error}"
    );
}
