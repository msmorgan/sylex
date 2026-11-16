//! Load rules on a declaration's `spelling` (`docs/design/spelling.md` §7).
//!
//! [`sylex_spelling::parse`] checks the notation's syntax alone. [`check`]
//! refuses a parsed [`Spelling`] when it breaks a load rule that its
//! declaration's parameters, the declared coercions, or the lexicon's bound
//! forms settle.

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::VecDeque;
use std::sync::LazyLock;

use sylex_lexicon::Binding;
use sylex_lexicon::Category;
use sylex_spelling::Hole;
use sylex_spelling::HoleKind;
use sylex_spelling::OwnName;
use sylex_spelling::Part;
use sylex_spelling::Spelling;
use sylex_spelling::Tail;
use sylex_spelling::TailPlacement;
use sylex_syl::Value;
use sylex_syl::ValueKind;

use super::Decl;
use super::Refusal;
use super::RefusalKind;
use super::Slot;
use super::coercion::Coercion;
use super::sort::SortKey;

/// The bound prefix and suffix forms the lexicon declares (`spelling.md` §7).
struct BoundAffixes {
    prefixes: BTreeSet<String>,
    suffixes: BTreeSet<String>,
}

static BOUND_AFFIXES: LazyLock<BoundAffixes> = LazyLock::new(|| {
    let (lexemes, _) = sylex_vocabulary::Sources::load()
        .expect("authored vocabulary RON loads")
        .vocabulary()
        .expect("authored vocabulary builds");
    let mut prefixes = BTreeSet::new();
    let mut suffixes = BTreeSet::new();
    for lexeme in &lexemes {
        let is_prefix = lexeme.category == Category::Affix
            && matches!(lexeme.binding, Binding::Prefix | Binding::Bound);
        let is_suffix = matches!(lexeme.category, Category::Affix | Category::Keyword)
            && matches!(lexeme.binding, Binding::Suffix | Binding::Bound);
        if !is_prefix && !is_suffix {
            continue;
        }
        for form in &lexeme.forms {
            let surface = form.surface.as_deref().unwrap_or(&lexeme.lemma);
            if !surface.is_empty() && surface.chars().all(char::is_alphabetic) {
                if is_prefix {
                    prefixes.insert(surface.to_owned());
                }
                if is_suffix {
                    suffixes.insert(surface.to_owned());
                }
            }
        }
    }
    BoundAffixes { prefixes, suffixes }
});

