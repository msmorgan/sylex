//! Clusters of Adjuncts alone: a head with no Complement heads a
//! coordination of parallel sequences of two Preposition Phrases, a later
//! one beginning with a remnant, its preposition elided (CGEL, Ch. 15 §4.3,
//! p. 1343, note 65: *We'll be in Paris for a week and Bonn for three days*).

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
        path: "tests/adjunct_clusters.rs".to_owned(),
        owner: "test".to_owned(),
    };
    Lexicon::new([
        Lexeme::verb("v:go", "go", core()),
        Lexeme::noun("n:card", "card", vec![Countability::Count], core()),
        Lexeme::noun("n:player", "player", vec![Countability::Count], core()),
        Lexeme::invariant("p:with", "with", Category::Preposition, core()),
        Lexeme::invariant("p:for", "for", Category::Preposition, core()),
        Lexeme::invariant("p:in", "in", Category::Preposition, core()),
        Lexeme::invariant("c:and", "and", Category::Coordinator, core()),
    ])
    .unwrap()
}

const FRAMES: &str = r#"(
    frames: {
        "v:go": [(family: Intransitive, slots: [])],
    },
    prepositions: {
        "p:with": [(complement: NP, hosts: Predicate)],
        "p:for": [(complement: NP, hosts: Predicate)],
        "p:in": [(complement: NP, hosts: Predicate)],
    },
)"#;

fn count(text: &str) -> usize {
    let source = r#"
        root Sentence
        feature VerbForm = Plain | Finite | GerundParticiple | PastParticiple
        category Adjunct
        category AdjunctCluster
        category AdjunctClusters : AdjunctCluster
        category Cluster
        category Clusters : Cluster
        category NounPhrase
        category PrepositionPhrase
        category PrepositionRemnant
        category Sentence
        category VerbPhrase [VerbForm]
        class Coordinator = ["c:and"]
        slot NP = NounPhrase
        Imperative: Sentence -> VerbPhrase '.' where head.VerbForm = Plain
        Noun: NounPhrase -> Lex(Noun)
        PredicateAdjunct: Adjunct -> filler:Hosted(PrepositionPhrase, Predicate)
        valence VerbPhrase {
            head: Verb,
            clusters: {
                cluster: Cluster,
                coordination: Clusters,
                adjunct: Adjunct,
                adjunct_clusters: {
                    cluster: AdjunctCluster,
                    coordination: AdjunctClusters,
                    preposition: PrepositionPhrase,
                    remnant: PrepositionRemnant,
                },
            },
        }
        valence PrepositionPhrase { head: Preposition, remnants: [NP -> PrepositionRemnant] }
        adjunct PrepositionPhrase { host: VerbPhrase, side: After, host_kind: Predicate }
        coordination AdjunctClusters { coordinators: Coordinator, conjunct: AdjunctCluster, parallel }
    "#;
    let lexicon = lexicon();
    let grammar = CompiledGrammar::bind(
        &Grammar::from_fug_str(source).unwrap(),
        &lexicon,
        &FrameCatalog::from_ron_str(FRAMES).unwrap(),
    )
    .unwrap();
    let found = grammar.parse(&lexicon, text).readings().collect::<Vec<_>>();
    for reading in &found {
        assert_eq!(grammar.admit(reading, &lexicon), Ok(()), "{text}");
        assert_eq!(grammar.realize(reading, &lexicon).unwrap(), text);
    }
    found.len()
}

#[test]
fn a_later_cluster_may_elide_the_initial_clusters_preposition() {
    // *enters with a counter for each red creature and a counter for each
    // green creature*.
    assert_eq!(count("Go with card for player and card for player."), 1);
    assert_eq!(
        count("Go with card for player and with card for player."),
        1
    );
}

#[test]
fn adjunct_clusters_are_parallel_in_their_prepositions() {
    // The elided preposition is the initial Cluster's first one, and each
    // Cluster ends in a Preposition Phrase of the initial Cluster's last.
    assert_eq!(count("Go with card for player and in card for player."), 0);
    assert_eq!(count("Go with card for player and card in player."), 0);
}

#[test]
fn a_remnant_never_begins_the_coordination() {
    assert_eq!(count("Go card for player and with card for player."), 0);
}

#[test]
fn a_single_adjunct_is_no_cluster() {
    assert_eq!(count("Go with card and card."), 0);
}
