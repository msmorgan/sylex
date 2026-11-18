//! A Production's `exempt: [child -> [Name]]` keeps the `dependents Name`
//! declaration from applying to that child, one its head relates to
//! differently from every other head (perfect *have*'s Complement gives it no
//! time). Naming a declaration that does not list the child's Category is a
//! load error. The grammar here has the temporal rule of `dependents.rs`: an
//! aside shares its host clause's time (*Whenever you cast a noncreature
//! spell, choose one*, Kykar, Zephyr Awakener).

use sylex_lexicon::Category;
use sylex_lexicon::Lexeme;
use sylex_lexicon::Lexicon;
use sylex_lexicon::Source;
use sylex_lexicon::SourceKind;
use sylex_syntax::CompiledGrammar;
use sylex_syntax::FrameCatalog;
use sylex_syntax::Grammar;
use sylex_syntax::LoadError;

fn lexicon() -> Lexicon {
    let core = || Source {
        kind: SourceKind::Core,
        path: "tests/exempt_dependents.rs".to_owned(),
        owner: "test".to_owned(),
    };
    Lexicon::new([
        Lexeme::invariant("a:drew", "drew", Category::Adverb, core()),
        Lexeme::invariant("a:draws", "draws", Category::Adverb, core()),
        Lexeme::invariant("a:then", "then", Category::Adverb, core()),
        Lexeme::invariant("a:now", "now", Category::Adverb, core()),
    ])
    .unwrap()
}

const TIMED: &str = "dependents Timed = [Aside] where head.Tense = dependent.Tense";

fn grammar(lexicon: &Lexicon, rules: &str, exempt: &str) -> Result<CompiledGrammar, LoadError> {
    let text = format!(
        r#"
            root Sentence
            feature Tense = Past | NonPast
            category Aside [Tense]
            category Clause [Tense]
            category Sentence
            word Drew = "a:drew"
            word Draws = "a:draws"
            word Then = "a:then"
            word Now = "a:now"
            Old: Clause -> Word(Drew) where $.Tense = Past
            Current: Clause -> Word(Draws) where $.Tense = NonPast
            Before: Aside -> Word(Then) where $.Tense = Past
            After: Aside -> Word(Now) where $.Tense = NonPast
            Tagged: Sentence -> head:Clause aside:Aside '.' {exempt}
            Plain: Sentence -> head:Clause aside:Aside ';'
            Pair: Sentence -> head:Clause second:Clause '?'
            {rules}
        "#
    );
    CompiledGrammar::bind(
        &Grammar::from_fug_str(&text).unwrap(),
        lexicon,
        &FrameCatalog::default(),
    )
}

/// The message of the load error the rules and exemption raise.
fn load_error(lexicon: &Lexicon, rules: &str, exempt: &str) -> String {
    match grammar(lexicon, rules, exempt) {
        Err(error) => error.to_string(),
        Ok(_) => panic!("{exempt}: expected a load error"),
    }
}

fn accepted(rules: &str, exempt: &str, input: &str) -> bool {
    let lexicon = lexicon();
    let grammar = grammar(&lexicon, rules, exempt).unwrap();
    grammar.parse(&lexicon, input).readings().next().is_some()
}

#[test]
fn a_dependents_rule_rejects_a_clause_and_aside_of_two_times() {
    assert!(accepted(TIMED, "", "drew then."));
    assert!(accepted(TIMED, "", "draws now."));
    assert!(!accepted(TIMED, "", "drew now."));
    assert!(!accepted(TIMED, "", "draws then."));
    // Control: with no rule, every combination parses.
    assert!(accepted("", "", "drew now."));
}

#[test]
fn an_exempt_child_is_not_a_dependent_of_the_rule() {
    let exempt = "{ exempt: [aside -> [Timed]] }";
    assert!(accepted(TIMED, exempt, "drew now."));
    assert!(accepted(TIMED, exempt, "draws then."));
    // Only the exempt Production: another with the same child keeps the rule.
    assert!(!accepted(TIMED, exempt, "drew now;"));
    assert!(accepted(TIMED, exempt, "drew then;"));
}

#[test]
fn an_exemption_is_of_one_declaration_only() {
    let rules = format!(
        "{TIMED}\n            dependents Other = [Aside] where head.Tense = dependent.Tense"
    );
    assert!(!accepted(
        &rules,
        "{ exempt: [aside -> [Timed]] }",
        "drew now."
    ));
    assert!(accepted(
        &rules,
        "{ exempt: [aside -> [Timed, Other]] }",
        "drew now."
    ));
}

#[test]
fn a_declaration_not_listing_the_childs_category_is_a_load_error() {
    let lexicon = lexicon();
    // `Clauses` lists the Category of `second`, not of `aside`.
    let rules = format!(
        "{TIMED}\n            dependents Clauses = [Clause] where head.Tense = dependent.Tense"
    );
    let error = load_error(&lexicon, &rules, "{ exempt: [aside -> [Clauses]] }");
    assert!(
        error.contains("`aside` is exempt from `dependents Clauses`, which does not apply to it"),
        "{error}"
    );
    // Nor does a declaration that does not exist.
    let error = load_error(&lexicon, TIMED, "{ exempt: [aside -> [Missing]] }");
    assert!(error.contains("dependents Missing"), "{error}");
}
