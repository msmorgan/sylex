//! Readings: enumeration from the Packed Forest, Admission without parsing,
//! Realization, and the Ranking Key.

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::ops::Add;
use std::ops::Range;

use dpsi::Ident;
use serde::Deserialize;
use serde::Serialize;
use sylex_lexicon::Binding;
use sylex_lexicon::CardScope;
use sylex_lexicon::LexemeId;
use sylex_lexicon::LexicalReading;
use sylex_lexicon::LexicalValue;
use sylex_lexicon::Lexicon;

use crate::chart::ChildRef;
use crate::chart::Parse;
use crate::chart::QUOTATION_MARK;
use crate::chart::quotation_mark;
use crate::compile::CompiledGrammar;
use crate::frames::Frame;
use crate::step::ChildView;
use crate::step::Gap;
use crate::step::Summary;
use crate::step::Symbol;
use crate::walk::ForestMemo;
use crate::walk::Visit;

/// One whole tree: a grammatical analysis of an expression.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Ord, PartialOrd, Serialize)]
pub enum Reading {
    Node(ReadingNode),
    Leaf(LexicalReading),
}

/// A Constituent formed by one Production.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Ord, PartialOrd, Serialize)]
pub struct ReadingNode {
    /// The stable id of the Production.
    pub production: Ident,
    pub category: Ident,
    /// Bound features of the Constituent, by feature name.
    pub features: BTreeMap<Ident, Ident>,
    pub gap: Gap,
    /// The Selected Frame of a frame-headed phrase.
    pub selected_frame: Option<Frame>,
    /// Children in surface order; punctuation is not stored.
    pub children: Vec<Reading>,
}

impl Reading {
    #[must_use]
    pub fn node(&self) -> Option<&ReadingNode> {
        match self {
            Self::Node(node) => Some(node),
            Self::Leaf(_) => None,
        }
    }
}

/// The Ranking Key (`docs/decisions/english.md` §5): the Preference that
/// orders a Parse's Readings, compared component by component in field order,
/// lower first. A Reading's own order is the final tie-break, so the cheapest
/// Reading is unique. Every component is a sum over the Reading's Productions
/// or attachments, which keeps extraction over the Packed Forest
/// compositional.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize,
)]
pub struct RankingKey {
    /// Attachments that violate a structural Attachment Class.
    pub attachment_class: u64,
    /// Right Association: the words between unselected dependents and the
    /// heads of their hosts, reversed for a dependent that attaches high.
    pub right_association_words: i64,
    /// The summed Production Cost.
    pub production_cost: u64,
}

impl Add for RankingKey {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            attachment_class: self.attachment_class.saturating_add(other.attachment_class),
            right_association_words: self
                .right_association_words
                .saturating_add(other.right_association_words),
            production_cost: self.production_cost.saturating_add(other.production_cost),
        }
    }
}

/// Why a Reading is not admitted.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum AdmissionError {
    #[error("unknown production `{0}`")]
    UnknownProduction(String),
    #[error("production `{production}`: {reason}")]
    Rejected { production: String, reason: String },
    #[error("root is `{0}`, not the grammar root without a gap")]
    Root(String),
    #[error("a leaf is not a value of this lexicon")]
    Leaf,
    /// A leaf the lexicon licenses but that does not stand in the card scope
    /// ([`sylex_lexicon::Lexicon::stands_in`]).
    #[error("a leaf does not stand in this card scope")]
    OutOfScope,
}

