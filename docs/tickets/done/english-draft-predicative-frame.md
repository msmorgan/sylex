---
needs: []
---

# Retire `ComplexTransitive(Pred)` from `draft`

## Why

`crates/vocabulary/lexicon/english/verbs.ron` gives `draft` `frames: [Monotransitive,
ComplexTransitive(Pred)]` for `"Draft this card face up."`. Unlike `turn`, whose directional result
is obligatory (`*Turn this card`), `face up` on `draft` is an optional depictive adjunct (`Draft
this card face up` entails `Draft this card`, CGEL, Ch. 4, §5.3, p. 261), as on `cast`, `exile` and
`put … face down`, none of which carries `ComplexTransitive(Pred)`. Because `face up` is hosted on
`NominalAndPredicate`, all 13 `"Draft this card face up."` Corpus Units (*Agent of Acquisitions*,
*Canal Dredger*, *Cogwork Grinder*) build both a `predicative_complement` Reading and an `adjunct`
Reading on `Draft`, and any `Nominal`-hosted Preposition Phrase after `draft <NP>` (*Cogwork
Librarian*'s `"draft an additional card from that booster pack"`, *Cogwork Spy*, *Spire Phantasm*)
gains a spurious `predicative_complement` Reading.

## Design

- Remove `ComplexTransitive(Pred)` from `draft` in `crates/vocabulary/lexicon/english/verbs.ron`,
  leaving `frames: [Monotransitive]`.
- Mark `round` as `kind: Temporal` in `crates/vocabulary/lexicon/english/nouns.ron` (matching
  `turn`, `phase`, `step`, `combat` and `game`), so `"how many cards you've drafted this draft
  round"` (*Lurking Automaton*, *Pyretic Hunter*, *Garbage Fire*, *Custodi Peacekeeper*) reads
  `"this draft round"` as a temporal `Adjunct` rather than an accidental `NominalPredicative`
  `PredicativeComplement`.

## Done when

- `"Draft this card face up."` (*Agent of Acquisitions*) reads `face up` as `Adjunct` of `draft`
  with no `PredicativeComplement` Reading, and *Cogwork Librarian* has no Reading with `from that
  booster pack` as `PredicativeComplement` of `draft`; judgments pin both.
- Before/after census shows zero lost faces on the supported corpus, and `cargo xtask english
  strata --check`, `cargo test` and `cargo clippy --tests -- -D warnings` pass for
  `sylex_vocabulary` and `sylex_english`.

## Pointers

- `crates/vocabulary/lexicon/english/verbs.ron` (`draft`, `turn`).
- `crates/english/grammar/english.fug`: `PrepositionalPredicative`.
