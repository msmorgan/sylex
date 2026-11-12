//! Brute-force oracles for [`sylex_syntax::Counts`], over the public forest
//! API only: each count against the unit's total minus a recount without that
//! node, item or edge, and against an enumeration of the forest's
//! Derivations. Shared by the syntax crate's hand grammars and the English
//! crate's real units (`#[path]`).

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;

use sylex_syntax::ChildRef;
use sylex_syntax::Counts;
use sylex_syntax::Parse;

/// A use of the forest in a Derivation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Use {
    Node(usize),
    Item(usize),
    /// A node's `index`th (Production, item) alternative.
    NodeFamily(usize, usize),
    /// An item's `index`th (previous item, child) family.
    ItemFamily(usize, usize),
}

/// Every node, item and edge the roots reach.
pub fn reachable(parse: &Parse<'_>) -> Vec<Use> {
    let forest = &parse.forest;
    let mut seen = BTreeSet::new();
    let mut stack: Vec<Use> = forest.roots().iter().map(|&r| Use::Node(r)).collect();
    while let Some(vertex) = stack.pop() {
        if !seen.insert(vertex) {
            continue;
        }
        match vertex {
            Use::Node(n) => {
                for (k, item) in forest.node_items(n).enumerate() {
                    seen.insert(Use::NodeFamily(n, k));
                    stack.push(Use::Item(item));
                }
            }
            Use::Item(i) => {
                for (k, (previous, child)) in forest.item_families(i).enumerate() {
                    seen.insert(Use::ItemFamily(i, k));
                    stack.extend(previous.map(Use::Item));
                    if let ChildRef::Node(c) = child {
                        stack.push(Use::Node(c));
                    }
                }
            }
            Use::NodeFamily(..) | Use::ItemFamily(..) => unreachable!("families are not stacked"),
        }
    }
    seen.into_iter().collect()
}

/// The count [`Counts`] gives a use.
pub fn counted(counts: &Counts<'_>, use_: Use) -> u128 {
    match use_ {
        Use::Node(n) => counts.node(n),
        Use::Item(i) => counts.item(i),
        Use::NodeFamily(n, k) => counts.node_family(n, k),
        Use::ItemFamily(i, k) => counts.item_family(i, k),
    }
}

/// The unit's Derivations with one use removed from the forest.
pub fn recount_without(parse: &Parse<'_>, removed: Use) -> u128 {
    struct Recount<'a, 'p> {
        parse: &'a Parse<'p>,
        removed: Use,
        nodes: HashMap<usize, u128>,
        items: HashMap<usize, u128>,
    }
    impl Recount<'_, '_> {
        fn node(&mut self, n: usize) -> u128 {
            if self.removed == Use::Node(n) {
                return 0;
            }
            if let Some(&c) = self.nodes.get(&n) {
                return c;
            }
            let items: Vec<usize> = self.parse.forest.node_items(n).collect();
            let mut total = 0;
            for (k, item) in items.into_iter().enumerate() {
                if self.removed != Use::NodeFamily(n, k) {
                    total += self.item(item);
                }
            }
            self.nodes.insert(n, total);
            total
        }
        fn item(&mut self, i: usize) -> u128 {
            if self.removed == Use::Item(i) {
                return 0;
            }
            if let Some(&c) = self.items.get(&i) {
                return c;
            }
            let families: Vec<_> = self.parse.forest.item_families(i).collect();
            let mut total = 0;
            for (k, (previous, child)) in families.into_iter().enumerate() {
                if self.removed == Use::ItemFamily(i, k) {
                    continue;
                }
                let before = previous.map_or(1, |p| self.item(p));
                let here = match child {
                    ChildRef::Node(c) => self.node(c),
                    ChildRef::Leaf(_) | ChildRef::Punct => 1,
                };
                total += before * here;
            }
            self.items.insert(i, total);
            total
        }
    }
    let mut recount = Recount {
        parse,
        removed,
        nodes: HashMap::new(),
        items: HashMap::new(),
    };
    parse.forest.roots().iter().map(|&r| recount.node(r)).sum()
}

