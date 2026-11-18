//! Set Names: an expansion's name, which a card's text cites before the noun
//! *expansion* [CR#206.3a..206.3c] (*a name originally printed in the
//! Antiquities expansion*). No inventory of the set names a text may cite is
//! consulted, so a Set Name is an open lexical slot, read from the text by its
//! orthographic title shape alone.

use std::collections::BTreeSet;

use crate::LexicalMatch;
use crate::LexicalReading;

/// The lowercase words and marks a set name holds between its title words
/// (*Adventures in the Forgotten Realms*, *Shadows over Innistrad*, *Duel
/// Decks: Elves vs. Goblins*, *Magic × ...*).
const CONNECTORS: [&str; 14] = [
    "of", "the", "to", "in", "for", "into", "at", "over", "and", "a", "vs.", "&", "/", "×",
];

/// Whether a space-separated word is a title word of a set name: it begins
/// with an ASCII capital or digit, and holds only alphanumerics and the marks
/// set names write inside a word (*Phyrexia:*, *Brothers'*, *Intl.*,
/// *40,000*, *Sword-and-Sorcery*, *R/D*).
fn title_word(word: &str) -> bool {
    word.starts_with(|ch: char| ch.is_ascii_uppercase() || ch.is_ascii_digit())
        && word
            .chars()
            .all(|ch| ch.is_alphanumeric() || matches!(ch, ':' | '\'' | '-' | '.' | ',' | '/'))
}

/// Whether `name` has the shape of a set name: title words separated by
/// single spaces, with connectors between them, beginning and ending on a
/// title word.
#[must_use]
pub(crate) fn valid_set_name(name: &str) -> bool {
    let words: Vec<&str> = name.split(' ').collect();
    words.first().is_some_and(|word| title_word(word))
        && words.last().is_some_and(|word| title_word(word))
        && words
            .iter()
            .all(|word| title_word(word) || CONNECTORS.contains(word))
}

/// The run of `tokens` with a set name's shape that ends right before each of
/// `hosts`, the starts of the words that take a Set Name before them
/// ([`crate::Lexeme::set_name_host`]): the longest span of whole
/// space-separated words that begins and ends on a title word, holds only
/// title words and connectors, and is followed by a space and the host. A set
/// name is the whole run, never a shorter run inside it (*Urza's Saga*, never
/// *Saga*; *Adventures in the Forgotten Realms*, never *Forgotten Realms*). No
/// run is refused for spelling an English word or a card's name.
pub(crate) fn set_name_matches(
    tokens: &[char],
    hosts: &BTreeSet<usize>,
    matches: &mut Vec<LexicalMatch>,
) {
    for &host in hosts {
        if host < 2 || tokens[host - 1] != ' ' {
            continue;
        }
        let end = host - 1;
        let line_start = tokens[..end]
            .iter()
            .rposition(|ch| ch.is_control())
            .map_or(0, |at| at + 1);
        // The words before the host, nearest first, as (start, is a title
        // word, is a connector); words are separated by single spaces.
        let mut words = Vec::new();
        let mut word_end = end;
        loop {
            let word_start = tokens[line_start..word_end]
                .iter()
                .rposition(|ch| *ch == ' ')
                .map_or(line_start, |at| line_start + at + 1);
            if word_start == word_end {
                break;
            }
            let word: String = tokens[word_start..word_end].iter().collect();
            let (title, connector) = (title_word(&word), CONNECTORS.contains(&word.as_str()));
            if !title && !connector {
                break;
            }
            words.push((word_start, title));
            if word_start == line_start {
                break;
            }
            word_end = word_start - 1;
        }
        if !words.first().is_some_and(|&(_, title)| title) {
            continue;
        }
        let &(start, _) = words
            .iter()
            .rev()
            .find(|&&(_, title)| title)
            .expect("the word nearest the host is a title word");
        matches.push(LexicalMatch {
            start,
            end,
            reading: LexicalReading::SetName {
                name: tokens[start..end].iter().collect(),
            },
            case: None,
        });
    }
}
