//! Membership between sorts and the two resolution modes (`syl.md` §2, §3).
//!
//! A `member(S): T` declaration makes every value of `S` a value of `T`,
//! written as itself; a `denotes(K): T` declaration lets a body write a `T`
//! where a phrase of kind `K` is expected and such a phrase where a `T` is.
//! [`Sorts::read`] reads both and refuses, at load, a cycle and a second path
//! between two sorts. [`ambiguities`] then refuses every word and literal
//! that stands at two sorts of one Reaches set, in either mode.

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use sylex_syl::Span;

use super::Decl;
use super::Refusal;
use super::RefusalKind;
use super::Registry;
use super::schema;
use super::sort::SortKey;

/// How a term is resolved (`syl.md` §3).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Mode {
    /// A card's term: a value stands at the expected sort or at a sort that
    /// reaches it through membership.
    Surface,
    /// A body or a parameter's default: membership, and a test where a
    /// describing phrase is expected or such a phrase where a test is.
    Body,
}

impl Mode {
    pub(super) fn name(self) -> &'static str {
        match self {
            Mode::Surface => "surface",
            Mode::Body => "body",
        }
    }
}

/// The memberships and the kinds that denote a test.
#[derive(Debug, Default)]
pub(crate) struct Sorts {
    /// Each sort's direct members, in declaration order.
    members: BTreeMap<SortKey, Vec<SortKey>>,
    /// Each `denotes(kind): test`, as `(kind, test)`.
    denotes: Vec<(SortKey, SortKey)>,
    /// Where each relation is declared, for refusals.
    spans: Vec<(SortKey, SortKey, Span)>,
}

/// Whether a kind of literal stands at a sort.
type StandsAt = fn(&SortKey) -> bool;

/// Whether a string literal stands at `sort`.
pub(super) fn string_stands_at(sort: &SortKey) -> bool {
    *sort == SortKey::named("String")
}

/// Whether the integer literal `number` stands at `sort`.
pub(super) fn integer_stands_at(number: i64, sort: &SortKey) -> bool {
    *sort == SortKey::named("Int") || (*sort == SortKey::named("Nat") && number >= 0)
}

/// Whether a list literal stands at `sort`.
pub(super) fn list_stands_at(sort: &SortKey) -> bool {
    sort.list_inner().is_some()
}

/// Whether a record literal stands at `sort`.
pub(super) fn record_stands_at(sort: &SortKey) -> bool {
    sort.args.is_empty()
        && schema::RECORDS
            .iter()
            .any(|record| record.sort == sort.head)
}

