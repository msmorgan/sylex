//! Earley chart parsing into a Packed Forest.

use std::hash::BuildHasher;
use std::hash::BuildHasherDefault;
use std::ops::Range;
use std::sync::OnceLock;

use hashbrown::HashTable;
use hashbrown::hash_table::Entry;
use sylex_lexicon::AnalyzedText;
use sylex_lexicon::LexicalReading;
use sylex_lexicon::Lexicon;

use crate::alternatives::HeadLeaf;
use crate::alternatives::Inside;
use crate::compile::CompiledGrammar;
use crate::counts::ForestCycle;
use crate::counts::Vertex;
use crate::hash::ChartHasher;
use crate::reading::glues_after;
use crate::step::ChildView;
use crate::step::CoordinatorId;
use crate::step::LeafFacts;
use crate::step::LinkedCoordination;
use crate::step::Opening;
use crate::step::State;
use crate::step::Summary;
use crate::step::Symbol;

/// The hasher of the chart's indexes, which hold indices into
/// [`PackedForest`] and hash the key each index names.
const CHART_HASHER: BuildHasherDefault<ChartHasher> = BuildHasherDefault::new();

/// An item's identity among those ending at one position: (Production, dot,
/// start, state, seam bits).
type ItemKey = (usize, usize, usize, State, u64);

/// A completed Constituent's identity: (Category, start, end, summary).
type NodeKey = (usize, usize, usize, Summary);

/// Sizes of one parse, for measurement only.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Metrics {
    /// Distinct chart items (Production, dot, span, state), predictions
    /// included.
    pub chart_items: usize,
    /// Completed Constituents in the Packed Forest.
    pub completed_nodes: usize,
    /// Alternative Derivations recorded across completed Constituents.
    pub families: usize,
    /// Lexical alternatives offered to the chart.
    pub leaves: usize,
}

#[derive(Debug, Clone)]
pub(crate) struct Leaf {
    pub start: usize,
    pub end: usize,
    pub reading: LexicalReading,
    /// The case its first letter is written in (`LexicalMatch::case`).
    pub case: Option<sylex_lexicon::LetterCase>,
    pub facts: LeafFacts,
    pub edges: sylex_lexicon::Edges,
}

/// What a coordination names as its coordinator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Coordinator {
    /// The Lexeme of its Coordinator.
    Lexeme(sylex_lexicon::LexemeId),
    /// A Linked Coordination, which has no Coordinator: the name of its
    /// serial tail Category, its identity.
    Linked(dpsi::Ident),
}

/// A child of a partial Production in a Packed Forest: a completed
/// Constituent or a lexical alternative (by index), or structural
/// punctuation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChildRef {
    Node(usize),
    Leaf(usize),
    Punct,
}

/// A partial Production: its found children are a chain of families.
#[derive(Debug)]
pub(crate) struct Item {
    pub production: usize,
    pub dot: usize,
    pub start: usize,
    pub end: usize,
    pub state: State,
    /// For each multiword noun whose words `end` falls between, whether this
    /// item's children have a boundary at its start: see [`Seams`].
    pub seam: u64,
    /// (previous item, last child) alternatives.
    pub families: Vec<(Option<usize>, ChildRef)>,
}

impl Item {
    fn key(&self) -> ItemKey {
        (self.production, self.dot, self.start, self.state, self.seam)
    }
}

/// A completed Constituent with its alternative Derivations.
#[derive(Debug)]
pub(crate) struct Node {
    pub category: usize,
    pub start: usize,
    pub end: usize,
    pub summary: Summary,
    /// (Production, completed item) alternatives.
    pub families: Vec<(usize, usize)>,
}

impl Node {
    fn key(&self) -> NodeKey {
        (self.category, self.start, self.end, self.summary)
    }
}

/// Every Derivation of an input, packed by (Category, span, summary).
#[derive(Debug, Default)]
pub struct PackedForest {
    pub(crate) items: Vec<Item>,
    pub(crate) nodes: Vec<Node>,
    pub(crate) leaves: Vec<Leaf>,
    pub(crate) roots: Vec<usize>,
}

