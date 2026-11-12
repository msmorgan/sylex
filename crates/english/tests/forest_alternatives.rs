//! The distributions of `sylex_syntax`'s alternatives (head words, conjunct
//! counts and Embeddings) on real Corpus Units, against an enumeration of
//! every Derivation as a tree: for each node's alternative and value, the
//! enumerated Derivations that use the alternative with that value equal the
//! distribution's entry times the counterpart count (the node's outside count
//! for a head word or a conjunct count, the completed item's inside count for
//! an Embedding).

use std::collections::BTreeMap;
use std::rc::Rc;

use dpsi::Ident;
use sylex_english::corpus::Corpus;
use sylex_lexicon::LexicalReading;
use sylex_syntax::ChildRef;
use sylex_syntax::EMBEDDING_DEPTH;
use sylex_syntax::Embedding;
use sylex_syntax::EmbeddingStep;
use sylex_syntax::HeadWord;
use sylex_syntax::Parse;
use sylex_vocabulary::LexicalEnvironment;

/// Real units from one Derivation to thousands: *Crook of Condemnation*
/// (one and two), *Delver of Secrets* (four, a passive in an *if* clause),
/// *Hauken's Insight* (tens), *Coram, the Undertaker* (hundreds) and
/// *Draugr Necromancer* (3,312).
const SAMPLE: &[&str] = &[
    "Crook of Condemnation",
    "Delver of Secrets // Insectile Aberration",
    "Hauken's Insight",
    "Coram, the Undertaker",
    "Draugr Necromancer",
];

/// One alternative of a node in one Derivation, with its children's trees.
struct Tree {
    node: usize,
    alternative: usize,
    production: usize,
    children: Vec<Child>,
}

enum Child {
    Tree(Rc<Tree>),
    Leaf(usize),
    Punct,
}

/// Every Derivation of a node, as trees.
fn trees(
    parse: &Parse<'_>,
    node: usize,
    memo: &mut BTreeMap<usize, Vec<Rc<Tree>>>,
) -> Vec<Rc<Tree>> {
    if let Some(trees) = memo.get(&node) {
        return trees.clone();
    }
    let mut out = Vec::new();
    let alternatives: Vec<(usize, usize)> = parse.forest.node_alternatives(node).collect();
    for (alternative, (production, item)) in alternatives.into_iter().enumerate() {
        for children in sequences(parse, item, memo) {
            out.push(Rc::new(Tree {
                node,
                alternative,
                production,
                children,
            }));
        }
    }
    memo.insert(node, out.clone());
    out
}

/// Every child sequence of an item.
fn sequences(
    parse: &Parse<'_>,
    item: usize,
    memo: &mut BTreeMap<usize, Vec<Rc<Tree>>>,
) -> Vec<Vec<Child>> {
    let mut out = Vec::new();
    let families: Vec<_> = parse.forest.item_families(item).collect();
    for (previous, child) in families {
        let befores = previous.map_or_else(|| vec![Vec::new()], |p| sequences(parse, p, memo));
        let heres: Vec<Child> = match child {
            ChildRef::Node(n) => trees(parse, n, memo).into_iter().map(Child::Tree).collect(),
            ChildRef::Leaf(l) => vec![Child::Leaf(l)],
            ChildRef::Punct => vec![Child::Punct],
        };
        for before in &befores {
            for here in &heres {
                let mut sequence: Vec<Child> = before.iter().map(copy).collect();
                sequence.push(copy(here));
                out.push(sequence);
            }
        }
    }
    out
}

fn copy(child: &Child) -> Child {
    match child {
        Child::Tree(t) => Child::Tree(Rc::clone(t)),
        Child::Leaf(l) => Child::Leaf(*l),
        Child::Punct => Child::Punct,
    }
}

/// What the enumeration finds, per (node, alternative).
#[derive(Default)]
struct Found {
    heads: BTreeMap<(usize, usize, Option<HeadWord>), u128>,
    conjuncts: BTreeMap<(usize, usize, usize), u128>,
    embeddings: BTreeMap<(usize, usize, Embedding), u128>,
}

struct Walk<'a, 'g> {
    parse: &'a Parse<'g>,
    found: Found,
}

impl Walk<'_, '_> {
    fn function(&self, production: usize, at: usize) -> &str {
        self.parse.grammar().production_functions(production)[at].as_str()
    }

    /// The word reached through `head` children.
    fn head(&self, tree: &Tree) -> Option<HeadWord> {
        let functions = self.parse.grammar().production_functions(tree.production);
        let at = functions.iter().position(|f| *f == "head")?;
        match &tree.children[at] {
            Child::Tree(t) => self.head(t),
            Child::Leaf(l) => match self.parse.forest.leaf_reading(*l) {
                LexicalReading::Word(word) => Some((word.lexeme, word.form)),
                _ => None,
            },
            Child::Punct => None,
        }
    }

