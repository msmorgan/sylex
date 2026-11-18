//! A face's lines are read once each, in order, and the words they
//! introduce (Introduced Names [CR#111.4], Anchor Words [CR#614.12c], Vote
//! Options [CR#701.38b]) are carried to the lines after them
//! (`sylex_english::face`). The parse a line gets that way is the parse of
//! the line alone in the Card Scope it is left with, and admission takes
//! that scope (`docs/design/english.md` §7).

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use rayon::prelude::*;
use sylex_english::cleave::CleaveText;
use sylex_english::corpus::Corpus;
use sylex_english::corpus::Face;
use sylex_english::corpus::Unit;
use sylex_lexicon::LexicalReading;
use sylex_syntax::CompiledGrammar;
use sylex_syntax::Reading;
use sylex_vocabulary::LexicalEnvironment;

/// Each face with the Anchor Words and Vote Options its text introduces:
/// the 14 anchor-word cards, the 26 cards whose vote offers words, and
/// faces that offer or label without introducing either (Roots of Life's
/// land types [CR#607.2d], Council Guardian's colors, the flavor-labeled
/// modes of Cult of Skaro and Astarion, the Decadent [CR#207.2d]) or that
/// introduce names (the last six; Volo, Itinerant Scholar, Jungle Patrol,
/// Rite of the Raging Storm and The Curse of Fenric write a name on two
/// lines).
const FACES: [(&str, &[&str]); 52] = [
    ("Barrensteppe Siege", &["Abzan", "Mardu"]),
    ("Battle of Hoover Dam", &["Legion", "NCR"]),
    ("Citadel Siege", &["Dragons", "Khans"]),
    ("Frontier Siege", &["Dragons", "Khans"]),
    ("Frostcliff Siege", &["Jeskai", "Temur"]),
    ("Glacierwood Siege", &["Sultai", "Temur"]),
    ("Hollowmurk Siege", &["Abzan", "Sultai"]),
    ("Mirrodin Besieged", &["Mirran", "Phyrexian"]),
    ("Monastery Siege", &["Dragons", "Khans"]),
    ("Outpost Siege", &["Dragons", "Khans"]),
    ("Palace Siege", &["Dragons", "Khans"]),
    ("Phenomenon Investigators", &["Believe", "Doubt"]),
    ("Struggle for Project Purity", &["Brotherhood", "Enclave"]),
    ("Windcrag Siege", &["Jeskai", "Mardu"]),
    ("Bite of the Black Rose", &["psychosis", "sickness"]),
    ("Capital Punishment", &["death", "taxes"]),
    ("Coercive Portal", &["carnage", "homage"]),
    ("Elrond of the White Council", &["aid", "fellowship"]),
    ("Emissary Green", &["profit", "security"]),
    ("Expropriate", &["money", "time"]),
    ("Fateful Tempest", &["past", "present"]),
    ("Galadriel, Elven-Queen", &["dominion", "guidance"]),
    ("Lieutenants of the Guard", &["numbers", "strength"]),
    ("Magister of Worth", &["condemnation", "grace"]),
    ("Messenger Jays", &["feather", "quill"]),
    ("Orchard Elemental", &["harvest", "sprout"]),
    ("Path of the Animist", &["chaos", "planeswalk"]),
    ("Path of the Enigma", &["chaos", "planeswalk"]),
    ("Path of the Ghosthunter", &["chaos", "planeswalk"]),
    ("Path of the Pyromancer", &["chaos", "planeswalk"]),
    ("Path of the Schemer", &["chaos", "planeswalk"]),
    ("Plea for Power", &["knowledge", "time"]),
    ("Sail into the West", &["embark", "return"]),
    ("Selvala's Stampede", &["free", "wild"]),
    ("Split Decision", &["denial", "duplication"]),
    ("Tivit, Seller of Secrets", &["bribery", "evidence"]),
    (
        "Travel Through Caradhras",
        &["Mines of Moria", "Redhorn Pass"],
    ),
    ("Trial of a Time Lord", &["guilty", "innocent"]),
    ("Truth or Consequences", &["consequences", "truth"]),
    ("Tyrant's Choice", &["death", "torture"]),
    ("Roots of Life", &[]),
    ("Council Guardian", &[]),
    ("Cult of Skaro", &[]),
    ("Astarion, the Decadent", &[]),
    ("Rinoa Heartilly", &[]),
    ("Kari Zev, Skyship Raider", &[]),
    ("Volo, Itinerant Scholar", &[]),
    ("Jungle Patrol", &[]),
    ("Rite of the Raging Storm", &[]),
    ("The Curse of Fenric", &[]),
    ("Rufus Shinra", &[]),
    ("Hand of Vecna", &[]),
];

/// Units of more Readings than this are compared by their Derivation
/// counts alone.
const ENUMERATED: u128 = 64;

