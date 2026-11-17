//! Engine invariants for postmodification over coordinated Noun Phrases:
//! - `Shape::keeps_coordination` keeps `Summary::coordinated` across a
//!   coordination-scoping postmodifier (`coordinated: [head]`), so a second
//!   coordination-scoping postmodifier (*Finale of Promise*'s floated *each*
//!   phrase) wraps `[NP and/or NP] from your graveyard`, while a Nominal
//!   postmodifier resets `Summary::coordinated` so `BarePlural`
//!   (`uncoordinated: [head]`) wraps `[Artifact cards and red creature cards]
//!   in your hand` (*Tannuk, Steadfast Second*).
//! - `Summary::unlike_omissions` holds only on a coordination of undetermined
//!   bare-head phrases (*Ice Cauldron*'s *the [type and amount] of mana*),
//!   never across conjuncts ending in a dependent (*Johann, Apprentice
//!   Sorcerer*'s *instant or sorcery spell from the top of your library*,
//!   *Sleight of Hand*'s *your hand and the other on the bottom of your
//!   library*) or across determined Noun Phrases (*Altar of the Pantheon*'s
//!   *each color and each combination of colors*).

use sylex_lexicon::Category;
use sylex_lexicon::Countability;
use sylex_lexicon::Lexeme;
use sylex_lexicon::Lexicon;
use sylex_lexicon::Source;
use sylex_lexicon::SourceKind;
use sylex_syntax::CompiledGrammar;
use sylex_syntax::FrameCatalog;
use sylex_syntax::Grammar;
use sylex_syntax::Reading;

fn lexicon() -> Lexicon {
    let core = || Source {
        kind: SourceKind::Core,
        path: "tests/coordinated_postmodifiers.rs".to_owned(),
        owner: "test".to_owned(),
    };
    let count = |id: &str, lemma: &str| Lexeme::noun(id, lemma, vec![Countability::Count], core());
    let mass = |id: &str, lemma: &str| Lexeme::noun(id, lemma, vec![Countability::Mass], core());
    Lexicon::new([
        Lexeme::verb("v:cast", "cast", core()),
        Lexeme::verb("v:note", "note", core()),
        Lexeme::invariant("d:the", "the", Category::Determinative, core()),
        Lexeme::invariant("d:each", "each", Category::Determinative, core()),
        Lexeme::invariant("c:and", "and", Category::Coordinator, core()),
        Lexeme::invariant("c:or", "or", Category::Coordinator, core()),
        Lexeme::invariant("p:from", "from", Category::Preposition, core()),
        Lexeme::invariant("p:with", "with", Category::Preposition, core()),
        Lexeme::invariant("p:on", "on", Category::Preposition, core()),
        Lexeme::invariant("p:of", "of", Category::Preposition, core()),
        count("n:instant", "instant"),
        count("n:sorcery", "sorcery"),
        count("n:graveyard", "graveyard"),
        count("n:value", "value"),
        count("n:type", "type"),
        count("n:amount", "amount"),
        mass("n:mana", "mana"),
        count("n:color", "color"),
        count("n:combination", "combination"),
        count("n:hand", "hand"),
        count("n:rest", "rest"),
        count("n:top", "top"),
        count("n:bottom", "bottom"),
        count("n:library", "library"),
    ])
    .unwrap()
}

const FRAMES: &str = r#"(
    frames: {
        "v:cast": [(family: Monotransitive, slots: [(relation: Object, fill: NP)])],
        "v:note": [(family: Monotransitive, slots: [(relation: Object, fill: NP)])],
        "n:amount": [(family: NounComplement, slots: [(relation: Complement, fill: PP(["p:of"]))])],
        "n:combination": [(family: NounComplement, slots: [(relation: Complement, fill: PP(["p:of"]))])],
        "n:top": [(family: NounComplement, slots: [(relation: Complement, fill: PP(["p:of"]))])],
        "n:bottom": [(family: NounComplement, slots: [(relation: Complement, fill: PP(["p:of"]))])],
    },
    prepositions: {
        "p:from": [(complement: NP, hosts: Nominal)],
        "p:with": [(complement: NP, hosts: Nominal)],
        "p:on": [(complement: NP, hosts: Nominal)],
        "p:of": [(complement: NP, hosts: Nominal)],
    },
)"#;