impl Parse<'_> {
    /// A Reading node: a forest node built by `production` over `children`,
    /// with the node's bound features.
    pub(crate) fn reading_node(
        &self,
        node: usize,
        production: usize,
        features: BTreeMap<Ident, Ident>,
        children: Vec<Reading>,
    ) -> Reading {
        let grammar = self.grammar;
        let production = &grammar.productions[production];
        Reading::Node(ReadingNode {
            production: production.id,
            category: grammar.categories[production.lhs],
            features,
            gap: self.forest.nodes[node].summary.gap,
            selected_frame: production.frame.map(|f| grammar.frames[f].clone()),
            children,
        })
    }

    /// Number of Derivations in the forest, counted without building them
    /// (saturating). A Derivation never contains its own Constituent, so a
    /// unary cycle contributes no Derivation of its own.
    #[must_use]
    pub fn derivation_count(&self) -> u128 {
        let mut memo = ForestMemo::default();
        self.forest
            .roots
            .iter()
            .map(|&r| self.count_node(r, &mut memo))
            .fold(0u128, u128::saturating_add)
    }

    /// The Derivations of a completed Constituent.
    #[must_use]
    pub fn count_node(&self, node: usize, memo: &mut ForestMemo<u128, u128>) -> u128 {
        let frame = match memo.nodes.enter(node) {
            Visit::Known(count) => return count,
            Visit::Cycle => return 0,
            Visit::Open(frame) => frame,
        };
        let mut total = 0u128;
        for &(_, item) in &self.forest.nodes[node].families {
            total = total.saturating_add(self.count_item(item, memo));
        }
        memo.nodes.leave(frame, &total);
        total
    }

    fn count_item(&self, item: usize, memo: &mut ForestMemo<u128, u128>) -> u128 {
        if let Some(&count) = memo.items.get(&item) {
            return count;
        }
        let mark = memo.nodes.mark();
        let mut total = 0u128;
        for &(previous, child) in &self.forest.items[item].families {
            let before = previous.map_or(1, |p| self.count_item(p, memo));
            let here = match child {
                ChildRef::Node(n) => self.count_node(n, memo),
                ChildRef::Leaf(_) | ChildRef::Punct => 1,
            };
            total = total.saturating_add(before.saturating_mul(here));
        }
        if memo.nodes.settle(mark) {
            memo.items.insert(item, total);
        }
        total
    }

    /// Every Reading, deduplicated and ranked by its Ranking Key, then by its
    /// own order. Each iterator step materializes only the requested Reading.
    pub fn readings(&self) -> impl Iterator<Item = Reading> + '_ {
        self.best_readings(usize::MAX)
    }

    /// The `k` best Readings, enumerated lazily over an internal flyweight
    /// arena.
    pub fn best_readings(&self, k: usize) -> impl Iterator<Item = Reading> + '_ {
        self.best_ranked(k).map(|(_, reading)| reading)
    }

    /// The `k` best Readings with their Ranking Keys, best first. Huang &
    /// Chiang (2005), [Algorithm 3](https://aclanthology.org/W05-1506.pdf)
    /// (lazy k-best extraction from a forest), advances child streams only on
    /// demand; features and Selected Frames are cloned only for the emitted
    /// Reading.
    pub fn best_ranked(&self, k: usize) -> impl Iterator<Item = (RankingKey, Reading)> + '_ {
        let mut enumeration = Enumeration::new(self);
        let mut index = 0;
        std::iter::from_fn(move || {
            if index == k {
                return None;
            }
            let Some((key, tree)) = enumeration.ranked(index) else {
                index = k;
                return None;
            };
            index += 1;
            Some((key, enumeration.materialize(tree)))
        })
    }

    /// The least Ranking Key in the forest and the number of Derivations that
    /// have it: the size of the cheapest tier before the final tie-break.
    /// Counted without building trees; `None` without a Derivation.
    #[must_use]
    pub fn cheapest_tier(&self) -> Option<(RankingKey, u128)> {
        let mut memo = ForestMemo::default();
        let mut best = None;
        for &root in &self.forest.roots {
            best = least(best, self.tier_node(root, &mut memo));
        }
        best
    }

    fn tier_node(&self, node: usize, memo: &mut TierMemo) -> Tier {
        let frame = match memo.nodes.enter(node) {
            Visit::Known(tier) => return tier,
            Visit::Cycle => return None,
            Visit::Open(frame) => frame,
        };
        let mut best = None;
        for &(production, item) in &self.forest.nodes[node].families {
            let own = self.grammar.production_key(production);
            for (_, tier) in self.tier_item(item, memo) {
                best = least(best, tier.map(|(key, n)| (key + own, n)));
            }
        }
        memo.nodes.leave(frame, &best);
        best
    }

    /// An item's cheapest tier for each [`Anchors`] its Derivations leave.
    fn tier_item(&self, item: usize, memo: &mut TierMemo) -> Vec<(Anchors, Tier)> {
        if let Some(tiers) = memo.items.get(&item) {
            return tiers.clone();
        }
        let mark = memo.nodes.mark();
        let mut best: BTreeMap<Anchors, Tier> = BTreeMap::new();
        for &(previous, child) in &self.forest.items[item].families {
            let befores = match previous {
                Some(p) => self.tier_item(p, memo),
                None => vec![(Anchors::default(), Some((RankingKey::default(), 1)))],
            };
            let here = match child {
                ChildRef::Node(n) => self.tier_node(n, memo),
                ChildRef::Leaf(_) | ChildRef::Punct => Some((RankingKey::default(), 1)),
            };
            for (anchors, before) in befores {
                let edge = self.edge_key(item, anchors, previous, child);
                if let (Some((k1, n1)), Some((k2, n2))) = (before, here) {
                    let slot = best.entry(self.anchors(item, anchors, child)).or_default();
                    *slot = least(*slot, Some((k1 + k2 + edge, n1.saturating_mul(n2))));
                }
            }
        }
        let tiers: Vec<(Anchors, Tier)> = best.into_iter().collect();
        if memo.nodes.settle(mark) {
            memo.items.insert(item, tiers.clone());
        }
        tiers
    }

    /// The Ranking Key a child adds where an item takes it after `previous`,
    /// whose Derivation left `anchors`: its Right Association words
    /// ([`CompiledGrammar::association_words`]).
    fn edge_key(
        &self,
        item: usize,
        anchors: Anchors,
        previous: Option<usize>,
        child: ChildRef,
    ) -> RankingKey {
        let high = match child {
            ChildRef::Node(n) => self.high_attachment(n),
            _ => false,
        };
        self.edge_key_with_high(item, anchors, previous, child, high)
    }

    fn edge_key_with_high(
        &self,
        item: usize,
        anchors: Anchors,
        previous: Option<usize>,
        child: ChildRef,
        high_attachment: bool,
    ) -> RankingKey {
        let ChildRef::Node(n) = child else {
            return RankingKey::default();
        };
        let it = &self.forest.items[item];
        let node = &self.forest.nodes[n];
        let child = ChildFacts {
            words: self.words(node.start, node.end),
            coordination: node.summary.coordinator.is_some(),
            head_final: node.summary.head_final,
            high_attachment,
        };
        let head = || anchors.head.map(|(start, end)| self.words(start, end));
        let first = || {
            let (start, end) = anchors.first?;
            // Only punctuation and the coordinator, neither in the head line,
            // follow the first conjunct, so the previous item's `head_final`
            // is the first conjunct's, for this family.
            let head_final = self.forest.items[previous?].state.head_final;
            Some((self.words(start, end), head_final))
        };
        self.grammar
            .attachment_key(it.production, it.dot - 1, child, head, first, || {
                self.words(it.start, node.start)
            })
    }

    /// Whether a forest node attaches high by its Attachment Class, by the
    /// grammar's `high_attachment` declarations.
    fn high_attachment(&self, node: usize) -> bool {
        let packed = &self.forest.nodes[node];
        self.grammar.attaches_high(
            &self.grammar.feature_map(packed.category, &packed.summary),
            packed.category,
        )
    }

    /// The [`Anchors`] a Derivation leaves once `item` takes `child` after a
    /// prefix that left `anchors`: the child's extent when it is the head, or
    /// the first conjunct, whose words a later child's key reads.
    fn anchors(&self, item: usize, mut anchors: Anchors, child: ChildRef) -> Anchors {
        let it = &self.forest.items[item];
        let names = &self.grammar.productions[it.production].names;
        let at = it.dot - 1;
        let later = |read: &[&str]| names[at + 1..].iter().any(|n| read.contains(&n.as_str()));
        let extent = || match child {
            ChildRef::Node(n) => Some((self.forest.nodes[n].start, self.forest.nodes[n].end)),
            ChildRef::Leaf(l) => Some((self.forest.leaves[l].start, self.forest.leaves[l].end)),
            ChildRef::Punct => None,
        };
        match names[at].as_str() {
            "head" if later(&["adjunct", "modifier", "supplement"]) => anchors.head = extent(),
            "first" if later(&["last"]) => anchors.first = extent(),
            _ => {}
        }
        anchors
    }

    /// The words of the text between two chart positions: maximal runs of
    /// characters that are not whitespace.
    fn words(&self, start: usize, end: usize) -> u64 {
        let tokens = &self.tokens[start..end];
        let starts = tokens
            .iter()
            .enumerate()
            .filter(|&(i, c)| !c.is_whitespace() && (i == 0 || tokens[i - 1].is_whitespace()))
            .count();
        u64::try_from(starts).unwrap_or(u64::MAX)
    }
}

// Huang & Chiang (2005), Algorithm 3: each stream retains its best-so-far
// values and a frontier of (family, child ranks). Only a request for another
// value expands the last candidate's immediate successors.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct TreeId(u32);
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
struct SeqId(u32);
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum TreeEntry {
    Node {
        node: usize,
        production: usize,
        children: SeqId,
    },
    Leaf(usize),
}
#[derive(Clone, Copy)]
struct SeqEntry {
    prefix: SeqId,
    last: TreeId,
    len: usize,
}
#[derive(Clone, Copy)]
enum Value {
    Tree(TreeId),
    Seq(SeqId),
}
#[derive(Clone, Copy)]
enum Operation {
    Wrap { node: usize, production: usize },
    Append,
    Pass,
}
#[derive(Clone, Copy)]
struct Edge {
    tails: [Option<usize>; 2],
    operation: Operation,
    key: RankingKey,
}
#[derive(Clone, Copy)]
struct Candidate {
    edge: usize,
    ranks: [usize; 2],
    key: RankingKey,
    value: Value,
}
#[derive(Default)]
struct Stream {
    edges: Vec<Edge>,
    best: Vec<(RankingKey, Value)>,
    frontier: Vec<Candidate>,
    seen: BTreeSet<(usize, [usize; 2])>,
    pending: Option<Candidate>,
    initialized: bool,
}

struct Enumeration<'a, 'g> {
    parse: &'a Parse<'g>,
    trees: Vec<TreeEntry>,
    sequences: Vec<SeqEntry>,
    features: Vec<Option<BTreeMap<Ident, Ident>>>,
    streams: Vec<Stream>,
    root: Option<usize>,
}

type StreamMemo = ForestMemo<usize, Vec<(Anchors, usize)>>;

impl<'a, 'g> Enumeration<'a, 'g> {
    fn new(parse: &'a Parse<'g>) -> Self {
        Self {
            parse,
            trees: Vec::new(),
            sequences: Vec::new(),
            features: vec![None; parse.forest.nodes.len()],
            streams: Vec::new(),
            root: None,
        }
    }