/// Reads one face and checks each of its units: the parse it got with its
/// face is the parse of its text alone in the scope it is left with, by
/// Derivation count and, up to [`ENUMERATED`] of them, by Readings, each of
/// which admission takes in that scope. Returns the words the face
/// introduces.
fn check_face(
    env: &LexicalEnvironment,
    grammar: &CompiledGrammar,
    face: &Face,
    units: &[Unit],
    cleaves: &[Option<CleaveText>],
) -> Result<BTreeSet<LexicalReading>, String> {
    let parses = sylex_english::face::read_units(face, units, cleaves, &env.lexicon, grammar);
    for ((unit, cleave), parse) in units.iter().zip(cleaves).zip(&parses) {
        let text = sylex_english::census::line_text(unit, cleave.as_ref());
        let name = |why: &str| format!("{} | {text}: {why}", face.name);
        let analyzed = env.lexicon.analyze_source_scoped(text, None, parse.scope());
        let alone = grammar.parse_analyzed(&analyzed);
        if alone.derivation_count() != parse.derivation_count() {
            return Err(name(&format!(
                "{} Derivations with its face, {} alone",
                parse.derivation_count(),
                alone.derivation_count()
            )));
        }
        if parse.derivation_count() > ENUMERATED {
            continue;
        }
        let together: BTreeSet<Reading> = parse.readings().collect();
        if alone.readings().collect::<BTreeSet<_>>() != together {
            return Err(name("the Readings differ"));
        }
        for reading in &together {
            grammar
                .admit(reading, &env.lexicon, parse.scope())
                .map_err(|error| name(&error.to_string()))?;
        }
    }
    Ok(sylex_english::face::introduced_words(&parses))
}

/// [`check_face`] on every face of `corpus`, in parallel: the words each
/// introduces, by face.
fn check_corpus(
    env: &LexicalEnvironment,
    grammar: &CompiledGrammar,
    corpus: &Corpus,
) -> Vec<BTreeSet<LexicalReading>> {
    let cleaves = sylex_english::census::cleave_texts(corpus, &env.lexicon);
    let results: Vec<Result<BTreeSet<LexicalReading>, String>> = corpus
        .face_units()
        .into_par_iter()
        .enumerate()
        .map(|(face, range)| {
            check_face(
                env,
                grammar,
                &corpus.faces[face],
                &corpus.units[range.clone()],
                &cleaves[range],
            )
        })
        .collect();
    let failures: Vec<&str> = results
        .iter()
        .filter_map(|result| result.as_ref().err().map(String::as_str))
        .collect();
    assert!(failures.is_empty(), "{}", failures.join("\n"));
    results.into_iter().map(Result::unwrap).collect()
}

#[test]
fn a_line_read_with_its_face_is_the_line_read_alone_in_its_scope() {
    let env = LexicalEnvironment::load().unwrap();
    let grammar = sylex_english::bind(&env.lexicon, &env.frames).unwrap();
    let expected: BTreeMap<&str, Vec<String>> = FACES
        .iter()
        .map(|(face, words)| (*face, words.iter().map(|word| (*word).to_owned()).collect()))
        .collect();
    let corpus = Corpus::load(|face| expected.contains_key(face.name.as_str())).unwrap();
    assert_eq!(corpus.faces.len(), FACES.len());
    let mut names = BTreeSet::new();
    for (face, introduced) in corpus
        .faces
        .iter()
        .zip(check_corpus(&env, &grammar, &corpus))
    {
        let mut words = Vec::new();
        for word in introduced {
            match word {
                LexicalReading::AnchorWord { word } | LexicalReading::VoteOption { word } => {
                    words.push(word);
                }
                LexicalReading::IntroducedName { name } => {
                    names.insert(name);
                }
                other => panic!("{}: {other:?} is introduced", face.name),
            }
        }
        words.sort();
        assert_eq!(words, expected[face.name.as_str()], "{}", face.name);
    }
    for name in [
        "Angelo",
        "Ragavan",
        "Volo's Journal",
        "Wood",
        "Lightning Rager",
        "Fenric",
    ] {
        assert!(names.contains(name), "{name}: {names:?}");
    }
}

#[test]
#[ignore = "parses every Corpus Unit twice and admits the Readings of most; run with `cargo test \
            --release -p sylex_english --test introduced_words -- --ignored --nocapture`"]
fn every_line_of_the_pool_read_with_its_face_is_the_line_read_alone_in_its_scope() {
    let env = LexicalEnvironment::load().unwrap();
    let grammar = sylex_english::bind(&env.lexicon, &env.frames).unwrap();
    let corpus = Corpus::load(|_| true).unwrap();
    let introduced = check_corpus(&env, &grammar, &corpus);
    // The faces whose text introduces an Anchor Word or a Vote Option are
    // the sample's.
    let found: BTreeSet<&str> = corpus
        .faces
        .iter()
        .zip(&introduced)
        .filter(|(_, words)| {
            words.iter().any(|word| {
                matches!(
                    word,
                    LexicalReading::AnchorWord { .. } | LexicalReading::VoteOption { .. }
                )
            })
        })
        .map(|(face, _)| face.name.as_str())
        .collect();
    let sample: BTreeSet<&str> = FACES
        .iter()
        .filter(|(_, words)| !words.is_empty())
        .map(|(face, _)| *face)
        .collect();
    assert_eq!(found, sample);
    eprintln!("{} faces, {} units", corpus.faces.len(), corpus.units.len());
}