impl PackedForest {
    /// The completed Constituents that span the whole input.
    #[must_use]
    pub fn roots(&self) -> &[usize] {
        &self.roots
    }

    /// The Category index of a completed Constituent.
    #[must_use]
    pub fn node_category(&self, node: usize) -> usize {
        self.nodes[node].category
    }

    /// The extent of a completed Constituent, in chars.
    #[must_use]
    pub fn node_span(&self, node: usize) -> Range<usize> {
        self.nodes[node].start..self.nodes[node].end
    }

    /// The completed items that are a Constituent's alternative Derivations.
    pub fn node_items(&self, node: usize) -> impl Iterator<Item = usize> + '_ {
        self.nodes[node].families.iter().map(|&(_, item)| item)
    }

    /// A Constituent's alternatives: each a (Production, completed item)
    /// pair, in the order of [`Self::node_items`].
    pub fn node_alternatives(&self, node: usize) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.nodes[node].families.iter().copied()
    }

    /// The number of completed Constituents; a node's index is below it.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// The number of partial Productions; an item's index is below it.
    #[must_use]
    pub fn item_count(&self) -> usize {
        self.items.len()
    }

    /// The Production of a partial Production.
    #[must_use]
    pub fn item_production(&self, item: usize) -> usize {
        self.items[item].production
    }

    /// The number of children a partial Production has found.
    #[must_use]
    pub fn item_dot(&self, item: usize) -> usize {
        self.items[item].dot
    }

    /// The extent of a partial Production, in chars.
    #[must_use]
    pub fn item_span(&self, item: usize) -> Range<usize> {
        self.items[item].start..self.items[item].end
    }

    /// The lexical alternative a leaf is.
    #[must_use]
    pub fn leaf_reading(&self, leaf: usize) -> &LexicalReading {
        &self.leaves[leaf].reading
    }

    /// The (previous item, last child) alternatives of a partial Production.
    pub fn item_families(
        &self,
        item: usize,
    ) -> impl Iterator<Item = (Option<usize>, ChildRef)> + '_ {
        self.items[item].families.iter().copied()
    }

    /// The extent of a lexical alternative, in chars.
    #[must_use]
    pub fn leaf_span(&self, leaf: usize) -> Range<usize> {
        self.leaves[leaf].start..self.leaves[leaf].end
    }

    #[must_use]
    pub fn metrics(&self) -> Metrics {
        Metrics {
            chart_items: self.items.len(),
            completed_nodes: self.nodes.len(),
            families: self.nodes.iter().map(|n| n.families.len()).sum(),
            leaves: self.leaves.len(),
        }
    }
}

/// The result of parsing one input.
#[derive(Debug)]
pub struct Parse<'g> {
    pub(crate) grammar: &'g CompiledGrammar,
    /// The Category parsed for.
    pub(crate) root: usize,
    /// The analyzed text, as chars, in logical order: a period or comma
    /// written inside a closing quotation mark follows it ([`quotations`]).
    pub(crate) tokens: Vec<char>,
    /// The analyzed text as written.
    pub(crate) source: String,
    /// Immutable once parsed: the walks below are memoized over it.
    pub(crate) forest: PackedForest,
    /// Words no declared lexeme spells, as char ranges.
    pub vocabulary_gaps: Vec<Range<usize>>,
    /// The card names the text was analyzed in the scope of.
    pub(crate) scope: sylex_lexicon::CardScope,
    /// The reachable forest, children first ([`Parse::children_first`]).
    pub(crate) children_first: OnceLock<Result<Vec<Vertex>, ForestCycle>>,
    /// The head leaves of every node and item ([`Parse::head_inside`]).
    pub(crate) head_inside: OnceLock<Inside<HeadLeaf>>,
}

// Its memoized walks keep a Parse shareable across threads.
const _: fn() = || {
    fn shareable<T: Send + Sync>() {}
    shareable::<Parse<'static>>();
};

impl<'g> Parse<'g> {
    /// Every Derivation of the text.
    #[must_use]
    pub fn forest(&self) -> &PackedForest {
        &self.forest
    }

