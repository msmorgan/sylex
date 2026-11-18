//! The shape of a Vote Option: a word a vote offers [CR#701.38a,701.38b].
//!
//! No inventory lists these words. A face's text offers them where a vote
//! is taken (*votes for sickness or psychosis*) and mentions them where it
//! says what the votes do (*If psychosis gets more votes*, *each numbers
//! vote*), so they are read from that text and held in its Card Scope
//! ([`crate::CardScope`]). Lexical analysis knows no grammar: it proposes a
//! Vote Option only where the scope holds its word, and this module says
//! which spellings of a text could be one.

use std::collections::BTreeSet;

use crate::Binding;
use crate::LexicalMatch;
use crate::LexicalReading;

/// The end of the orthographic word that starts at `start`: letters, with
/// an apostrophe or hyphen only between two of them.
fn word_end(tokens: &[char], start: usize) -> usize {
    let mut end = start;
    loop {
        match tokens.get(end) {
            Some(ch) if ch.is_alphabetic() => end += 1,
            Some('\'' | '-')
                if end > start && tokens.get(end + 1).is_some_and(|ch| ch.is_alphabetic()) =>
            {
                end += 2;
            }
            _ => return end,
        }
    }
}

/// Whether `word` has the shape of a Vote Option: one orthographic word
/// (*psychosis*), or a whole title run (*Mines of Moria*,
/// [`crate::introduced::title_run`]).
#[must_use]
pub fn valid_vote_option(word: &str) -> bool {
    let tokens: Vec<char> = word.chars().collect();
    !tokens.is_empty() && word_end(&tokens, 0) == tokens.len()
        || crate::introduced::valid_introduced_name(word)
}

/// Every spelling in `text` that has the shape of a Vote Option
/// ([`valid_vote_option`]): each orthographic word, and the title run at
/// each capitalized word. Which of them a vote offers is the parse's to
/// say.
#[must_use]
pub fn vote_option_candidates(text: &str) -> BTreeSet<String> {
    let tokens: Vec<char> = text.chars().collect();
    let mut candidates = BTreeSet::new();
    for start in 0..tokens.len() {
        if !tokens[start].is_alphabetic()
            || !crate::analysis::edges_match(&tokens, start, start + 1, Binding::Prefix)
        {
            continue;
        }
        let ends = [
            Some(word_end(&tokens, start)),
            crate::introduced::starts_title_run(&tokens, start)
                .then(|| crate::introduced::title_run(&tokens, start, |_| false))
                .flatten(),
        ];
        for end in ends.into_iter().flatten() {
            let spelling: String = tokens[start..end].iter().collect();
            if valid_vote_option(&spelling) {
                candidates.insert(spelling);
            }
        }
    }
    candidates
}

/// Each Vote Option the scope holds, wherever its spelling is written as
/// whole words.
pub(crate) fn vote_option_matches(
    tokens: &[char],
    scope: &crate::CardScope,
    matches: &mut Vec<LexicalMatch>,
) {
    for word in scope.vote_options() {
        let spelling: Vec<char> = word.chars().collect();
        for start in 0..tokens.len().saturating_sub(spelling.len() - 1) {
            let end = start + spelling.len();
            if tokens[start..].starts_with(&spelling)
                && crate::analysis::edges_match(tokens, start, end, Binding::Free)
            {
                matches.push(LexicalMatch {
                    start,
                    end,
                    reading: LexicalReading::VoteOption {
                        word: word.to_owned(),
                    },
                    case: None,
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_vote_option_is_one_word_or_a_whole_title_run() {
        for word in ["psychosis", "Mines of Moria", "Redhorn Pass", "numbers"] {
            assert!(valid_vote_option(word), "{word:?}");
        }
        for word in [
            "",
            " aid",
            "aid ",
            "aid or",
            "{G}",
            "+1/+1",
            "2",
            "mines of Moria",
        ] {
            assert!(!valid_vote_option(word), "{word:?}");
        }
    }

    /// Travel Through Caradhras.
    #[test]
    fn the_candidates_are_each_word_and_each_title_run() {
        let candidates = vote_option_candidates(
            "Starting with you, each player votes for Redhorn Pass or Mines of Moria. For each \
             Redhorn Pass vote, shuffle.",
        );
        for word in [
            "Redhorn Pass",
            "Mines of Moria",
            "Pass",
            "votes",
            "or",
            "For",
        ] {
            assert!(candidates.contains(word), "{word:?}");
        }
        assert!(!candidates.contains("Redhorn Pass or Mines of Moria"));
    }
}
