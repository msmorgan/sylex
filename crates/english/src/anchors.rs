//! Anchor Words [CR#614.12c], read from a face's text across its units.
//!
//! No inventory lists an Anchor Word (`docs/design/lexicon.md` §2), and no
//! one Corpus Unit settles it. A bulleted line that opens with a label
//! (*• Khans — ...*) may as well be a flavor-labeled mode [CR#207.2d], and a
//! line that offers two capitalized words (*choose Island or Swamp*) may
//! offer two subtypes. The words are Anchor Words where the face has both:
//! bulleted lines labeled with them, and a line some Reading of which
//! offers the two as its Anchor Choice.
//!
//! So the words are found by a parse made to find them: the face's lines
//! are read with every bulleted label in scope as an Anchor Word, and the
//! labels an Anchor Choice of some Derivation offers are the face's Anchor
//! Words. Every unit of the face is then parsed, and its Readings admitted,
//! with those words in its Card Scope: the parse and admission take the one
//! scope, so they agree on each unit (`docs/design/english.md` §7).

use std::collections::BTreeSet;

use sylex_lexicon::CardScope;
use sylex_lexicon::LexicalReading;
use sylex_lexicon::Lexicon;
use sylex_syntax::ChildRef;
use sylex_syntax::CompiledGrammar;
use sylex_syntax::Parse;

/// The Category of the two Anchor Words a line offers (*Khans or Dragons*).
pub const ANCHOR_CHOICE: &str = "AnchorChoice";

/// The Anchor Words of a face whose rules-text lines are `lines` and whose
/// card names are `names`: each label of a bulleted line that an Anchor
/// Choice in some Derivation of a line offers.
///
/// # Panics
/// On a grammar with no [`ANCHOR_CHOICE`] Category, or a parse whose forest
/// has a unary cycle.
#[must_use]
pub fn anchor_words(
    lines: &[String],
    names: &CardScope,
    lexicon: &Lexicon,
    grammar: &CompiledGrammar,
) -> BTreeSet<String> {
    let labels: BTreeSet<&str> = lines
        .iter()
        .filter_map(|line| sylex_lexicon::bulleted_label(line))
        .collect();
    // A choice is between two.
    if labels.len() < 2 {
        return BTreeSet::new();
    }
    let mut labeled = names.clone();
    for label in &labels {
        labeled.insert_anchor_word(label);
    }
    let mut offered = BTreeSet::new();
    for line in lines {
        let analyzed = lexicon.analyze_source_scoped(line, None, &labeled);
        let written: BTreeSet<&str> = analyzed
            .matches
            .iter()
            .filter_map(|found| match &found.reading {
                LexicalReading::AnchorWord { word } => Some(word.as_str()),
                LexicalReading::Word(_)
                | LexicalReading::OwnName { .. }
                | LexicalReading::IntroducedName { .. }
                | LexicalReading::FlavorWord { .. }
                | LexicalReading::SetName { .. }
                | LexicalReading::VoteOption { .. }
                | LexicalReading::Numeral { .. } => None,
            })
            .collect();
        if written.len() >= 2 {
            offered_words(&grammar.parse_analyzed(&analyzed), &mut offered);
        }
    }
    offered
}

/// Adds the Anchor Words that an Anchor Choice in some Derivation of
/// `parse` offers.
fn offered_words(parse: &Parse<'_>, offered: &mut BTreeSet<String>) {
    let grammar = parse.grammar();
    let category = grammar
        .category_names()
        .iter()
        .position(|name| name.as_str() == ANCHOR_CHOICE)
        .unwrap_or_else(|| panic!("the grammar has no Category `{ANCHOR_CHOICE}`"));
    let counts = parse
        .counts()
        .unwrap_or_else(|cycle| panic!("reading Anchor Words: {cycle}"));
    let forest = parse.forest();
    for node in 0..forest.node_count() {
        if forest.node_category(node) != category || counts.node(node) == 0 {
            continue;
        }
        let mut items: Vec<usize> = forest.node_items(node).collect();
        while let Some(item) = items.pop() {
            for (previous, child) in forest.item_families(item) {
                items.extend(previous);
                match child {
                    ChildRef::Leaf(leaf) => match forest.leaf_reading(leaf) {
                        LexicalReading::AnchorWord { word } => {
                            offered.insert(word.clone());
                        }
                        LexicalReading::Word(_)
                        | LexicalReading::OwnName { .. }
                        | LexicalReading::IntroducedName { .. }
                        | LexicalReading::FlavorWord { .. }
                        | LexicalReading::SetName { .. }
                        | LexicalReading::VoteOption { .. }
                        | LexicalReading::Numeral { .. } => {}
                    },
                    ChildRef::Node(inner) => {
                        unreachable!("an Anchor Choice holds the Constituent {inner}")
                    }
                }
            }
        }
    }
}