    fn stream(&mut self, edges: Vec<Edge>) -> usize {
        let id = self.streams.len();
        self.streams.push(Stream {
            edges,
            ..Stream::default()
        });
        id
    }

    fn singleton(&mut self, key: RankingKey, value: Value) -> usize {
        let id = self.stream(Vec::new());
        self.streams[id].initialized = true;
        self.streams[id].best.push((key, value));
        id
    }

    fn tree(&mut self, entry: TreeEntry) -> TreeId {
        let id = TreeId(u32::try_from(self.trees.len()).expect("Reading arena exceeds u32"));
        self.trees.push(entry);
        id
    }

    fn sequence(&mut self, prefix: SeqId, last: TreeId) -> SeqId {
        let len = self.sequence_len(prefix) + 1;
        self.sequences.push(SeqEntry { prefix, last, len });
        SeqId(u32::try_from(self.sequences.len()).expect("Reading sequence arena exceeds u32"))
    }

    fn sequence_len(&self, id: SeqId) -> usize {
        if id.0 == 0 { 0 } else { self.sequences[id.0 as usize - 1].len }
    }

    fn features(&mut self, node: usize) -> &BTreeMap<Ident, Ident> {
        self.features[node].get_or_insert_with(|| {
            let packed = &self.parse.forest.nodes[node];
            self.parse
                .grammar
                .feature_map(packed.category, &packed.summary)
        })
    }

    // Build the stream graph with the same path exclusion as the other forest
    // walks. A cycle-tainted graph is private to the path that built it; only
    // cycle-free stream graphs enter the memo. Resuming a stream therefore
    // cannot accidentally change its ancestor exclusions.
    fn node_stream(&mut self, node: usize, memo: &mut StreamMemo) -> usize {
        let frame = match memo.nodes.enter(node) {
            Visit::Known(id) => return id,
            Visit::Cycle => return self.stream(Vec::new()),
            Visit::Open(frame) => frame,
        };
        self.features(node);
        let mut edges = Vec::new();
        for &(production, item) in &self.parse.forest.nodes[node].families {
            for (_, sequence) in self.item_streams(item, memo) {
                edges.push(Edge {
                    tails: [Some(sequence), None],
                    operation: Operation::Wrap { node, production },
                    key: self.parse.grammar.production_key(production),
                });
            }
        }
        let id = self.stream(edges);
        memo.nodes.leave(frame, &id);
        id
    }

    fn item_streams(&mut self, item: usize, memo: &mut StreamMemo) -> Vec<(Anchors, usize)> {
        if let Some(groups) = memo.items.get(&item) {
            return groups.clone();
        }
        let mark = memo.nodes.mark();
        let mut groups: BTreeMap<Anchors, Vec<Edge>> = BTreeMap::new();
        for &(previous, child) in &self.parse.forest.items[item].families {
            let prefixes = if let Some(p) = previous {
                self.item_streams(p, memo)
            } else {
                let empty = self.singleton(RankingKey::default(), Value::Seq(SeqId(0)));
                vec![(Anchors::default(), empty)]
            };
            let last = match child {
                ChildRef::Node(n) => Some(self.node_stream(n, memo)),
                ChildRef::Leaf(l) => {
                    let tree = self.tree(TreeEntry::Leaf(l));
                    Some(self.singleton(RankingKey::default(), Value::Tree(tree)))
                }
                ChildRef::Punct => None,
            };
            let high = if let ChildRef::Node(n) = child {
                let grammar = self.parse.grammar;
                let category = self.parse.forest.nodes[n].category;
                grammar.attaches_high(self.features(n), category)
            } else {
                false
            };
            for (anchors, prefix) in prefixes {
                let key = self
                    .parse
                    .edge_key_with_high(item, anchors, previous, child, high);
                groups
                    .entry(self.parse.anchors(item, anchors, child))
                    .or_default()
                    .push(Edge {
                        tails: [Some(prefix), last],
                        operation: Operation::Append,
                        key,
                    });
            }
        }
        let groups: Vec<_> = groups
            .into_iter()
            .map(|(anchors, edges)| (anchors, self.stream(edges)))
            .collect();
        if memo.nodes.settle(mark) {
            memo.items.insert(item, groups.clone());
        }
        groups
    }

    fn ranked(&mut self, index: usize) -> Option<(RankingKey, TreeId)> {
        let root = if let Some(root) = self.root {
            root
        } else {
            let mut memo = StreamMemo::default();
            let mut edges = Vec::new();
            for &node in &self.parse.forest.roots {
                let stream = self.node_stream(node, &mut memo);
                edges.push(Edge {
                    tails: [Some(stream), None],
                    operation: Operation::Pass,
                    key: RankingKey::default(),
                });
            }
            let root = self.stream(edges);
            self.root = Some(root);
            root
        };
        let (key, Value::Tree(tree)) = self.get(root, index)? else { unreachable!() };
        Some((key, tree))
    }

    fn get(&mut self, stream: usize, index: usize) -> Option<(RankingKey, Value)> {
        if !self.streams[stream].initialized {
            self.streams[stream].initialized = true;
            for edge in 0..self.streams[stream].edges.len() {
                self.offer(stream, edge, [0, 0]);
            }
        }
        while self.streams[stream].best.len() <= index {
            if let Some(last) = self.streams[stream].pending.take() {
                let edge = self.streams[stream].edges[last.edge];
                for dimension in 0..2 {
                    if edge.tails[dimension].is_some() {
                        let mut ranks = last.ranks;
                        ranks[dimension] += 1;
                        self.offer(stream, last.edge, ranks);
                    }
                }
            }
            let candidate = self.pop(stream)?;
            self.streams[stream].pending = Some(candidate);
            let duplicate = self.streams[stream]
                .best
                .last()
                .is_some_and(|&(key, value)| {
                    key == candidate.key && self.compare(value, candidate.value).is_eq()
                });
            if !duplicate {
                self.streams[stream]
                    .best
                    .push((candidate.key, candidate.value));
            }
        }
        self.streams[stream].best.get(index).copied()
    }

    fn offer(&mut self, stream: usize, edge: usize, ranks: [usize; 2]) {
        if !self.streams[stream].seen.insert((edge, ranks)) {
            return;
        }
        let definition = self.streams[stream].edges[edge];
        let Some((first_key, first)) = self.get(definition.tails[0].unwrap(), ranks[0]) else {
            return;
        };
        let (last_key, last) = if let Some(tail) = definition.tails[1] {
            let Some((key, value)) = self.get(tail, ranks[1]) else { return };
            (key, Some(value))
        } else {
            (RankingKey::default(), None)
        };
        let value = match definition.operation {
            Operation::Wrap { node, production } => {
                let Value::Seq(children) = first else { unreachable!() };
                Value::Tree(self.tree(TreeEntry::Node {
                    node,
                    production,
                    children,
                }))
            }
            Operation::Append => {
                let Value::Seq(prefix) = first else { unreachable!() };
                match last {
                    Some(Value::Tree(tree)) => Value::Seq(self.sequence(prefix, tree)),
                    None => first,
                    _ => unreachable!(),
                }
            }
            Operation::Pass => first,
        };
        let candidate = Candidate {
            edge,
            ranks,
            key: first_key + last_key + definition.key,
            value,
        };
        self.push(stream, candidate);
    }

    fn candidate_cmp(&self, a: Candidate, b: Candidate) -> std::cmp::Ordering {
        a.key
            .cmp(&b.key)
            .then_with(|| self.compare(a.value, b.value))
    }

