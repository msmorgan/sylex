//! Reading Normal Form: identifies only the reviewed structural alternatives
//! as one analysis. It never endorses a Reading; it only lets two sets of
//! Readings be compared under exactly the approved canonicalizations.
//!
//! Every canonicalization is built structurally in the chart, so the parser
//! never builds the second tree. [`CompiledGrammar::normal_form`] re-applies a
//! listed subset of them to a `Reading`, so that a census taken before a
//! canonicalization existed can be compared tree for tree with one taken
//! after. The full set, with where each is built:
//!
//! - the grouping within a chain joined by the same coordinator, or linked by
//!   the same connective: the Coordination schemas' serial tails and the Linked
//!   Coordination's identity (`Summary::link`); re-applied;
//! - clause versus verb-phrase coordination of bare imperatives:
//!   `uncoordinated` children; re-applied;
//! - a bare coordination is of Noun Phrases, never of Nominals under one bare
//!   Noun Phrase: `uncoordinated` children;
//! - a dependent that scopes over a whole coordination where a single conjunct
//!   has a site of its own: `coordinated` children;
//! - an adverb before a bare imperative is its Clause's front Adjunct: the
//!   grammar data's `CentralPosition` and `FrontPosition` features;
//! - same-head complement priority: frame `Extension`s (`step.rs`) and a bare
//!   head's `omitted_complement`; re-applied for a Preposition Phrase;
//! - adjuncts below a non-modal auxiliary: `Production::auxiliary` and the
//!   `auxiliary` summary fact; re-applied;
//! - fronted dependent outermost: the `fronted` summary fact; re-applied;
//! - added element versus coordination, and frequency interval: `unadjoined`
//!   children;
//! - an existential's Extension after its Displaced Subject: `unextended`
//!   children;
//! - medial remnants: a Valence schema's `remnants` and a Coordination's
//!   `remnant`;
//! - single Objects in Complement Clusters: the `Bare` summary fact.

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use dpsi::Ident;
use sylex_lexicon::LexicalReading;

use crate::compile::CompiledGrammar;
use crate::frames::FrameSlot;
use crate::frames::SlotFill;
use crate::frames::SlotRelation;
use crate::grammar::HostKind;
use crate::reading::Reading;
use crate::reading::ReadingNode;

/// What Reading Normal Form reads of a bound grammar's Categories and
/// Productions, resolved at bind time ([`NormalForms::compile`]).
#[derive(Debug, Clone, Default)]
pub(crate) struct NormalForms {
    /// Each Category's base Category name, by Category: an engine-owned
    /// serial tail's is its coordination schema's Category, any other's its
    /// own reported name.
    base: Vec<Ident>,
    /// The id of each Category's flat coordination node, by Category:
    /// `Coordination:` and its base.
    flat: Vec<Ident>,
    /// Every flat coordination node's id.
    flat_ids: BTreeSet<Ident>,
    /// Each frame Production's longer-frame links, by Production.
    links: Vec<Option<FrameLink>>,
}

/// A frame Production's Valence Alternation variant, as same-head
/// complement priority re-applies it: its id is its phrase's frame name
/// (`frame_name`) followed by the variant.
#[derive(Debug, Clone)]
struct FrameLink {
    /// The variant is a passive with its Internalised Complement (`+by`).
    passive_by: bool,
    /// Each frame that extends the Production's by one last `PP`
    /// Complement slot naming no Complement kind, ascending, with the
    /// Production of that frame and the same variant, if there is one.
    longer: Vec<(usize, Option<usize>)>,
}