const GRAMMAR: &str = r#"
    root Sentence
    feature Countability = Count | Mass
    feature Determination = Bare | Mixed | Overt
    feature Inflection = Invariant | IndependentGenitive | Singular | Plural | Plain | Present | Preterite | Irrealis | GerundParticiple | PastParticiple
    feature Number = Singular | Plural
    feature VerbForm = Plain | Finite | GerundParticiple | PastParticiple
    category Nominal [Number, Countability]
    category NounPhrase [Number, Determination]
    category PrepositionPhrase
    category Sentence
    category VerbPhrase [VerbForm]
    word Each = "d:each"
    class Coordinator = ["c:and", "c:or"]
    slot NP = NounPhrase
    slot PP = PrepositionPhrase
    Imperative: Sentence -> clause:VerbPhrase '.' { capital: [clause] } where clause.VerbForm = Plain
    Determined: NounPhrase -> determiner:Lex(Determinative) Nominal
        where $.Determination = Overt
    BarePlural: NounPhrase -> Nominal { uncoordinated: [head] }
        where head.Countability = Count,
              head.Number = Plural,
              $.Determination = Bare
    BareMass: NounPhrase -> Nominal { uncoordinated: [head] }
        where head.Countability = Mass,
              head.Number = Singular,
              $.Determination = Bare
    CoordinatedNounPhrasePostmodifier: NounPhrase ->
        NounPhrase modifier:Hosted(PrepositionPhrase, Nominal) {
        coordinated: [head],
    }
        where head.Determination != Bare
    FloatedEachCoordinationPostmodifier: NounPhrase ->
        NounPhrase adjunct:Word(Each) modifier:Hosted(PrepositionPhrase, Nominal) {
        coordinated: [head],
    }
    valence VerbPhrase { head: Verb }
    valence PrepositionPhrase { head: Preposition }
    valence Nominal { head: Noun }
    adjunct PrepositionPhrase { host: Nominal, side: After, host_kind: Nominal }
    coordination NounPhrase {
        coordinators: Coordinator,
        resolve: [Number -> Plural, Determination -> Mixed],
    }
    coordination Nominal {
        coordinators: Coordinator,
        resolve: [Number -> Plural, Countability -> Count],
    }
"#;

fn grammar(lexicon: &Lexicon) -> CompiledGrammar {
    CompiledGrammar::bind(
        &Grammar::from_fug_str(GRAMMAR).unwrap(),
        lexicon,
        &FrameCatalog::from_ron_str(FRAMES).unwrap(),
    )
    .unwrap()
}

fn has_production(reading: &Reading, target: &str) -> bool {
    match reading {
        Reading::Leaf(_) => false,
        Reading::Node(node) => {
            node.production.as_str() == target
                || node.children.iter().any(|c| has_production(c, target))
        }
    }
}

