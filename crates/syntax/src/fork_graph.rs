//! The split copy of a Packed Forest that forks are read from
//! ([`crate::forks`]), and its narrowing.
//!
//! The Packed Forest packs a node by (Category, span, summary) and an item by
//! (Production, dot, span, state), so one node can stand for alternatives
//! whose head lines end in different words or pass through different frame
//! Productions, and one item can be completed by head-line children with
//! different heads. The split copy keys each node by its head (its head
//! line's word and frame Productions) as well, and each item by the head of
//! its Production's head-line child, whether that child is already found or
//! still to come. Then what a Derivation answers for each word (which leaf,
//! what its maximal projection depends on, which frame Productions its head
//! line has) is settled by one edge: the leaf's family, the family that
//! attaches the maximal projection, or the root it is. Narrowing by an answer
//! is then a matter of edges.

use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;

use dpsi::Ident;

use crate::chart::ChildRef;
use crate::chart::Parse;
use crate::counts::ForestCycle;
use crate::counts::Vertex;

/// A word of the unit: the start of a leaf some Derivation uses, named by the
/// surface of the shortest such leaf and that surface's 1-based occurrence
/// among the unit's words.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ForkWord {
    /// Its start, in chars of the analyzed text.
    pub start: usize,
    pub surface: String,
    pub occurrence: usize,
}

/// What a word's maximal projection depends on, by interned word indices.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) enum Dep {
    Root,
    Dependent {
        function: Ident,
        /// The governor's Category, by name: a private Category the
        /// grammar compiles reads as the one it copies.
        category: Ident,
        governor: Option<u32>,
    },
}

/// What a Derivation answers at one edge.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct Facts {
    /// The word a leaf starts at, and the leaf.
    pub lexical: Option<(u32, u32)>,
    /// The word whose maximal projection is attached here, what it depends
    /// on, and its head line's frame Productions (interned).
    pub attachment: Option<(u32, u32, u32)>,
}

/// What every narrowing of one unit's split forest shares: its words and
/// the interned answers.
#[derive(Debug)]
pub(crate) struct Unit {
    pub words: Vec<ForkWord>,
    pub dependencies: Vec<Dep>,
    pub frames: Vec<Vec<usize>>,
}

#[derive(Debug, Clone)]
struct GNode {
    original: usize,
    items: Vec<usize>,
}

#[derive(Debug, Clone)]
struct GItem {
    original: usize,
    families: Vec<usize>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum GChild {
    Node(usize),
    Leaf(usize),
    Punct,
}

#[derive(Debug, Clone)]
struct GFamily {
    item: usize,
    previous: Option<usize>,
    child: GChild,
    facts: Facts,
}

#[derive(Debug, Clone)]
struct GRoot {
    node: usize,
    facts: Facts,
}

/// An edge of the split forest: an item's family, or the choice of a root.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) enum Edge {
    Family(usize),
    Root(usize),
}

/// A split forest: every node and item has a Derivation and is used by one.
#[derive(Debug, Clone, Default)]
pub(crate) struct Graph {
    nodes: Vec<GNode>,
    items: Vec<GItem>,
    families: Vec<GFamily>,
    roots: Vec<GRoot>,
}

#[derive(Debug, Clone, Copy)]
enum V {
    Node(usize),
    Item(usize),
}

/// The exact Derivation counts of a split forest.
pub(crate) struct GraphCounts {
    pub total: u128,
    node_inside: Vec<u128>,
    item_inside: Vec<u128>,
    item_outside: Vec<u128>,
}

/// A node's head: its head line's word and frame Productions (interned).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Head {
    word: Option<u32>,
    frames: u32,
}

/// Interned heads and frame sequences while splitting.
struct Interner {
    heads: Vec<Head>,
    head_ids: HashMap<Head, u32>,
    frames: Vec<Vec<usize>>,
    frame_ids: HashMap<Vec<usize>, u32>,
    extended: HashMap<(u32, usize), u32>,
    dependencies: Vec<Dep>,
    dependency_ids: HashMap<Dep, u32>,
}