impl Sorts {
    /// Reads every `member` and `denotes` declaration, refusing a relation of
    /// a sort to itself, a duplicate, a cycle and a second path.
    pub(super) fn read(decls: &[Decl<'_>], refusals: &mut Vec<Refusal>) -> Sorts {
        let mut sorts = Sorts::default();
        for decl in decls {
            let Some(subject) = &decl.subject else {
                continue;
            };
            let span = decl.declaration.span;
            let word = decl.kind.word;
            if *subject == decl.sort {
                refusals.push(Refusal {
                    span,
                    kind: RefusalKind::RelationToItself {
                        kind: word,
                        sort: subject.to_string(),
                    },
                });
                continue;
            }
            let duplicate = match word {
                schema::MEMBER => sorts
                    .members
                    .get(&decl.sort)
                    .is_some_and(|members| members.contains(subject)),
                schema::DENOTES => sorts.denotes.iter().any(|(kind, _)| kind == subject),
                other => unreachable!("`{other}` relates no sorts"),
            };
            if duplicate {
                refusals.push(Refusal {
                    span,
                    kind: RefusalKind::DuplicateRelation {
                        kind: word,
                        subject: subject.to_string(),
                    },
                });
                continue;
            }
            match word {
                schema::MEMBER => sorts
                    .members
                    .entry(decl.sort.clone())
                    .or_default()
                    .push(subject.clone()),
                schema::DENOTES => sorts.denotes.push((subject.clone(), decl.sort.clone())),
                other => unreachable!("`{other}` relates no sorts"),
            }
            sorts.spans.push((subject.clone(), decl.sort.clone(), span));
        }
        sorts.refuse_cycles_and_second_paths(refusals);
        sorts
    }

    fn span_of(&self, subject: &SortKey, sort: &SortKey) -> Span {
        self.spans
            .iter()
            .find(|(s, t, _)| s == subject && t == sort)
            .map(|(_, _, span)| *span)
            .expect("every relation read has a span")
    }

    /// Refuses a membership that closes a cycle, and, under each sort, a
    /// member reached by two paths. On a cycle the offending membership is
    /// dropped, so every later walk ends.
    fn refuse_cycles_and_second_paths(&mut self, refusals: &mut Vec<Refusal>) {
        // A membership `member(S): T` closes a cycle when `T` is already
        // among what `S` reaches.
        let spans = self.spans.clone();
        let mut kept: BTreeMap<SortKey, Vec<SortKey>> = BTreeMap::new();
        for (subject, sort, span) in &spans {
            if !self
                .members
                .get(sort)
                .is_some_and(|members| members.contains(subject))
            {
                continue;
            }
            let mut below = BTreeSet::new();
            descend(&kept, subject, &mut below);
            if below.contains(sort) {
                refusals.push(Refusal {
                    span: *span,
                    kind: RefusalKind::MembershipCycle {
                        member: subject.to_string(),
                        of: sort.to_string(),
                    },
                });
                continue;
            }
            kept.entry(sort.clone()).or_default().push(subject.clone());
        }
        self.members = kept;
        for root in self.members.keys() {
            let mut seen: BTreeMap<SortKey, SortKey> = BTreeMap::new();
            let mut stack: Vec<(SortKey, SortKey)> = self.members[root]
                .iter()
                .map(|member| (member.clone(), root.clone()))
                .collect();
            while let Some((sort, via)) = stack.pop() {
                if let Some(first) = seen.get(&sort) {
                    refusals.push(Refusal {
                        span: self.span_of(&sort, &via),
                        kind: RefusalKind::SecondPath {
                            member: sort.to_string(),
                            of: root.to_string(),
                            first: first.to_string(),
                            second: via.to_string(),
                        },
                    });
                    continue;
                }
                seen.insert(sort.clone(), via);
                for member in self.members.get(&sort).into_iter().flatten() {
                    stack.push((member.clone(), sort.clone()));
                }
            }
        }
    }

    /// The test sort a phrase of kind `sort` denotes.
    fn denoted(&self, sort: &SortKey) -> Option<&SortKey> {
        self.denotes
            .iter()
            .find(|(kind, _)| kind == sort)
            .map(|(_, test)| test)
    }

    /// Whether `sort` is a test sort: what some kind denotes.
    pub(super) fn is_test(&self, sort: &SortKey) -> bool {
        self.denotes.iter().any(|(_, test)| test == sort)
    }

    /// Whether `sort` is a position sort: no kind of phrase itself, with a
    /// member that is a kind of describing phrase (`surface.md` §1).
    pub(super) fn is_position(&self, sort: &SortKey) -> bool {
        self.denoted(sort).is_none()
            && self
                .members
                .get(sort)
                .is_some_and(|members| members.iter().any(|m| self.denoted(m).is_some()))
    }

    /// The test sorts a body may write where `sort` is expected: what `sort`
    /// denotes, or for a position sort what its member kinds denote.
    fn tests_at(&self, sort: &SortKey) -> Vec<&SortKey> {
        if let Some(test) = self.denoted(sort) {
            return vec![test];
        }
        let mut tests: Vec<&SortKey> = self
            .members
            .get(sort)
            .into_iter()
            .flatten()
            .filter_map(|member| self.denoted(member))
            .collect();
        tests.sort();
        tests.dedup();
        tests
    }

    /// The sorts directly under `sort` in the tree of Reaches(`root`): its
    /// members, and in body mode under the root itself the tests a body may
    /// write there, or for a test sort the kinds that denote it.
    fn children(&self, sort: &SortKey, root: &SortKey, mode: Mode) -> Vec<&SortKey> {
        let mut children: Vec<&SortKey> = self.members.get(sort).into_iter().flatten().collect();
        if mode == Mode::Body && sort == root {
            children.extend(self.tests_at(sort));
            children.extend(
                self.denotes
                    .iter()
                    .filter(|(_, test)| test == sort)
                    .map(|(kind, _)| kind),
            );
        }
        children
    }

    /// Reaches(`root`) in `mode`, the root first (`syl.md` §3).
    pub(super) fn reaches(&self, root: &SortKey, mode: Mode) -> Vec<SortKey> {
        let mut out = Vec::new();
        let mut stack = vec![root];
        while let Some(sort) = stack.pop() {
            out.push(sort.clone());
            stack.extend(self.children(sort, root, mode));
        }
        out
    }

    /// The path from `root` down to `to` in the tree of Reaches(`root`), both
    /// ends included; `None` when `to` is not in it.
    pub(super) fn path(&self, root: &SortKey, to: &SortKey, mode: Mode) -> Option<Vec<SortKey>> {
        let mut path = vec![root.clone()];
        self.descend_to(root, root, to, mode, &mut path)
            .then_some(path)
    }

    fn descend_to(
        &self,
        at: &SortKey,
        root: &SortKey,
        to: &SortKey,
        mode: Mode,
        path: &mut Vec<SortKey>,
    ) -> bool {
        if at == to {
            return true;
        }
        for child in self.children(at, root, mode) {
            path.push(child.clone());
            if self.descend_to(child, root, to, mode, path) {
                return true;
            }
            path.pop();
        }
        false
    }

    /// Every sort a value may be expected at whose Reaches holds another
    /// sort, in some mode.
    fn roots(&self) -> BTreeSet<&SortKey> {
        self.spans
            .iter()
            .flat_map(|(subject, target, _)| [subject, target])
            .collect()
    }
}

fn descend(members: &BTreeMap<SortKey, Vec<SortKey>>, sort: &SortKey, out: &mut BTreeSet<SortKey>) {
    if !out.insert(sort.clone()) {
        return;
    }
    for member in members.get(sort).into_iter().flatten() {
        descend(members, member, out);
    }
}

/// Refuses every word two declarations declare at two sorts of one Reaches
/// set, in either mode; every word declared at `Option(T)` and at a sort
/// that reaches `T` (at an `Option(T)` position the first would shadow the
/// second); and every kind of literal that stands at two sorts of one Reaches
/// set. One declaration with an `at` list is one declaration at all its
/// sorts.
pub(super) fn ambiguities(sorts: &Sorts, registry: &Registry, refusals: &mut Vec<Refusal>) {
    let mut reported: BTreeSet<(String, Vec<String>)> = BTreeSet::new();
    for root in sorts.roots() {
        for mode in [Mode::Surface, Mode::Body] {
            let reaches = sorts.reaches(root, mode);
            if reaches.len() < 2 {
                continue;
            }
            let refs: Vec<&SortKey> = reaches.iter().collect();
            words(&refs, root, mode, None, registry, &mut reported, refusals);
            let optional = SortKey {
                head: "Option".to_owned(),
                args: vec![root.clone()],
            };
            let shadowing: Vec<&SortKey> = std::iter::once(&optional)
                .chain(reaches.iter().skip(1))
                .collect();
            words(
                &shadowing,
                &optional,
                mode,
                Some(&optional),
                registry,
                &mut reported,
                refusals,
            );
            // Only `String` takes a string literal, so no string reaches a
            // sort from two sorts.
            let literals: [(&str, StandsAt); 3] = [
                ("an integer", |sort| integer_stands_at(0, sort)),
                ("a list", list_stands_at),
                ("a record", record_stands_at),
            ];
            for (literal, stands_at) in literals {
                let mut standing: Vec<String> = reaches
                    .iter()
                    .filter(|sort| stands_at(sort))
                    .map(ToString::to_string)
                    .collect();
                standing.sort();
                if standing.len() < 2 || !reported.insert((literal.to_owned(), standing.clone())) {
                    continue;
                }
                // The latest relation that brings a sort into this Reaches set.
                let span = sorts
                    .spans
                    .iter()
                    .filter(|(subject, _, _)| reaches.contains(subject))
                    .map(|(_, _, span)| *span)
                    .max_by_key(|span| (span.file, span.start))
                    .expect("two sorts reach the root, so a relation is declared");
                refusals.push(Refusal {
                    span,
                    kind: RefusalKind::AmbiguousLiteral {
                        literal: literal.to_owned(),
                        sorts: standing,
                        target: root.to_string(),
                    },
                });
            }
        }
    }
}

/// Refuses every word two declarations declare at two of `sorts`, all of
/// which reach `target` in `mode`; with `including`, only a word one of whose
/// sorts it is.
fn words(
    sorts: &[&SortKey],
    target: &SortKey,
    mode: Mode,
    including: Option<&SortKey>,
    registry: &Registry,
    reported: &mut BTreeSet<(String, Vec<String>)>,
    refusals: &mut Vec<Refusal>,
) {
    let mut words: BTreeMap<&str, Vec<(&SortKey, usize, Span)>> = BTreeMap::new();
    for ((name, sort), signature) in registry {
        if sorts.contains(&sort) {
            words
                .entry(name.as_str())
                .or_default()
                .push((sort, signature.decl, signature.span));
        }
    }
    for (word, declared) in words {
        let included = including.is_none_or(|sort| declared.iter().any(|(s, _, _)| *s == sort));
        let declarations: BTreeSet<usize> = declared.iter().map(|(_, decl, _)| *decl).collect();
        if declarations.len() < 2 || !included {
            continue;
        }
        let standing: Vec<String> = declared
            .iter()
            .map(|(sort, _, _)| sort.to_string())
            .collect();
        // One pair is reported once, at the first Reaches set that joins it.
        if !reported.insert((word.to_owned(), standing.clone())) {
            continue;
        }
        let span = declared
            .iter()
            .map(|(_, _, span)| *span)
            .max_by_key(|span| (span.file, span.start))
            .expect("two declarations");
        refusals.push(Refusal {
            span,
            kind: RefusalKind::AmbiguousWord {
                word: word.to_owned(),
                sorts: standing,
                target: target.to_string(),
                mode: mode.name(),
            },
        });
    }
}
