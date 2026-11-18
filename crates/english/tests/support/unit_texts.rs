//! The texts the census parses for each Corpus Unit, each with its parse,
//! shared by the tests that check every unit's forest (`#[path]`).

use rayon::prelude::*;
use sylex_english::corpus::Corpus;
use sylex_english::corpus::Face;
use sylex_syntax::CompiledGrammar;
use sylex_syntax::Parse;
use sylex_vocabulary::LexicalEnvironment;

/// `check` of every text the census parses for a unit (`census::run`), in
/// unit order: the unit's, or a cleave line's uncleaved and cleaved texts.
/// Each face is read as the census reads it: its lines once each, in order
/// (`sylex_english::face::read_units`), and a Cleaved Text in its line's
/// Card Scope. Faces are read in parallel.
pub fn map<'c, 'g, R: Send>(
    corpus: &'c Corpus,
    env: &LexicalEnvironment,
    grammar: &'g CompiledGrammar,
    check: impl Fn(&'c Face, &str, Parse<'g>) -> R + Sync,
) -> Vec<R> {
    let cleaves = sylex_english::census::cleave_texts(corpus, &env.lexicon);
    corpus
        .face_units()
        .into_par_iter()
        .enumerate()
        .flat_map_iter(|(face, range)| {
            let face = &corpus.faces[face];
            let units = &corpus.units[range.clone()];
            let cleaves = &cleaves[range];
            let parses =
                sylex_english::face::read_units(face, units, cleaves, &env.lexicon, grammar);
            let mut out = Vec::new();
            for ((unit, cleave), parse) in units.iter().zip(cleaves).zip(parses) {
                let second = cleave.as_ref().map(|cleave| {
                    let analyzed =
                        env.lexicon
                            .analyze_source_scoped(&cleave.cleaved, None, parse.scope());
                    grammar.parse_analyzed(&analyzed)
                });
                let text = sylex_english::census::line_text(unit, cleave.as_ref());
                out.push(check(face, text, parse));
                if let (Some(cleave), Some(parse)) = (cleave, second) {
                    out.push(check(face, &cleave.cleaved, parse));
                }
            }
            out
        })
        .collect()
}
