//! A Production declares which of its marks are spoken; the lexicon's sign
//! words say how (*plus*, *minus*), so a spoken mark no sign word speaks is a
//! load error.

use sylex_lexicon::Lexicon;
use sylex_syntax::CompiledGrammar;
use sylex_syntax::FrameCatalog;
use sylex_syntax::Grammar;

fn bind(text: &str) -> Result<CompiledGrammar, String> {
    CompiledGrammar::bind(
        &Grammar::from_fug_str(text).unwrap(),
        &Lexicon::new([]).unwrap(),
        &FrameCatalog::default(),
    )
    .map_err(|error| error.to_string())
}

#[test]
fn a_spoken_mark_needs_a_sign_word() {
    let grammar = |mark: &str| {
        format!(
            "
            root Sentence
            category Sentence
            Signed: Sentence -> '{mark}':Prefix:Spoken '.'
            "
        )
    };
    for sign in ["+", "-", "−"] {
        assert!(bind(&grammar(sign)).is_ok(), "{sign}");
    }
    let error = bind(&grammar("—")).unwrap_err();
    assert!(error.contains("mark `—`"), "{error}");
}