    // A binary min-heap whose comparison borrows the arena, rather than
    // storing owned Reading keys or self-referential comparator objects.
    fn push(&mut self, stream: usize, candidate: Candidate) {
        let mut at = self.streams[stream].frontier.len();
        self.streams[stream].frontier.push(candidate);
        while at > 0 {
            let parent = (at - 1) / 2;
            if !self
                .candidate_cmp(candidate, self.streams[stream].frontier[parent])
                .is_lt()
            {
                break;
            }
            self.streams[stream].frontier.swap(at, parent);
            at = parent;
        }
    }

    fn pop(&mut self, stream: usize) -> Option<Candidate> {
        let heap = &mut self.streams[stream].frontier;
        if heap.is_empty() {
            return None;
        }
        let first = heap.swap_remove(0);
        let mut at = 0;
        let len = self.streams[stream].frontier.len();
        while at * 2 + 1 < len {
            let mut child = at * 2 + 1;
            let heap = &self.streams[stream].frontier;
            if child + 1 < len && self.candidate_cmp(heap[child + 1], heap[child]).is_lt() {
                child += 1;
            }
            if !self.candidate_cmp(heap[child], heap[at]).is_lt() {
                break;
            }
            self.streams[stream].frontier.swap(at, child);
            at = child;
        }
        Some(first)
    }

    fn compare(&self, a: Value, b: Value) -> std::cmp::Ordering {
        match (a, b) {
            (Value::Tree(a), Value::Tree(b)) => self.compare_tree(a, b),
            (Value::Seq(a), Value::Seq(b)) => self.compare_sequence(a, b),
            _ => unreachable!(),
        }
    }

    fn compare_tree(&self, a: TreeId, b: TreeId) -> std::cmp::Ordering {
        use std::cmp::Ordering;
        if a == b {
            return Ordering::Equal;
        }
        match (self.trees[a.0 as usize], self.trees[b.0 as usize]) {
            (TreeEntry::Leaf(a), TreeEntry::Leaf(b)) => {
                if a == b {
                    Ordering::Equal
                } else {
                    self.parse.forest.leaves[a]
                        .reading
                        .cmp(&self.parse.forest.leaves[b].reading)
                }
            }
            (TreeEntry::Node { .. }, TreeEntry::Leaf(_)) => Ordering::Less,
            (TreeEntry::Leaf(_), TreeEntry::Node { .. }) => Ordering::Greater,
            (
                TreeEntry::Node {
                    node: a,
                    production: pa,
                    children: ca,
                },
                TreeEntry::Node {
                    node: b,
                    production: pb,
                    children: cb,
                },
            ) => {
                let grammar = self.parse.grammar;
                let pa = &grammar.productions[pa];
                let pb = &grammar.productions[pb];
                pa.id
                    .cmp(&pb.id)
                    .then_with(|| grammar.categories[pa.lhs].cmp(&grammar.categories[pb.lhs]))
                    .then_with(|| {
                        self.features[a]
                            .as_ref()
                            .unwrap()
                            .cmp(self.features[b].as_ref().unwrap())
                    })
                    .then_with(|| {
                        self.parse.forest.nodes[a]
                            .summary
                            .gap
                            .cmp(&self.parse.forest.nodes[b].summary.gap)
                    })
                    .then_with(|| {
                        pa.frame
                            .map(|f| &grammar.frames[f])
                            .cmp(&pb.frame.map(|f| &grammar.frames[f]))
                    })
                    .then_with(|| self.compare_sequence(ca, cb))
            }
        }
    }

    fn compare_sequence(&self, mut a: SeqId, mut b: SeqId) -> std::cmp::Ordering {
        if a == b {
            return std::cmp::Ordering::Equal;
        }
        let la = self.sequence_len(a);
        let lb = self.sequence_len(b);
        for _ in lb..la {
            a = self.sequences[a.0 as usize - 1].prefix;
        }
        for _ in la..lb {
            b = self.sequences[b.0 as usize - 1].prefix;
        }
        self.compare_equal_length(a, b).then_with(|| la.cmp(&lb))
    }

    fn compare_equal_length(&self, a: SeqId, b: SeqId) -> std::cmp::Ordering {
        if a == b {
            return std::cmp::Ordering::Equal;
        }
        let a = self.sequences[a.0 as usize - 1];
        let b = self.sequences[b.0 as usize - 1];
        self.compare_equal_length(a.prefix, b.prefix)
            .then_with(|| self.compare_tree(a.last, b.last))
    }

    fn materialize(&self, tree: TreeId) -> Reading {
        match self.trees[tree.0 as usize] {
            TreeEntry::Leaf(leaf) => Reading::Leaf(self.parse.forest.leaves[leaf].reading.clone()),
            TreeEntry::Node {
                node,
                production,
                mut children,
            } => {
                let mut readings = Vec::with_capacity(self.sequence_len(children));
                while children.0 != 0 {
                    let entry = self.sequences[children.0 as usize - 1];
                    readings.push(self.materialize(entry.last));
                    children = entry.prefix;
                }
                readings.reverse();
                self.parse.reading_node(
                    node,
                    production,
                    self.features[node].as_ref().unwrap().clone(),
                    readings,
                )
            }
        }
    }
}

/// The extents, as chart positions, of the children of a Derivation's prefix
/// that a later child's Ranking Key reads: its head, read by an Adjunct,
/// Modifier or Supplement, and its first conjunct, read by the last. One
/// item packs prefixes that may divide its text differently, so the forest's
/// walks keep a result per `Anchors`, and each key is the one its tree has.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Ord, PartialOrd)]
struct Anchors {
    head: Option<(usize, usize)>,
    first: Option<(usize, usize)>,
}

/// What the Ranking Key reads of a child Constituent.
#[derive(Debug, Clone, Copy)]
pub(crate) struct ChildFacts {
    /// Its length in words.
    pub words: u64,
    /// It is a coordination.
    pub coordination: bool,
    /// It ends in its head, not in a dependent.
    pub head_final: bool,
    /// It attaches high by its Attachment Class, by the grammar's
    /// `high_attachment` declarations (a distributive Preposition Phrase,
    /// *for each …*).
    pub high_attachment: bool,
}

/// A forest node's or item's cheapest tier: its least key and how many
/// Derivations have it.
type Tier = Option<(RankingKey, u128)>;
type TierMemo = ForestMemo<Tier, Vec<(Anchors, Tier)>>;

/// The lesser of two tiers, adding their counts when their keys are equal.
fn least(a: Tier, b: Tier) -> Tier {
    match (a, b) {
        (None, t) | (t, None) => t,
        (Some((ka, na)), Some((kb, nb))) => Some(match ka.cmp(&kb) {
            std::cmp::Ordering::Less => (ka, na),
            std::cmp::Ordering::Greater => (kb, nb),
            std::cmp::Ordering::Equal => (ka, na.saturating_add(nb)),
        }),
    }
}

impl CompiledGrammar {
    /// Whether a Constituent of `category` with these features attaches high
    /// by its Attachment Class: the grammar declares its Category with a
    /// value it carries in `high_attachment` (a Preposition Phrase whose
    /// Complement has a distributive determinative, *for each …*).
    pub(crate) fn attaches_high(&self, features: &BTreeMap<Ident, Ident>, category: usize) -> bool {
        self.high_attachment
            .iter()
            .any(|(declared, feature, value)| {
                self.categories[category] == *declared && features.get(feature) == Some(value)
            })
    }

