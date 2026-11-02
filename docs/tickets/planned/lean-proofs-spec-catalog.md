---
needs: []
---
# Distil `lean/Semantics/Proofs/` into a spec catalog

`lean/Semantics/Proofs/` is 82 files, 29,418 lines and about 3,500 `decide` theorems.
`semantics-rust-rewrite` retires Lean, and nobody should hand-port that. Most of it is
scaffolding: hand-written card fragments (`Cards*.lean`) written to show the AST could express a
construct before the canon `.syl` cards existed, and many near-duplicate negative tests. What is
worth keeping is the behavioural spec of reference resolution and refusals. This ticket distils it
into one catalog while Lean is still the oracle; the rewrite ports the catalog to native Rust tests.

## The catalog

One document, a few hundred entries of one or two lines each. Each entry has:

- the obligation, in prose, with its CR citation where it has one;
- a minimal setup;
- the expected result;
- the source theorem(s) it was taken from, which pass today. An entry whose setup does not come
  from a passing theorem is not an entry.

Group by obligation, the rule a test pins, not by refusal kind. A refusal kind is an outcome many
rules share: `.anaphor` is expected by 94 tests, mostly different reference-resolution obligations
(clause-mate exclusion, plurality, caller scope, "this way" grounding). Deduplicate only within an
obligation, keeping one witness per distinct boundary.

Cover at least the core suites: `DrtResolution`, `DrtRecency`, `DrtWitnesses`, `ContrastAnchor`,
`CallerScope`, `ReferenceScopes`, `BindingIdentity`, `HistoricalMove`, `MovementScopes`,
`LinkedReadback` and `RefusalReachability`, and every negative test elsewhere whose obligation they
do not already cover.

## Positive fragments

Drop the `Cards*.lean` positive fragments, except where a fragment is the only evidence for an AST
constructor or form: list the constructors and forms that occur in `Cards*.lean` but nowhere in
`plugins_v3/`. Each one gets a catalog entry or is recorded as a deliberate gap.

## Method

Script first: extract each theorem's name, docstring, CR citations, file and the refusal kinds its
right-hand side expects, plus the constructor diff between `Cards*.lean` and `plugins_v3/`. Then
fan the file groups out to cheap subagents (e.g. Gemini Flash through `agy`) to draft entries, and
review the merged catalog. Theorems that are not plain equations (`∧`, `≠`, `.all`, `let`, or
checker internals such as `stepAction`) are read by hand from the checklist the rewrite's exporter
emits, or found by the same script.

Nothing here changes `lean/`.
