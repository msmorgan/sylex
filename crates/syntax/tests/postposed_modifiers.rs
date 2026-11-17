//! A Valence schema's `postposed` Modifier Categories stand at the end of an
//! active verb phrase whose Noun Phrase Object is followed either by selected
//! Preposition Phrase Complements (*put a number of counters [on it] [equal
//! to …]*) or by none, adding a Predicate Preposition Phrase Adjunct (*choose
//! a card [at random] [that …]*; CGEL, Ch. 12 §4, p. 1066). Each agrees with
//! the Object in shared features, bears `modifier` on the Object too, obeys
//! same-head complement priority on the Adjunct, and steals no non-Object Gap.

use sylex_lexicon::CardScope;
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
        path: "tests/postposed_modifiers.rs".to_owned(),
        owner: "test".to_owned(),
    };
    Lexicon::new([
        Lexeme::verb("v:choose", "choose", core()),
        Lexeme::verb("v:put", "put", core()),
        Lexeme::verb("v:search", "search", core()),
        Lexeme::verb("v:spend", "spend", core()),
        Lexeme::verb("v:gain", "gain", core()),
        Lexeme::verb("v:shine", "shine", core()),
        Lexeme::noun("n:card", "card", vec![Countability::Count], core()),
        Lexeme::noun("n:library", "library", vec![Countability::Count], core()),
        Lexeme::noun("n:hand", "hand", vec![Countability::Count], core()),
        Lexeme::invariant("k:flying", "flying", Category::Keyword, core()),
        Lexeme::invariant("d:a", "a", Category::Determinative, core()),
        Lexeme::invariant("d:two", "two", Category::Determinative, core()),
        Lexeme::invariant("p:at", "at", Category::Preposition, core()),
        Lexeme::invariant("p:into", "into", Category::Preposition, core()),
        Lexeme::invariant("p:for", "for", Category::Preposition, core()),
        Lexeme::invariant("p:from", "from", Category::Preposition, core()),
        Lexeme::invariant("adj:random", "random", Category::Adjective, core()),
        Lexeme::invariant("adj:red", "red", Category::Adjective, core()),
        Lexeme::invariant("sub:that", "that", Category::Subordinator, core()),
        Lexeme::invariant("sub:to", "to", Category::Subordinator, core()),
    ])
    .unwrap()
}

const FRAMES: &str = r#"(
    frames: {
        "v:choose": [(family: Monotransitive, slots: [(relation: Object, fill: NP)])],
        "v:put": [(family: TransitiveWithPP, slots: [(relation: Object, fill: NP), (relation: Complement, fill: PP(["p:into"]))])],
        "v:search": [
            (family: Monotransitive, slots: [(relation: Object, fill: NP)]),
            (family: TransitiveWithPP, slots: [(relation: Object, fill: NP), (relation: Complement, fill: PP(["p:for"]))]),
        ],
        "v:spend": [(family: Monotransitive, slots: [(relation: Object, fill: NP)])],
        "v:gain": [(family: Monotransitive, slots: [(relation: Object, fill: KeywordPhrase)])],
        "v:shine": [(family: Intransitive, slots: [])],
    },
    prepositions: {
        "p:at": [(complement: AdjP, hosts: Predicate)],
        "p:into": [(complement: NP, hosts: ComplementOnly)],
        "p:for": [(complement: NP, hosts: Predicate)],
        "p:from": [(complement: NP, hosts: NominalAndPredicate)],
    },
)"#;