/// Every Derivation of the forest as the list of its uses, in no order.
pub fn derivations(parse: &Parse<'_>) -> Vec<Vec<Use>> {
    fn node(parse: &Parse<'_>, n: usize, memo: &mut Memo) -> Vec<Vec<Use>> {
        if let Some(d) = memo.nodes.get(&n) {
            return d.clone();
        }
        let mut out = Vec::new();
        let items: Vec<usize> = parse.forest.node_items(n).collect();
        for (k, i) in items.into_iter().enumerate() {
            for mut d in item(parse, i, memo) {
                d.push(Use::Node(n));
                d.push(Use::NodeFamily(n, k));
                out.push(d);
            }
        }
        memo.nodes.insert(n, out.clone());
        out
    }
    fn item(parse: &Parse<'_>, i: usize, memo: &mut Memo) -> Vec<Vec<Use>> {
        if let Some(d) = memo.items.get(&i) {
            return d.clone();
        }
        let mut out = Vec::new();
        let families: Vec<_> = parse.forest.item_families(i).collect();
        for (k, (previous, child)) in families.into_iter().enumerate() {
            let befores = previous.map_or_else(|| vec![Vec::new()], |p| item(parse, p, memo));
            let heres = match child {
                ChildRef::Node(c) => node(parse, c, memo),
                ChildRef::Leaf(_) | ChildRef::Punct => vec![Vec::new()],
            };
            for before in &befores {
                for here in &heres {
                    let mut d = before.clone();
                    d.extend_from_slice(here);
                    d.push(Use::Item(i));
                    d.push(Use::ItemFamily(i, k));
                    out.push(d);
                }
            }
        }
        memo.items.insert(i, out.clone());
        out
    }
    #[derive(Default)]
    struct Memo {
        nodes: HashMap<usize, Vec<Vec<Use>>>,
        items: HashMap<usize, Vec<Vec<Use>>>,
    }
    let mut memo = Memo::default();
    parse
        .forest
        .roots()
        .iter()
        .flat_map(|&r| node(parse, r, &mut memo))
        .collect()
}

/// Checks every count of a unit's forest against the recount without it,
/// and, when the unit has at most `enumerate` Derivations, against the
/// enumerated Derivations, each of which must use every node, item and edge
/// at most once. Returns the first disagreement.
pub fn check(parse: &Parse<'_>, enumerate: u128) -> Result<(), String> {
    let counts = parse.counts().map_err(|e| e.to_string())?;
    let total = counts.total();
    if total != parse.derivation_count() {
        return Err(format!(
            "total {total}, derivation_count {}",
            parse.derivation_count()
        ));
    }
    let uses = reachable(parse);
    for &use_ in &uses {
        let containing = total - recount_without(parse, use_);
        if counted(&counts, use_) != containing {
            return Err(format!(
                "{use_:?}: counted {}, in {containing} Derivations by recount",
                counted(&counts, use_)
            ));
        }
    }
    if total <= enumerate {
        let all = derivations(parse);
        if all.len() as u128 != total {
            return Err(format!("{} enumerated, total {total}", all.len()));
        }
        let mut containing: BTreeMap<Use, u128> = BTreeMap::new();
        for derivation in &all {
            let distinct: BTreeSet<Use> = derivation.iter().copied().collect();
            if distinct.len() != derivation.len() {
                return Err(format!("a Derivation uses something twice: {derivation:?}"));
            }
            for use_ in distinct {
                *containing.entry(use_).or_default() += 1;
            }
        }
        for &use_ in &uses {
            let enumerated = containing.get(&use_).copied().unwrap_or(0);
            if counted(&counts, use_) != enumerated {
                return Err(format!(
                    "{use_:?}: counted {}, in {enumerated} enumerated Derivations",
                    counted(&counts, use_)
                ));
            }
        }
    }
    Ok(())
}
