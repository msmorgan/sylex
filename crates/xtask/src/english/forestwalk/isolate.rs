//! The judgment assertions that isolate one Reading of a unit among its
//! Readings, found by greedy set cover.
//!
//! The candidates are the positive assertions a Reading's own Constituents
//! suggest, in the forms of `judgments/README.md`: a Category and surface (with
//! its occurrence where the surface repeats), then with a head word, a
//! function or a Gap Antecedent, then with both a head word and a function.
//! Each holds on the Reading ([`Judge::holds`]). A candidate covers the other
//! Readings it fails on; the cover takes, tier by tier from the simplest,
//! the candidate that covers most (the shortest on a tie) until every other
//! Reading is covered or no candidate covers one, and drops each assertion the
//! rest make redundant.
//!
//! The cover reports the Readings it could not separate: no assertion of
//! these forms tells them from the Reading, as when they differ only in
//! Productions, which no assertion names.

use std::collections::BTreeMap;
use std::collections::HashSet;

use anyhow::Result;
use sylex_english::judgment::Assertion;
use sylex_english::judgment::Function;
use sylex_english::judgment::Judge;
use sylex_english::judgment::Located;
use sylex_english::judgment::occurrences;
use sylex_lexicon::LexemeId;
use sylex_syntax::Constituent;
use sylex_syntax::Gap;

/// The assertions found for one Reading.
#[derive(Debug)]
pub(super) struct Isolation {
    /// The assertions, each holding on the Reading.
    pub(super) assertions: Vec<Assertion>,
    /// The other Readings that satisfy every assertion of these forms that
    /// holds on the Reading.
    pub(super) unseparated: usize,
}

/// A candidate assertion and how simple it is.
struct Candidate {
    assertion: Assertion,
    text: String,
    /// Clauses beyond the Category and surface.
    tier: usize,
}

/// The assertions that tell `target` from `others`.
///
/// # Errors
/// An assertion that holds on `target` by construction but does not, or
/// whose display [`sylex_english::judgment::parse_file`] does not read back
/// as itself, either of which is a defect of the generator.
pub(super) fn isolate(
    judge: &Judge<'_>,
    target: &Located,
    others: &[&Located],
) -> Result<Isolation> {
    let candidates = candidates(judge, target)?;
    let present: Vec<HashSet<(&'static str, &str)>> = others
        .iter()
        .map(|other| {
            other
                .constituents
                .iter()
                .map(|c| (c.category.as_str(), surface(&other.text, c)))
                .collect()
        })
        .collect();
    let fails = |candidate: &Candidate, other: usize| {
        let a = &candidate.assertion;
        if !present[other].contains(&(a.category.as_str(), a.surface.as_str())) {
            return true;
        }
        // A Category and surface alone hold where a Constituent has them.
        let bare =
            a.occurrence.is_none() && a.head.is_none() && a.function.is_none() && a.gap.is_none();
        !bare && !judge.holds(a, others[other])
    };
    let mut remaining: Vec<usize> = (0..others.len()).collect();
    let mut chosen: Vec<usize> = Vec::new();
    for tier in 0..=2 {
        let tiered: Vec<usize> = (0..candidates.len())
            .filter(|&c| candidates[c].tier == tier)
            .collect();
        // What each candidate fails on among the Readings left at this tier.
        let failing: Vec<(usize, Vec<usize>)> = tiered
            .iter()
            .map(|&c| {
                (
                    c,
                    remaining
                        .iter()
                        .copied()
                        .filter(|&r| fails(&candidates[c], r))
                        .collect(),
                )
            })
            .collect();
        while !remaining.is_empty() {
            let left: HashSet<usize> = remaining.iter().copied().collect();
            let gain = |failed: &[usize]| failed.iter().filter(|r| left.contains(r)).count();
            let best = failing
                .iter()
                .map(|(c, failed)| (gain(failed), *c, failed))
                .filter(|(gain, _, _)| *gain > 0)
                .min_by(|a, b| {
                    b.0.cmp(&a.0)
                        .then_with(|| candidates[a.1].text.len().cmp(&candidates[b.1].text.len()))
                        .then_with(|| candidates[a.1].text.cmp(&candidates[b.1].text))
                });
            let Some((_, c, failed)) = best else {
                break;
            };
            chosen.push(c);
            let covered: HashSet<usize> = failed.iter().copied().collect();
            remaining.retain(|r| !covered.contains(r));
        }
    }
    let unseparated = remaining.len();
    // Drop each assertion the others make redundant, the last chosen first.
    let covers: Vec<HashSet<usize>> = chosen
        .iter()
        .map(|&c| {
            (0..others.len())
                .filter(|&r| fails(&candidates[c], r))
                .collect()
        })
        .collect();
    let covered = |kept: &[bool]| -> HashSet<usize> {
        covers
            .iter()
            .zip(kept)
            .filter(|(_, keep)| **keep)
            .flat_map(|(cover, _)| cover.iter().copied())
            .collect()
    };
    let mut kept = vec![true; chosen.len()];
    let all = covered(&kept);
    for at in (0..chosen.len()).rev() {
        kept[at] = false;
        if covered(&kept) != all {
            kept[at] = true;
        }
    }
    let assertions: Vec<Assertion> = chosen
        .iter()
        .zip(&kept)
        .filter(|(_, keep)| **keep)
        .map(|(&c, _)| candidates[c].assertion.clone())
        .collect();
    for assertion in &assertions {
        let text = assertion.to_string();
        let entries = sylex_english::judgment::parse_file("isolate", &format!("+ text => {text}"))?;
        anyhow::ensure!(
            entries.len() == 1 && entries[0].groups == [vec![assertion.clone()]],
            "the assertion `{text}` is not read back as itself"
        );
    }
    Ok(Isolation {
        assertions,
        unseparated,
    })
}

/// The text a Constituent spans in its Reading's realization.
fn surface<'t>(text: &'t str, c: &Constituent) -> &'t str {
    text.get(c.span.clone())
        .expect("a Constituent's span lies in its realized text")
}

