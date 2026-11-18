//! Introduced Names: a title run is a name only where a Production
//! introduces it (`IntroducedName()`) or, in a Reading that introduces it,
//! names it again (`RepeatedName()`). The grammar here is a miniature of
//! the positions `english.fug` has: the Complement of *named*, the Object of
//! a naming verb before its description, and a card's name before a noun.

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
use sylex_syntax::Reading;

fn source(kind: SourceKind) -> Source {
    Source {
        kind,
        path: "tests/introduced_names.rs".to_owned(),
        owner: "test".to_owned(),
    }
}

/// *create* and *token* stand for themselves, as members of the classes the
/// grammar names; *card* is a noun of neither. The words the grammar names
/// are invariant here, whatever their Part of Speech in English.
fn lexicon() -> Lexicon {
    let core = |id: &str, lemma: &str, category| {
        let mut lexeme = Lexeme::invariant(id, lemma, category, source(SourceKind::Core));
        lexeme.capitalization = sylex_lexicon::Capitalization::Exact;
        lexeme
    };
    let noun = |id: &str, lemma: &str| {
        Lexeme::noun(
            id,
            lemma,
            vec![sylex_lexicon::Countability::Count],
            source(SourceKind::Core),
        )
    };
    let classed = |mut lexeme: Lexeme, class: &str| {
        lexeme.properties.classes.insert(class.into());
        lexeme
    };
    let catalog = source(SourceKind::Catalog);
    Lexicon::new([
        core("v:named", "named", Category::Adverb),
        core("v:attacks", "attacks", Category::Adverb),
        classed(core("v:create", "create", Category::Adverb), "NamingVerb"),
        classed(noun("n:token", "token"), "TokenNoun"),
        noun("n:card", "card"),
        core("d:a", "a", Category::Determinative),
        // Disa the Restless's *create a Tarmogoyf token*; Blood Fountain's
        // *create a Blood token*, where a card and a predefined token are
        // both named Blood.
        Lexeme::card_name("Tarmogoyf", catalog.clone()),
        Lexeme::card_name("Blood", catalog.clone()),
        Lexeme::invariant("token:Blood", "Blood", Category::Catalog, catalog),
    ])
    .unwrap()
}

const GRAMMAR: &str = r#"
    root Text
    category Text
    category Sentence
    category Clause
    category Phrase
    category Name
    category NameValue
    word Named = "v:named"
    word Attacks = "v:attacks"
    word A = "d:a"
    class NamingVerb { typed }
    class TokenNoun { typed }
    One: Text -> Sentence
    More: Text -> first:Sentence rest:Text
    Naming: Sentence -> Word(Named) complement:NameValue '.'
    Attack: Sentence -> subject:Phrase Word(Attacks) '.'
    Creation: Sentence -> Clause '.'
    Description: Sentence -> Clause ',' supplement:Phrase '.' {
        describes: head,
        headed_by: [supplement -> TokenNoun],
    }
    Supplement: Sentence -> Clause ',' supplement:Phrase '.'
    Create: Clause -> Class(NamingVerb) object:Phrase
    Introduced: NameValue -> IntroducedName() { within: Named }
    Offered: Phrase -> IntroducedName() { undescribed, complement_of: NamingVerb }
    Proper: Phrase -> Name
    Repeated: Name -> RepeatedName() { not_complement_of: NamingVerb }
    Listed: Name -> Lex(Catalog)
    Common: Phrase -> determiner:Word(A) Lex(Noun)
    Premodified: Phrase -> determiner:Word(A) modifier:Name Lex(Noun)
    CardNamed: Phrase -> determiner:Word(A) modifier:DistinctOtherCardName() Class(TokenNoun) {
        within: NamingVerb,
    }
"#;

fn bind(text: &str, lexicon: &Lexicon) -> CompiledGrammar {
    CompiledGrammar::bind(
        &Grammar::from_fug_str(text).unwrap(),
        lexicon,
        &FrameCatalog::default(),
    )
    .unwrap()
}

