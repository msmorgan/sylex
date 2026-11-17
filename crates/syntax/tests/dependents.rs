//! Equations stated once for every dependent of some Categories
//! (`dependents Name = [Category, ...] where ...`): the binder adds them to
//! every Production with such a dependent, authored, Schema-made or derived.
//! The grammar here gives a temporal clause's time to the phrase it modifies,
//! which may not carry the other time (*Whenever you cast a noncreature
//! spell, choose one*, Kykar, Zephyr Awakener: *cast* is present only; *the
//! life you lost when it entered*, Soulgorger Orgg: *entered* modifies
//! *lost*; CGEL, Ch. 8 §6, p. 700).

use std::collections::BTreeSet;

use sylex_lexicon::Category;
use sylex_lexicon::Countability;
use sylex_lexicon::Lexeme;
use sylex_lexicon::LexicalReading;
use sylex_lexicon::Lexicon;
use sylex_lexicon::Source;
use sylex_lexicon::SourceKind;
use sylex_lexicon::WordForm;
use sylex_syntax::CompiledGrammar;
use sylex_syntax::FrameCatalog;
use sylex_syntax::Grammar;
use sylex_syntax::Reading;

fn lexicon() -> Lexicon {
    let core = || Source {
        kind: SourceKind::Core,
        path: "tests/dependents.rs".to_owned(),
        owner: "test".to_owned(),
    };
    let verb = |id: &str, lemma: &str, past: &str| {
        let mut verb = Lexeme::verb(id, lemma, core());
        for slot in &mut verb.forms {
            if matches!(slot.form, WordForm::Preterite | WordForm::PastParticiple) {
                slot.surface = Some(past.to_owned());
            }
        }
        verb
    };
    Lexicon::new([
        verb("v:cast", "cast", "cast"),
        verb("v:draw", "draw", "drew"),
        verb("v:put", "put", "put"),
        verb("v:give", "give", "gave"),
        Lexeme::noun("n:card", "card", vec![Countability::Count], core()),
        Lexeme::noun("n:hand", "hand", vec![Countability::Count], core()),
        Lexeme::noun("n:spell", "spell", vec![Countability::Count], core()),
        Lexeme::invariant("p:when", "when", Category::Preposition, core()),
        Lexeme::invariant("p:into", "into", Category::Preposition, core()),
        Lexeme::invariant("pr:you", "you", Category::Pronoun, core()),
    ])
    .unwrap()
}

const FRAMES: &str = r#"(
    frames: {
        "v:cast": [(family: Monotransitive, slots: [(relation: Object, fill: NP)])],
        "v:draw": [(family: Monotransitive, slots: [(relation: Object, fill: NP)])],
        "v:put": [(family: TransitiveWithPP, slots: [
            (relation: Object, fill: NP),
            (relation: Complement, fill: PP(["p:into"])),
        ])],
        "v:give": [(family: TransitiveWithPP, slots: [
            (relation: Object, fill: NP),
            (relation: Complement, fill: PP(["p:into"])),
        ])],
    },
    prepositions: {
        "p:when": [(complement: Clause(Temporal), hosts: Predicate)],
        "p:into": [(complement: NP, hosts: ComplementOnly)],
    },
)"#;

/// Indented under `grammar`'s items.
const RULE: &str = "dependents Timed = [PrepositionPhrase, Interleaved]
            where $.PastTime = dependent.PastTime,
                  $.NonPastTime = dependent.NonPastTime,
                  !(head.PastTime = Past & dependent.NonPastTime = NonPast),
                  !(head.NonPastTime = NonPast & dependent.PastTime = Past)";

fn grammar(lexicon: &Lexicon, rule: &str) -> Result<CompiledGrammar, sylex_syntax::LoadError> {
    let source = format!(
        "
        root Sentence
        feature VerbForm = Plain | Finite | GerundParticiple | PastParticiple
        feature PastTime = Past
        feature NonPastTime = NonPast
        category Clause [VerbForm, PastTime, NonPastTime]
        category Interleaved [PastTime, NonPastTime] {{ function }}
        category NounPhrase
        category PrepositionPhrase [PastTime, NonPastTime]
        category Sentence
        category TemporalClause [PastTime, NonPastTime]
        category VerbPhrase [VerbForm, PastTime, NonPastTime]
        word You = \"pr:you\"
        slot NP = NounPhrase
        slot PP = PrepositionPhrase
        slot TemporalClause = TemporalClause
        Declarative: Sentence -> clause:Clause '.' {{ capital: [clause] }}
        Imperative: Clause -> VerbPhrase
            where head.VerbForm = Plain,
                  $.NonPastTime = NonPast,
                  head.PastTime != Past
        SubjectPredicate: Clause -> subject:Word(You) VerbPhrase where head.VerbForm = Finite
        Temporal: TemporalClause -> Clause where head.VerbForm = Finite
        Noun: NounPhrase -> Lex(Noun)
        InterleavedPreposition: Interleaved -> PrepositionPhrase
        valence VerbPhrase {{ head: Verb, interleaved: Interleaved }}
        valence PrepositionPhrase {{
            head: Preposition,
            agree: [PastTime -> PastTime, NonPastTime -> NonPastTime],
        }}
        adjunct PrepositionPhrase {{ host: Clause, side: Before, host_kind: Predicate, delimiter: ',' }}
        adjunct PrepositionPhrase {{ host: VerbPhrase, side: After, host_kind: Predicate }}
        {rule}
        "
    );
    CompiledGrammar::bind(
        &Grammar::from_fug_str(&source).unwrap(),
        lexicon,
        &FrameCatalog::from_ron_str(FRAMES).unwrap(),
    )
}