    /// The analyzed text, as chars, in logical order (a period or comma
    /// written inside a closing quotation mark follows it).
    #[must_use]
    pub fn tokens(&self) -> &[char] {
        &self.tokens
    }

    /// The grammar the text was parsed with.
    #[must_use]
    pub fn grammar(&self) -> &'g CompiledGrammar {
        self.grammar
    }

    /// The bound features of a completed Constituent, by feature name.
    #[must_use]
    pub fn node_features(
        &self,
        node: usize,
    ) -> std::collections::BTreeMap<dpsi::Ident, dpsi::Ident> {
        let node = &self.forest.nodes[node];
        self.grammar.feature_map(node.category, &node.summary)
    }

    /// The unfilled Gap a completed Constituent contains.
    #[must_use]
    pub fn node_gap(&self, node: usize) -> crate::Gap {
        self.forest.nodes[node].summary.gap
    }

    /// The coordinator of a completed Constituent that is a coordination.
    #[must_use]
    pub fn node_coordinator(&self, node: usize) -> Option<Coordinator> {
        self.forest.nodes[node]
            .summary
            .coordinator
            .map(|coordinator| match coordinator {
                CoordinatorId::Lexeme(lexeme) => {
                    Coordinator::Lexeme(self.grammar.lexeme_id(lexeme))
                }
                CoordinatorId::Linked(LinkedCoordination(tail)) => {
                    Coordinator::Linked(self.grammar.categories[usize::from(tail)])
                }
            })
    }

    /// Parses another analyzed text as the Category this Parse was made for.
    /// Unlike [`CompiledGrammar::parse_category`] with [`Parse::category`],
    /// this keeps a Category that Readings report under another Category's
    /// name.
    #[must_use]
    pub fn reparse(&self, analyzed: &AnalyzedText) -> Parse<'g> {
        self.grammar.run(analyzed, self.root)
    }
}

/// The quotation mark a `Quoted` child is compiled with
/// ([`crate::grammar::SymbolDecl::Quoted`]): it stands for whichever mark its
/// nesting depth takes ([`quotation_mark`]).
pub(crate) const QUOTATION_MARK: char = '"';

/// Whether a char is a quotation mark, double or single.
pub(crate) fn is_quotation_mark(c: char) -> bool {
    matches!(c, '"' | '\'')
}

/// The quotation mark at a nesting depth: double at even depth, single at odd
/// ("If quoted rules text itself quotes an ability, use straight single
/// quotation marks for the inner quotation"; `docs/oracle-style-guide.md`
/// §3, Quotation marks and apostrophes).
pub(crate) fn quotation_mark(depth: usize) -> char {
    if depth.is_multiple_of(2) { '"' } else { '\'' }
}

/// A char's part in quotation: a mark that opens or closes one, or a period
/// or comma written after a closing mark, where Oracle never writes one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Quote {
    Open,
    Close,
    Outside,
}

