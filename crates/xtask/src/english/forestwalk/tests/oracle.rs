//! What one Reading answers for each word: the oracle the narrowing tests
//! count Readings with, read by the Reading-side rules
//! (`CompiledGrammar::reading_head_line_child`, `child_names`) and not from
//! the forest. Only the lexical and dependency questions are read, which are
//! what a selector spelled in the survey's own words names most often; a frame
//! answer is the head line's frame Productions and is not needed here.

use std::collections::BTreeMap;

use sylex_lexicon::LexicalReading;
use sylex_syntax::Answer;
use sylex_syntax::CompiledGrammar;
use sylex_syntax::Dependency;
use sylex_syntax::ForkKind;
use sylex_syntax::ForkWord;
use sylex_syntax::Parse;
use sylex_syntax::Reading;

/// What a Reading answers, by the word's start and the kind of question.
pub(super) type Answers = BTreeMap<(usize, ForkKind), Answer>;

pub(super) struct Oracle<'a> {
    grammar: &'a CompiledGrammar,
    /// The unit's words, by start.
    words: BTreeMap<usize, ForkWord>,
    /// The forest's leaves: start, end and lexical reading.
    leaves: Vec<(usize, usize, LexicalReading)>,
}

/// One Reading's walk: where the next leaf may start, and the answers so far.
struct Walk {
    at: usize,
    answers: Answers,
}

impl Walk {
    fn answer(&mut self, start: usize, kind: ForkKind, answer: Answer) {
        let old = self.answers.insert((start, kind), answer);
        assert!(old.is_none(), "two {kind:?} answers at char {start}");
    }
}

impl<'a> Oracle<'a> {
    pub(super) fn new(parse: &Parse<'a>, words: &[ForkWord]) -> Self {
        let forest = parse.forest();
        Self {
            grammar: parse.grammar(),
            words: words.iter().map(|w| (w.start, w.clone())).collect(),
            leaves: (0..forest.metrics().leaves)
                .map(|l| {
                    let span = forest.leaf_span(l);
                    (span.start, span.end, forest.leaf_reading(l).clone())
                })
                .collect(),
        }
    }

    pub(super) fn answers(&self, reading: &Reading) -> Answers {
        let mut walk = Walk {
            at: 0,
            answers: Answers::new(),
        };
        if let Some(word) = self.walk(reading, &mut walk) {
            walk.answer(
                word,
                ForkKind::Dependency,
                Answer::Dependency(Dependency::Root),
            );
        }
        walk.answers
    }

    /// The start of a Reading's head-line word.
    fn walk(&self, reading: &Reading, walk: &mut Walk) -> Option<usize> {
        let node = match reading {
            Reading::Leaf(lexical) => {
                let (start, end) = self
                    .leaves
                    .iter()
                    .filter(|(s, _, r)| *s >= walk.at && r == lexical)
                    .map(|(s, e, _)| (*s, *e))
                    .min()
                    .unwrap_or_else(|| panic!("no leaf {lexical:?} after char {}", walk.at));
                walk.at = end;
                walk.answer(
                    start,
                    ForkKind::Lexical,
                    Answer::Lexical {
                        reading: lexical.clone(),
                    },
                );
                return Some(start);
            }
            Reading::Node(node) => node,
        };
        let line = self.grammar.reading_head_line_child(node);
        let names = self.grammar.child_names(node);
        let children: Vec<Option<usize>> = node
            .children
            .iter()
            .map(|child| self.walk(child, walk))
            .collect();
        let word = line.and_then(|k| children[k]);
        for (k, child_word) in children.into_iter().enumerate() {
            let Some(child_word) = child_word else {
                continue;
            };
            if Some(k) == line {
                continue;
            }
            walk.answer(
                child_word,
                ForkKind::Dependency,
                Answer::Dependency(Dependency::Dependent {
                    governor: word.map(|w| self.words[&w].clone()),
                    function: names[k],
                    governor_category: node.category,
                }),
            );
        }
        word
    }
}