impl Interner {
    fn new() -> Self {
        Self {
            heads: Vec::new(),
            head_ids: HashMap::new(),
            frames: vec![Vec::new()],
            frame_ids: HashMap::from([(Vec::new(), 0)]),
            extended: HashMap::new(),
            dependencies: Vec::new(),
            dependency_ids: HashMap::new(),
        }
    }

    fn head(&mut self, head: Head) -> u32 {
        *self.head_ids.entry(head).or_insert_with(|| {
            self.heads.push(head);
            u32::try_from(self.heads.len() - 1).expect("heads exceed u32")
        })
    }

    fn extend(&mut self, frames: u32, production: usize) -> u32 {
        if let Some(&id) = self.extended.get(&(frames, production)) {
            return id;
        }
        let mut sequence = self.frames[frames as usize].clone();
        sequence.push(production);
        let id = *self.frame_ids.entry(sequence.clone()).or_insert_with(|| {
            self.frames.push(sequence);
            u32::try_from(self.frames.len() - 1).expect("frame sequences exceed u32")
        });
        self.extended.insert((frames, production), id);
        id
    }

    fn dependency(&mut self, dep: Dep) -> u32 {
        *self.dependency_ids.entry(dep).or_insert_with(|| {
            self.dependencies.push(dep);
            u32::try_from(self.dependencies.len() - 1).expect("dependencies exceed u32")
        })
    }
}

fn add(a: u128, b: u128) -> u128 {
    a.checked_add(b).expect("a Derivation count overflows u128")
}

fn mul(a: u128, b: u128) -> u128 {
    a.checked_mul(b).expect("a Derivation count overflows u128")
}

fn u32_of(n: usize) -> u32 {
    u32::try_from(n).expect("a split forest index exceeds u32")
}

