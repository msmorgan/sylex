//! Exact Derivation counts (`sylex_syntax::Counts`) on real Corpus Units:
//! every count against a brute-force recount and an enumeration of the
//! forest's Derivations, and each unit's Derivations against its distinct
//! Readings, a difference being two trees of one analysis
//! (`docs/decisions/english.md` §8).

#[path = "../../syntax/tests/support/forest_oracle.rs"]
mod forest_oracle;

use std::collections::BTreeSet;
use std::time::Duration;
use std::time::Instant;

use rayon::prelude::*;
use sylex_english::cleave::BracketKeywords;
use sylex_english::cleave::CleaveText;
use sylex_english::corpus::Corpus;
use sylex_english::corpus::Face;
use sylex_english::corpus::split_lines;
use sylex_syntax::CompiledGrammar;
use sylex_vocabulary::LexicalEnvironment;

/// Units with at most this many Derivations are also checked against an
/// enumeration of them: every unit of the pool.
const ENUMERATE: u128 = 10_000;

/// Real units of every size class, by face: one Derivation (*+2 Mace*); a
/// few (*Crook of Condemnation*, *Cruel Calculations*, and *Aetheric
/// Amplifier*'s three live roots); tens (*Hauken's Insight*); hundreds
/// (*Dawnhand Dissident*'s two live roots, *Coram, the Undertaker*); and
/// thousands (*Draugr Necromancer* 3,312, *Maralen, Fae Ascendant* 5,955,
/// *Remove Enchantments* 9,216).
const SAMPLE: &[&str] = &[
    "+2 Mace",
    "Crook of Condemnation",
    "Cruel Calculations",
    "Aetheric Amplifier",
    "Hauken's Insight",
    "Dawnhand Dissident",
    "Coram, the Undertaker",
    "Draugr Necromancer",
    "Maralen, Fae Ascendant",
    "Remove Enchantments",
];

/// Every text the census parses for a unit (`census::run`): the unit's, or
/// a cleave line's uncleaved and cleaved texts.
fn texts<'c>(corpus: &'c Corpus, env: &LexicalEnvironment) -> Vec<(&'c Face, String)> {
    let brackets = BracketKeywords::new(&env.lexicon);
    let cleave: Vec<bool> = corpus
        .faces
        .iter()
        .map(|face| {
            face.oracle_text
                .as_deref()
                .is_some_and(|text| brackets.mark(&split_lines(text)))
        })
        .collect();
    let mut out = Vec::new();
    for unit in &corpus.units {
        let face = &corpus.faces[unit.face];
        match cleave[unit.face]
            .then(|| CleaveText::split(&unit.text))
            .flatten()
        {
            Some(split) => {
                out.push((face, split.uncleaved));
                out.push((face, split.cleaved));
            }
            None => out.push((face, unit.text.clone())),
        }
    }
    out
}

/// Time spent in each pass over one text.
#[derive(Default, Clone, Copy)]
struct Timing {
    parse: Duration,
    inside: Duration,
    counts: Duration,
}

/// Checks one text's counts, naming the unit on a disagreement.
fn check(
    grammar: &CompiledGrammar,
    env: &LexicalEnvironment,
    face: &Face,
    text: &str,
) -> Result<Timing, String> {
    let name = |why: String| format!("{} | {text}: {why}", face.name);
    let start = Instant::now();
    let analyzed = env.lexicon.analyze_source_scoped(text, None, &face.scope());
    let parse = grammar.parse_analyzed(&analyzed);
    let parsed = Instant::now();
    let total = parse.derivation_count();
    let inside = Instant::now();
    let counts = parse.counts().map_err(|e| name(e.to_string()))?;
    let counted = Instant::now();
    if counts.total() != total {
        return Err(name(format!(
            "counted {} Derivations, derivation_count {total}",
            counts.total()
        )));
    }
    let readings = parse.readings().collect::<BTreeSet<_>>().len() as u128;
    if readings != total {
        return Err(name(format!(
            "{total} Derivations but {readings} distinct Readings: two trees of one analysis \
             (english.md §8)"
        )));
    }
    forest_oracle::check(&parse, ENUMERATE).map_err(name)?;
    Ok(Timing {
        parse: parsed - start,
        inside: inside - parsed,
        counts: counted - inside,
    })
}

/// Checks every text of the faces `keep` admits, in parallel on threads with
/// room for the recursive oracles. Returns the faces' names, the number of
/// texts and the summed timings.
fn check_corpus(keep: impl Fn(&Face) -> bool) -> (BTreeSet<String>, usize, Timing) {
    let env = LexicalEnvironment::load().unwrap();
    let grammar = sylex_english::bind(&env.lexicon, &env.frames).unwrap();
    let corpus = Corpus::load(keep).unwrap();
    let texts = texts(&corpus, &env);
    let pool = rayon::ThreadPoolBuilder::new()
        .stack_size(256 << 20)
        .build()
        .unwrap();
    let results: Vec<Result<Timing, String>> = pool.install(|| {
        texts
            .par_iter()
            .map(|(face, text)| check(&grammar, &env, face, text))
            .collect()
    });
    let failures: Vec<&str> = results
        .iter()
        .filter_map(|r| r.as_ref().err().map(String::as_str))
        .collect();
    assert!(failures.is_empty(), "{}", failures.join("\n"));
    let mut sum = Timing::default();
    for timing in results.into_iter().flatten() {
        sum.parse += timing.parse;
        sum.inside += timing.inside;
        sum.counts += timing.counts;
    }
    let faces = corpus.faces.iter().map(|face| face.name.clone()).collect();
    (faces, texts.len(), sum)
}

#[test]
fn counts_agree_with_recounts_enumeration_and_readings_on_real_units() {
    let (faces, texts, _) = check_corpus(|face| SAMPLE.contains(&face.name.as_str()));
    assert_eq!(faces.len(), SAMPLE.len(), "{faces:?}");
    assert!(texts > SAMPLE.len());
}

#[test]
#[ignore = "parses and cross-checks every Corpus Unit, about 22 CPU-minutes; run with \
            `cargo test -p sylex_english --test forest_counts -- --ignored --nocapture`"]
fn counts_agree_with_recounts_enumeration_and_readings_on_the_pool() {
    let (_, texts, timing) = check_corpus(|_| true);
    eprintln!(
        "{texts} texts; summed over threads: parse {:?}, inside count {:?}, counts {:?}",
        timing.parse, timing.inside, timing.counts
    );
}