fn forms(reading: &Reading, lexeme: &str, found: &mut BTreeSet<WordForm>) {
    match reading {
        Reading::Leaf(LexicalReading::Word(value)) if value.lexeme.as_str() == lexeme => {
            found.insert(value.form);
        }
        Reading::Leaf(_) => {}
        Reading::Node(node) => {
            for child in &node.children {
                forms(child, lexeme, found);
            }
        }
    }
}

/// The forms *cast* takes in the text's Readings, with the rule and without.
fn cast_forms(text: &str) -> (BTreeSet<WordForm>, BTreeSet<WordForm>) {
    let lexicon = lexicon();
    let found = |rule: &str| {
        let grammar = grammar(&lexicon, rule).unwrap();
        let mut found = BTreeSet::new();
        for reading in grammar.parse(&lexicon, text).readings() {
            assert_eq!(
                grammar.admit(&reading, &lexicon, &sylex_lexicon::CardScope::default()),
                Ok(()),
                "{text}"
            );
            forms(&reading, "v:cast", &mut found);
        }
        found
    };
    (found(RULE), found(""))
}

const PRESENT: WordForm = WordForm::Present;
const PRETERITE: WordForm = WordForm::Preterite;

#[test]
fn a_dependent_before_a_clause_takes_its_time() {
    // Kykar, Zephyr Awakener: the imperative is non-past.
    let (with, without) = cast_forms("When you cast spell, draw card.");
    assert_eq!(with, BTreeSet::from([PRESENT]));
    assert_eq!(without, BTreeSet::from([PRESENT, PRETERITE]));
    // A past host takes only a past clause.
    let (with, _) = cast_forms("When you cast spell, you drew card.");
    assert_eq!(with, BTreeSet::from([PRETERITE]));
}

#[test]
fn a_dependent_after_a_verb_phrase_gives_it_its_time() {
    // *Draw a card whenever you cast a spell*: the plain Verb Phrase takes
    // the clause's time, which the imperative refuses when past.
    let (with, without) = cast_forms("Draw card when you cast spell.");
    assert_eq!(with, BTreeSet::from([PRESENT]));
    assert_eq!(without, BTreeSet::from([PRESENT, PRETERITE]));
    // Soulgorger Orgg's *the life you lost when it entered*.
    let (with, _) = cast_forms("You drew card when you cast spell.");
    assert_eq!(with, BTreeSet::from([PRETERITE]));
}

#[test]
fn a_derived_interleaved_dependent_takes_the_rule() {
    // The engine derives the Production that interleaves an Adjunct before a
    // selected Complement (*deals 3 damage [this way] to any target*); the
    // rule reaches it with no statement of its own.
    let (with, without) = cast_forms("You gave card when you cast spell into hand.");
    assert_eq!(with, BTreeSet::from([PRETERITE]));
    assert_eq!(without, BTreeSet::from([PRESENT, PRETERITE]));
}

#[test]
fn an_equation_that_applies_nowhere_is_a_load_error() {
    let lexicon = lexicon();
    let rule = "dependents Timed = [NounPhrase] where $.PastTime = dependent.PastTime";
    let error = grammar(&lexicon, rule).unwrap_err().to_string();
    assert!(error.contains("applies to no Production"), "{error}");
}

#[test]
fn a_child_named_dependent_is_a_load_error() {
    let lexicon = lexicon();
    let rule = format!(
        "{RULE}
        Named: Clause -> VerbPhrase dependent:PrepositionPhrase where head.VerbForm = Plain"
    );
    let error = grammar(&lexicon, &rule).unwrap_err().to_string();
    assert!(error.contains("a child named `dependent`"), "{error}");
}