/// Splits a Parse's reachable forest by head (see the module doc).
pub(crate) fn split(parse: &Parse<'_>) -> Result<(Unit, Graph), ForestCycle> {
    let order = parse.children_first()?;
    let forest = &parse.forest;
    let grammar = parse.grammar;

    // The words: the starts of the leaves the roots reach.
    let mut used = BTreeSet::new();
    for &vertex in &order {
        if let Vertex::Item(i) = vertex {
            for &(_, child) in &forest.items[i].families {
                if let ChildRef::Leaf(l) = child {
                    used.insert(l);
                }
            }
        }
    }
    let mut seen = BTreeSet::new();
    let mut shortest: std::collections::BTreeMap<usize, usize> = std::collections::BTreeMap::new();
    for &l in &used {
        let leaf = &forest.leaves[l];
        assert!(
            seen.insert((leaf.start, leaf.end, leaf.reading.clone())),
            "two leaves at chars {}..{} have one lexical reading",
            leaf.start,
            leaf.end
        );
        let end = shortest.entry(leaf.start).or_insert(leaf.end);
        *end = (*end).min(leaf.end);
    }
    let mut occurrences: HashMap<String, usize> = HashMap::new();
    let mut words = Vec::new();
    let mut word_at = HashMap::new();
    for (&start, &end) in &shortest {
        let surface: String = parse.tokens[start..end].iter().collect();
        let occurrence = occurrences.entry(surface.clone()).or_insert(0);
        *occurrence += 1;
        word_at.insert(start, u32_of(words.len()));
        words.push(ForkWord {
            start,
            surface,
            occurrence: *occurrence,
        });
    }
    let mut leaf_word = vec![None; forest.leaves.len()];
    for &l in &used {
        leaf_word[l] = Some(word_at[&forest.leaves[l].start]);
    }

    // Bottom up: the heads each node can have, and each item whose
    // Production's head-line child is found.
    let mut intern = Interner::new();
    let headless = intern.head(Head {
        word: None,
        frames: 0,
    });
    let line = |production: usize| grammar.head_line_child(production);
    let mut node_heads: Vec<Vec<u32>> = vec![Vec::new(); forest.nodes.len()];
    let mut item_heads: Vec<Vec<u32>> = vec![Vec::new(); forest.items.len()];
    let leaf_head = |intern: &mut Interner, l: usize| {
        intern.head(Head {
            word: leaf_word[l],
            frames: 0,
        })
    };
    for &vertex in &order {
        match vertex {
            Vertex::Item(i) => {
                let item = &forest.items[i];
                let at = item.dot - 1;
                let mut heads = BTreeSet::new();
                match line(item.production) {
                    None => {
                        heads.insert(headless);
                    }
                    Some(h) if at == h => {
                        for &(_, child) in &item.families {
                            match child {
                                ChildRef::Node(c) => heads.extend(node_heads[c].iter().copied()),
                                ChildRef::Leaf(l) => {
                                    heads.insert(leaf_head(&mut intern, l));
                                }
                                ChildRef::Punct => {
                                    heads.insert(headless);
                                }
                            }
                        }
                    }
                    Some(h) if at > h => {
                        for &(previous, _) in &item.families {
                            let p = previous.expect("a child after the head line has a previous");
                            heads.extend(item_heads[p].iter().copied());
                        }
                    }
                    Some(_) => {}
                }
                item_heads[i] = heads.into_iter().collect();
            }
            Vertex::Node(n) => {
                let mut heads = BTreeSet::new();
                for &(production, item) in &forest.nodes[n].families {
                    for &head in &item_heads[item] {
                        heads.insert(lift(&mut intern, grammar, head, production));
                    }
                }
                node_heads[n] = heads.into_iter().collect();
            }
        }
    }

    // Top down from the roots: each node copy by head, each item copy by the
    // head of its Production's head-line child.
    let mut down = Down {
        parse,
        intern,
        headless,
        leaf_word: &leaf_word,
        node_heads: &node_heads,
        item_heads: &item_heads,
        graph: Graph::default(),
        node_copies: HashMap::new(),
        item_copies: HashMap::new(),
        queue: Vec::new(),
    };
    for &root in &forest.roots {
        for &head in &node_heads[root] {
            let node = down.node(root, head);
            let h = down.intern.heads[head as usize];
            let attachment = h
                .word
                .map(|word| (word, down.intern.dependency(Dep::Root), h.frames));
            down.graph.roots.push(GRoot {
                node,
                facts: Facts {
                    lexical: None,
                    attachment,
                },
            });
        }
    }
    while let Some((vertex, original, head)) = down.queue.pop() {
        match vertex {
            V::Node(n) => down.expand_node(n, original, head),
            V::Item(i) => down.expand_item(i, original, head),
        }
    }
    let Down { intern, graph, .. } = down;
    Ok((
        Unit {
            words,
            dependencies: intern.dependencies,
            frames: intern.frames,
        },
        graph,
    ))
}

/// The top-down pass of [`split`].
struct Down<'a, 'p> {
    parse: &'a Parse<'p>,
    intern: Interner,
    headless: u32,
    leaf_word: &'a [Option<u32>],
    node_heads: &'a [Vec<u32>],
    item_heads: &'a [Vec<u32>],
    graph: Graph,
    node_copies: HashMap<(usize, u32), usize>,
    item_copies: HashMap<(usize, u32), usize>,
    /// Copies to expand, with their original and head.
    queue: Vec<(V, usize, u32)>,
}

impl Down<'_, '_> {
    fn node(&mut self, original: usize, head: u32) -> usize {
        if let Some(&n) = self.node_copies.get(&(original, head)) {
            return n;
        }
        let n = self.graph.nodes.len();
        self.graph.nodes.push(GNode {
            original,
            items: Vec::new(),
        });
        self.node_copies.insert((original, head), n);
        self.queue.push((V::Node(n), original, head));
        n
    }

    fn item(&mut self, original: usize, head: u32) -> usize {
        if let Some(&i) = self.item_copies.get(&(original, head)) {
            return i;
        }
        let i = self.graph.items.len();
        self.graph.items.push(GItem {
            original,
            families: Vec::new(),
        });
        self.item_copies.insert((original, head), i);
        self.queue.push((V::Item(i), original, head));
        i
    }

