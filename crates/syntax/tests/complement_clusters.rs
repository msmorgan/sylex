//! Complement Clusters: a verb heads a coordination of parallel sequences of
//! its Frame Slots, each optionally ending in Adjuncts (right
//! nonce-constituent coordination, CGEL, Ch. 15 §4.3, pp. 1341–1343).

use std::collections::BTreeSet;

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
        path: "tests/complement_clusters.rs".to_owned(),
        owner: "test".to_owned(),
    };
    Lexicon::new([
        Lexeme::verb("v:give", "give", core()),
        Lexeme::noun("n:card", "card", vec![Countability::Count], core()),
        Lexeme::noun("n:hand", "hand", vec![Countability::Count], core()),
        Lexeme::noun("n:player", "player", vec![Countability::Count], core()),
        Lexeme::invariant("p:to", "to", Category::Preposition, core()),
        Lexeme::invariant("p:in", "in", Category::Preposition, core()),
        Lexeme::invariant("c:and", "and", Category::Coordinator, core()),
    ])
    .unwrap()
}

const FRAMES: &str = r#"(
    frames: {
        "v:give": [
            (family: Monotransitive, slots: [(relation: Object, fill: NP)]),
            (family: TransitiveWithPP, slots: [
                (relation: Object, fill: NP),
                (relation: Complement, fill: PP(["p:to"])),
            ]),
        ],
    },
    prepositions: {
        "p:to": [(complement: NP, hosts: ComplementOnly)],
        "p:in": [(complement: NP, hosts: Predicate)],
    },
)"#;

fn grammar(lexicon: &Lexicon) -> CompiledGrammar {
    let source = r#"
        root Sentence
        feature VerbForm = Plain | Finite | GerundParticiple | PastParticiple
        category Adjunct
        category Cluster
        category Clusters : Cluster
        category NounPhrase
        category PrepositionPhrase
        category Sentence
        category VerbPhrase [VerbForm]
        class Coordinator = ["c:and"]
        slot NP = NounPhrase
        slot PP = PrepositionPhrase
        Imperative: Sentence -> VerbPhrase '.' where head.VerbForm = Plain
        Noun: NounPhrase -> Lex(Noun)
        PredicateAdjunct: Adjunct -> filler:Hosted(PrepositionPhrase, Predicate)
        valence VerbPhrase {
            head: Verb,
            clusters: {
                cluster: Cluster,
                coordination: Clusters,
                adjunct: Adjunct,
            },
        }
        valence PrepositionPhrase { head: Preposition }
        adjunct PrepositionPhrase { host: VerbPhrase, side: After, host_kind: Predicate }
        coordination Clusters { coordinators: Coordinator, conjunct: Cluster }
    "#;
    CompiledGrammar::bind(
        &Grammar::from_fug_str(source).unwrap(),
        lexicon,
        &FrameCatalog::from_ron_str(FRAMES).unwrap(),
    )
    .unwrap()
}

fn count(text: &str) -> usize {
    let lexicon = lexicon();
    let grammar = grammar(&lexicon);
    let found = grammar.parse(&lexicon, text).readings().collect::<Vec<_>>();
    for reading in &found {
        assert_eq!(
            grammar.admit(reading, &lexicon, &BTreeSet::new()),
            Ok(()),
            "{text}"
        );
        assert_eq!(grammar.realize(reading, &lexicon).unwrap(), text);
    }
    found.len()
}

#[test]
fn clusters_of_two_slots_coordinate() {
    assert_eq!(count("Give card to player and card to player."), 1);
}

#[test]
fn a_single_slot_clusters_only_beside_parallel_clusters_with_adjuncts() {
    // Single Objects coordinate as themselves, never as Clusters
    // (`docs/decisions/english.md` §8); this grammar has no Noun Phrase
    // coordination.
    assert_eq!(count("Give card and card."), 0);
    // A single Object is a Cluster only where it alone breaks parallelism:
    // before at least two Clusters that end in as many Adjuncts (CGEL,
    // Ch. 15 §4.3, p. 1342, note 64).
    assert_eq!(count("Give card, card in hand, and card in hand."), 1);
    assert_eq!(count("Give card and card in hand."), 0);
    assert_eq!(count("Give card in hand, card in hand, and card."), 0);
    // The final *in hand* ends the last Cluster, never the Verb Phrase's
    // Adjunct after a last Cluster that is a single Object.
    assert_eq!(count("Give card in hand and card in hand."), 1);
}

#[test]
fn a_cluster_ends_in_a_sequence_of_adjuncts() {
    // *exiles the top card of their library, [a card] [at random] [from
    // their graveyard], and [a card] [at random] [from their hand]*
    let lexicon = lexicon();
    let grammar = grammar(&lexicon);
    let text = "Give card, card in hand in hand, and card in hand in hand.";
    let found = grammar.parse(&lexicon, text).readings().collect::<Vec<_>>();
    assert_eq!(found.len(), 1);
    let (realized, constituents) = grammar.constituents(&found[0], &lexicon).unwrap();
    let verb = constituents
        .iter()
        .position(|c| c.category.as_str() == "VerbPhrase")
        .unwrap();
    let adjuncts: Vec<_> = constituents
        .iter()
        .filter(|c| c.category.as_str() == "Adjunct" && &realized[c.span.clone()] == "in hand")
        .collect();
    assert_eq!(adjuncts.len(), 4);
    for adjunct in adjuncts {
        assert_eq!(adjunct.also, [(dpsi::Ident::new("adjunct"), verb)]);
    }
    // Clusters with Adjuncts beside a single Object end in as many: the
    // last *in hand* is the Verb Phrase's.
    assert_eq!(
        count("Give card, card in hand, and card in hand in hand."),
        1
    );
}

