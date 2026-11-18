//! Vote Options [CR#701.38a,701.38b], read from a face's text.
//!
//! No inventory lists a Vote Option (`docs/design/lexicon.md` §2): a vote
//! offers "words with no rules meaning that are each connected to a
//! different effect" [CR#701.38b], and the text that offers them is the
//! only place they are given. Two positions in it together make a word a
//! Vote Option. A vote offers it (*votes for sickness or psychosis*), which
//! alone could as well offer what two words name (*votes for red or
//! green*); and the text then mentions it where it says what the votes for
//! it do (*If psychosis gets more votes*, *for each numbers vote*), which
//! connects it to its effect.
//!
//! Both are structure, so both are read from parses made to find them. A
//! line in which a word is written whose frame selects an Option Choice is
//! read with every spelling in it that has the shape of a Vote Option in
//! scope ([`sylex_lexicon::vote_option_candidates`]), and the words an
//! Option Choice of some Derivation offers are the words its vote offers.
//! The face's lines are then read with those words in scope, and an offer
//! one of whose words some Derivation mentions is confirmed: its words are
//! the face's Vote Options. Every unit of the face is parsed, and its
//! Readings admitted, with them in its Card Scope: the parse and admission
//! take the one scope, so they agree on each unit
//! (`docs/design/english.md` §7).

use std::collections::BTreeSet;

use sylex_lexicon::CardScope;
use sylex_lexicon::LexicalReading;
use sylex_lexicon::Lexicon;
use sylex_syntax::ChildRef;
use sylex_syntax::CompiledGrammar;
use sylex_syntax::Parse;
use sylex_syntax::frames::SlotKind;

/// The Category of the words a vote offers (*sickness or psychosis*).
pub const OPTION_CHOICE: &str = "OptionChoice";
/// The Category of one such word, cited (*psychosis* in *If psychosis gets
/// more votes*).
pub const OPTION_MENTION: &str = "OptionMention";

/// The Vote Options of a face whose rules-text lines are `lines`, read in
/// `scope`, its card names and Anchor Words: the words of each Option
/// Choice in some Derivation of a line, one of which an Option Mention in
/// some Derivation of a line cites.
///
/// # Panics
/// On a grammar with no [`OPTION_CHOICE`] or [`OPTION_MENTION`] Category,
/// or a parse whose forest has a unary cycle.
#[must_use]
pub fn vote_options(
    lines: &[String],
    scope: &CardScope,
    lexicon: &Lexicon,
    grammar: &CompiledGrammar,
) -> BTreeSet<String> {
    let mut offers: BTreeSet<BTreeSet<String>> = BTreeSet::new();
    for line in lines {
        let selected = lexicon
            .analyze_source_scoped(line, None, scope)
            .matches
            .iter()
            .any(|found| match &found.reading {
                LexicalReading::Word(value) => {
                    grammar.selects_complement_kind(&value.lexeme, SlotKind::OptionChoice)
                }
                LexicalReading::OwnName { .. }
                | LexicalReading::IntroducedName { .. }
                | LexicalReading::FlavorWord { .. }
                | LexicalReading::SetName { .. }
                | LexicalReading::AnchorWord { .. }
                | LexicalReading::VoteOption { .. }
                | LexicalReading::Numeral { .. } => false,
            });
        if !selected {
            continue;
        }
        let mut shaped = scope.clone();
        for candidate in sylex_lexicon::vote_option_candidates(line) {
            shaped.insert_vote_option(&candidate);
        }
        let analyzed = lexicon.analyze_source_scoped(line, None, &shaped);
        offers.extend(cited(&grammar.parse_analyzed(&analyzed), OPTION_CHOICE));
    }
    if offers.is_empty() {
        return BTreeSet::new();
    }
    let mut offered = scope.clone();
    for word in offers.iter().flatten() {
        offered.insert_vote_option(word);
    }
    let mut mentioned = BTreeSet::new();
    for line in lines {
        let analyzed = lexicon.analyze_source_scoped(line, None, &offered);
        let written = analyzed
            .matches
            .iter()
            .any(|found| matches!(found.reading, LexicalReading::VoteOption { .. }));
        if written {
            let parse = grammar.parse_analyzed(&analyzed);
            mentioned.extend(cited(&parse, OPTION_MENTION).into_iter().flatten());
        }
    }
    offers
        .into_iter()
        .filter(|offer| !offer.is_disjoint(&mentioned))
        .flatten()
        .collect()
}

/// The Vote Options under each Constituent of `category` that some
/// Derivation of `parse` holds.
fn cited(parse: &Parse<'_>, category: &str) -> Vec<BTreeSet<String>> {
    let grammar = parse.grammar();
    let category = grammar
        .category_id(category)
        .unwrap_or_else(|| panic!("the grammar has no Category `{category}`"));
    let counts = parse
        .counts()
        .unwrap_or_else(|cycle| panic!("reading Vote Options: {cycle}"));
    let forest = parse.forest();
    let mut found = Vec::new();
    for node in 0..forest.node_count() {
        if forest.node_category(node) != category || counts.node(node) == 0 {
            continue;
        }
        let mut words = BTreeSet::new();
        let mut items: Vec<usize> = forest.node_items(node).collect();
        while let Some(item) = items.pop() {
            for (previous, child) in forest.item_families(item) {
                items.extend(previous);
                match child {
                    ChildRef::Leaf(leaf) => match forest.leaf_reading(leaf) {
                        LexicalReading::VoteOption { word } => {
                            words.insert(word.clone());
                        }
                        // The coordinator between the two.
                        LexicalReading::Word(_) => {}
                        reading @ (LexicalReading::OwnName { .. }
                        | LexicalReading::IntroducedName { .. }
                        | LexicalReading::FlavorWord { .. }
                        | LexicalReading::SetName { .. }
                        | LexicalReading::AnchorWord { .. }
                        | LexicalReading::Numeral { .. }) => {
                            unreachable!("a cited option holds {reading:?}")
                        }
                    },
                    ChildRef::Node(inner) => {
                        unreachable!("a cited option holds the Constituent {inner}")
                    }
                }
            }
        }
        found.push(words);
    }
    found
}