    fn leaf_head(&mut self, leaf: usize) -> u32 {
        self.intern.head(Head {
            word: self.leaf_word[leaf],
            frames: 0,
        })
    }

    fn expand_node(&mut self, n: usize, original: usize, head: u32) {
        let grammar = self.parse.grammar;
        let mut items = Vec::new();
        for &(production, item) in &self.parse.forest.nodes[original].families {
            for &item_head in &self.item_heads[item] {
                if lift(&mut self.intern, grammar, item_head, production) == head {
                    items.push(self.item(item, item_head));
                }
            }
        }
        self.graph.nodes[n].items = items;
    }

    fn expand_item(&mut self, i: usize, original: usize, head: u32) {
        let parse = self.parse;
        let grammar = parse.grammar;
        let item = &parse.forest.items[original];
        let production = item.production;
        let at = item.dot - 1;
        let line = grammar.head_line_child(production);
        let governor = self.intern.heads[head as usize].word;
        for &(previous, child) in &item.families {
            let previous = match previous {
                None => None,
                Some(p) => {
                    // A previous item past the head-line child settled its
                    // head below; one before it takes the head from here.
                    let settled = line.is_none_or(|h| at > h);
                    if settled && self.item_heads[p].binary_search(&head).is_err() {
                        continue;
                    }
                    Some(self.item(p, head))
                }
            };
            let mut choices: Vec<(GChild, u32)> = Vec::new();
            match child {
                ChildRef::Node(c) => {
                    for &child_head in &self.node_heads[c] {
                        if line != Some(at) || child_head == head {
                            choices.push((GChild::Node(self.node(c, child_head)), child_head));
                        }
                    }
                }
                ChildRef::Leaf(l) => {
                    let leaf_head = self.leaf_head(l);
                    if line != Some(at) || leaf_head == head {
                        choices.push((GChild::Leaf(l), leaf_head));
                    }
                }
                ChildRef::Punct => {
                    if line != Some(at) || self.headless == head {
                        choices.push((GChild::Punct, self.headless));
                    }
                }
            }
            for (child, child_head) in choices {
                let lexical = match child {
                    GChild::Leaf(l) => Some((
                        self.leaf_word[l].expect("a used leaf has a word"),
                        u32_of(l),
                    )),
                    GChild::Node(_) | GChild::Punct => None,
                };
                let attachment = if line == Some(at) || matches!(child, GChild::Punct) {
                    None
                } else {
                    let h = self.intern.heads[child_head as usize];
                    h.word.map(|word| {
                        let dep = self.intern.dependency(Dep::Dependent {
                            function: grammar.productions[production].names[at],
                            category: grammar.categories[grammar.productions[production].lhs],
                            governor,
                        });
                        (word, dep, h.frames)
                    })
                };
                let family = self.graph.families.len();
                self.graph.families.push(GFamily {
                    item: i,
                    previous,
                    child,
                    facts: Facts {
                        lexical,
                        attachment,
                    },
                });
                self.graph.items[i].families.push(family);
            }
        }
    }
}

/// A head after a node's Production: the same word, and the Production
/// appended to its frame Productions when it is one.
fn lift(
    intern: &mut Interner,
    grammar: &crate::CompiledGrammar,
    head: u32,
    production: usize,
) -> u32 {
    let h = intern.heads[head as usize];
    let frames = if grammar.is_frame_production(production) {
        intern.extend(h.frames, production)
    } else {
        h.frames
    };
    intern.head(Head {
        word: h.word,
        frames,
    })
}
impl Graph {
    pub(crate) fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub(crate) fn item_count(&self) -> usize {
        self.items.len()
    }