/// Checks `value` as `decl`'s `spelling` field (`spelling.md` §7).
pub(super) fn check(
    decl: &Decl<'_>,
    value: &Value,
    coercions: &[Coercion],
    refusals: &mut Vec<Refusal>,
) {
    let span = value.span;
    let ValueKind::String(text) = &value.kind else {
        refusals.push(Refusal {
            span,
            kind: RefusalKind::SpellingNotString,
        });
        return;
    };
    let parsed = match sylex_spelling::parse(text) {
        Ok(parsed) => parsed,
        Err(error) => {
            refusals.push(Refusal {
                span,
                kind: RefusalKind::InvalidSpelling(error.kind),
            });
            return;
        }
    };

    let mut holes: Vec<(&Hole, Option<usize>)> = Vec::new();
    let mut optionals: Vec<bool> = Vec::new();
    if let Spelling::Parts { parts, tail, .. } = &parsed {
        collect_parts(parts, tail.as_ref(), None, &mut holes, &mut optionals);
        let has_certain = tail.is_some() || parts.iter().any(part_has_certain_material);
        if !has_certain {
            refusals.push(Refusal {
                span,
                kind: RefusalKind::SpellingNothingCertain,
            });
        }
    }

    let mut valid_for_skeleton = true;
    let mut seen_params: BTreeMap<&str, BTreeSet<Option<usize>>> = BTreeMap::new();
    let affixes = &*BOUND_AFFIXES;

    for &(hole, enclosing_opt) in &holes {
        let name = hole.name.as_str();
        let Some(param) = decl.params.iter().find(|p| p.name == name) else {
            refusals.push(Refusal {
                span,
                kind: RefusalKind::SpellingUnknownHole(name.to_owned()),
            });
            valid_for_skeleton = false;
            continue;
        };

        let is_list = is_list_or_option_list(&param.sort);
        match &hole.kind {
            HoleKind::List { .. } => {
                if !is_list {
                    refusals.push(Refusal {
                        span,
                        kind: RefusalKind::SpellingJoinNotList {
                            name: name.to_owned(),
                            sort: param.sort.to_string(),
                        },
                    });
                    valid_for_skeleton = false;
                }
            }
            HoleKind::Argument { prefix, suffix, .. } => {
                if is_list {
                    refusals.push(Refusal {
                        span,
                        kind: RefusalKind::SpellingListWithoutJoin {
                            name: name.to_owned(),
                            sort: param.sort.to_string(),
                        },
                    });
                    valid_for_skeleton = false;
                }
                if let Some(prefix) = prefix
                    && !affixes.prefixes.contains(prefix.text.as_str())
                {
                    refusals.push(Refusal {
                        span,
                        kind: RefusalKind::SpellingUnknownAffix {
                            affix: prefix.text.clone(),
                            side: "prefix",
                        },
                    });
                }
                if let Some(suffix) = suffix
                    && !affixes.suffixes.contains(suffix.text.as_str())
                {
                    refusals.push(Refusal {
                        span,
                        kind: RefusalKind::SpellingUnknownAffix {
                            affix: suffix.text.clone(),
                            side: "suffix",
                        },
                    });
                }
            }
        }

        if enclosing_opt.is_some() && !is_omissible(param) {
            refusals.push(Refusal {
                span,
                kind: RefusalKind::SpellingOptionalInomissible {
                    name: name.to_owned(),
                    sort: param.sort.to_string(),
                },
            });
        }
        seen_params.entry(name).or_default().insert(enclosing_opt);
    }

    for (name, scopes) in &seen_params {
        if scopes.len() > 1 {
            refusals.push(Refusal {
                span,
                kind: RefusalKind::SpellingSplitParameter((*name).to_owned()),
            });
        }
    }

    if optionals.contains(&false) {
        refusals.push(Refusal {
            span,
            kind: RefusalKind::SpellingOptionalWithoutHole,
        });
        valid_for_skeleton = false;
    }

    for param in &decl.params {
        if !seen_params.contains_key(param.name.as_str()) && !is_omissible(param) {
            refusals.push(Refusal {
                span,
                kind: RefusalKind::SpellingUnprintedParameter {
                    name: param.name.clone(),
                    sort: param.sort.to_string(),
                },
            });
        }
    }

    if valid_for_skeleton && skeleton_is_ambiguous(&parsed, &decl.params, coercions) {
        refusals.push(Refusal {
            span,
            kind: RefusalKind::SpellingAmbiguousSkeleton,
        });
    }
}

fn is_omissible(param: &Slot) -> bool {
    param.optional || param.sort.option_inner().is_some() || param.sort.list_inner().is_some()
}

fn is_list_or_option_list(sort: &SortKey) -> bool {
    sort.option_inner().unwrap_or(sort).list_inner().is_some()
}

fn part_has_certain_material(part: &Part) -> bool {
    match part {
        Part::Hole(_) => true,
        Part::Text(text) => text.text.chars().any(|c| !c.is_whitespace()),
        Part::Optional(_) => false,
    }
}

