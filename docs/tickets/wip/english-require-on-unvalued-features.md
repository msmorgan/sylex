---
needs: []
---
# Gates that test with `=` a feature the child may leave unvalued

## Why

Several gates use `=` (Require) on a child feature the child can leave unvalued. If `=`
passes on an unvalued feature, as `MeasuredComparative`'s comment says it does, these gates
admit everything they were written to exclude:

- `DelimitedFrontConjunct` writes `last.FrontPosition = Occupied`. `FrontPosition` is
  privative, so a Clause with no fronted Adjunct leaves it unvalued, and the Production appears
  to admit a stray comma after any coordinator (*C1 and, C2*). The Clause coordination writes
  `last.FrontPosition := Occupied` for the same test.
- `coordination AdjectivePhrase` leaves `Grade` unshared, so a coordinated Adjective Phrase
  (*red or green*) has no Grade. These test it with `=` and appear to admit one:
  `ComparativeBoundedNumeralName` (`last.Grade = Comparative`), `ComparativeBoundedAlternative`
  (`last.Grade = Comparative`), `ComparativeBoundedAmount` (`last.Grade = Comparative`),
  `MeasuredStandardComparative` (`head.Grade = Comparative`), `ComparativeIndirectComplement`
  (`modifier.Grade = Comparative`), `EquativeIndirectComplement` (`modifier.Grade = Equative`),
  `SuperlativeBareNominal` (`modifier.Grade = Superlative`). `MeasuredComparative`,
  `NonScalarIndirectComplement`, `SuperlativeFusedHead` and `PostposedAdjective` write `:=`.
- `DeterminedModifiedQuantity`'s comment says its determiner is singular, but no equation
  requires `determiner.Number`. A number-neutral determiner (*the*, *any*) passes
  `AgreeingModifiedQuantity`'s `determiner.Number = head.QuantityNumber`, and a plural
  Deictic or Numerical one (*those additional two turns*) appears to match both Productions.
  The exclusion `determiner.DeterminativeDistribution != Dependent | FreeChoice | Distributive`,
  which the `DeterminativeDistribution` comment calls an admission gate, may be standing in for
  the missing requirement.

## Open questions

- Should the `.fug` compiler reject, or warn on, `=` against a constant for a privative feature
  or one a coordination leaves unshared, or leave it to authors?

## Done when

- How `=` treats an unvalued feature is confirmed in `crates/syntax/src/step.rs` (and how
  `compile.rs` lowers `=` and `:=`). If `=` fails on an unvalued feature, the ticket closes with
  no grammar change.
- Each declaration above either requires the value with `:=` or has a comment saying why `=` is
  wanted there.
- `DeterminedModifiedQuantity` requires a singular determiner, and its
  `DeterminativeDistribution` exclusion is kept only as far as an ablation shows it prevents
  spurious Readings; the `DeterminativeDistribution` comment matches.
- A plural determiner over a modified quantity (*those additional two turns*) builds one tree.
- The census shows no Reading lost on the supported corpus, and every unit whose Readings
  change is checked for whether the lost Readings were the spurious ones.

## Not in scope

- Whether the *tied for* Productions should exist: `english-tied-for-superlatives`.
- Restructuring the comparative Productions: `english-comparative-productions`.

## Pointers

- `crates/english/grammar/english.fug`: the declarations above, `feature FrontPosition`,
  `feature DeterminativeDistribution`, `coordination AdjectivePhrase`, the Clause
  coordination with `last.FrontPosition := Occupied`.
- `crates/syntax/src/step.rs`, `crates/syntax/src/compile.rs`.
- Local evidence (gitignored): `docs/evidence/english-fug-hax-check/gemini.md` (B1 to B3),
  `docs/evidence/english-fug-hax-check/fable.md` (F12, `MeasuredComparative`; §3).
