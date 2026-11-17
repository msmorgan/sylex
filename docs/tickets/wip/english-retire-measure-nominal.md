---
needs: []
---

# Retire `MeasureNominal` and build determined slash adjustments as `MeasureNounPhrase`

## Why

`MeasureNominal` (`BareNominal -> Measure` with `$.Determination = Overt`) projects every slash
measure — signed (`+2/+2`) and unsigned (`2/2`, `3/3`) — into a general count `Nominal`, solely so
that the 26 *"gets an additional +X/+Y"* units (*Akawalli, the Seething Tower*, *Wild Might*,
*True-Faith Censer*) read as the Object of *get*. No Vintage-legal card prints an unsigned fused
size nominal (*"a 2/2"*, *"a 3/3"*): a creature's or token's size premodifies a noun (`MeasurePremodifier`,
*"a 3/3 green Elephant creature token"*), or stands bare as a `MeasureNounPhrase` predicative or
selected complement (*"except it's 4/3"*, *"your choice of 5/1 or 1/5"*). Projecting `Measure` into
`BareNominal` makes *"a 2/2"* and *"a 2/2 or 3/3"* ordinary `NounPhrase`s accepted by every
monotransitive verb (`*Destroy a 2/2.`, `*Sacrifice a +1/+1.`) and adds 22 spurious derivations
across 7 real cards (*Germinate Recruits*, *Prossh, Skyraider of Kher*, *Return of the Mole Man*,
*Aquamorph Entity*, *Eidolon of Countless Battles*, *Fangs of Kalonia*, *Hindervines*).

## Design

- Retire `MeasureNominal` and `feature MeasureHead = Fused` (along with the `MeasureHead` exclusions
  on `coordination Nominal`).
- Distinguish signed slash adjustments (`+2/+0`, `-1/-1`) from unsigned slash sizes (`2/2`, `0/1`)
  on `Measure` (or `SignedAmount`), and build determined adjustments (*"an additional +2/+2"*) into
  `MeasureNounPhrase` rather than `BareNominal` so only heads that select `MeasureNP` (*get*,
  *choice of*, *among*) admit them and unsigned *"a 2/2"* is no `NounPhrase`.

## Done when

- `MeasureNominal` and `feature MeasureHead` are retired.
- Every *"gets an additional +X/+Y"* / *"-X/-Y"* card (*Akawalli, the Seething Tower*, *Wild Might*,
  *True-Faith Censer*, *Desert's Due*, *Taste for Mayhem*) reads through *get*'s `Transitive(MeasureNP)`
  frame, while `*NounPhrase a 2/2`, `*NounPhrase a 2/2 or 3/3`, `*Destroy a 2/2.` and
  `*Sacrifice an additional +2/+2.` are refused.
- The 7 cards with spurious `MeasureNominal` derivations (*Germinate Recruits*, *Prossh, Skyraider
  of Kher*, *Return of the Mole Man*, *Aquamorph Entity*, *Eidolon of Countless Battles*, *Fangs of
  Kalonia*, *Hindervines*) lose those spurious Readings, and no supported Corpus Unit loses its
  verdict.
- `cargo xtask english strata --check`, `cargo test -p sylex_english` and
  `cargo clippy -p sylex_english --tests -- -D warnings` pass.

## Pointers

- `crates/english/grammar/english.fug`: `Measure`, `SignedAmount`, `SlashMeasure`, `MeasureNominal`,
  `MeasureNounPhrase`, `feature MeasureHead`, `coordination Nominal`.
- `crates/vocabulary/lexicon/english/verbs.ron`: `get` (`Transitive(MeasureNP)`).
- `crates/english/judgments/additional_slash_adjustment.txt`,
  `crates/english/judgments/slash_measure_selection.txt`,
  `crates/english/judgments/counter_compounds.txt`.