/// Walks `parts` and `tail` under `enclosing_opt`, recording every [`Hole`]
/// with its directly enclosing optional part (`None` outside every optional
/// part) and whether each optional part directly holds at least one [`Hole`].
fn collect_parts<'a>(
    parts: &'a [Part],
    tail: Option<&'a Tail>,
    enclosing_opt: Option<usize>,
    holes: &mut Vec<(&'a Hole, Option<usize>)>,
    optionals: &mut Vec<bool>,
) {
    let mut direct_hole = false;
    for part in parts {
        match part {
            Part::Text(_) => {}
            Part::Hole(hole) => {
                direct_hole = true;
                holes.push((hole, enclosing_opt));
            }
            Part::Optional(opt) => {
                let opt_id = optionals.len();
                optionals.push(false);
                collect_parts(
                    &opt.parts,
                    opt.tail.as_ref(),
                    Some(opt_id),
                    holes,
                    optionals,
                );
            }
        }
    }
    if let Some(tail) = tail {
        for part in &tail.parts {
            match part {
                Part::Text(_) => {}
                Part::Hole(hole) => {
                    direct_hole = true;
                    holes.push((hole, enclosing_opt));
                }
                Part::Optional(opt) => {
                    let opt_id = optionals.len();
                    optionals.push(false);
                    collect_parts(
                        &opt.parts,
                        opt.tail.as_ref(),
                        Some(opt_id),
                        holes,
                        optionals,
                    );
                }
            }
        }
    }
    if let Some(opt_id) = enclosing_opt {
        optionals[opt_id] = direct_hole;
    }
}

/// An atom of a `spelling`'s skeleton (`spelling.md` §7).
#[derive(Debug, Clone, PartialEq, Eq)]
enum Atom {
    /// One lowercase word, symbol, or newline token.
    Word(String),
    /// A Hole's stripped parameter sort (or list element sort).
    Sort(SortKey),
    /// Opens a Tail at its host Category and placement.
    TailOpen(TailPlacement, String),
    /// Closes a Tail at the end of its optional part or `spelling`.
    TailClose,
    /// `^`: the whole spelling of a Gap's word.
    Gap,
    /// `#` or `##`: the whole spelling of an own-name word.
    OwnName(OwnName),
}

#[derive(Debug, Clone)]
enum Regex {
    Seq(Vec<Regex>),
    Opt(Box<Regex>),
    /// `elem (join elem)*`: one or more elements separated by `join`.
    Plus {
        elem: Box<Regex>,
        join: Box<Regex>,
    },
    Atom(Atom),
}

fn skeleton_is_ambiguous(spelling: &Spelling, params: &[Slot], coercions: &[Coercion]) -> bool {
    let regex = match spelling {
        Spelling::Gap(_) => Regex::Atom(Atom::Gap),
        Spelling::OwnName(form, _) => Regex::Atom(Atom::OwnName(*form)),
        Spelling::Parts { parts, tail, .. } => {
            let mut seq = build_parts_regex(parts, params);
            if let Some(tail) = tail {
                seq.push(Regex::Atom(Atom::TailOpen(
                    tail.placement,
                    tail.category.clone(),
                )));
                seq.extend(build_parts_regex(&tail.parts, params));
                seq.push(Regex::Atom(Atom::TailClose));
            }
            Regex::Seq(seq)
        }
    };
    is_regex_ambiguous(&regex, coercions)
}

fn build_parts_regex(parts: &[Part], params: &[Slot]) -> Vec<Regex> {
    let mut seq = Vec::new();
    for part in parts {
        match part {
            Part::Text(text) => {
                seq.extend(
                    words_of_text(&text.text)
                        .into_iter()
                        .map(|w| Regex::Atom(Atom::Word(w))),
                );
            }
            Part::Hole(hole) => {
                let param = params
                    .iter()
                    .find(|p| p.name == hole.name)
                    .expect("holes resolved before skeleton check");
                match &hole.kind {
                    HoleKind::Argument { prefix, suffix, .. } => {
                        if let Some(prefix) = prefix {
                            seq.push(Regex::Atom(Atom::Word(prefix.text.to_lowercase())));
                        }
                        seq.push(Regex::Atom(Atom::Sort(strip_options(&param.sort).clone())));
                        if let Some(suffix) = suffix {
                            seq.push(Regex::Atom(Atom::Word(suffix.text.to_lowercase())));
                        }
                    }
                    HoleKind::List { join, .. } => {
                        let list_sort = strip_options(&param.sort);
                        let elem_sort = strip_options(
                            list_sort
                                .list_inner()
                                .expect("list Hole has a List parameter"),
                        );
                        let join_atoms: Vec<Regex> = words_of_text(join.as_str())
                            .into_iter()
                            .map(|w| Regex::Atom(Atom::Word(w)))
                            .collect();
                        seq.push(Regex::Plus {
                            elem: Box::new(Regex::Atom(Atom::Sort(elem_sort.clone()))),
                            join: Box::new(Regex::Seq(join_atoms)),
                        });
                    }
                }
            }
            Part::Optional(opt) => {
                let mut inner = build_parts_regex(&opt.parts, params);
                if let Some(tail) = &opt.tail {
                    inner.push(Regex::Atom(Atom::TailOpen(
                        tail.placement,
                        tail.category.clone(),
                    )));
                    inner.extend(build_parts_regex(&tail.parts, params));
                    inner.push(Regex::Atom(Atom::TailClose));
                }
                seq.push(Regex::Opt(Box::new(Regex::Seq(inner))));
            }
        }
    }
    seq
}