/// The Readings of `text` in `scope`, each one admitted apart from its parse
/// and realized as the text.
fn readings(text: &str, scope: &CardScope) -> Vec<Reading> {
    let lexicon = lexicon();
    let grammar = bind(GRAMMAR, &lexicon);
    let analyzed = lexicon.analyze_source_scoped(text, None, scope);
    let parse = grammar.parse_analyzed(&analyzed);
    let found: Vec<_> = parse.readings().collect();
    for reading in &found {
        assert_eq!(grammar.admit(reading, &lexicon, scope), Ok(()), "{text}");
        assert_eq!(grammar.realize(reading, &lexicon).unwrap(), text);
    }
    found
}

/// The Productions over each Introduced Name leaf of a Reading, with the
/// name.
fn names(reading: &Reading, found: &mut Vec<(String, String)>) {
    let Reading::Node(node) = reading else { return };
    for child in &node.children {
        match child {
            Reading::Leaf(LexicalReading::IntroducedName { name }) => {
                found.push((node.production.to_string(), name.clone()));
            }
            child => names(child, found),
        }
    }
}

fn named(text: &str, scope: &CardScope) -> Vec<Vec<(String, String)>> {
    readings(text, scope)
        .iter()
        .map(|reading| {
            let mut found = Vec::new();
            names(reading, &mut found);
            found
        })
        .collect()
}

fn pair(production: &str, name: &str) -> (String, String) {
    (production.to_owned(), name.to_owned())
}

fn gaps(text: &str) -> Vec<std::ops::Range<usize>> {
    let lexicon = lexicon();
    let grammar = bind(GRAMMAR, &lexicon);
    grammar.parse(&lexicon, text).vocabulary_gaps
}

#[test]
fn a_title_run_is_a_name_where_a_production_introduces_it() {
    let scope = CardScope::default();
    // Peer Review: *a ... creature token named Cadet*.
    assert_eq!(
        named("named Cadet.", &scope),
        [[pair("Introduced", "Cadet")]]
    );
    assert_eq!(gaps("named Cadet."), []);
    // Where nothing introduces it, it is no name and its word is a gap.
    assert_eq!(named("Cadet attacks.", &scope), [] as [Vec<_>; 0]);
    assert_eq!(gaps("Cadet attacks."), vec![0..5]);
}

#[test]
fn a_name_is_named_again_only_in_a_reading_that_introduces_it() {
    let scope = CardScope::default();
    // Kari Zev, Skyship Raider: *create Ragavan, a legendary 2/1 red Monkey
    // creature token. Ragavan enters tapped and attacking.*
    assert_eq!(
        named("create Ragavan, a token. Ragavan attacks.", &scope),
        [[pair("Offered", "Ragavan"), pair("Repeated", "Ragavan")]]
    );
    // The name may be named again before the sentence that introduces it,
    // and one name's introduction introduces no other.
    assert_eq!(
        named("Ragavan attacks. named Ragavan.", &scope),
        [[pair("Repeated", "Ragavan"), pair("Introduced", "Ragavan")]]
    );
    assert_eq!(
        named("Ragavan attacks. Ragavan attacks.", &scope),
        [] as [Vec<_>; 0]
    );
    assert_eq!(
        gaps("Ragavan attacks. Ragavan attacks."),
        vec![0..7, 17..24]
    );
    // An Object no description follows introduces nothing, so its word is
    // a gap.
    assert_eq!(gaps("create Angelo, Angelo attacks."), vec![7..13, 15..21]);
    assert_eq!(
        named("named Cadet. Ragavan attacks.", &scope),
        [] as [Vec<_>; 0]
    );
    // A Reading built apart from a parse is held to the same: the second
    // sentence alone names a name nothing introduces.
    let lexicon = lexicon();
    let grammar = bind(GRAMMAR, &lexicon);
    let whole = readings("named Ragavan. Ragavan attacks.", &scope);
    let [Reading::Node(text)] = whole.as_slice() else { panic!("one Reading") };
    let Reading::Node(rest) = &text.children[1] else {
        panic!("the rest of the text")
    };
    assert_eq!(
        grammar.admit_category(&rest.children[0], &lexicon, "Sentence", &scope),
        Err(AdmissionError::NotIntroduced)
    );
}