    /// The Attachment Class and Right Association words
    /// (`docs/decisions/english.md` §5) a child adds at position `index`
    /// of a Production. `head` gives the phrase's head's words; `first` a
    /// coordination's first conjunct's words and whether it ends in its head;
    /// `before` the words from the phrase's start to the child.
    /// - The head of a phrase that unselected dependents (`adjunct`,
    ///   `modifier`) follow is the host they attach to: the head adds its words
    ///   for each of them, unless it is a coordination, and each adds the words
    ///   between the host and itself (a selected Object before a postposed or
    ///   interleaved dependent).
    /// - A dependent whose class attaches high counts its distance from its
    ///   host's head reversed instead: a Supplement (`supplement`), and a
    ///   dependent the grammar declares in `high_attachment` (a distributive
    ///   adjunct, *for each …*), which also takes back what the head added for
    ///   it.
    /// - The last conjunct of a coordination adds the difference between its
    ///   length and its first conjunct's (conjunct balance), and violates an
    ///   Attachment Class when it ends in a dependent its first conjunct, which
    ///   ends in its head, lacks: that dependent belongs to the whole
    ///   coordination.
    /// - A dependent that precedes its head after another dependent of that
    ///   head violates an Attachment Class: it belongs inside the dependent
    ///   before it, where that can hold it (*At the beginning of each combat
    ///   this turn, …*).
    pub(crate) fn attachment_key(
        &self,
        production: usize,
        index: usize,
        child: ChildFacts,
        head: impl FnOnce() -> Option<u64>,
        first: impl FnOnce() -> Option<(u64, bool)>,
        before: impl FnOnce() -> u64,
    ) -> RankingKey {
        let names = &self.productions[production].names;
        let signed = |words: u64| i64::try_from(words).unwrap_or(i64::MAX);
        let after_head = || names[..index].iter().any(|name| name.as_str() == "head");
        let mut key = RankingKey::default();
        match names[index].as_str() {
            "head" if !child.coordination => {
                let dependents = names[index + 1..]
                    .iter()
                    .filter(|name| matches!(name.as_str(), "adjunct" | "modifier"))
                    .count();
                key.right_association_words = signed(child.words)
                    .saturating_mul(i64::try_from(dependents).unwrap_or(i64::MAX));
            }
            name @ ("adjunct" | "modifier" | "supplement") if after_head() => {
                if let Some(head_words) = head() {
                    let host = signed(head_words);
                    let between = signed(before().saturating_sub(head_words));
                    key.right_association_words = if name == "supplement" {
                        -host - between
                    } else if child.high_attachment {
                        // The head added the host for it, unless the host is
                        // a coordination; this does not depend on which.
                        -2 * host - between
                    } else {
                        between
                    };
                }
            }
            "last" => {
                if let Some((first_words, first_head_final)) = first() {
                    key.right_association_words = (signed(first_words) - signed(child.words)).abs();
                    key.attachment_class = u64::from(first_head_final && !child.head_final);
                }
            }
            _ => {}
        }
        let dependent = |name: &Ident| matches!(name.as_str(), "adjunct" | "modifier");
        if dependent(&names[index])
            && names[..index].iter().any(dependent)
            && names[index + 1..]
                .iter()
                .any(|name| name.as_str() == "head")
        {
            key.attachment_class += 1;
        }
        key
    }

    /// One occurrence of a Production's own contribution to a Ranking Key: its
    /// Production Cost. Right Association is added per child
    /// ([`CompiledGrammar::association_words`]); the Attachment Class is not
    /// computed yet, and is zero.
    pub(crate) fn production_key(&self, production: usize) -> RankingKey {
        RankingKey {
            production_cost: u64::from(self.productions[production].cost),
            ..RankingKey::default()
        }
    }

    pub(crate) fn feature_map(&self, category: usize, summary: &Summary) -> BTreeMap<Ident, Ident> {
        self.category_features[category]
            .iter()
            .zip(summary.values.iter())
            .filter_map(|(&f, v)| {
                v.get()
                    .map(|v| (self.features[f], self.values[f][usize::from(v)]))
            })
            .collect()
    }

    /// Admission without parsing: whether `reading` is a Reading of the root
    /// Category that this grammar and lexicon accept in the text of a face
    /// whose own card names are `scope`.
    ///
    /// # Errors
    /// The first violation found.
    pub fn admit(
        &self,
        reading: &Reading,
        lexicon: &Lexicon,
        scope: &CardScope,
    ) -> Result<(), AdmissionError> {
        self.admit_as(reading, lexicon, self.root, scope)
    }

    /// Admission without parsing as the named Category instead of the root.
    ///
    /// # Errors
    /// The first violation found, or [`AdmissionError::Root`] when the grammar
    /// has no such Category.
    pub fn admit_category(
        &self,
        reading: &Reading,
        lexicon: &Lexicon,
        category: &str,
        scope: &CardScope,
    ) -> Result<(), AdmissionError> {
        let root = self
            .category_id(category)
            .ok_or_else(|| AdmissionError::Root(category.to_owned()))?;
        self.admit_as(reading, lexicon, root, scope)
    }

    /// Admission as Category `root`.
    pub(crate) fn admit_as(
        &self,
        reading: &Reading,
        lexicon: &Lexicon,
        root: usize,
        scope: &CardScope,
    ) -> Result<(), AdmissionError> {
        let (category, summary) = self.summarize(reading, lexicon, scope)?;
        if category != root || summary.gap != Gap::None {
            return Err(AdmissionError::Root(self.categories[category].to_string()));
        }
        let mut atoms = Vec::new();
        self.atoms(reading, lexicon, &mut atoms, &mut |_, _| {})?;
        let (text, offsets) = join(&atoms);
        let atomic = self.atomic_spans(&lexicon.analyze(&text));
        if atomic.is_empty() {
            return Ok(());
        }
        let mut chars = vec![0; text.len() + 1];
        for (count, (at, c)) in text.char_indices().enumerate() {
            chars[at] = count;
            chars[at + c.len_utf8()] = count + 1;
        }
        let atoms: Vec<Range<usize>> = offsets
            .iter()
            .map(|offset| chars[offset.start]..chars[offset.end])
            .collect();
        let mut next = 0;
        if self
            .atomic_reading(reading, &atoms, &atomic, &mut next)?
            .is_none()
        {
            return Err(AdmissionError::Rejected {
                production: "lexical atomicity".into(),
                reason: "a declared multiword noun's words rebuild it".into(),
            });
        }
        Ok(())
    }

    /// Lexical atomicity, decided on the Reading's own structure as the chart
    /// decides it (`docs/decisions/lexicon.md` §5): a word
    /// inside a multiword noun of `atomic` stands alone only where the term's
    /// words may stand apart, and then only where they straddle a constituent
    /// boundary. In the smallest Constituent spanning the term, the child
    /// holding its last word must end after it and no child may begin where
    /// it begins. Returns the subtree's character span, or `None` where it
    /// rebuilds a term. `atoms` holds each atom's character span, in the
    /// order [`CompiledGrammar::atoms`] realizes them.
    fn atomic_reading(
        &self,
        reading: &Reading,
        atoms: &[Range<usize>],
        atomic: &[(Range<usize>, bool)],
        next: &mut usize,
    ) -> Result<Option<Range<usize>>, AdmissionError> {
        let inside = |at: usize, span: &Range<usize>| span.start < at && at < span.end;
        let node = match reading {
            Reading::Leaf(value) => {
                let at = atoms[*next].clone();
                *next += 1;
                let word = matches!(value, LexicalReading::Word(_));
                let parted = atomic.iter().any(|(span, splits)| {
                    (inside(at.start, span) || inside(at.end, span)) && !(*splits && word)
                });
                return Ok((!parted).then_some(at));
            }
            Reading::Node(node) => node,
        };
        let &index = self
            .by_id
            .get(&node.production)
            .ok_or_else(|| AdmissionError::UnknownProduction(node.production.to_string()))?;
        let mut children = node.children.iter();
        let mut spans = Vec::new();
        for symbol in &self.productions[index].rhs {
            let span = if let Symbol::Punct(..) = symbol {
                *next += 1;
                atoms[*next - 1].clone()
            } else {
                let child = children.next().ok_or_else(|| AdmissionError::Rejected {
                    production: node.production.to_string(),
                    reason: "missing child".to_owned(),
                })?;
                match self.atomic_reading(child, atoms, atomic, next)? {
                    Some(span) => span,
                    None => return Ok(None),
                }
            };
            spans.push(span);
        }
        let (Some(first), Some(last)) = (spans.first(), spans.last()) else {
            return Ok(Some(0..0));
        };
        let whole = first.start..last.end;
        let rebuilt = atomic
            .iter()
            .filter(|(term, splits)| *splits && whole.start <= term.start)
            .any(|(term, _)| {
                spans.iter().enumerate().any(|(i, child)| {
                    inside(child.start, term)
                        && term.end <= child.end
                        && (child.end == term.end
                            || spans[..i].iter().any(|before| before.start == term.start))
                })
            });
        Ok((!rebuilt).then_some(whole))
    }

