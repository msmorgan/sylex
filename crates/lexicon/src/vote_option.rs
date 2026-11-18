//! The shape of a Vote Option: a word a vote offers [CR#701.38a,701.38b].
//!
//! No inventory lists these words. A face's text offers them where a vote
//! is taken (*votes for sickness or psychosis*) and mentions them where it
//! says what the votes do (*If psychosis gets more votes*, *each numbers
//! vote*), so they are read from that text. Lexical analysis knows no
//! grammar: beside its matches it offers a Vote Option at every spelling
//! that has the shape of one (`AnalyzedText::open_words`), and the grammar
//! admits it only where a vote offers it or the text mentions a word a
//! vote offers (`docs/design/lexicon.md` §2). An Anchor Word [CR#614.12c]
//! is offered the same way, at each capitalized spelling of that shape.

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

/// The spans of `tokens` that have the shape of a Vote Option
/// ([`valid_vote_option`]): each orthographic word, and the title run at
/// each capitalized word.
fn shaped_spans(tokens: &[char]) -> Vec<(usize, usize)> {
    let mut spans = Vec::new();
    for start in 0..tokens.len() {
        if !tokens[start].is_alphabetic()
            || !crate::analysis::edges_match(tokens, start, start + 1, Binding::Prefix)
        {
            continue;
        }
        let word = word_end(tokens, start);
        let run = crate::introduced::starts_title_run(tokens, start)
            .then(|| crate::introduced::title_run(tokens, start, |_| false))
            .flatten();
        for end in [Some(word), run.filter(|&run| run != word)]
            .into_iter()
            .flatten()
        {
            let spelling: String = tokens[start..end].iter().collect();
            if valid_vote_option(&spelling) {
                spans.push((start, end));
            }
        }
    }
    spans
}

/// The spans at which `spelling` is written as whole words.
fn written(tokens: &[char], spelling: &str) -> Vec<(usize, usize)> {
    let spelling: Vec<char> = spelling.chars().collect();
    (0..tokens
        .len()
        .saturating_sub(spelling.len().saturating_sub(1)))
        .map(|start| (start, start + spelling.len()))
        .filter(|&(start, end)| {
            tokens[start..].starts_with(&spelling)
                && crate::analysis::edges_match(tokens, start, end, Binding::Free)
        })
        .collect()
}

/// The Anchor Words and Vote Options of `tokens`. Each one the scope
/// introduces is a lexical match wherever it is written, whatever its shape
/// (a stated Anchor Word may be any label), and joins `matches`. Returned
/// are the open words (`AnalyzedText::open_words`): a Vote Option at every
/// other spelling of its shape, and an Anchor Word at every capitalized one.
pub(crate) fn introduced_word_matches(
    tokens: &[char],
    scope: &crate::CardScope,
    matches: &mut Vec<LexicalMatch>,
) -> Vec<LexicalMatch> {
    let mut held = std::collections::BTreeSet::new();
    for word in scope.anchor_words() {
        for (start, end) in written(tokens, word) {
            let word = word.to_owned();
            held.insert((start, end, LexicalReading::AnchorWord { word }));
        }
    }
    for word in scope.vote_options() {
        for (start, end) in written(tokens, word) {
            let word = word.to_owned();
            held.insert((start, end, LexicalReading::VoteOption { word }));
        }
    }
    let mut open = std::collections::BTreeSet::new();
    for (start, end) in shaped_spans(tokens) {
        let word: String = tokens[start..end].iter().collect();
        if tokens[start].is_uppercase() && crate::valid_anchor_word(&word) {
            open.insert((
                start,
                end,
                LexicalReading::AnchorWord { word: word.clone() },
            ));
        }
        open.insert((start, end, LexicalReading::VoteOption { word }));
    }
    let found = |(start, end, reading)| LexicalMatch {
        start,
        end,
        reading,
        case: None,
    };
    let open = open.difference(&held).cloned().map(found).collect();
    matches.extend(held.into_iter().map(found));
    open
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
    fn the_shaped_spans_are_each_word_and_each_title_run() {
        let text = "Starting with you, each player votes for Redhorn Pass or Mines of Moria. For \
                    each Redhorn Pass vote, shuffle.";
        let tokens: Vec<char> = text.chars().collect();
        let spellings: Vec<String> = shaped_spans(&tokens)
            .into_iter()
            .map(|(start, end)| tokens[start..end].iter().collect())
            .collect();
        for word in [
            "Redhorn Pass",
            "Mines of Moria",
            "Pass",
            "votes",
            "or",
            "For",
        ] {
            assert!(spellings.iter().any(|s| s == word), "{word:?}");
        }
        assert!(
            !spellings
                .iter()
                .any(|s| s == "Redhorn Pass or Mines of Moria")
        );
        assert_eq!(spellings.iter().filter(|s| *s == "Redhorn Pass").count(), 2);
    }
}