#[test]
fn a_coordinated_postmodifier_stays_coordinated_under_floated_each() {
    let lexicon = lexicon();
    let grammar = grammar(&lexicon);
    // Finale of Promise: *from graveyard* scopes over `[the instant and the
    // sorcery]` and leaves the resulting NounPhrase coordinated so floated
    // *each with value* can wrap it.
    let readings: Vec<Reading> = grammar
        .parse(
            &lexicon,
            "Cast the instant and the sorcery from the graveyard each with the value.",
        )
        .readings()
        .collect();
    assert!(
        readings.iter().any(|r| {
            has_production(r, "CoordinatedNounPhrasePostmodifier")
                && has_production(r, "FloatedEachCoordinationPostmodifier")
        }),
        "{readings:#?}"
    );

    // Kaya, Bane of the Dead: a mixed coordination of a determined NounPhrase
    // and a BarePlural (`Determination: Mixed`) admits one wide Reading at
    // `CoordinatedNounPhrasePostmodifier` beside the narrow NP-coordination
    // Reading (and the two Readings where `the` scopes over the Nominal
    // coordination `instant and sorceries`).
    let mixed: Vec<Reading> = grammar
        .parse(
            &lexicon,
            "Cast the instant and sorceries from the graveyard.",
        )
        .readings()
        .collect();
    assert_eq!(mixed.len(), 4, "{mixed:#?}");
    assert_eq!(
        mixed
            .iter()
            .filter(|r| has_production(r, "CoordinatedNounPhrasePostmodifier"))
            .count(),
        1,
        "{mixed:#?}"
    );

    // Tannuk, Steadfast Second: bare nominals coordinate at Nominal, where the
    // Nominal postmodifier resets `coordinated` so `BarePlural` wraps
    // `[instants and sorceries] from the graveyard` without a second wide tree
    // at `CoordinatedNounPhrasePostmodifier`.
    let bare: Vec<Reading> = grammar
        .parse(&lexicon, "Cast instants and sorceries from the graveyard.")
        .readings()
        .collect();
    assert_eq!(bare.len(), 2, "{bare:#?}");
    assert!(
        bare.iter()
            .all(|r| !has_production(r, "CoordinatedNounPhrasePostmodifier")),
        "{bare:#?}"
    );
}

#[test]
fn unlike_omissions_hold_only_for_undetermined_bare_head_coordinations() {
    let lexicon = lexicon();
    let grammar = grammar(&lexicon);

    // Ice Cauldron: *the [type and amount] of mana* coordinates undetermined
    // bare-head Nominals (`type` takes no *of* Complement; `amount` omits one),
    // so *of mana* may postmodify `type and amount` beside attaching to
    // `amount` alone.
    let cauldron: Vec<Reading> = grammar
        .parse(&lexicon, "Note the type and amount of mana.")
        .readings()
        .collect();
    assert_eq!(cauldron.len(), 2, "{cauldron:#?}");

    // Johann, Apprentice Sorcerer: in *cast an instant or sorcery spell from
    // the top of your library*, the second Nominal conjunct *sorcery from the
    // top* ends in the modifier *from the top* (`!bare_head`), so *of the
    // library* stays `top`'s Complement alone and never modifies `instant or
    // sorcery from the top`.
    let johann: Vec<Reading> = grammar
        .parse(
            &lexicon,
            "Cast the instant or sorcery from the top of the library.",
        )
        .readings()
        .collect();
    assert_eq!(johann.len(), 2, "{johann:#?}");

    // Altar of the Pantheon: *each color and each combination of colors*
    // coordinates determined NounPhrases, so *of colors* is `combination`'s
    // Complement alone, never a modifier of `each color and each combination`.
    let altar: Vec<Reading> = grammar
        .parse(&lexicon, "Note each color and each combination of colors.")
        .readings()
        .collect();
    assert_eq!(altar.len(), 1, "{altar:#?}");
    assert!(
        !has_production(&altar[0], "CoordinatedNounPhrasePostmodifier"),
        "{altar:#?}"
    );

    // Sleight of Hand / Cream of the Crop: in *the hand and the rest on the
    // bottom of the library*, `bottom` stands inside the modifier *on the
    // bottom* (`!head_final`), so *of the library* is `bottom`'s Complement
    // alone and never modifies `the hand and the rest on the bottom`.
    let sleight: Vec<Reading> = grammar
        .parse(
            &lexicon,
            "Note the hand and the rest on the bottom of the library.",
        )
        .readings()
        .collect();
    // Narrow (*the rest [on the bottom of the library]*) and wide (*[the hand
    // and the rest] [on the bottom of the library]*), never splitting *of the
    // library* off *bottom*.
    assert_eq!(sleight.len(), 2, "{sleight:#?}");
}