/// The quotation marks of a text, paired, and the text in logical order.
///
/// A quotation mark that no word of two or more chars covers (an apostrophe
/// inside *can't*) opens a quotation where it is the mark of its
/// depth, follows a space, a line start or an opening mark, and precedes no
/// space; it closes the innermost open quotation of the same mark where it
/// follows no space and precedes no letter or digit. A mark left unpaired is
/// no quotation mark.
///
/// Oracle writes a period or comma that follows a quotation inside its
/// closing mark (`docs/oracle-style-guide.md` §3, Periods; Quotation marks
/// and apostrophes): *has "… a card."*, *"A," "B," and "C."*. In logical
/// order it follows the closing marks, where the Production that punctuates
/// the quotation finds it, and the quotation's own final period is not
/// written at all: a `.` matches no char just before a closing mark, and no
/// written period or comma does. One written after a closing mark is
/// `Outside`, and matches nothing.
pub(crate) fn quotations(tokens: &[char], covered: &[bool]) -> (Vec<char>, Vec<Option<Quote>>) {
    let n = tokens.len();
    let mut marks = vec![None; n];
    let mut open: Vec<usize> = Vec::new();
    for at in 0..n {
        let c = tokens[at];
        if covered[at] || !is_quotation_mark(c) {
            continue;
        }
        let before = at.checked_sub(1).map(|b| tokens[b]);
        let after = tokens.get(at + 1).copied();
        if open.last().is_some_and(|&o| tokens[o] == c)
            && before.is_some_and(|b| !b.is_whitespace())
            && after.is_none_or(|a| !a.is_alphanumeric())
        {
            let o = open.pop().expect("an open quotation");
            marks[o] = Some(Quote::Open);
            marks[at] = Some(Quote::Close);
        } else if c == quotation_mark(open.len())
            && before.is_none_or(|b| b.is_whitespace() || open.last() == Some(&(at - 1)))
            && after.is_some_and(|a| !a.is_whitespace())
        {
            open.push(at);
        }
    }
    let mut logical = tokens.to_vec();
    let mut at = 0;
    while at < n {
        if marks[at] != Some(Quote::Close) {
            at += 1;
            continue;
        }
        let run = at;
        while at < n && marks[at] == Some(Quote::Close) {
            at += 1;
        }
        if at < n && matches!(logical[at], '.' | ',') && !covered[at] {
            marks[at] = Some(Quote::Outside);
        }
        if run > 0 && matches!(logical[run - 1], '.' | ',') && !covered[run - 1] {
            logical[run - 1..at].rotate_left(1);
            marks[run - 1..at].rotate_left(1);
        }
    }
    (logical, marks)
}

/// The spans of the multiword nouns of one input whose words also reach the
/// chart as leaves of their own.
///
/// The separate words stand apart only where they straddle a constituent
/// boundary: in the smallest Constituent spanning the term, the child holding
/// its first word begins before the term and the child holding its last word
/// ends after it. Where either child begins or ends with the term, its words
/// rebuild the term's own analysis, alone (*[first] [strike]*) or with a
/// dependent of one part (*mana [cost for ...]*), beside the term itself.
///
/// The rule is checked where a Production that began at or before the term
/// takes the child that reaches the term's end from inside it: an item
/// records, for each term whose words its end falls between, whether one of
/// its children begins where that term begins.
struct Seams {
    spans: Vec<Range<usize>>,
    /// For each position, the spans strictly containing it.
    inside: Vec<Vec<usize>>,
}

impl Seams {
    fn new(mut spans: Vec<Range<usize>>, n: usize) -> Self {
        spans.sort_by_key(|span| (span.start, span.end));
        spans.dedup();
        let mut inside = vec![Vec::new(); n + 1];
        for (index, span) in spans.iter().enumerate() {
            for at in &mut inside[span.start + 1..span.end] {
                at.push(index);
            }
        }
        Self { spans, inside }
    }

    /// Whether an item ending at `end` has a boundary at the start of span
    /// `span`, given its seam bits.
    fn bit(&self, end: usize, seam: u64, span: usize) -> bool {
        self.inside[end]
            .iter()
            .position(|&s| s == span)
            .is_some_and(|bit| seam >> bit & 1 == 1)
    }

    /// The seam bits of an item whose first word is at `first` and that ends
    /// at `end`, after a child whose first word is at `child`, following
    /// `previous` (its end and seam bits); or `None` when that child leaves
    /// a term's words not straddling a constituent boundary.
    fn after(
        &self,
        first: usize,
        previous: Option<(usize, u64)>,
        child: usize,
        end: usize,
    ) -> Option<u64> {
        if self.spans.is_empty() {
            return Some(0);
        }
        let had_start = |span: usize| {
            let start = self.spans[span].start;
            child == start
                || (child > start && previous.is_some_and(|(at, seam)| self.bit(at, seam, span)))
        };
        let rebuilt = self.spans.iter().enumerate().any(|(index, span)| {
            first <= span.start
                && span.start < child
                && child < span.end
                && span.end <= end
                && (span.end == end || had_start(index))
        });
        if rebuilt {
            return None;
        }
        Some(
            self.inside[end]
                .iter()
                .enumerate()
                .filter(|&(_, &span)| had_start(span))
                .fold(0, |seam, (bit, _)| seam | 1 << bit),
        )
    }
}

