---
needs: []
---
# No gate in the grammar records its ablation

## Why

`crates/english/README.md`'s working rule "A gate's ablation" requires, before a gate (ADR §6)
is added or retired, removing it, running the census, and comparing with the census that has
it: the spurious Readings and units the forest gains without it, and the growth of the largest
units. A new gate states that result in its comment in `english.fug`. No comment in
`english.fug` states such a result. Seven comments label a declaration "an admission gate (ADR
§6)": `feature DeterminativeDistribution` (with `DeterminedModifiedQuantity`), `category
SubjectInitialClause`, `category RelativeTemporalNominal`, `category Possessor`, `category
AttributeChain`, `valence PrepositionPhrase`'s `closed_complement`, and a `coordination
NounPhrase` (a Numeric first conjunct). Their grounds are CGEL or the style guide; only
`closed_complement` names the card and spurious structure it prevents, and none gives a measured
effect. Many more `Exclude` and `Require` equations are gates by §6's definition and carry no
label.

## Design

- A gate's ablation is not kept. The gate's comment in `english.fug` names one real card and the
  spurious structure the gate prevents on it, as the Preposition Phrase valence's
  `closed_complement` comment does; the measured census comparison is reported with the change
  that adds or retires the gate.
- The rule applies to the labeled gates now, and to any other gate when a change adds, retires or
  edits it.

## Done when

- The README rule states where the record lives, and the ADR §6 text agrees with it.
- Each gate labeled "admission gate" in `english.fug` names a card and the spurious structure it
  prevents, or the label is removed because the declaration is not a gate.
- The census before and after shows no change in Readings (the records change no behavior).

## Pointers

- `crates/english/README.md`, "Working rules" ("A gate's ablation").
- `docs/decisions/english.md` §6.
- `crates/english/grammar/english.fug`: the labeled declarations above.
- Local evidence (gitignored): `docs/evidence/english-fug-hax-check/fable.md` (opening
  observation, F10).