impl NormalForms {
    /// Resolves them once binding has settled the Categories' reported
    /// names; `tail_bases` maps each serial tail Category to its
    /// coordination schema's.
    pub(crate) fn compile(grammar: &CompiledGrammar, tail_bases: &BTreeMap<Ident, Ident>) -> Self {
        let base: Vec<Ident> = grammar
            .categories
            .iter()
            .map(|name| tail_bases.get(name).copied().unwrap_or(*name))
            .collect();
        let flat: Vec<Ident> = base
            .iter()
            .map(|base| Ident::new(&format!("Coordination:{base}")))
            .collect();
        // Every frame Production, by the phrase, frame and variant it was
        // built as.
        let built: BTreeMap<(Ident, usize, &str), usize> = grammar
            .productions
            .iter()
            .enumerate()
            .filter_map(|(index, production)| {
                let variant = production.frame_variant.as_ref()?;
                let frame = production.frame?;
                Some(((variant.phrase, frame, variant.variant.as_str()), index))
            })
            .collect();
        let links = grammar
            .productions
            .iter()
            .map(|production| {
                let frame_index = production.frame?;
                let variant = production
                    .frame_variant
                    .as_ref()
                    .expect("binding gives every frame Production its frame variant");
                // Same-head complement priority re-applies only to a phrase
                // reported under its Valence phrase's own name: not to an
                // Adjectival Passive, of another Category, nor to a phrase of
                // a Category reported under its parent's name.
                if grammar.categories[production.lhs] != variant.phrase {
                    return None;
                }
                let frame = &grammar.frames[frame_index];
                let longer = grammar
                    .frames
                    .iter()
                    .enumerate()
                    .filter(|(_, longer)| {
                        longer.slots.len() == frame.slots.len() + 1
                            && longer.slots[..frame.slots.len()] == frame.slots[..]
                            && matches!(
                                longer.slots.last(),
                                Some(FrameSlot {
                                    relation: SlotRelation::Complement,
                                    fill: SlotFill::PP(_, None),
                                })
                            )
                    })
                    .map(|(index, _)| {
                        let key = (variant.phrase, index, variant.variant.as_str());
                        (index, built.get(&key).copied())
                    })
                    .collect();
                Some(FrameLink {
                    passive_by: variant.alternation.by(),
                    longer,
                })
            })
            .collect();
        Self {
            flat_ids: flat.iter().copied().collect(),
            base,
            flat,
            links,
        }
    }
}

impl CompiledGrammar {
    /// The Reading Normal Form of `reading`: the canonicalizations the
    /// module doc marks re-applied, applied to an existing tree:
    ///
    /// - the grouping within a chain joined by the same coordinator: every
    ///   grouping, binary or serial, becomes one node listing the conjuncts and
    ///   then the coordinator, without features of its own;
    /// - clause versus verb-phrase coordination of bare imperatives: a
    ///   Production whose child is `uncoordinated` (where the coordination one
    ///   level up is the same analysis) over a coordination of that child
    ///   becomes the coordination of the Production over each conjunct;
    /// - adjuncts below a non-modal auxiliary: a post-verbal Adjunct of an
    ///   uncoordinated auxiliary Verb Phrase becomes an Adjunct of the lexical
    ///   participial Verb Phrase that is its Complement;
    /// - same-head complement priority: a Preposition Phrase attached as a
    ///   Predicate adjunct to a head that has a longer frame selecting it
    ///   becomes that frame's complement;
    /// - fronted dependent outermost: a comma-set post-dependent of a Clause
    ///   that begins with a fronted dependent becomes a post-dependent of that
    ///   Clause's head, *Pre[,] [Clause, Post]* for *[Pre[,] Clause], Post*.
    #[must_use]
    pub fn normal_form(&self, reading: &Reading) -> Reading {
        self.normalize(&self.lower_adjuncts(reading))
    }

    fn normalize(&self, reading: &Reading) -> Reading {
        let Reading::Node(node) = reading else {
            return reading.clone();
        };
        let children: Vec<Reading> = node.children.iter().map(|c| self.normalize(c)).collect();
        let production = self
            .by_id
            .get(&node.production)
            .map(|&p| &self.productions[p]);
        if let Some(p) = production
            && p.coordinator.is_some()
        {
            return self.flat_coordination(node, p.lhs, children);
        }
        // A Reading stores no punctuation: a Production position becomes a
        // child index by skipping the punctuation before it.
        let child = |p: &crate::step::Production, position: usize| {
            p.rhs[..position]
                .iter()
                .filter(|s| !matches!(s, crate::step::Symbol::Punct(..)))
                .count()
        };
        if let Some(p) = production
            && let Some(at) = p.distributed
            && let at = child(p, at)
            && let Some(Reading::Node(coordination)) = children.get(at)
            && self
                .normal_forms
                .flat_ids
                .contains(&coordination.production)
        {
            return self.distribute(node, &children, at, p.lhs);
        }
        if let Some(p) = production
            && p.hosts_comma_set_dependent()
            && let Some(outermost) = self.front_outermost(node, &children)
        {
            return outermost;
        }
        if let Some(p) = production
            && let Some((host, adjunct, HostKind::Predicate)) = p.host_check
            && adjunct > host
            && let Some(complement) =
                self.as_complement(&children[child(p, host)], &children[child(p, adjunct)])
        {
            return complement;
        }
        Reading::Node(ReadingNode {
            children,
            ..node.clone()
        })
    }

