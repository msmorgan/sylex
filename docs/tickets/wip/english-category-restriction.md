---
needs: []
---
# Restricting a Category to some of another's Productions

## Why

The `.fug` language cannot say "this Category is that one minus some of its Productions", so a
restricted Category restates the Productions it keeps, and each copy must be changed with its
twin by hand; nothing checks that they agree. `SubjectInitialClause` (a Clause that opens with
its Subject, for bare relatives) restates five Productions, each flagged `// Twin:` or listed
above `RelativeSubjectPredicate`:

- `SubjectPredicate` / `RelativeSubjectPredicate`
- `CliticSubjectPredicate` / `RelativeCliticSubjectPredicate`
- `FloatedEach` / `RelativeFloatedEach`
- `FloatedAll` / `RelativeFloatedAll`
- `AdjunctGap` / `RelativeAdjunctGap`

`category QuotedCostNominal` likewise restates `Determined` as `DeterminedQuotedCost` (flagged
`// Twin: Determined`, for *Brutal Suppression* and *Drought*) to keep `QuotedCost` out of `Nominal`
coordination. Other Categories restate Productions without naming twins: `category Possessor` (the
Noun Phrase Productions over `BareNominal`), `category RelativeTemporalNominal` (`RelativeTime` and
`RelativeTimePostmodifier` restating `NominalPostmodifier` and `TemporalPostmodifier`),
`PerfectParticiple`, `TemporalComplementClause`, `InternalizedInterleaved`, `NominalModifier`, and
`ContrastConjunct` / `ContrastCoordination` / `ContrastPrepositionConjunct` /
`ContrastPrepositionCoordination`.

## Design

- Unify restricted twin categories onto their base categories via `State` features/summaries and a
  shared `Premodifier` category rather than cloning categories across the Earley chart: fold
  `SubjectInitialClause` onto `Clause`, `Possessor` onto `NounPhrase` (via `Postmodification` /
  `Postmodified`, which also closes `*the mana value less than 3's owner`), `QuotedCostNominal`,
  `PerfectParticiple`, `TemporalComplementClause`, `InternalizedInterleaved`, `NominalModifier`,
  `RelativeTemporalNominal` and consumer-gated `TemporalNounPhrase` cases.
- Extend `coordination` in `sylex_syntax` and `.fug` to factor multi-shape coordination declarations
  (`over [...]`) and contrastive `but not` (`Contrast*` on *Drain Life*, *Soul Burn* and *Haakon,
  Stromgald Scourge*), deleting the hand-copied category pairs and coordination blocks.

## Done when

- The twin categories and `Contrast*` category pairs above are retired onto their base categories,
  multi-shape and contrastive `but not` coordinations are derived from `coordination`, and no
  `// Twin:` production pairs remain in `english.fug`.
- The census shows no verdict loss on the supported corpus, every Corpus Unit whose Readings change
  (such as closing `Possessor` leaks) is checked, and `cargo xtask english strata --check`, `cargo
  test` and `cargo clippy --tests -- -D warnings` pass for `sylex_syntax` and `sylex_english`.

## Not in scope

- `TargetedNounPhrase` (`english-targeting-marker-number`), which builds plural phrases with
  unquantified *target* admitted only inside `TargetedQuantity` (*any number of target creatures*)
  and never as standalone `NounPhrase`s.

## Pointers

- `crates/english/grammar/english.fug`: `SubjectInitialClause`, `QuotedCostNominal`,
  `DeterminedQuotedCost`, `Possessor`, `RelativeTemporalNominal`, `TemporalNounPhrase`,
  `PerfectParticiple`, `TemporalComplementClause`, `InternalizedInterleaved`, `NominalModifier`,
  and the `Contrast*` categories and coordinations.
- `crates/syntax/src/fug.rs`, `crates/syntax/src/grammar.rs`, `crates/syntax/src/compile.rs`,
  `crates/syntax/src/step.rs`.
