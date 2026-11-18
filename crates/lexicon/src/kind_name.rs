//! Kind Names: a counter's kind, which a card's text writes as the word
//! directly before the noun *counter* [CR#122.1] (*a charge counter*, *an
//! everything counter*). Cards keep coining kinds, so no inventory of them is
//! consulted: a Kind Name is an open lexical slot, read from the text at the
//! one position it stands in.

use std::collections::BTreeSet;

use crate::LexicalMatch;
use crate::LexicalReading;
use crate::Onset;

/// The start of the orthographic word that ends at `end`: letters, with a
/// hyphen only between two of them.
fn word_start(tokens: &[char], end: usize) -> usize {
    let mut start = end;
    loop {
        match start.checked_sub(1).map(|at| tokens[at]) {
            Some(ch) if ch.is_alphabetic() => start -= 1,
            Some('-') if start < end && start >= 2 && tokens[start - 2].is_alphabetic() => {
                start -= 1;
            }
            _ => return start,
        }
    }
}

/// Whether `kind` has the shape of a Kind Name: one orthographic word.
#[must_use]
pub(crate) fn valid_kind_name(kind: &str) -> bool {
    let tokens: Vec<char> = kind.chars().collect();
    !tokens.is_empty()
        && tokens.first().is_some_and(|ch| ch.is_alphabetic())
        && word_start(&tokens, tokens.len()) == 0
}

/// The Onset of a Kind Name that begins at `start`: the one the article
/// before it agrees with (*an hour counter*, *a unity counter*), where one
/// stands there, and otherwise its first letter's.
fn onset(tokens: &[char], start: usize, kind: &str) -> Option<Onset> {
    let article = start
        .checked_sub(1)
        .filter(|&space| tokens[space] == ' ')
        .map(|space| {
            let begin = word_start(tokens, space);
            tokens[begin..space].iter().collect::<String>()
        });
    match article.as_deref() {
        Some("a" | "A") => Some(Onset::Consonant),
        Some("an" | "An") => Some(Onset::Vowel),
        _ => crate::pronunciation::default_onset(kind),
    }
}

/// The Kind Name that ends right before each of `hosts`, the starts of the
/// words a counter's kind premodifies ([`crate::Lexeme::kind_name_host`]):
/// the one orthographic word before the host, after a space, that begins
/// at a word edge. No word is a Kind Name that `closed` says a word of a
/// closed class, a numeral or an adjective spells (*those counters*, *two
/// counters*, *an additional counter*): English coins no word of a closed
/// class, and a card coins its kinds, while a numeral or an adjective
/// before the noun determines or premodifies it as itself. Any other word
/// may be one, an English word included (*time*, *plot*, *healing*).
pub(crate) fn kind_name_matches(
    tokens: &[char],
    hosts: &BTreeSet<usize>,
    closed: impl Fn(usize, usize) -> bool,
    matches: &mut Vec<LexicalMatch>,
) {
    for &host in hosts {
        if host < 2 || tokens[host - 1] != ' ' {
            continue;
        }
        let end = host - 1;
        let start = word_start(tokens, end);
        if start == end
            || !tokens[start].is_alphabetic()
            || start > 0
                && !matches!(tokens[start - 1], ' ' | '(' | '"' | '\u{201c}')
                && !tokens[start - 1].is_control()
            || closed(start, end)
        {
            continue;
        }
        let kind: String = tokens[start..end].iter().collect();
        matches.push(LexicalMatch {
            start,
            end,
            reading: LexicalReading::KindName {
                onset: onset(tokens, start, &kind),
                kind,
            },
            case: None,
        });
    }
}