/// The lemma and the lexeme id that name a head word, as the assertion's
/// clauses take them.
fn names(judge: &Judge<'_>, id: &LexemeId) -> Vec<String> {
    let lexeme = judge.lexicon.lexemes().get(id).unwrap_or_else(|| {
        panic!(
            "a Constituent's head {} is no Lexeme of the lexicon",
            id.as_str()
        )
    });
    let mut names = vec![lexeme.lemma.clone()];
    if names[0] != id.as_str() {
        names.push(id.as_str().to_owned());
    }
    names
}

/// The candidates for `target`, simplest first, each holding on it.
fn candidates(judge: &Judge<'_>, target: &Located) -> Result<Vec<Candidate>> {
    let text = &target.text;
    let cs = &target.constituents;
    let mut out: BTreeMap<String, Candidate> = BTreeMap::new();
    let mut add = |assertion: Assertion| {
        let shown = assertion.to_string();
        let tier = usize::from(assertion.head.is_some())
            + usize::from(assertion.function.is_some())
            + usize::from(assertion.gap.is_some());
        out.entry(shown.clone()).or_insert(Candidate {
            assertion,
            text: shown,
            tier,
        });
    };
    for (index, c) in cs.iter().enumerate() {
        // A Category the assertion syntax cannot name (a private one, as
        // `NounPhrase#And`) or an empty Constituent has no assertion.
        if c.span.is_empty()
            || !c
                .category
                .as_str()
                .chars()
                .all(|ch| ch.is_ascii_alphabetic())
        {
            continue;
        }
        let surface = surface(text, c);
        let starts = occurrences(text, surface);
        let mut occurrence_forms = vec![None];
        if starts.len() > 1
            && let Some(at) = starts.iter().position(|&s| s == c.span.start)
        {
            occurrence_forms.push(Some(at + 1));
        }
        let mut functions: Vec<(Function, usize)> = Vec::new();
        for (name, parent) in c
            .function
            .iter()
            .map(|f| (*f, c.parent))
            .chain(c.also.iter().map(|(f, p)| (*f, Some(*p))))
        {
            let function = Function::ALL
                .into_iter()
                .find(|f| f.child_name() == name.as_str());
            if let (Some(function), Some(parent)) = (function, parent) {
                functions.push((function, parent));
            }
        }
        let heads = match &c.head {
            Some(id) => names(judge, id),
            None => Vec::new(),
        };
        for occurrence in occurrence_forms {
            let base = Assertion {
                category: c.category,
                surface: surface.to_owned(),
                occurrence,
                head: None,
                function: None,
                gap: None,
            };
            add(base.clone());
            for head in &heads {
                add(Assertion {
                    head: Some(head.clone()),
                    ..base.clone()
                });
            }
            for &(function, parent) in &functions {
                let phrases = match &cs[parent].head {
                    Some(id) => names(judge, id),
                    None => Vec::new(),
                };
                for of in phrases {
                    let with_function = Assertion {
                        function: Some((function, of)),
                        ..base.clone()
                    };
                    add(with_function.clone());
                    for head in &heads {
                        add(Assertion {
                            head: Some(head.clone()),
                            ..with_function.clone()
                        });
                    }
                }
            }
        }
        let gapped = c.gap != Gap::None
            || cs
                .iter()
                .any(|d| d.parent == Some(index) && d.gap != Gap::None);
        if gapped {
            for &antecedent in &surfaces_of(cs, text) {
                let assertion = Assertion {
                    category: c.category,
                    surface: surface.to_owned(),
                    occurrence: None,
                    head: None,
                    function: None,
                    gap: Some(antecedent.to_owned()),
                };
                if judge.holds(&assertion, target) {
                    add(assertion);
                }
            }
        }
    }
    let candidates: Vec<Candidate> = out.into_values().collect();
    for candidate in &candidates {
        anyhow::ensure!(
            judge.holds(&candidate.assertion, target),
            "the generated assertion `{}` does not hold on the Reading it was generated from",
            candidate.text
        );
    }
    Ok(candidates)
}

/// The distinct surfaces of a Reading's non-empty Constituents.
fn surfaces_of<'t>(cs: &[Constituent], text: &'t str) -> Vec<&'t str> {
    let mut surfaces: Vec<&str> = cs
        .iter()
        .filter(|c| !c.span.is_empty())
        .map(|c| surface(text, c))
        .collect();
    surfaces.sort_unstable();
    surfaces.dedup();
    surfaces
}