fn strip_options(mut sort: &SortKey) -> &SortKey {
    while let Some(inner) = sort.option_inner() {
        sort = inner;
    }
    sort
}

/// Tokenizes fixed text into lowercase word tokens, preserving line breaks as
/// their own token (`"\n"`) and dropping punctuation that a Reading derives
/// (`spelling.md` §6: `.`, `,`, `:`, `;`, `!`, `—`).
fn words_of_text(text: &str) -> Vec<String> {
    let normalized = text.replace('’', "'");
    let mut words = Vec::new();
    for (line_idx, line) in normalized.split('\n').enumerate() {
        if line_idx > 0 {
            words.push("\n".to_owned());
        }
        for raw in line.split_whitespace() {
            let trimmed = raw.trim_matches(['.', ',', ':', ';', '!', '—']);
            if !trimmed.is_empty() {
                words.push(trimmed.to_lowercase());
            }
        }
    }
    words
}

fn atoms_overlap(left: &Atom, right: &Atom, coercions: &[Coercion]) -> bool {
    match left {
        Atom::Word(a) => matches!(right, Atom::Word(b) if a == b),
        Atom::Sort(a) => matches!(right, Atom::Sort(b) if sorts_overlap(a, b, coercions)),
        Atom::TailOpen(pa, ca) => matches!(right, Atom::TailOpen(pb, cb) if pa == pb && ca == cb),
        Atom::TailClose => matches!(right, Atom::TailClose),
        Atom::Gap => matches!(right, Atom::Gap),
        Atom::OwnName(a) => matches!(right, Atom::OwnName(b) if a == b),
    }
}

fn sorts_overlap(left: &SortKey, right: &SortKey, coercions: &[Coercion]) -> bool {
    left == right
        || coercions
            .iter()
            .any(|c| (c.from == *left && c.to == *right) || (c.from == *right && c.to == *left))
}

struct Nfa {
    eps: Vec<Vec<usize>>,
    trans: Vec<Vec<(Atom, usize)>>,
}

impl Nfa {
    fn new() -> Self {
        Self {
            eps: Vec::new(),
            trans: Vec::new(),
        }
    }

    fn new_state(&mut self) -> usize {
        let state = self.eps.len();
        self.eps.push(Vec::new());
        self.trans.push(Vec::new());
        state
    }

    fn compile(&mut self, regex: &Regex) -> (usize, usize) {
        match regex {
            Regex::Atom(atom) => {
                let start = self.new_state();
                let end = self.new_state();
                self.trans[start].push((atom.clone(), end));
                (start, end)
            }
            Regex::Seq(items) => {
                let Some((first, rest)) = items.split_first() else {
                    let s = self.new_state();
                    return (s, s);
                };
                let (start, mut cur) = self.compile(first);
                for item in rest {
                    let (ns, ne) = self.compile(item);
                    self.eps[cur].push(ns);
                    cur = ne;
                }
                (start, cur)
            }
            Regex::Opt(inner) => {
                let start = self.new_state();
                let end = self.new_state();
                let (ins, ine) = self.compile(inner);
                self.eps[start].push(ins);
                self.eps[start].push(end);
                self.eps[ine].push(end);
                (start, end)
            }
            Regex::Plus { elem, join } => {
                let (start, e1) = self.compile(elem);
                let end = self.new_state();
                self.eps[e1].push(end);
                let (js, je) = self.compile(join);
                let (s2, e2) = self.compile(elem);
                self.eps[e1].push(js);
                self.eps[je].push(s2);
                self.eps[e2].push(end);
                self.eps[e2].push(js);
                (start, end)
            }
        }
    }
}