    /// The nodes and items, each after everything it derives from.
    fn order(&self) -> Vec<V> {
        let mut node_seen = vec![false; self.nodes.len()];
        let mut item_seen = vec![false; self.items.len()];
        let mut order = Vec::with_capacity(self.nodes.len() + self.items.len());
        let successors = |vertex: V| -> Vec<V> {
            match vertex {
                V::Node(n) => self.nodes[n].items.iter().map(|&i| V::Item(i)).collect(),
                V::Item(i) => self.items[i]
                    .families
                    .iter()
                    .flat_map(|&f| {
                        let family = &self.families[f];
                        let child = match family.child {
                            GChild::Node(c) => Some(V::Node(c)),
                            GChild::Leaf(_) | GChild::Punct => None,
                        };
                        family.previous.map(V::Item).into_iter().chain(child)
                    })
                    .collect(),
            }
        };
        for root in &self.roots {
            if node_seen[root.node] {
                continue;
            }
            node_seen[root.node] = true;
            let mut stack = vec![(V::Node(root.node), successors(V::Node(root.node)), 0)];
            while let Some(top) = stack.last_mut() {
                if top.2 == top.1.len() {
                    order.push(top.0);
                    stack.pop();
                    continue;
                }
                let next = top.1[top.2];
                top.2 += 1;
                let seen = match next {
                    V::Node(n) => &mut node_seen[n],
                    V::Item(i) => &mut item_seen[i],
                };
                if !*seen {
                    *seen = true;
                    stack.push((next, successors(next), 0));
                }
            }
        }
        order
    }

    fn child_inside(node_inside: &[u128], child: GChild) -> u128 {
        match child {
            GChild::Node(c) => node_inside[c],
            GChild::Leaf(_) | GChild::Punct => 1,
        }
    }

    /// Exact Derivation counts, by an inside and an outside pass.
    pub(crate) fn counts(&self) -> GraphCounts {
        let order = self.order();
        let mut node_inside = vec![0u128; self.nodes.len()];
        let mut item_inside = vec![0u128; self.items.len()];
        for &vertex in &order {
            match vertex {
                V::Node(n) => {
                    node_inside[n] = self.nodes[n]
                        .items
                        .iter()
                        .map(|&i| item_inside[i])
                        .fold(0, add);
                }
                V::Item(i) => {
                    item_inside[i] = self.items[i]
                        .families
                        .iter()
                        .map(|&f| {
                            let family = &self.families[f];
                            mul(
                                family.previous.map_or(1, |p| item_inside[p]),
                                Self::child_inside(&node_inside, family.child),
                            )
                        })
                        .fold(0, add);
                }
            }
        }
        let mut node_outside = vec![0u128; self.nodes.len()];
        let mut item_outside = vec![0u128; self.items.len()];
        for root in &self.roots {
            node_outside[root.node] = add(node_outside[root.node], 1);
        }
        for &vertex in order.iter().rev() {
            match vertex {
                V::Node(n) => {
                    for &i in &self.nodes[n].items {
                        item_outside[i] = add(item_outside[i], node_outside[n]);
                    }
                }
                V::Item(i) => {
                    let outside = item_outside[i];
                    for &f in &self.items[i].families {
                        let family = &self.families[f];
                        if let Some(p) = family.previous {
                            item_outside[p] = add(
                                item_outside[p],
                                mul(outside, Self::child_inside(&node_inside, family.child)),
                            );
                        }
                        if let GChild::Node(c) = family.child {
                            let before = family.previous.map_or(1, |p| item_inside[p]);
                            node_outside[c] = add(node_outside[c], mul(outside, before));
                        }
                    }
                }
            }
        }
        let total = self
            .roots
            .iter()
            .map(|root| node_inside[root.node])
            .fold(0, add);
        GraphCounts {
            total,
            node_inside,
            item_inside,
            item_outside,
        }
    }