    /// `node`, the host of a comma-set post-dependent over normalized
    /// `children`, as the fronted dependents its first child begins with over
    /// `node` on that child's head, if it begins with any.
    fn front_outermost(&self, node: &ReadingNode, children: &[Reading]) -> Option<Reading> {
        let Some(Reading::Node(front)) = children.first() else {
            return None;
        };
        if !self.productions[*self.by_id.get(&front.production)?].fronts() {
            return None;
        }
        let head = front.children.last()?;
        // The host inherited from `front` each feature they share. Its new
        // head is `front`'s head, so it takes those from there instead and
        // drops one only `front`'s own Production valued (a front position).
        let Reading::Node(head_node) = head else {
            return None;
        };
        let features = node
            .features
            .iter()
            .filter_map(|(&feature, &value)| {
                if front.features.get(&feature) == Some(&value) {
                    head_node.features.get(&feature).map(|&v| (feature, v))
                } else {
                    Some((feature, value))
                }
            })
            .collect();
        let node = &ReadingNode {
            features,
            ..node.clone()
        };
        let mut hosted = children.to_vec();
        hosted[0] = head.clone();
        let inner = self.front_outermost(node, &hosted).unwrap_or_else(|| {
            Reading::Node(ReadingNode {
                children: hosted,
                ..node.clone()
            })
        });
        let mut outer = front.children.clone();
        *outer.last_mut()? = inner;
        Some(Reading::Node(ReadingNode {
            children: outer,
            ..front.clone()
        }))
    }

    /// `node`, a coordination of Category `category` over normalized
    /// `children`, as one flat node.
    fn flat_coordination(
        &self,
        node: &ReadingNode,
        category: usize,
        children: Vec<Reading>,
    ) -> Reading {
        let id = self.normal_forms.flat[category];
        let category = self.normal_forms.base[category];
        // The chain's coordinator: its own leaf, or the one its serial tail
        // carries.
        let own = children.iter().rev().find_map(|child| match child {
            Reading::Leaf(leaf) => Some(leaf.clone()),
            Reading::Node(inner) if inner.production == id => split(inner).1,
            Reading::Node(_) => None,
        });
        // A Linked Coordination has no coordinator leaf: its serial tail is
        // the one inner coordination without one.
        let same = |other: Option<&LexicalReading>| match (&own, other) {
            (Some(LexicalReading::Word(a)), Some(LexicalReading::Word(b))) => a.lexeme == b.lexeme,
            (None, None) => true,
            _ => false,
        };
        let mut conjuncts = Vec::new();
        for child in children {
            match child {
                Reading::Leaf(_) => {}
                Reading::Node(inner)
                    if inner.production == id && same(split(&inner).1.as_ref()) =>
                {
                    conjuncts.extend(split(&inner).0);
                }
                other @ Reading::Node(_) => conjuncts.push(other),
            }
        }
        conjuncts.extend(own.map(Reading::Leaf));
        // A coordination's features follow from its conjuncts and
        // coordinator, so the one node carries none of its own.
        Reading::Node(ReadingNode {
            production: id,
            category,
            children: conjuncts,
            selected_frame: None,
            features: std::collections::BTreeMap::new(),
            ..node.clone()
        })
    }

    /// `node` over the flat coordination at child `at`, as the coordination
    /// of `node` over each conjunct (the coordinator stays last).
    fn distribute(
        &self,
        node: &ReadingNode,
        children: &[Reading],
        at: usize,
        lhs: usize,
    ) -> Reading {
        let Reading::Node(coordination) = &children[at] else {
            unreachable!("a coordination child")
        };
        let mut conjuncts: Vec<Reading> = Vec::new();
        let mut coordinator = None;
        for conjunct in &coordination.children {
            match conjunct {
                Reading::Leaf(leaf) => coordinator = Some(Reading::Leaf(leaf.clone())),
                Reading::Node(inner) => {
                    let mut own = children.to_vec();
                    own[at] = conjunct.clone();
                    // The mother takes the features it shares with this
                    // conjunct from the conjunct, as it would from its head.
                    let mut features = node.features.clone();
                    for &feature in &self.category_features[lhs] {
                        let name = self.features[feature];
                        if let Some(value) = inner.features.get(&name) {
                            features.insert(name, *value);
                        }
                    }
                    conjuncts.push(self.normalize(&Reading::Node(ReadingNode {
                        children: own,
                        features,
                        ..node.clone()
                    })));
                }
            }
        }
        let flat = Reading::Node(ReadingNode {
            production: self.normal_forms.flat[lhs],
            category: self.normal_forms.base[lhs],
            children: conjuncts.into_iter().chain(coordinator).collect(),
            selected_frame: None,
            ..node.clone()
        });
        // A coordination of coordinations by one coordinator is one chain.
        let Reading::Node(flat_node) = &flat else { unreachable!() };
        self.flat_coordination(flat_node, lhs, flat_node.children.clone())
    }

    /// Adjuncts below a non-modal auxiliary, on the Reading as built: every
    /// Adjunct Production over an uncoordinated auxiliary Verb Phrase becomes
    /// the auxiliary over that Production of its participial Complement's
    /// Verb Phrase, which the Production takes its features from.
    fn lower_adjuncts(&self, reading: &Reading) -> Reading {
        let Reading::Node(node) = reading else {
            return reading.clone();
        };
        let children = node
            .children
            .iter()
            .map(|c| self.lower_adjuncts(c))
            .collect();
        self.lower(node, children)
    }

