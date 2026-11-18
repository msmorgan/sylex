---
needs: []
---
# Split overloaded features (`DeterminerNeed`, `ConjunctKind`, `ExistentialLocation`)

## Why

Several features in `crates/english/grammar/english.fug` hold values for distinct concepts on
different categories:

- `Determination` means "has a determiner" (`Bare | Mixed | Overt`) on `NounPhrase` and
  `PrepositionPhrase`, and "needs an overt determiner" on `Nominal`, `BareNominal`,
  `QuantifiedNominal`, `ModifiedQuantifiedNominal` and `AdjectivePhrase`
  (propagating `Mixed` across categories that have no determination resolution).
- `ComplementKind` serves as the Slot Kind of a head's selected Complement, as the filler tag in
  unlike-coordination categories (`Depictive`, `PredicateAdjunct`, `Attributes`, `ObjectConjunct`,
  `PostmodifierConjunct`, `AmountOrNounConjunct`), and as a location-presence tag (`NP` vs `PP`) on
  `ExistentialPredicate` (`adjunct PrepositionPhrase { host: ExistentialPredicate }` requiring
  `head.ComplementKind != NP`).
- `Polarity`'s comment claims auxiliary negation alongside `non-` premodifiers and `but not`
  contrasts.

## Design

- Split a privative `feature DeterminerNeed = Required` on `Nominal`, `BareNominal`,
  `QuantifiedNominal`, `ModifiedQuantifiedNominal` and `AdjectivePhrase`
  (including `adjectives.ron` / `crates/vocabulary/src/build.rs`) from `Determination` (`Bare |
  Mixed | Overt` on `NounPhrase` and `PrepositionPhrase`).
- Split `ConjunctKind` for unlike-coordination filler categories from `ComplementKind`, and replace
  `ComplementKind` on `ExistentialPredicate` with a dedicated privative feature (such as
  `ExistentialLocation = Located` on `LocatedExistentialBe`, read by `adjunct PrepositionPhrase {
  host: ExistentialPredicate }`).
- Update `Polarity` and `AdjunctPosition` comments to state what each holds accurately, and keep
  flat `Oblique*` features without adding nested feature-bundle DSL syntax to `sylex_syntax`.

## Done when

- `DeterminerNeed`, `ConjunctKind` and `ExistentialPredicate`'s location tag are separated from
  `Determination` and `ComplementKind`, and no equation relies on accidental feature sharing.
- The census shows identical Readings before and after, and `cargo xtask english strata --check`,
  `cargo test` and `cargo clippy --tests -- -D warnings` pass for `sylex_syntax`,
  `sylex_vocabulary` and `sylex_english`.

## Not in scope

- `ComplementKind` on `Amount` in comparative Productions: `english-comparative-productions`.

## Pointers

- `crates/english/grammar/english.fug`: `Determination`, `ComplementKind`, `Polarity`,
  `AdjunctPosition`, `ExistentialPredicate` (`ExistentialBe`, `LocatedExistentialBe`), and the
  unlike-coordination and nominal categories.
- `crates/vocabulary/src/schema.rs`, `crates/vocabulary/src/build.rs`,
  `crates/vocabulary/lexicon/english/adjectives.ron`.
