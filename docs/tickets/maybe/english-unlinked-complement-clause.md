---
needs: []
---
# `unlinked` on a preposition's Clause Complement

## Why

`FiniteComplementClause` carries `{ unlinked: [head] }`, so a Linked Conjunct never joins a
preposition's Clause Complement: *until [they exile a nonland card], then you may cast it*,
never *until [they exile a nonland card, then you may cast it]*. The comment states it as a law
("a sole connective never links subordinate clauses"). The CGEL sentence behind that law
(Ch. 15 §2.10, pp. 1319–1320) is about *so* alone, and the next sentence has *yet* linking
subordinate clauses; the same sentence excludes non-finite Verb Phrases, which
`LinkedVerbPhrase` links (*may search ..., put ..., then shuffle*). Both scopes are grammatical
English, so the flag appears to decide a scope ambiguity between different heads by Oracle
convention, which ADR §4 says stays packed.

## Open questions

- Keep `unlinked` as a gate with a comment that describes it as fixing the scope of a
  comma-plus-*then* link and states its ablation, or drop it and let both scopes stand?

## Done when

- CGEL pp. 1319–1320 is re-read and whether it says anything about *then* or subordinate
  clauses in general is recorded.
- `unlinked` is ablated and the Readings and unit growth on the supported corpus are recorded.
- The flag follows the owner's ruling; if kept, its comment states the ablation result and no
  claim the source does not make.

## Not in scope

- The §2.10 citation on `category LinkedClause`: `english-cgel-citation-corrections`.

## Pointers

- `crates/english/grammar/english.fug`: `FiniteComplementClause`, `category LinkedClause`,
  `category LinkedVerbPhrase`, the coordinations that declare a `link`.
- `crates/syntax/src/grammar.rs` (`unlinked`).
- Local evidence (gitignored): `docs/evidence/english-fug-hax-check/fable.md` (F5).