#[test]
fn a_naming_verb_s_object_is_introduced_by_its_description() {
    let scope = CardScope::default();
    // Rinoa Heartilly: *create Angelo, a legendary 1/1 green and white Dog
    // creature token*.
    assert_eq!(
        named("create Angelo, a token.", &scope),
        [[pair("Offered", "Angelo")]]
    );
    // Without the description nothing introduces the name, and a
    // description is headed by *token*.
    assert_eq!(named("create Angelo.", &scope), [] as [Vec<_>; 0]);
    assert_eq!(named("create Angelo, a card.", &scope), [] as [Vec<_>; 0]);
    // Only a naming verb's Object offers a name.
    assert_eq!(named("Angelo attacks, a token.", &scope), [] as [Vec<_>; 0]);
    // Jiang Yanggu: *If you don't control a creature named Mowu, create
    // Mowu, a legendary 3/3 green Dog creature token.* The Object offers the
    // name anew: it is never the name named again.
    assert_eq!(
        named("named Mowu. create Mowu, a token.", &scope),
        [[pair("Introduced", "Mowu"), pair("Offered", "Mowu")]]
    );
    // A Reading that leaves the name undescribed is refused apart from a
    // parse too.
    let lexicon = lexicon();
    let grammar = bind(GRAMMAR, &lexicon);
    let described = readings("create Angelo, a token.", &scope);
    let [Reading::Node(text)] = described.as_slice() else { panic!("one Reading") };
    let Reading::Node(sentence) = &text.children[0] else { panic!("a sentence") };
    assert_eq!(
        grammar.admit_category(&sentence.children[0], &lexicon, "Clause", &scope),
        Err(AdmissionError::NotIntroduced)
    );
}

#[test]
fn a_token_is_named_for_a_card_no_other_word_is_spelled_like() {
    let scope = CardScope::default();
    let production = |text: &str| -> Vec<String> {
        readings(text, &scope)
            .iter()
            .map(|reading| {
                let Reading::Node(text) = reading else { panic!("a Reading's root is a node") };
                let Reading::Node(sentence) = &text.children[0] else { panic!("a sentence") };
                let Reading::Node(clause) = &sentence.children[0] else { panic!("a clause") };
                clause.children[1].node().unwrap().production.to_string()
            })
            .collect()
    };
    assert_eq!(production("create a Tarmogoyf token."), ["CardNamed"]);
    // *Blood* is the predefined token's name there, never the card's.
    assert_eq!(production("create a Blood token."), ["Premodified"]);
    // Only inside a naming verb's Object, and only before *token*.
    assert_eq!(readings("a Tarmogoyf token attacks.", &scope).len(), 0);
    assert_eq!(readings("create a Tarmogoyf card.", &scope).len(), 0);
}

#[test]
fn a_production_declares_its_name_rules_soundly() {
    let lexicon = lexicon();
    for (from, to, defect) in [
        (
            "Offered: Phrase -> IntroducedName() { undescribed, complement_of: NamingVerb }",
            "Offered: Phrase -> Name { undescribed }",
            "an undescribed Production is one `IntroducedName()` child",
        ),
        (
            "describes: head,",
            "describes: supplement,",
            "no child follows the described one",
        ),
        (
            "{ within: Named }",
            "{ within: Named, complement_of: NamingVerb }",
            "declares one of",
        ),
        (
            "{ within: Named }",
            "{ within: Nobody }",
            "unknown closed word or class",
        ),
    ] {
        assert!(GRAMMAR.contains(from), "{from}");
        let error = CompiledGrammar::bind(
            &Grammar::from_fug_str(&GRAMMAR.replace(from, to)).unwrap(),
            &lexicon,
            &FrameCatalog::default(),
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains(defect), "{error}");
    }
}
