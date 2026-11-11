//! A `Quoted` child is set in the quotation marks of its nesting depth,
//! double then single (`docs/oracle-style-guide.md` §3, Quotation marks and
//! apostrophes). The period or comma after a closing mark is written inside
//! it, and the quotation's own final period is not written at all (§3,
//! Periods): the punctuation is derived on realization, never stored, so
//! both round-trip laws hold.

use sylex_lexicon::Category;
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
        path: "tests/quotations.rs".to_owned(),
        owner: "test".to_owned(),
    };
    Lexicon::new([
        Lexeme::invariant("pro:it", "it", Category::Pronoun, core()),
        Lexeme::invariant("c:and", "and", Category::Coordinator, core()),
        Lexeme::verb("v:gain", "gain", core()),
        Lexeme::verb("v:draw", "draw", core()),
    ])
    .unwrap()
}

fn grammar(lexicon: &Lexicon, quotation: &str) -> Result<CompiledGrammar, String> {
    let text = format!(
        r#"
            root Sentence
            category Clause
            category Quotation
            category Sentence
            word And = "c:and"
            Declarative: Sentence -> clause:Clause '.'
            Intransitive: Clause -> subject:Lex(Pronoun) Lex(Verb)
            Granting: Clause -> subject:Lex(Pronoun) Lex(Verb) object:Quotation
            Quotation: Quotation -> {quotation}
            coordination Quotation {{ coordinators: And }}
        "#
    );
    let grammar = Grammar::from_fug_str(&text).map_err(|e| format!("{e:?}"))?;
    CompiledGrammar::bind(&grammar, lexicon, &FrameCatalog::default()).map_err(|e| format!("{e:?}"))
}

#[test]
fn quotations_round_trip_with_derived_punctuation() {
    let lexicon = lexicon();
    let grammar = grammar(&lexicon, r#""Sentence""#).unwrap();
    for text in [
        r#"it gains "it draws.""#,
        r#"it gains "it gains 'it draws.'""#,
        r#"it gains "it gains 'it gains "it draws."'""#,
        r#"it gains "it draws" and "it draws.""#,
        r#"it gains "it draws," "it draws," and "it draws.""#,
        r#"it gains "it gains 'it draws' and 'it draws'" and "it draws.""#,
    ] {
        let parse = grammar.parse(&lexicon, text);
        let readings = parse.readings().collect::<Vec<_>>();
        assert_eq!(readings.len(), 1, "{text}: {readings:#?}");
        assert_eq!(parse.verify_realization(&lexicon), Ok(()), "{text}");
        assert_eq!(
            grammar.admit(&readings[0], &lexicon, &sylex_lexicon::CardScope::default()),
            Ok(()),
            "{text}"
        );
        assert_eq!(grammar.realize(&readings[0], &lexicon).unwrap(), text);
        assert!(parse.contains(&readings[0]), "{text}");
    }
}

#[test]
fn quotation_marks_and_their_punctuation_are_oracle_s() {
    let lexicon = lexicon();
    let grammar = grammar(&lexicon, r#""Sentence""#).unwrap();
    for text in [
        // The outermost marks are double, the next single.
        "it gains 'it draws.'",
        r#"it gains "it gains "it draws."""#,
        // A period or comma is written inside the closing mark, once.
        r#"it gains "it draws"."#,
        r#"it gains "it draws..""#,
        r#"it gains "it draws", "it draws", and "it draws"."#,
    ] {
        assert!(
            grammar
                .parse(&lexicon, text)
                .readings()
                .collect::<Vec<_>>()
                .is_empty(),
            "{text}"
        );
    }
}

#[test]
fn quotation_marks_are_declared_only_by_quoting() {
    let lexicon = lexicon();
    for symbol in [r#"'"'"#, "''':Bound"] {
        assert!(grammar(&lexicon, symbol).is_err(), "{symbol}");
    }
}