    fn conjuncts(&self, tree: &Tree) -> usize {
        tree.children
            .iter()
            .enumerate()
            .map(
                |(at, child)| match (self.function(tree.production, at), child) {
                    ("first" | "last" | "coordinate", _) => 1,
                    ("rest", Child::Tree(t)) => self.conjuncts(t),
                    _ => 0,
                },
            )
            .sum()
    }

    /// Records a tree whose node stands under `above`, and, when it is a
    /// dependent, the step its own Production completes.
    fn visit(
        &mut self,
        tree: &Tree,
        above: &Embedding,
        step: Option<(Ident, usize, Option<HeadWord>)>,
    ) {
        let embedding: Embedding = match step {
            Some((function, governor, governor_head)) => std::iter::once(EmbeddingStep {
                projection: tree.production,
                function,
                governor,
                governor_head,
            })
            .chain(above.iter().cloned())
            .take(EMBEDDING_DEPTH)
            .collect(),
            None => above.clone(),
        };
        let key = (tree.node, tree.alternative);
        *self
            .found
            .heads
            .entry((key.0, key.1, self.head(tree)))
            .or_default() += 1;
        *self
            .found
            .conjuncts
            .entry((key.0, key.1, self.conjuncts(tree)))
            .or_default() += 1;
        *self
            .found
            .embeddings
            .entry((key.0, key.1, embedding.clone()))
            .or_default() += 1;
        let head = self.head(tree);
        let functions = self.parse.grammar().production_functions(tree.production);
        for (at, child) in tree.children.iter().enumerate() {
            let Child::Tree(t) = child else { continue };
            if functions[at] == "head" {
                self.visit(t, &embedding, None);
            } else {
                self.visit(t, &embedding, Some((functions[at], tree.production, head)));
            }
        }
    }
}

#[test]
fn alternatives_distributions_agree_with_enumerated_derivations_on_real_units() {
    let env = LexicalEnvironment::load().unwrap();
    let grammar = sylex_english::bind(&env.lexicon, &env.frames).unwrap();
    let corpus = Corpus::load(|face| SAMPLE.contains(&face.card.as_str())).unwrap();
    let cleaves = sylex_english::census::cleave_texts(&corpus, &env.lexicon);
    let mut most = 0;
    for (unit, cleave) in corpus.units.iter().zip(&cleaves) {
        let face = &corpus.faces[unit.face];
        let text = cleave
            .as_ref()
            .map_or(unit.text.as_str(), |c| c.uncleaved.as_str());
        let analyzed = env.lexicon.analyze_source_scoped(text, None, &face.scope());
        let parse = grammar.parse_analyzed(&analyzed);
        let counts = parse.counts().unwrap();
        most = most.max(counts.total());
        let mut memo = BTreeMap::new();
        let mut walk = Walk {
            parse: &parse,
            found: Found::default(),
        };
        let roots: Vec<usize> = parse.forest.roots().to_vec();
        for root in roots {
            for tree in trees(&parse, root, &mut memo) {
                walk.visit(&tree, &Vec::new(), None);
            }
        }
        let found = walk.found;
        let heads = parse.alternative_heads().unwrap();
        let conjuncts = parse.alternative_conjuncts().unwrap();
        let embeddings = parse.alternative_embeddings().unwrap();
        let name = format!("{} | {text}", face.name);
        let mut computed = Found::default();
        for node in (0..parse.forest.node_count()).filter(|&n| counts.node(n) > 0) {
            let outside = counts.node_outside(node);
            for (a, (_, item)) in parse.forest.node_alternatives(node).enumerate() {
                for (&head, &n) in &heads[node][a] {
                    computed.heads.insert((node, a, head), n * outside);
                }
                for (&c, &n) in &conjuncts[node][a] {
                    computed.conjuncts.insert((node, a, c), n * outside);
                }
                for (e, &n) in &embeddings[node][a] {
                    computed
                        .embeddings
                        .insert((node, a, e.clone()), n * counts.item_inside(item));
                }
            }
        }
        computed.heads.retain(|_, n| *n > 0);
        computed.conjuncts.retain(|_, n| *n > 0);
        computed.embeddings.retain(|_, n| *n > 0);
        assert_eq!(computed.heads, found.heads, "heads: {name}");
        assert_eq!(computed.conjuncts, found.conjuncts, "conjuncts: {name}");
        assert_eq!(computed.embeddings, found.embeddings, "embeddings: {name}");
    }
    assert!(most > 1000, "{most}");
}