    fn summarize(
        &self,
        reading: &Reading,
        lexicon: &Lexicon,
        scope: &CardScope,
    ) -> Result<(usize, Summary), AdmissionError> {
        let Reading::Node(node) = reading else {
            return Err(AdmissionError::Leaf);
        };
        let &index = self
            .by_id
            .get(&node.production)
            .ok_or_else(|| AdmissionError::UnknownProduction(node.production.to_string()))?;
        let production = &self.productions[index];
        let reject = |reason: &str| AdmissionError::Rejected {
            production: node.production.to_string(),
            reason: reason.to_owned(),
        };
        let mut children = node.children.iter();
        let mut state = production.start();
        for (position, symbol) in production.rhs.iter().enumerate() {
            let next = match symbol {
                Symbol::Punct(..) => production.accept(self, &state, position, &ChildView::Punct),
                Symbol::Leaf(test) => {
                    let Some(Reading::Leaf(value)) = children.next() else {
                        return Err(reject("expected a lexical leaf"));
                    };
                    lexicon.realize(value).map_err(|_| AdmissionError::Leaf)?;
                    if !lexicon.stands_in(value, scope) {
                        return Err(AdmissionError::OutOfScope);
                    }
                    let facts = self.leaf_facts(value, scope).ok_or(AdmissionError::Leaf)?;
                    let edges = self.leaf_edges(value).ok_or(AdmissionError::Leaf)?;
                    if !self.leaf_matches(test, &facts) {
                        return Err(reject("leaf does not satisfy its symbol"));
                    }
                    production.accept(self, &state, position, &ChildView::Leaf(&facts, edges))
                }
                Symbol::Cat { category, .. } => {
                    let Some(child) = children.next() else {
                        return Err(reject("missing child"));
                    };
                    let (child_category, summary) = self.summarize(child, lexicon, scope)?;
                    if child_category != *category {
                        return Err(reject("child has the wrong category"));
                    }
                    production.accept(self, &state, position, &ChildView::Node(&summary))
                }
            };
            state = next.ok_or_else(|| reject("child does not fit"))?;
        }
        if children.next().is_some() {
            return Err(reject("extra children"));
        }
        let summary = production
            .finish(self, &state)
            .ok_or_else(|| reject("constituent does not complete"))?;
        if self.categories[production.lhs] != node.category
            || self.feature_map(production.lhs, &summary) != node.features
            || summary.gap != node.gap
            || production.frame.map(|f| &self.frames[f]) != node.selected_frame.as_ref()
        {
            return Err(reject("recorded category, features, gap or frame differ"));
        }
        Ok((production.lhs, summary))
    }

    /// Realizes a Reading as text: words through the lexicon, punctuation from
    /// the Productions.
    ///
    /// # Errors
    /// A leaf the lexicon does not license, or a tree that does not match its
    /// Productions.
    pub fn realize(&self, reading: &Reading, lexicon: &Lexicon) -> Result<String, AdmissionError> {
        let mut atoms = Vec::new();
        self.atoms(reading, lexicon, &mut atoms, &mut |_, _| {})?;
        Ok(join(&atoms).0)
    }

    /// Every Constituent of a Reading in preorder, located in its
    /// realization, which is returned with them.
    ///
    /// # Errors
    /// As [`CompiledGrammar::realize`].
    pub fn constituents(
        &self,
        reading: &Reading,
        lexicon: &Lexicon,
    ) -> Result<(String, Vec<Constituent>), AdmissionError> {
        let mut atoms = Vec::new();
        let mut spans: Vec<(usize, usize)> = Vec::new();
        self.atoms(reading, lexicon, &mut atoms, &mut |index, range| {
            if index == spans.len() {
                spans.push(range);
            } else {
                spans[index] = range;
            }
        })?;
        let (text, offsets) = join(&atoms);
        let mut out = Vec::new();
        self.locate(reading, None, None, None, &mut out);
        for (constituent, &(first, last)) in out.iter_mut().zip(&spans) {
            constituent.span = if first == last {
                let at = offsets.get(first).map_or(text.len(), |r| r.start);
                at..at
            } else {
                let atoms = &offsets[first..last];
                let start = atoms.iter().map(|r| r.start).min().unwrap_or_default();
                start..atoms.iter().map(|r| r.end).max().unwrap_or_default()
            };
        }
        Ok((text, out))
    }

    /// Preorder Constituents without spans; `atoms` fills the spans in the
    /// same order. `clustered` is the preorder index of the phrase whose head
    /// a coordination of Complement Clusters at or above `reading` shares:
    /// each dependent in a Cluster bears its function in that phrase too.
    fn locate(
        &self,
        reading: &Reading,
        parent: Option<usize>,
        function: Option<Ident>,
        clustered: Option<usize>,
        out: &mut Vec<Constituent>,
    ) {
        let Reading::Node(node) = reading else {
            return;
        };
        let index = out.len();
        out.push(Constituent {
            category: node.category,
            span: 0..0,
            function,
            parent,
            head: self.head_lexeme(reading),
            gap: node.gap,
            also: Vec::new(),
        });
        let production = self
            .by_id
            .get(&node.production)
            .map(|&p| &self.productions[p]);
        // A Cluster's dependents bear their functions in the phrase whose
        // head the coordination of Clusters shares; a coordination passes
        // that phrase down to its Clusters.
        let cluster_host = clustered.filter(|_| production.is_some_and(|p| p.cluster.is_some()));
        let pass = |name: &Ident| match production {
            Some(p) if p.clusters.is_some_and(|(at, _)| p.names[at] == *name) => Some(index),
            Some(p) if p.cluster.is_some() => None,
            _ => clustered,
        };
        // The head of a function Category bears its function in its phrase.
        let wraps = self.function_categories.contains(&node.category);
        let names = self.child_names(node);
        let raising = self
            .by_id
            .get(&node.production)
            .is_some_and(|&p| self.productions[p].requires_coordination.is_some());
        let mut sites = Vec::new();
        let mut object = None;
        for (child, name) in node.children.iter().zip(names) {
            let start = out.len();
            if name == "object" {
                object = Some(start);
            }
            // A Modifier after its phrase's Object is postposed out of it
            // (*put a number of counters on it [equal to …]*, *choose a card
            // at random [that …]*; CGEL, Ch. 12 §4, p. 1066): it modifies
            // the Object too.
            let postposed = object.filter(|_| name == "modifier");
            if raising && name == "head" {
                let mut at = start;
                let gap = child.node().map_or(Gap::None, |c| c.gap);
                Self::gap_sites(child, gap, &mut at, &mut sites);
            }
            if wraps && name == "head" {
                self.locate(child, parent, function, pass(&name), out);
            } else {
                self.locate(child, Some(index), Some(name), pass(&name), out);
            }
            if let Some(host) = cluster_host {
                // A function Category's head bears it too.
                for constituent in &mut out[start..] {
                    if constituent.parent == Some(index) && constituent.function == Some(name) {
                        constituent.also.push((name, host));
                    }
                }
            }
            if raising && name == "shared" {
                // A function Category's head bears the shared function too
                // (*be and become [legendary]*).
                let (parent, function) = (out[start].parent, out[start].function);
                for constituent in &mut out[start..] {
                    if constituent.parent == parent && constituent.function == function {
                        constituent.also.clone_from(&sites);
                    }
                }
            }
            if let Some(object) = postposed {
                // A function Category's head bears it too.
                for constituent in &mut out[start..] {
                    if constituent.parent == Some(index) && constituent.function == Some(name) {
                        constituent.also.push((name, object));
                    }
                }
            }
        }
    }