/// The chart's item sets, each at a boundary: a position some item ends at.
/// Only a boundary has a set, an index and waiting items, not every char
/// offset between them (the offsets inside a word).
struct Boundaries {
    /// Each position's boundary: position 0's from the start, any other's
    /// once an item ends there.
    at: Vec<Option<usize>>,
    /// Each boundary's items, in the order found.
    sets: Vec<Vec<usize>>,
    /// Each boundary's items by [`ItemKey`], as indices into `forest.items`.
    index: Vec<HashTable<usize>>,
    /// Each boundary's items waiting for a Category, by Category; allocated
    /// when the parse reaches the boundary.
    waiting: Vec<Vec<Vec<usize>>>,
}

impl Boundaries {
    fn new(n: usize) -> Self {
        Self {
            at: vec![None; n + 1],
            sets: Vec::new(),
            index: Vec::new(),
            waiting: Vec::new(),
        }
    }

    /// The boundary at a position, made on first use.
    fn boundary(&mut self, position: usize) -> usize {
        if let Some(boundary) = self.at[position] {
            return boundary;
        }
        let boundary = self.sets.len();
        self.at[position] = Some(boundary);
        self.sets.push(Vec::new());
        self.index.push(HashTable::new());
        self.waiting.push(Vec::new());
        boundary
    }

    /// The boundary at a position the parse has reached.
    fn reached(&self, position: usize) -> usize {
        self.at[position].expect("a reached position is a boundary")
    }

    /// Adds the item `key` ending at `end`, unless it is there already, and
    /// a family to it.
    fn add(
        &mut self,
        forest: &mut PackedForest,
        end: usize,
        key: ItemKey,
        family: Option<(Option<usize>, ChildRef)>,
    ) {
        let boundary = self.boundary(end);
        let items = &forest.items;
        let index = match self.index[boundary].entry(
            CHART_HASHER.hash_one(key),
            |&i| items[i].key() == key,
            |&i| CHART_HASHER.hash_one(items[i].key()),
        ) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(entry) => {
                let i = forest.items.len();
                entry.insert(i);
                let (production, dot, start, state, seam) = key;
                forest.items.push(Item {
                    production,
                    dot,
                    start,
                    end,
                    state,
                    seam,
                    families: Vec::new(),
                });
                self.sets[boundary].push(i);
                i
            }
        };
        if let Some(family) = family {
            forest.items[index].families.push(family);
        }
    }
}

