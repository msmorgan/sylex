//! A face's rules text, read once: each line parsed one time, in order.
//!
//! A face's text introduces words no inventory lists
//! (`docs/design/lexicon.md` §2, §6): the name it gives an object
//! [CR#111.4], the two Anchor Words it offers as its permanent enters
//! [CR#614.12c], and the words a vote offers [CR#701.38b]. No one line
//! settles the last two. A line that offers two capitalized words (*choose
//! Island or Swamp*) may offer two subtypes, and a vote may offer what two
//! words name (*votes for red or green*): the words are an Anchor Choice or
//! an Option Choice only where the face goes on to use one of them, heading
//! a bulleted ability with it or saying what the votes for it do.
//!
//! So a position that introduces words reads whatever word stands there,
//! and the lines are read in order:
//!
//! - Each line is parsed with the words the lines before it introduce in its
//!   Card Scope, where it may use them. They are the words of every Reading so
//!   far that nothing has refused: whether a choice among them stands is known
//!   only once a later line uses one, and a choice nothing uses changes no
//!   later line (Roots of Life's second line is read with *Island* and *Swamp*
//!   introduced, and uses neither).
//! - A Reading that offers a choice stands once a use discharges it: in the
//!   same Reading, or in some Reading of a line after it. The lines after a
//!   line are read after it, so its parse keeps the roots that only they can
//!   discharge, and takes the words they use once they are read
//!   ([`sylex_syntax::Parse::discharge`]). No line is parsed again.
//!
//! A line's Card Scope then holds what its Readings depend on outside the
//! line: the face's names, the words the lines before it introduce and the
//! words the lines after it use. Parsing the line alone in that scope gives
//! the same Readings, and admission takes the same scope
//! (`docs/design/english.md` §7).

use std::collections::BTreeSet;

use sylex_lexicon::CardScope;
use sylex_lexicon::LexicalReading;
use sylex_lexicon::Lexicon;
use sylex_syntax::CompiledGrammar;
use sylex_syntax::Parse;

use crate::cleave::CleaveText;
use crate::corpus::Face;
use crate::corpus::Unit;
use crate::corpus::UnitKind;

/// The parse of each of `lines`, the rules-text lines of a face whose card
/// names are `names`, in order.
#[must_use]
pub fn read_lines<'g, S: AsRef<str>>(
    lines: &[S],
    names: &CardScope,
    lexicon: &Lexicon,
    grammar: &'g CompiledGrammar,
) -> Vec<Parse<'g>> {
    let mut scope = names.clone();
    let mut parses = Vec::with_capacity(lines.len());
    for line in lines {
        let analyzed = lexicon.analyze_source_scoped(line.as_ref(), None, &scope);
        let parse = grammar.parse_analyzed(&analyzed);
        for word in parse.may_introduce() {
            scope.introduce(word);
        }
        parses.push(parse);
    }
    let mut used = BTreeSet::new();
    for parse in parses.iter_mut().rev() {
        parse.discharge(used.iter().cloned());
        used.extend(parse.uses());
    }
    parses
}

/// The parse of each unit of `face`, in order: `units` are its units and
/// `cleaves` their [`CleaveText`]s. Its type line is read with its card
/// names alone, and its lines once each, in order ([`read_lines`]), a
/// bracketed line of a cleave face as its Uncleaved Text
/// ([`crate::census::line_text`]).
///
/// # Panics
/// As [`Face::name_scope`].
#[must_use]
pub fn read_units<'g>(
    face: &Face,
    units: &[Unit],
    cleaves: &[Option<CleaveText>],
    lexicon: &Lexicon,
    grammar: &'g CompiledGrammar,
) -> Vec<Parse<'g>> {
    let names = face.name_scope(lexicon);
    let lines: Vec<&str> = units
        .iter()
        .zip(cleaves)
        .filter(|(unit, _)| unit.kind == UnitKind::Line)
        .map(|(unit, cleave)| crate::census::line_text(unit, cleave.as_ref()))
        .collect();
    let mut lines = read_lines(&lines, &names, lexicon, grammar).into_iter();
    units
        .iter()
        .map(|unit| match unit.kind {
            UnitKind::TypeLine => {
                grammar.parse_analyzed(&lexicon.analyze_source_scoped(&unit.text, None, &names))
            }
            UnitKind::Line => lines.next().expect("a parse of each line read"),
        })
        .collect()
}

/// The words the Readings of a face's lines introduce: its Introduced
/// Names, Anchor Words and Vote Options.
#[must_use]
pub fn introduced_words(parses: &[Parse<'_>]) -> BTreeSet<LexicalReading> {
    parses.iter().flat_map(Parse::introduced).collect()
}

/// `names` with `words` introduced: the scope of a text read beside a face
/// whose lines introduce them.
#[must_use]
pub fn scope_with(names: &CardScope, words: &BTreeSet<LexicalReading>) -> CardScope {
    let mut scope = names.clone();
    for word in words {
        scope.introduce(word.clone());
    }
    scope
}
