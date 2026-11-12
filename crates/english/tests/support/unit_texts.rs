//! The texts the census parses for each Corpus Unit, shared by the tests
//! that check every unit's forest (`#[path]`).

use sylex_english::corpus::Corpus;
use sylex_english::corpus::Face;
use sylex_vocabulary::LexicalEnvironment;

/// Every text the census parses for a unit (`census::run`): the unit's, or
/// a cleave line's uncleaved and cleaved texts.
pub fn texts<'c>(corpus: &'c Corpus, env: &LexicalEnvironment) -> Vec<(&'c Face, String)> {
    let cleaves = sylex_english::census::cleave_texts(corpus, &env.lexicon);
    let mut out = Vec::new();
    for (unit, cleave) in corpus.units.iter().zip(cleaves) {
        let face = &corpus.faces[unit.face];
        match cleave {
            Some(split) => {
                out.push((face, split.uncleaved));
                out.push((face, split.cleaved));
            }
            None => out.push((face, unit.text.clone())),
        }
    }
    out
}