fn build(valence: &str, extra: &str) -> Result<CompiledGrammar, sylex_syntax::LoadError> {
    let text = format!(
        r#"
root Sentence
feature Number = Singular | Plural
feature VerbForm = Plain | Finite | GerundParticiple | PastParticiple
feature Voice = Active | Passive
feature ClauseMarking = Unmarked | Relative
feature ComplementKind = NP | AdjP | KeywordPhrase | Intransitive
category Sentence
category VerbPhrase [Number, VerbForm, Voice] {{ gap }}
category NounPhrase [Number] {{ gap }}
category Nominal [Number]
category PrepositionPhrase [ComplementKind] {{ gap }}
category AdjectivePhrase
category KeywordPhrase
category ToInf {{ gap }}
category RelativeClause [Number, ClauseMarking]
category PostposedModifier [Number] {{ function }}
category BarePostposedModifier : PostposedModifier [Number] {{ function }}
word A = "d:a"
word Two = "d:two"
word That = "sub:that"
word To = "sub:to"
slot NP = NounPhrase
slot PP = PrepositionPhrase
slot AdjP = AdjectivePhrase
slot KeywordPhrase = KeywordPhrase
slot ToInf = ToInf
Imperative: Sentence -> clause:VerbPhrase '.' where clause.VerbForm = Plain
SingularNP: NounPhrase -> det:Word(A) Nominal where $.Number = Singular
PluralNP: NounPhrase -> det:Word(Two) Nominal where $.Number = Plural
BareNoun: Nominal -> Lex(Noun)
NominalPP: Nominal -> Nominal modifier:Hosted(PrepositionPhrase, Nominal)
Adj: AdjectivePhrase -> Lex(Adjective)
Kw: KeywordPhrase -> Lex(Keyword)
ToInfinitival: ToInf -> marker:Word(To) VerbPhrase {{ transparent: [head] }} where head.VerbForm = Plain
MarkedRel: RelativeClause -> marker:Word(That) VerbPhrase
    where head.VerbForm = Finite, $.ClauseMarking = Relative
BareRel: RelativeClause -> subject:NounPhrase VerbPhrase {{ unshared: [Number] }}
    where head.VerbForm = Finite, head.Gap = NP, subject.Number = head.Number, $.ClauseMarking = Unmarked
PostposedRel: PostposedModifier -> RelativeClause where head.ClauseMarking = Relative
PostposedBareRel: BarePostposedModifier -> RelativeClause where head.ClauseMarking = Unmarked
{extra}
{valence}
valence PrepositionPhrase {{ head: Preposition, complement_kind: ComplementKind }}
adjunct PrepositionPhrase {{ host: VerbPhrase, side: After, host_kind: Predicate }}
adjunct ToInf {{ host: VerbPhrase, side: After, host_kind: Predicate }}
adjunct ToInf {{ host: VerbPhrase, side: After, host_kind: Predicate }} where adjunct.Gap = NP
        "#
    );
    CompiledGrammar::bind(
        &Grammar::from_fug_str(&text)?,
        &lexicon(),
        &FrameCatalog::from_ron_str(FRAMES).unwrap(),
    )
}

const DEFAULT_VALENCE: &str = r"
valence VerbPhrase {
    head: Verb,
    active: [$.Voice = Active],
    heavy_shift,
    postposed: [
        PostposedModifier -> [],
        BarePostposedModifier -> [adjunct.ComplementKind = AdjP],
    ],
}
";

fn parse_readings(grammar: &CompiledGrammar, text: &str) -> Vec<Reading> {
    let lex = lexicon();
    let parse = grammar.parse(&lex, text);
    let out: Vec<Reading> = parse.readings().collect();
    for r in &out {
        assert_eq!(grammar.realize(r, &lex).unwrap(), text);
        grammar.admit(r, &lex, &CardScope::default()).unwrap();
    }
    out
}

#[test]
fn postposes_marked_relative_across_selected_pp_and_predicate_adjunct_with_number_agreement() {
    let lex = lexicon();
    let grammar = build(DEFAULT_VALENCE, "").unwrap();

    // Selected `@ComplementOnly` PP (*into a hand*): postposes onto `a card`,
    // agrees in Number, and bears `modifier` on the Object too.
    let selected = parse_readings(&grammar, "Put a card into a hand that shines.");
    assert_eq!(selected.len(), 1);
    let (realized, cs) = grammar.constituents(&selected[0], &lex).unwrap();
    let object_idx = cs
        .iter()
        .position(|c| c.function.is_some_and(|f| f.as_str() == "object"))
        .unwrap();
    assert_eq!(&realized[cs[object_idx].span.clone()], "a card");
    let rel = cs
        .iter()
        .find(|c| c.category.as_str() == "RelativeClause")
        .unwrap();
    assert!(
        rel.also
            .iter()
            .any(|(f, host)| f.as_str() == "modifier" && *host == object_idx),
        "postposed RelativeClause modifies Object too: {:?}",
        rel.also
    );
    assert!(
        parse_readings(&grammar, "Put two cards into a hand that shines.").is_empty(),
        "plural Object refuses singular postposed subject relative"
    );

    // Predicate PP Adjunct (*at random*): postposes onto `a card`.
    let adjunct = parse_readings(&grammar, "Choose a card at random that shines.");
    assert_eq!(adjunct.len(), 1);
    assert!(
        parse_readings(&grammar, "Choose two cards at random that shines.").is_empty(),
        "plural Object refuses singular postposed subject relative across Adjunct"
    );
}