/// Whether two distinct paths through `regex` accept overlapping atom
/// sequences.
fn is_regex_ambiguous(regex: &Regex, coercions: &[Coercion]) -> bool {
    let mut nfa = Nfa::new();
    let (start, accept) = nfa.compile(regex);
    let state_count = nfa.eps.len();

    let mut eps_paths: Vec<BTreeMap<usize, u8>> = Vec::with_capacity(state_count);
    for origin in 0..state_count {
        let mut memo = BTreeMap::new();
        let mut visiting = BTreeSet::new();
        eps_paths.push(count_eps_paths(&nfa.eps, origin, &mut visiting, &mut memo));
    }

    if eps_paths[start].get(&accept).copied().unwrap_or(0) >= 2 {
        return true;
    }

    // Every non-epsilon edge has a unique id. Two runs differ if they take
    // different edge ids at any step or traverse an epsilon segment with >= 2
    // paths on an accepting run.
    let mut all_edges: Vec<(usize, usize, Atom, usize)> = Vec::new();
    for (u, edges) in nfa.trans.into_iter().enumerate() {
        for (atom, v) in edges {
            all_edges.push((all_edges.len(), u, atom, v));
        }
    }

    let mut next_edges: Vec<Vec<(usize, &Atom, usize, u8)>> = Vec::with_capacity(state_count);
    for ep in &eps_paths {
        let mut outgoing = Vec::new();
        for (&u, &mult) in ep {
            for (eid, eu, atom, v) in &all_edges {
                if *eu == u {
                    outgoing.push((*eid, atom, *v, mult));
                }
            }
        }
        next_edges.push(outgoing);
    }

    let mut queue: VecDeque<(usize, usize, bool)> = VecDeque::new();
    let mut seen: BTreeSet<(usize, usize, bool)> = BTreeSet::new();
    seen.insert((start, start, false));
    queue.push_back((start, start, false));

    while let Some((s1, s2, differed)) = queue.pop_front() {
        let acc1 = eps_paths[s1].get(&accept).copied().unwrap_or(0);
        let acc2 = eps_paths[s2].get(&accept).copied().unwrap_or(0);
        if acc1 > 0 && acc2 > 0 && (differed || acc1 >= 2 || acc2 >= 2) {
            return true;
        }
        for &(eid1, a1, v1, m1) in &next_edges[s1] {
            for &(eid2, a2, v2, m2) in &next_edges[s2] {
                if atoms_overlap(a1, a2, coercions) {
                    let next_diff = differed || eid1 != eid2 || m1 >= 2 || m2 >= 2;
                    if seen.insert((v1, v2, next_diff)) {
                        queue.push_back((v1, v2, next_diff));
                    }
                }
            }
        }
    }
    false
}

fn count_eps_paths(
    eps: &[Vec<usize>],
    node: usize,
    visiting: &mut BTreeSet<usize>,
    memo: &mut BTreeMap<usize, BTreeMap<usize, u8>>,
) -> BTreeMap<usize, u8> {
    if let Some(cached) = memo.get(&node) {
        return cached.clone();
    }
    assert!(
        visiting.insert(node),
        "skeleton NFA has an unreachable epsilon cycle at state {node}"
    );
    let mut counts = BTreeMap::from([(node, 1u8)]);
    for &next in &eps[node] {
        let sub = count_eps_paths(eps, next, visiting, memo);
        for (target, count) in sub {
            let entry = counts.entry(target).or_insert(0);
            *entry = (*entry).saturating_add(count).min(2);
        }
    }
    visiting.remove(&node);
    memo.insert(node, counts.clone());
    counts
}