    /// The phrases along the `gap` path of `reading` that introduce the
    /// Gap, with the function of the extracted Frame Slot, by preorder
    /// index from `at`.
    fn gap_sites(reading: &Reading, gap: Gap, at: &mut usize, sites: &mut Vec<(Ident, usize)>) {
        let Reading::Node(node) = reading else { return };
        let index = *at;
        *at += 1;
        let on_path = gap != Gap::None && node.gap == gap;
        let extracted = node
            .production
            .rsplit_once("/gap")
            .and_then(|(_, slot)| slot.split('/').next()?.parse::<usize>().ok());
        if let (true, Some(slot)) = (on_path, extracted) {
            // A preposition's phrase records no Selected Frame: its NP is its
            // Complement.
            let relation = node
                .selected_frame
                .as_ref()
                .and_then(|frame| frame.slots.get(slot))
                .map_or(crate::frames::SlotRelation::Complement, |s| s.relation);
            sites.push((Ident::new(crate::compile::relation_name(relation)), index));
            Self::count(node, at);
            return;
        }
        for child in &node.children {
            if on_path && child.node().is_some_and(|c| c.gap == gap) {
                Self::gap_sites(child, gap, at, sites);
            } else {
                Self::count_reading(child, at);
            }
        }
    }

    fn count(node: &ReadingNode, at: &mut usize) {
        for child in &node.children {
            Self::count_reading(child, at);
        }
    }

    fn count_reading(reading: &Reading, at: &mut usize) {
        if let Reading::Node(node) = reading {
            *at += 1;
            Self::count(node, at);
        }
    }

    /// The names of a node's children, in order: the grammatical function
    /// each bears in it.
    #[must_use]
    pub fn child_names(&self, node: &ReadingNode) -> Vec<Ident> {
        self.by_id
            .get(&node.production)
            .map_or_else(Vec::new, |&p| {
                let production = &self.productions[p];
                production
                    .rhs
                    .iter()
                    .zip(&production.names)
                    .filter(|(symbol, _)| !matches!(symbol, Symbol::Punct(..)))
                    .map(|(_, name)| *name)
                    .collect()
            })
    }

    /// The lexeme reached by following `head` children down from `reading`.
    #[must_use]
    pub fn head_lexeme(&self, reading: &Reading) -> Option<LexemeId> {
        self.head_word(reading).map(|value| value.lexeme)
    }

    /// The word reached by following `head` children down from `reading`:
    /// its lexeme, Word Form and features. Two Readings that differ only in
    /// the form of a syncretic head (*cast* as present or preterite) differ
    /// here.
    #[must_use]
    pub fn head_word<'r>(&self, reading: &'r Reading) -> Option<&'r LexicalValue> {
        match reading {
            Reading::Leaf(LexicalReading::Word(value)) => Some(value),
            Reading::Leaf(_) => None,
            Reading::Node(node) => {
                let production = *self.by_id.get(&node.production)?;
                let at = self.head_index(production)?;
                self.head_word(node.children.get(self.reading_child(production, at))?)
            }
        }
    }

    /// The child of a Reading node its head line runs through
    /// ([`Self::head_line_child`]), by its index among the node's children;
    /// `None` when it is punctuation, which a Reading does not store.
    ///
    /// # Panics
    /// When no Production has the node's id.
    #[must_use]
    pub fn reading_head_line_child(&self, node: &ReadingNode) -> Option<usize> {
        let production = self.by_id[&node.production];
        let at = self.head_line_child(production)?;
        (!matches!(self.productions[production].rhs[at], Symbol::Punct(..)))
            .then(|| self.reading_child(production, at))
    }

    /// Pushes a Reading's atoms (text, binding, punctuation) in order, and
    /// reports each node's preorder index with its atom range.
    pub(crate) fn atoms(
        &self,
        reading: &Reading,
        lexicon: &Lexicon,
        out: &mut Vec<Atom>,
        located: &mut dyn FnMut(usize, (usize, usize)),
    ) -> Result<(), AdmissionError> {
        let mut next = 0;
        let mut depth = 0;
        self.atoms_from(reading, lexicon, out, located, &mut next, &mut depth)
    }

    fn atoms_from(
        &self,
        reading: &Reading,
        lexicon: &Lexicon,
        out: &mut Vec<Atom>,
        located: &mut dyn FnMut(usize, (usize, usize)),
        next: &mut usize,
        depth: &mut usize,
    ) -> Result<(), AdmissionError> {
        match reading {
            Reading::Leaf(value) => {
                let text = lexicon.realize(value).map_err(|_| AdmissionError::Leaf)?;
                let binding = match value {
                    LexicalReading::Word(v) => self
                        .lexemes
                        .get(&v.lexeme)
                        .map_or(Binding::Free, |f| f.binding),
                    _ => Binding::Free,
                };
                out.push((text, binding, AtomKind::Word));
            }
            Reading::Node(node) => {
                let &index = self.by_id.get(&node.production).ok_or_else(|| {
                    AdmissionError::UnknownProduction(node.production.to_string())
                })?;
                let me = *next;
                *next += 1;
                let first = out.len();
                located(me, (first, first));
                let mut children = node.children.iter();
                for symbol in &self.productions[index].rhs {
                    if let Symbol::Punct(QUOTATION_MARK, binding, _) = symbol {
                        if glues_after(*binding) {
                            out.push((
                                quotation_mark(*depth).to_string(),
                                *binding,
                                AtomKind::Punct,
                            ));
                            *depth += 1;
                        } else {
                            *depth -= 1;
                            // A quotation's own final period is not written.
                            if let Some(last) = out.last_mut()
                                && last.2 == AtomKind::Punct
                                && last.0 == "."
                            {
                                last.0.clear();
                            }
                            out.push((
                                quotation_mark(*depth).to_string(),
                                *binding,
                                AtomKind::Close,
                            ));
                        }
                    } else if let Symbol::Punct(c, binding, _) = symbol {
                        // Nor is a period after a word that ends in terminal
                        // punctuation (`CompiledGrammar::run`).
                        let absorbed = *c == '.'
                            && out.last().is_some_and(|last| {
                                last.2 == AtomKind::Word && last.0.ends_with(['.', '!', '?'])
                            });
                        let text = if absorbed { String::new() } else { c.to_string() };
                        out.push((text, *binding, AtomKind::Punct));
                    } else {
                        let child = children.next().ok_or_else(|| AdmissionError::Rejected {
                            production: node.production.to_string(),
                            reason: "missing child".to_owned(),
                        })?;
                        let first = out.len();
                        self.atoms_from(child, lexicon, out, located, next, depth)?;
                        if let Symbol::Cat {
                            binding: Some(binding),
                            ..
                        } = symbol
                        {
                            if glues_before(*binding) {
                                out[first].1 = with_before(out[first].1);
                            }
                            if glues_after(*binding) {
                                let last = out.len() - 1;
                                out[last].1 = with_after(out[last].1);
                            }
                        }
                    }
                }
                located(me, (first, out.len()));
            }
        }
        Ok(())
    }
}