impl CompiledGrammar {
    /// Parses `text` as the grammar's root Category.
    #[must_use]
    pub fn parse<'g>(&'g self, lexicon: &Lexicon, text: &str) -> Parse<'g> {
        self.parse_analyzed(&lexicon.analyze(text))
    }

    /// Parses an already analyzed text, such as one analyzed with a card's
    /// names in scope, as the grammar's root Category.
    #[must_use]
    pub fn parse_analyzed<'g>(&'g self, analyzed: &AnalyzedText) -> Parse<'g> {
        self.run(analyzed, self.root)
    }

    /// Parses an analyzed text as the named Category, or `None` when the
    /// grammar has no such Category.
    #[must_use]
    pub fn parse_category<'g>(
        &'g self,
        analyzed: &AnalyzedText,
        category: &str,
    ) -> Option<Parse<'g>> {
        let root = self.category_id(category)?;
        Some(self.run(analyzed, root))
    }

    /// The spans of the multiword nouns of `analyzed`, each with whether its
    /// words may stand apart where they straddle a constituent boundary:
    /// those of a term declared outside the core lexicon may
    /// (`docs/design/lexicon.md` §5). A noun whose first
    /// word is also a preposition is none: its words form a Preposition
    /// Phrase, never the noun's own analysis (*cards you own [outside the
    /// game]* beside the noun *outside the game*).
    pub(crate) fn atomic_spans(&self, analyzed: &AnalyzedText) -> Vec<(Range<usize>, bool)> {
        let prepositional = |start: usize| {
            analyzed.matches.iter().any(|p| {
                p.start == start
                    && matches!(&p.reading, LexicalReading::Word(w)
                    if self.lexeme(&w.lexeme).is_some_and(|l| {
                        let f = self.facts(l);
                        f.category == sylex_lexicon::Category::Preposition
                    }))
            })
        };
        analyzed
            .matches
            .iter()
            .filter_map(|m| {
                let LexicalReading::Word(word) = &m.reading else { return None };
                let facts = self.facts(self.lexeme(&word.lexeme)?);
                (m.start == 0 || !analyzed.tokens[m.start - 1].is_alphanumeric()).then_some(())?;
                (facts.category == sylex_lexicon::Category::Noun
                    && analyzed.tokens[m.start..m.end].contains(&' ')
                    && !prepositional(m.start))
                .then_some((
                    m.start..m.end,
                    facts.source != sylex_lexicon::SourceKind::Core,
                ))
            })
            .collect()
    }

    #[expect(clippy::too_many_lines, reason = "one Earley loop reads best whole")]
    pub(crate) fn run<'g>(&'g self, analyzed: &AnalyzedText, root: usize) -> Parse<'g> {
        let n = analyzed.tokens.len();
        let mut covered = vec![false; n];
        // A word spelled by a quotation mark alone (the genitive apostrophe
        // of *owners'*) leaves it a candidate mark, and so does a Flavor
        // Word's label, which spans any text before an em dash.
        for found in analyzed
            .matches
            .iter()
            .filter(|m| matches!(m.reading, LexicalReading::Word(_)) && m.end - m.start > 1)
        {
            covered[found.start..found.end].fill(true);
        }
        let (logical, marks) = quotations(&analyzed.tokens, &covered);
        let tokens = &logical;
        let skip = |mut p: usize| {
            // A structural line break consumes its own boundary. Indentation
            // is not an interword space and has no atom to realize it.
            if p > 0 && matches!(tokens[p - 1], '\n' | '\r') {
                return p;
            }
            while p < n && tokens[p].is_whitespace() && tokens[p] != '\n' && tokens[p] != '\r' {
                p += 1;
            }
            p
        };
        let mut forest = PackedForest::default();
        let mut by_start: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
        let atomic = self.atomic_spans(analyzed);
        for found in &analyzed.matches {
            // No Constituent spans nothing: see
            // `Binder::no_zero_width_start` for the other half.
            assert!(
                found.start < found.end,
                "lexical analysis matched an empty span at char {}",
                found.start
            );
            // Inside a multiword noun, only an ordinary word of a term whose
            // words may stand apart reaches the chart.
            let word = matches!(found.reading, LexicalReading::Word(_));
            if atomic.iter().any(|(span, splits)| {
                ((span.start < found.start && found.start < span.end)
                    || (span.start < found.end && found.end < span.end))
                    && !(*splits && word)
            }) {
                continue;
            }
            let licensed = "an analyzed lexical match is licensed by the grammar's lexicon";
            let facts = self
                .leaf_facts(&found.reading, &analyzed.scope)
                .expect(licensed);
            let edges = self.leaf_edges(&found.reading).expect(licensed);
            by_start[found.start].push(forest.leaves.len());
            forest.leaves.push(Leaf {
                start: found.start,
                end: found.end,
                reading: found.reading.clone(),
                case: found.case,
                facts,
                edges,
            });
        }
        // A word whose spelling ends in terminal punctuation (*Blood for the
        // Blood God!*, *Agents of S.H.I.E.L.D.*) ends a sentence with it: the
        // sentence's period is not written after it, as it is not after a
        // quotation's own (CGEL, Ch. 20 §8.4, p. 1764, and §6, p. 1757, n. 21).
        let mut terminal = vec![false; n + 1];
        for leaf in &forest.leaves {
            if leaf.end > leaf.start + 1 && matches!(tokens[leaf.end - 1], '.' | '!' | '?') {
                terminal[leaf.end] = true;
            }
        }
        let seams = Seams::new(
            atomic
                .into_iter()
                .filter_map(|(span, splits)| splits.then_some(span))
                .collect(),
            n,
        );
        let mut boundaries = Boundaries::new(n);
        boundaries.boundary(0);
        // Indices into `forest.nodes`, by [`NodeKey`].
        let mut node_index: HashTable<usize> = HashTable::new();
        let categories = self.categories.len();
        let mut predicted_at = vec![usize::MAX; categories];

        let mut active = self.prediction.signature();
        for position in 0..=n {
            let Some(here) = boundaries.at[position] else {
                continue;
            };
            boundaries.waiting[here] = vec![Vec::new(); categories];
            active.fill(0);
            let at = skip(position);
            for &leaf in &by_start[at] {
                self.prediction
                    .add_leaf(self, &forest.leaves[leaf].facts, &mut active);
            }
            self.prediction.add_punctuation(
                tokens.get(position).copied(),
                if marks.get(at).is_some_and(Option::is_some) {
                    Some(QUOTATION_MARK)
                } else {
                    tokens.get(at).copied()
                },
                &mut active,
            );
            if position == 0 {
                predicted_at[root] = position;
                for &p in &self.by_lhs[root] {
                    if self.prediction.production(p, &active) {
                        boundaries.add(
                            &mut forest,
                            0,
                            (p, 0, 0, self.productions[p].start(), 0),
                            None,
                        );
                    }
                }
            }
            let mut cursor = 0;
            while cursor < boundaries.sets[here].len() {
                let id = boundaries.sets[here][cursor];
                cursor += 1;
                let (production_id, dot, start) = {
                    let item = &forest.items[id];
                    (item.production, item.dot, item.start)
                };
                let production = &self.productions[production_id];
                let previous = (dot > 0).then_some(id);
                if dot == production.rhs.len() {
                    let Some(summary) = production.finish(self, &forest.items[id].state) else {
                        continue;
                    };
                    let key = (production.lhs, start, position, summary);
                    let nodes = &forest.nodes;
                    let node = match node_index.entry(
                        CHART_HASHER.hash_one(key),
                        |&node| nodes[node].key() == key,
                        |&node| CHART_HASHER.hash_one(nodes[node].key()),
                    ) {
                        Entry::Occupied(entry) => {
                            forest.nodes[*entry.get()]
                                .families
                                .push((production_id, id));
                            continue;
                        }
                        Entry::Vacant(entry) => {
                            let node = forest.nodes.len();
                            entry.insert(node);
                            node
                        }
                    };
                    forest.nodes.push(Node {
                        category: production.lhs,
                        start,
                        end: position,
                        summary,
                        families: vec![(production_id, id)],
                    });
                    let from = boundaries.reached(start);
                    // Adding items never adds to a waiting list.
                    for waiter in 0..boundaries.waiting[from][production.lhs].len() {
                        let parent = boundaries.waiting[from][production.lhs][waiter];
                        let item = &forest.items[parent];
                        let parent_production = &self.productions[item.production];
                        let view = ChildView::Node(&forest.nodes[node].summary);
                        if !parent_production.spacing_matches(
                            &item.state,
                            item.dot,
                            &view,
                            &tokens[item.end..skip(item.end)],
                        ) {
                            continue;
                        }
                        if let Some(state) =
                            parent_production.accept(self, &item.state, item.dot, &view)
                        {
                            let (p, d, s) = (item.production, item.dot, item.start);
                            let prev = (d > 0).then_some(parent);
                            let Some(seam) = seams.after(
                                skip(s),
                                prev.map(|_| (item.end, item.seam)),
                                skip(start),
                                position,
                            ) else {
                                continue;
                            };
                            boundaries.add(
                                &mut forest,
                                position,
                                (p, d + 1, s, state, seam),
                                Some((prev, ChildRef::Node(node))),
                            );
                        }
                    }
                    continue;
                }
                match &production.rhs[dot] {
                    Symbol::Cat { category, .. } => {
                        boundaries.waiting[here][*category].push(id);
                        if predicted_at[*category] == position {
                            continue;
                        }
                        predicted_at[*category] = position;
                        if !self.prediction.category(*category, &active) {
                            continue;
                        }
                        for &p in &self.by_lhs[*category] {
                            if self.prediction.production(p, &active) {
                                boundaries.add(
                                    &mut forest,
                                    position,
                                    (p, 0, position, self.productions[p].start(), 0),
                                    None,
                                );
                            }
                        }
                    }
                    Symbol::Leaf(leaf_test) => {
                        let at = skip(position);
                        for &leaf in &by_start[at] {
                            let facts = &forest.leaves[leaf].facts;
                            if !self.leaf_matches(leaf_test, facts) {
                                continue;
                            }
                            let view = ChildView::Leaf(
                                facts,
                                forest.leaves[leaf].edges,
                                Opening::of(forest.leaves[leaf].case),
                            );
                            if !production.spacing_matches(
                                &forest.items[id].state,
                                dot,
                                &view,
                                &tokens[position..at],
                            ) {
                                continue;
                            }
                            if let Some(state) =
                                production.accept(self, &forest.items[id].state, dot, &view)
                            {
                                let end = forest.leaves[leaf].end;
                                let item = &forest.items[id];
                                let Some(seam) = seams.after(
                                    skip(start),
                                    previous.map(|_| (item.end, item.seam)),
                                    at,
                                    end,
                                ) else {
                                    continue;
                                };
                                boundaries.add(
                                    &mut forest,
                                    end,
                                    (production_id, dot + 1, start, state, seam),
                                    Some((previous, ChildRef::Leaf(leaf))),
                                );
                            }
                        }
                    }
                    Symbol::Punct(c, binding, _) => {
                        let mut at = if c.is_whitespace() { position } else { skip(position) };
                        let mut end = at + 1;
                        let found = if *c == QUOTATION_MARK {
                            let role =
                                if glues_after(*binding) { Quote::Open } else { Quote::Close };
                            marks.get(at) == Some(&Some(role))
                        } else if at < n && tokens[at] == *c {
                            // Written punctuation is no quotation's, and
                            // never precedes a closing mark.
                            marks[at].is_none()
                                && !(matches!(c, '.' | ',')
                                    && marks.get(at + 1) == Some(&Some(Quote::Close)))
                                && !(*c == '.' && at == position && terminal[at])
                        } else if *c == '.'
                            && (marks.get(position) == Some(&Some(Quote::Close))
                                || terminal[position])
                            && !forest.items[id].state.unwritten_period
                        {
                            // A quotation's own final period is not written,
                            // nor is one after a word ending in terminal
                            // punctuation, and it is the one period there.
                            (at, end) = (position, position);
                            true
                        } else {
                            false
                        };
                        if found
                            && production.spacing_matches(
                                &forest.items[id].state,
                                dot,
                                &ChildView::Punct,
                                &tokens[position..at],
                            )
                            && let Some(mut state) = production.accept(
                                self,
                                &forest.items[id].state,
                                dot,
                                &ChildView::Punct,
                            )
                            && let Some(seam) = seams.after(
                                skip(start),
                                previous.map(|_| (forest.items[id].end, forest.items[id].seam)),
                                at,
                                end,
                            )
                        {
                            state.unwritten_period = at == end;
                            boundaries.add(
                                &mut forest,
                                end,
                                (production_id, dot + 1, start, state, seam),
                                Some((previous, ChildRef::Punct)),
                            );
                        }
                    }
                }
            }
        }
        forest.roots = (0..forest.nodes.len())
            .filter(|&i| {
                let node = &forest.nodes[i];
                !node.families.is_empty()
                    && node.category == root
                    && node.start == 0
                    && skip(node.end) == n
                    && node.summary.gap == crate::Gap::None
                    // No Production puts the input in a position that opens
                    // with a capital, so it opens as a Constituent inside a
                    // sentence does, where none states one.
                    && node.summary.opening != Opening::Capital
            })
            .collect();
        Parse {
            grammar: self,
            root,
            source: analyzed.tokens.iter().collect(),
            tokens: logical,
            forest,
            vocabulary_gaps: analyzed.unknown_words(),
            scope: analyzed.scope.clone(),
            children_first: OnceLock::new(),
            head_inside: OnceLock::new(),
        }
    }
}