    /// `node` over its lowered `children`, lowered if it is an Adjunct
    /// Production over an auxiliary Verb Phrase.
    fn lower(&self, node: &ReadingNode, children: Vec<Reading>) -> Reading {
        let lowered = self
            .by_id
            .get(&node.production)
            .filter(|&&p| self.productions[p].post_adjunct_host(0))
            .and_then(|&p| self.below_auxiliary(node, self.productions[p].lhs, &children));
        lowered.unwrap_or_else(|| {
            Reading::Node(ReadingNode {
                children,
                ..node.clone()
            })
        })
    }

    /// `node`, an Adjunct Production over the auxiliary Verb Phrase that is
    /// its first child, as the auxiliary over that Production of its
    /// participial Complement's head Verb Phrase.
    fn below_auxiliary(
        &self,
        node: &ReadingNode,
        category: usize,
        children: &[Reading],
    ) -> Option<Reading> {
        let Some(Reading::Node(auxiliary)) = children.first() else {
            return None;
        };
        let p = &self.productions[*self.by_id.get(&auxiliary.production)?];
        p.auxiliary?;
        let Some(Reading::Node(complement)) = auxiliary.children.last() else {
            return None;
        };
        let c = &self.productions[*self.by_id.get(&complement.production)?];
        let head = c.rhs[..c.head?]
            .iter()
            .filter(|s| !matches!(s, crate::step::Symbol::Punct(..)))
            .count();
        let Some(Reading::Node(lexical)) = complement.children.get(head) else {
            return None;
        };
        let lexical_category = self.productions[*self.by_id.get(&lexical.production)?].lhs;
        if self.normal_forms.base[lexical_category] != self.normal_forms.base[category] {
            return None;
        }
        // The Production over the lexical Verb Phrase takes its features from
        // that head; the Gap passes through the auxiliary and its Complement
        // unchanged.
        let mut own = children.to_vec();
        own[0] = Reading::Node(lexical.clone());
        let lowered = self.lower(
            &ReadingNode {
                features: lexical.features.clone(),
                ..node.clone()
            },
            own,
        );
        let mut inner = complement.children.clone();
        inner[head] = lowered;
        let mut outer = auxiliary.children.clone();
        *outer.last_mut()? = Reading::Node(ReadingNode {
            children: inner,
            gap: node.gap,
            ..complement.clone()
        });
        Some(Reading::Node(ReadingNode {
            children: outer,
            gap: node.gap,
            ..auxiliary.clone()
        }))
    }

    /// The complement analysis equivalent to attaching `adjunct` to `host`,
    /// if `host`'s head has a frame selecting it.
    fn as_complement(&self, host: &Reading, adjunct: &Reading) -> Option<Reading> {
        let (Reading::Node(host), Reading::Node(phrase)) = (host, adjunct) else {
            return None;
        };
        let production = *self.by_id.get(&host.production)?;
        let link = self.normal_forms.links[production].as_ref()?;
        if host.selected_frame.as_ref()? != &self.frames[self.productions[production].frame?]
            || link.passive_by
        {
            return None;
        }
        let Some(Reading::Leaf(LexicalReading::Word(head))) = host.children.first() else {
            return None;
        };
        let Some(Reading::Leaf(LexicalReading::Word(preposition))) = phrase.children.first() else {
            return None;
        };
        let facts = self.facts(self.lexeme(&head.lexeme)?);
        for &(candidate, longer_production) in &link.longer {
            let longer = &self.frames[candidate];
            // Only a slot that names no Complement kind: a Reading does not
            // record which frame of its preposition a phrase has.
            if facts.frames.contains(&candidate)
                && let Some(FrameSlot {
                    fill: SlotFill::PP(list, None),
                    ..
                }) = longer.slots.last()
                && list.contains(&preposition.lexeme)
            {
                let mut children = host.children.clone();
                children.push(Reading::Node(phrase.clone()));
                return Some(Reading::Node(ReadingNode {
                    production: self.productions[longer_production?].id,
                    selected_frame: Some(longer.clone()),
                    children,
                    ..host.clone()
                }));
            }
        }
        None
    }
}

fn split(node: &ReadingNode) -> (Vec<Reading>, Option<LexicalReading>) {
    let mut conjuncts = node.children.clone();
    let coordinator = match conjuncts.last() {
        Some(Reading::Leaf(leaf)) => Some(leaf.clone()),
        _ => None,
    };
    if coordinator.is_some() {
        conjuncts.pop();
    }
    (conjuncts, coordinator)
}