#[test]
fn bare_relative_gates_on_adjp_adjunct_and_same_head_complement_priority_holds() {
    let grammar = build(DEFAULT_VALENCE, "").unwrap();

    // `BarePostposedModifier` crosses *at random* (`ComplementKind = AdjP`),
    // never *from a hand* (`ComplementKind = NP`) or selected *into a hand*.
    assert_eq!(
        parse_readings(&grammar, "Choose a card at random a library chooses.").len(),
        1
    );
    assert!(
        parse_readings(&grammar, "Choose a card from a hand a library chooses.").is_empty(),
        "bare relative does not postpose past non-AdjP PP"
    );
    assert!(
        parse_readings(&grammar, "Put a card into a hand a library chooses.").is_empty(),
        "bare relative has no `adjunct` child on selected-PP frame"
    );

    // `search` declares both `Monotransitive(NP)` and `TransitiveWithPP(NP,
    // PP[for])`: same-head complement priority admits `for a card` only as
    // `complement`, never also as `adjunct` on `Monotransitive(NP)`.
    let searched = parse_readings(&grammar, "Search a library for a card that shines.");
    assert_eq!(
        searched.len(),
        1,
        "selected `for` is not also an unselected postposed-adjunct host"
    );
}

#[test]
fn steals_no_non_object_gap_and_ignores_non_np_objects() {
    let lex = lexicon();
    let grammar = build(DEFAULT_VALENCE, "").unwrap();

    // `Put into a hand` (heavy-shifted with stranded `into` Gap = NP) builds
    // no double-Object VP with `a card at random that shines`.
    assert!(
        parse_readings(&grammar, "Put into a hand a card at random that shines.").is_empty(),
        "stranded preposition Gap is not stolen as an Object"
    );
    // `Spend a library to choose` (gapped purpose `adjunct ToInf` raising
    // `Gap = NP`) does not let `spend` steal `choose`'s Object as a second
    // Object: only `choose` takes `a card` with postposed `that shines`.
    let purpose = parse_readings(
        &grammar,
        "Spend a library to choose a card at random that shines.",
    );
    assert_eq!(purpose.len(), 1);
    let (realized, cs) = grammar.constituents(&purpose[0], &lex).unwrap();
    let objects: Vec<(&str, &str)> = cs
        .iter()
        .filter(|c| c.function.is_some_and(|f| f.as_str() == "object"))
        .map(|c| {
            let parent = cs[c.parent.unwrap()].head.unwrap().as_str();
            (parent, &realized[c.span.clone()])
        })
        .collect();
    assert_eq!(objects, [("v:spend", "a library"), ("v:choose", "a card")]);

    // Non-NP Object (`KeywordPhrase`) derives no postposed Modifier.
    assert!(
        parse_readings(&grammar, "Gain flying at random that shines.").is_empty(),
        "KeywordPhrase Object takes no postposed Modifier"
    );
}

#[test]
fn rejects_invalid_postposed_declarations_at_bind_time() {
    // Non-verb head.
    let err = build(
        "valence VerbPhrase { head: Verb, active: [$.Voice = Active] }",
        "valence Nominal { head: Noun, postposed: [PostposedModifier -> []] }",
    )
    .unwrap_err();
    assert!(err.to_string().contains("non-verb head"), "{err}");

    // Non-function Category.
    let err = build(
        "valence VerbPhrase { head: Verb, active: [$.Voice = Active], postposed: [RelativeClause -> []] }",
        "",
    )
    .unwrap_err();
    assert!(err.to_string().contains("not a function Category"), "{err}");

    // Duplicate Category.
    let err = build(
        "valence VerbPhrase { head: Verb, active: [$.Voice = Active], postposed: [PostposedModifier -> [], PostposedModifier -> []] }",
        "",
    )
    .unwrap_err();
    assert!(
        err.to_string().contains("duplicate postposed Category"),
        "{err}"
    );

    // Equations that apply to no derived phrase.
    let err = build(
        "valence VerbPhrase { head: Verb, active: [$.Voice = Active], postposed: [PostposedModifier -> [nonexistent.ComplementKind = AdjP]] }",
        "",
    )
    .unwrap_err();
    assert!(
        err.to_string().contains("apply to no derived phrase"),
        "{err}"
    );
}