    /// Every edge with what it answers and the Derivations that use it.
    pub(crate) fn edges<'g>(
        &'g self,
        counts: &'g GraphCounts,
    ) -> impl Iterator<Item = (Edge, Facts, u128)> + 'g {
        let roots = self
            .roots
            .iter()
            .enumerate()
            .map(|(r, root)| (Edge::Root(r), root.facts, counts.node_inside[root.node]));
        let families = self.families.iter().enumerate().map(|(f, family)| {
            let count = mul(
                mul(
                    counts.item_outside[family.item],
                    family.previous.map_or(1, |p| counts.item_inside[p]),
                ),
                Self::child_inside(&counts.node_inside, family.child),
            );
            (Edge::Family(f), family.facts, count)
        });
        roots.chain(families)
    }

    /// The Derivations that use headless of `edges`.
    pub(crate) fn exclude(&self, edges: &HashSet<Edge>) -> Graph {
        // Bottom up: which copies still have a Derivation.
        let order = self.order();
        let mut node_live = vec![false; self.nodes.len()];
        let mut item_live = vec![false; self.items.len()];
        let family_live = |f: usize, node_live: &[bool], item_live: &[bool]| {
            let family = &self.families[f];
            !edges.contains(&Edge::Family(f))
                && family.previous.is_none_or(|p| item_live[p])
                && match family.child {
                    GChild::Node(c) => node_live[c],
                    GChild::Leaf(_) | GChild::Punct => true,
                }
        };
        for &vertex in &order {
            match vertex {
                V::Node(n) => node_live[n] = self.nodes[n].items.iter().any(|&i| item_live[i]),
                V::Item(i) => {
                    item_live[i] = self.items[i]
                        .families
                        .iter()
                        .any(|&f| family_live(f, &node_live, &item_live));
                }
            }
        }
        // Top down from the live roots.
        let mut out = Rebuild::new(self);
        for (r, root) in self.roots.iter().enumerate() {
            if node_live[root.node] && !edges.contains(&Edge::Root(r)) {
                let node = out.node((root.node, false));
                out.graph.roots.push(GRoot {
                    node,
                    facts: root.facts,
                });
            }
        }
        while let Some(vertex) = out.queue.pop() {
            match vertex {
                V::Node(n) => {
                    let (old, _) = out.nodes[n];
                    let items = self.nodes[old]
                        .items
                        .iter()
                        .filter(|&&i| item_live[i])
                        .map(|&i| out.item((i, false)))
                        .collect();
                    out.graph.nodes[n].items = items;
                }
                V::Item(i) => {
                    let (old, _) = out.items[i];
                    for &f in &self.items[old].families {
                        if !family_live(f, &node_live, &item_live) {
                            continue;
                        }
                        let family = &self.families[f];
                        let previous = family.previous.map(|p| out.item((p, false)));
                        let child = match family.child {
                            GChild::Node(c) => GChild::Node(out.node((c, false))),
                            other => other,
                        };
                        out.family(i, previous, child, family.facts);
                    }
                }
            }
        }
        out.graph
    }

    /// The Derivations that use one of `edges`. No Derivation uses two: the
    /// edges are one answer's, and a Derivation answers once for a word.
    ///
    /// # Panics
    /// When some Derivation uses two of `edges`.
    pub(crate) fn restrict(&self, edges: &HashSet<Edge>) -> Graph {
        // Bottom up: each copy's Derivations that use headless of `edges`, and
        // those that use one.
        let order = self.order();
        let mut node_inside = vec![[0u128; 2]; self.nodes.len()];
        let mut item_inside = vec![[0u128; 2]; self.items.len()];
        let child_inside = |node_inside: &[[u128; 2]], child: GChild| match child {
            GChild::Node(c) => node_inside[c],
            GChild::Leaf(_) | GChild::Punct => [1, 0],
        };
        for &vertex in &order {
            match vertex {
                V::Node(n) => {
                    let mut sum = [0, 0];
                    for &i in &self.nodes[n].items {
                        sum = [
                            add(sum[0], item_inside[i][0]),
                            add(sum[1], item_inside[i][1]),
                        ];
                    }
                    node_inside[n] = sum;
                }
                V::Item(i) => {
                    let mut sum = [0, 0];
                    for &f in &self.items[i].families {
                        let family = &self.families[f];
                        let before = family.previous.map_or([1, 0], |p| item_inside[p]);
                        let last = child_inside(&node_inside, family.child);
                        assert_eq!(
                            mul(before[1], last[1]),
                            0,
                            "a Derivation uses two edges of one answer"
                        );
                        if edges.contains(&Edge::Family(f)) {
                            assert_eq!(
                                add(mul(before[1], last[0]), mul(before[0], last[1])),
                                0,
                                "a Derivation uses two edges of one answer"
                            );
                            sum[1] = add(sum[1], mul(before[0], last[0]));
                        } else {
                            sum[0] = add(sum[0], mul(before[0], last[0]));
                            sum[1] = add(
                                sum[1],
                                add(mul(before[1], last[0]), mul(before[0], last[1])),
                            );
                        }
                    }
                    item_inside[i] = sum;
                }
            }
        }
        // Top down from the roots, each copy by whether it uses an edge.
        let mut out = Rebuild::new(self);
        for (r, root) in self.roots.iter().enumerate() {
            let used = !edges.contains(&Edge::Root(r));
            if node_inside[root.node][usize::from(used)] > 0 {
                let node = out.node((root.node, used));
                out.graph.roots.push(GRoot {
                    node,
                    facts: root.facts,
                });
            }
        }
        while let Some(vertex) = out.queue.pop() {
            match vertex {
                V::Node(n) => {
                    let (old, used) = out.nodes[n];
                    let items = self.nodes[old]
                        .items
                        .iter()
                        .filter(|&&i| item_inside[i][usize::from(used)] > 0)
                        .map(|&i| out.item((i, used)))
                        .collect();
                    out.graph.nodes[n].items = items;
                }
                V::Item(i) => {
                    let (old, used) = out.items[i];
                    for &f in &self.items[old].families {
                        let family = &self.families[f];
                        let mine = edges.contains(&Edge::Family(f));
                        // (previous uses one, child uses one) splits of `used`.
                        let splits: &[(bool, bool)] = match (used, mine) {
                            (false, true) => &[],
                            (true, true) | (false, false) => &[(false, false)],
                            (true, false) => &[(true, false), (false, true)],
                        };
                        for &(p_used, c_used) in splits {
                            let before = family.previous.map_or([1, 0], |p| item_inside[p])
                                [usize::from(p_used)];
                            let last =
                                child_inside(&node_inside, family.child)[usize::from(c_used)];
                            if before == 0 || last == 0 {
                                continue;
                            }
                            let previous = family.previous.map(|p| out.item((p, p_used)));
                            let child = match family.child {
                                GChild::Node(c) => GChild::Node(out.node((c, c_used))),
                                other => other,
                            };
                            out.family(i, previous, child, family.facts);
                        }
                    }
                }
            }
        }
        out.graph
    }

    /// The Derivations as Readings.
    pub(crate) fn trees(&self, parse: &Parse<'_>) -> Vec<crate::Reading> {
        let mut nodes: Vec<Vec<Rc<Tree>>> = vec![Vec::new(); self.nodes.len()];
        let mut items: Vec<Vec<Option<Rc<Sequence>>>> = vec![Vec::new(); self.items.len()];
        for vertex in self.order() {
            match vertex {
                V::Node(n) => {
                    let mut out = Vec::new();
                    for &i in &self.nodes[n].items {
                        let production = parse.forest.items[self.items[i].original].production;
                        for children in &items[i] {
                            out.push(Rc::new(Tree::Node {
                                node: self.nodes[n].original,
                                production,
                                children: children.clone(),
                            }));
                        }
                    }
                    nodes[n] = out;
                }
                V::Item(i) => {
                    let mut out = Vec::new();
                    for &f in &self.items[i].families {
                        let family = &self.families[f];
                        let prefixes: &[Option<Rc<Sequence>>] = match family.previous {
                            Some(p) => &items[p],
                            None => &[None],
                        };
                        let lasts: Vec<Option<Rc<Tree>>> = match family.child {
                            GChild::Node(c) => nodes[c].iter().cloned().map(Some).collect(),
                            GChild::Leaf(l) => vec![Some(Rc::new(Tree::Leaf(l)))],
                            GChild::Punct => vec![None],
                        };
                        for prefix in prefixes {
                            for last in &lasts {
                                out.push(match last {
                                    Some(tree) => Some(Rc::new(Sequence {
                                        prefix: prefix.clone(),
                                        last: Rc::clone(tree),
                                    })),
                                    None => prefix.clone(),
                                });
                            }
                        }
                    }
                    items[i] = out;
                }
            }
        }
        let mut features = HashMap::new();
        self.roots
            .iter()
            .flat_map(|root| nodes[root.node].iter())
            .map(|tree| tree.reading(parse, &mut features))
            .collect()
    }
}

