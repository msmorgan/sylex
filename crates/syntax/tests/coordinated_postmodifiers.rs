//! Engine invariant for `Summary::unlike_omissions`: it holds only on a
//! coordination of undetermined bare-head phrases (*Ice Cauldron*'s *the [type
//! and amount] of mana*), never across conjuncts ending in a dependent
//! (*Johann, Apprentice Sorcerer*'s *instant or sorcery spell from the top of
//! your library*, *Sleight of Hand*'s *your hand and the other on the bottom
//! of your library*) or across determined Noun Phrases (*Altar of the
//! Pantheon*'s *each color and each combination of colors*).

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
    let count =
        |id: &str, lemma: &str| Lexeme::noun(id, lemma, vec![Countability::Count], core());
    let mass =
        |id: &str, lemma: &str| Lexeme::noun(id, lemma, vec![Countability::Mass], core());
    Lexicon::new([
        Lexeme::verb("v:cast", "cast", core()),
        Lexeme::verb("v:note", "note", core()),
        Lexeme::invariant("d:the", "the", Category::Determinative, core()),
        Lexeme::invariant("d:each", "each", Category::Determinative, core()),
        Lexeme::invariant("c:and", "and", Category::Coordinator, core()),
        Lexeme::invariant("c:or", "or", Category::Coordinator, core()),
        Lexeme::invariant("p:from", "from", Category::Preposition, core()),
        Lexeme::invariant("p:on", "on", Category::Preposition, core()),
        Lexeme::invariant("p:of", "of", Category::Preposition, core()),
        count("n:instant", "instant"),
        count("n:sorcery", "sorcery"),
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
        "p:on": [(complement: NP, hosts: Nominal)],
        "p:of": [(complement: NP, hosts: Nominal)],
    },
)"#;

const GRAMMAR: &str = r#"
    root Sentence
    feature Countability = Count | Mass
    feature Determination = Bare | Overt
    feature Inflection = Invariant | IndependentGenitive | Singular | Plural | Plain | Present | Preterite | Irrealis | GerundParticiple | PastParticiple
    feature Number = Singular | Plural
    feature VerbForm = Plain | Finite | GerundParticiple | PastParticiple
    category Nominal [Number, Countability]
    category NounPhrase [Number, Determination]
    category PrepositionPhrase
    category Sentence
    category VerbPhrase [VerbForm]
    class Coordinator = ["c:and", "c:or"]
    slot NP = NounPhrase
    slot PP = PrepositionPhrase
    Imperative: Sentence -> clause:VerbPhrase '.' where clause.VerbForm = Plain
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
        where head.Determination := Overt
    valence VerbPhrase { head: Verb }
    valence PrepositionPhrase { head: Preposition }
    valence Nominal { head: Noun }
    adjunct PrepositionPhrase { host: Nominal, side: After, host_kind: Nominal }
    coordination NounPhrase {
        coordinators: Coordinator,
        resolve: [Number -> Plural, Determination -> Bare],
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
        .parse(&lexicon, "Cast the instant or sorcery from the top of the library.")
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