/// One Constituent of a Reading, as [`CompiledGrammar::constituents`] locates
/// it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constituent {
    pub category: Ident,
    /// Byte range in the realized text.
    pub span: Range<usize>,
    /// Its name in its mother's Production: the grammatical function it
    /// bears there (`subject`, `object`, `modifier`, ...); none for the
    /// root.
    pub function: Option<Ident>,
    /// Preorder index of its mother; the head of a function Category
    /// (`CategoryDecl::function`) takes that Category's mother and function.
    pub parent: Option<usize>,
    /// The lexeme reached through its `head` children.
    pub head: Option<LexemeId>,
    pub gap: Gap,
    /// The other functions it bears, each with the preorder index of the
    /// phrase it bears it in: a Noun Phrase shared by Right Node Raising
    /// fills each conjunct's Gap (*draw and discard [cards]*), and a
    /// Modifier postposed out of an Object modifies that Object.
    pub also: Vec<(Ident, usize)>,
}

/// What a realized atom is: a word, punctuation, or a closing quotation
/// mark.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AtomKind {
    Word,
    Punct,
    Close,
}

/// A realized atom: its text, binding and kind.
pub(crate) type Atom = (String, Binding, AtomKind);

/// Joins atoms with the spacing their bindings call for, returning the text
/// and each atom's byte range in it. A period or comma after closing
/// quotation marks is written inside them (`docs/oracle-style-guide.md` §3,
/// Periods; Quotation marks and apostrophes), so ranges need not ascend.
pub(crate) fn join(atoms: &[Atom]) -> (String, Vec<Range<usize>>) {
    let mut out = String::new();
    let mut ranges: Vec<Range<usize>> = Vec::with_capacity(atoms.len());
    let mut glue_next = true;
    // The byte and atom index where the closing marks the text ends in begin.
    let mut closing: Option<(usize, usize)> = None;
    for (text, binding, kind) in atoms {
        if !(glue_next || glues_before(*binding)) {
            out.push(' ');
        }
        glue_next = glues_after(*binding);
        if let (AtomKind::Punct, Some((at, first)), "." | ",") = (kind, closing, text.as_str()) {
            out.insert_str(at, text);
            for range in &mut ranges[first..] {
                *range = range.start + text.len()..range.end + text.len();
            }
            ranges.push(at..at + text.len());
            closing = Some((at + text.len(), first));
            continue;
        }
        let start = out.len();
        out.push_str(text);
        ranges.push(start..out.len());
        closing = match kind {
            AtomKind::Close => closing.or(Some((start, ranges.len() - 1))),
            _ if text.is_empty() => closing,
            _ => None,
        };
    }
    (out, ranges)
}

/// Whether an atom with this binding attaches to the atom before it.
pub(crate) fn glues_before(binding: Binding) -> bool {
    matches!(binding, Binding::Suffix | Binding::Bound)
}

/// Whether an atom with this binding attaches to the atom after it.
pub(crate) fn glues_after(binding: Binding) -> bool {
    matches!(binding, Binding::Prefix | Binding::Bound)
}

pub(crate) fn with_before(binding: Binding) -> Binding {
    if glues_after(binding) { Binding::Bound } else { Binding::Suffix }
}
pub(crate) fn with_after(binding: Binding) -> Binding {
    if glues_before(binding) { Binding::Bound } else { Binding::Prefix }
}

#[cfg(test)]
mod enumeration_tests {
    use super::*;

    fn fixture() -> (Lexicon, CompiledGrammar) {
        let lexicon = Lexicon::new(vec![sylex_lexicon::Lexeme::noun(
            "noun",
            "card",
            vec![sylex_lexicon::Countability::Count],
            sylex_lexicon::Source {
                kind: sylex_lexicon::SourceKind::Core,
                path: "reading.rs".to_owned(),
                owner: "test".to_owned(),
            },
        )])
        .unwrap();
        let grammar = crate::Grammar::from_fug_str(
            r"
                root S
                category S
                Leaf: S -> Lex(Noun)
                Pair: S -> left:S right:S
            ",
        )
        .unwrap();
        let grammar =
            CompiledGrammar::bind(&grammar, &lexicon, &crate::FrameCatalog::default()).unwrap();
        (lexicon, grammar)
    }

    #[test]
    fn lazy_stream_advances_only_requested_ranks() {
        let (lexicon, grammar) = fixture();
        let parse = grammar.parse(&lexicon, "card card card card card card card card card");
        assert_eq!(parse.derivation_count(), 1430);
        let mut enumeration = Enumeration::new(&parse);
        assert!(enumeration.streams.is_empty());
        let first = enumeration.ranked(0).unwrap();
        let sizes: Vec<_> = enumeration.streams.iter().map(|s| s.best.len()).collect();
        assert!(sizes.iter().all(|&n| n <= 1));
        assert_eq!(enumeration.ranked(0).unwrap().0, first.0);
        assert_eq!(
            sizes,
            enumeration
                .streams
                .iter()
                .map(|s| s.best.len())
                .collect::<Vec<_>>()
        );
        assert_eq!(enumeration.streams[enumeration.root.unwrap()].best.len(), 1);
        let second = enumeration.ranked(1).unwrap();
        assert!(enumeration.materialize(first.1) < enumeration.materialize(second.1));
        assert_eq!(enumeration.streams[enumeration.root.unwrap()].best.len(), 2);
        assert!(parse.best_readings(0).next().is_none());
    }

    #[test]
    fn arena_order_matches_owned_reading_and_sequence_order() {
        let (lexicon, grammar) = fixture();
        let parse = grammar.parse(&lexicon, "card card card card");
        let mut enumeration = Enumeration::new(&parse);
        for index in 0..5 {
            assert!(enumeration.ranked(index).is_some());
        }
        assert!(enumeration.ranked(5).is_none());
        for a in 0..enumeration.trees.len() {
            for b in 0..enumeration.trees.len() {
                let a = TreeId(u32::try_from(a).unwrap());
                let b = TreeId(u32::try_from(b).unwrap());
                assert_eq!(
                    enumeration.compare_tree(a, b),
                    enumeration.materialize(a).cmp(&enumeration.materialize(b))
                );
            }
        }
        let owned = |mut id: SeqId| {
            let mut out = Vec::new();
            while id.0 != 0 {
                let entry = enumeration.sequences[id.0 as usize - 1];
                out.push(enumeration.materialize(entry.last));
                id = entry.prefix;
            }
            out.reverse();
            out
        };
        for a in 0..=enumeration.sequences.len() {
            for b in 0..=enumeration.sequences.len() {
                let a = SeqId(u32::try_from(a).unwrap());
                let b = SeqId(u32::try_from(b).unwrap());
                assert_eq!(enumeration.compare_sequence(a, b), owned(a).cmp(&owned(b)));
            }
        }
    }
}