/// A Derivation's tree before it is read as a Reading.
enum Tree {
    Node {
        node: usize,
        production: usize,
        children: Option<Rc<Sequence>>,
    },
    Leaf(usize),
}

/// A sequence of children, last first.
struct Sequence {
    prefix: Option<Rc<Sequence>>,
    last: Rc<Tree>,
}

impl Tree {
    fn reading(
        &self,
        parse: &Parse<'_>,
        features: &mut HashMap<usize, std::collections::BTreeMap<Ident, Ident>>,
    ) -> crate::Reading {
        match self {
            Tree::Leaf(l) => crate::Reading::Leaf(parse.forest.leaves[*l].reading.clone()),
            Tree::Node {
                node,
                production,
                children,
            } => {
                let mut readings = Vec::new();
                let mut at = children.as_deref();
                while let Some(sequence) = at {
                    readings.push(sequence.last.reading(parse, features));
                    at = sequence.prefix.as_deref();
                }
                readings.reverse();
                let bound = features
                    .entry(*node)
                    .or_insert_with(|| parse.node_features(*node))
                    .clone();
                parse.reading_node(*node, *production, bound, readings)
            }
        }
    }
}

/// A new split forest built top down from copies of an old one's nodes and
/// items, each keyed by the old index and a flag.
struct Rebuild {
    graph: Graph,
    node_ids: HashMap<(usize, bool), usize>,
    item_ids: HashMap<(usize, bool), usize>,
    nodes: Vec<(usize, bool)>,
    items: Vec<(usize, bool)>,
    old_nodes: Vec<usize>,
    old_items: Vec<usize>,
    queue: Vec<V>,
}