#[test]
fn any_number_of_clusters_coordinate() {
    assert_eq!(
        count("Give card in hand, card in hand, and card in hand."),
        1
    );
}

#[test]
fn clusters_are_parallel_in_shape() {
    // An Object and an Adjunct do not coordinate with an Object and a
    // selected Complement (CGEL, Ch. 15 §4.3, p. 1341, [17ii]).
    assert_eq!(count("Give card to player and card in hand."), 0);
    // A Cluster of more than one Slot may end in an Adjunct its parallel
    // Cluster lacks.
    assert_eq!(count("Give card to player in hand and card to player."), 1);
}

#[test]
fn each_dependent_in_a_cluster_bears_its_function_in_the_verbs_phrase() {
    let lexicon = lexicon();
    let grammar = grammar(&lexicon);
    let text = "Give card to player in hand and card to player.";
    let found = grammar.parse(&lexicon, text).readings().collect::<Vec<_>>();
    assert_eq!(found.len(), 1);
    let (realized, constituents) = grammar.constituents(&found[0], &lexicon).unwrap();
    let verb = constituents
        .iter()
        .position(|c| c.category.as_str() == "VerbPhrase")
        .unwrap();
    let in_hand = constituents
        .iter()
        .find(|c| c.category.as_str() == "Adjunct" && &realized[c.span.clone()] == "in hand")
        .unwrap();
    assert_eq!(in_hand.also, [(dpsi::Ident::new("adjunct"), verb)]);
}

fn remnant_lexicon() -> Lexicon {
    let core = || Source {
        kind: SourceKind::Core,
        path: "tests/complement_clusters.rs".to_owned(),
        owner: "test".to_owned(),
    };
    Lexicon::new([
        Lexeme::verb("v:gain", "gain", core()),
        Lexeme::noun("n:card", "card", vec![Countability::Count], core()),
        Lexeme::noun("n:hand", "hand", vec![Countability::Count], core()),
        Lexeme::invariant("k:protection", "protection", Category::Keyword, core()),
        Lexeme::invariant("k:hexproof", "hexproof", Category::Keyword, core()),
        Lexeme::invariant("p:from", "from", Category::Preposition, core()),
        Lexeme::invariant("p:in", "in", Category::Preposition, core()),
        Lexeme::invariant("c:and", "and", Category::Coordinator, core()),
    ])
    .unwrap()
}

const REMNANT_FRAMES: &str = r#"(
    frames: {
        "v:gain": [(family: Monotransitive, slots: [(relation: Object, fill: KeywordPhrase)])],
    },
    prepositions: {
        "p:in": [(complement: NP, hosts: Predicate)],
    },
    parameters: {
        "k:protection": [[Head, Word("p:from"), Slot(NP)]],
        "k:hexproof": [[Head, Word("p:from"), Slot(NP)]],
    },
)"#;

fn remnant_count(text: &str) -> usize {
    let source = r#"
        root Sentence
        feature VerbForm = Plain | Finite | GerundParticiple | PastParticiple
        category Adjunct
        category Cluster
        category Clusters : Cluster
        category KeywordParameters
        category KeywordPhrase
        category NounPhrase
        category ParameterRemnant
        category PrepositionPhrase
        category Sentence
        category VerbPhrase [VerbForm]
        class Coordinator = ["c:and"]
        slot NP = NounPhrase
        slot KeywordPhrase = KeywordPhrase
        Imperative: Sentence -> VerbPhrase '.' where head.VerbForm = Plain
        Noun: NounPhrase -> Lex(Noun)
        Keyword: KeywordPhrase -> KeywordParameters
        PredicateAdjunct: Adjunct -> filler:Hosted(PrepositionPhrase, Predicate)
        parameters KeywordParameters { head: Keyword, remnant: ParameterRemnant }
        valence VerbPhrase {
            head: Verb,
            clusters: {
                cluster: Cluster,
                coordination: Clusters,
                adjunct: Adjunct,
                remnants: [KeywordPhrase -> ParameterRemnant],
            },
        }
        valence PrepositionPhrase { head: Preposition }
        adjunct PrepositionPhrase { host: VerbPhrase, side: After, host_kind: Predicate }
        coordination Clusters { coordinators: Coordinator, conjunct: Cluster }
    "#;
    let lexicon = remnant_lexicon();
    let grammar = CompiledGrammar::bind(
        &Grammar::from_fug_str(source).unwrap(),
        &lexicon,
        &FrameCatalog::from_ron_str(REMNANT_FRAMES).unwrap(),
    )
    .unwrap();
    let found = grammar.parse(&lexicon, text).readings().collect::<Vec<_>>();
    for reading in &found {
        assert_eq!(
            grammar.admit(reading, &lexicon, &BTreeSet::new()),
            Ok(()),
            "{text}"
        );
        assert_eq!(grammar.realize(reading, &lexicon).unwrap(), text);
    }
    found.len()
}

#[test]
fn a_later_cluster_may_elide_the_head_of_the_first_clusters_object() {
    // *gain protection from white if ..., [from blue] if ...* (CGEL, Ch. 15
    // §4.3, p. 1343, note 65). The remnant elides *protection*, the initial
    // Cluster's Object head, and not *hexproof*, which takes the same
    // parameters: one Reading.
    assert_eq!(
        remnant_count("Gain protection from card in hand and from card in hand."),
        1
    );
    assert_eq!(
        remnant_count("Gain hexproof from card in hand, from card in hand, and from card in hand."),
        1
    );
}

#[test]
fn the_initial_cluster_is_no_remnant() {
    assert_eq!(
        remnant_count("Gain from card in hand and protection from card in hand."),
        0
    );
}