impl Rebuild {
    fn new(old: &Graph) -> Self {
        Self {
            graph: Graph::default(),
            node_ids: HashMap::new(),
            item_ids: HashMap::new(),
            nodes: Vec::new(),
            items: Vec::new(),
            old_nodes: old.nodes.iter().map(|n| n.original).collect(),
            old_items: old.items.iter().map(|i| i.original).collect(),
            queue: Vec::new(),
        }
    }

    fn node(&mut self, key: (usize, bool)) -> usize {
        if let Some(&n) = self.node_ids.get(&key) {
            return n;
        }
        let n = self.graph.nodes.len();
        self.graph.nodes.push(GNode {
            original: self.old_nodes[key.0],
            items: Vec::new(),
        });
        self.nodes.push(key);
        self.node_ids.insert(key, n);
        self.queue.push(V::Node(n));
        n
    }

    fn item(&mut self, key: (usize, bool)) -> usize {
        if let Some(&i) = self.item_ids.get(&key) {
            return i;
        }
        let i = self.graph.items.len();
        self.graph.items.push(GItem {
            original: self.old_items[key.0],
            families: Vec::new(),
        });
        self.items.push(key);
        self.item_ids.insert(key, i);
        self.queue.push(V::Item(i));
        i
    }

    fn family(&mut self, item: usize, previous: Option<usize>, child: GChild, facts: Facts) {
        let f = self.graph.families.len();
        self.graph.families.push(GFamily {
            item,
            previous,
            child,
            facts,
        });
        self.graph.items[item].families.push(f);
    }
}
