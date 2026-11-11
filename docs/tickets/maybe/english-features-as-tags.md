---
needs: []
---
# Features that carry several unrelated meanings

## Why

Several features in `english.fug` hold values for concepts that share nothing but the feature,
and some gates work only because of the sharing:

- `ComplementKind` is the Slot Kind of a Preposition Phrase's, an adjective's and a noun's
  Complement; the kind of filler in the unlike-coordination Categories (`Depictive`,
  `PredicateAdjunct`, `Attributes`, `ObjectConjunct`, `PostmodifierConjunct`,
  `AmountOrNounConjunct`); and a mark on `Amount` and `ExistentialPredicate`, which have no
  Complement of that kind.
- `Polarity` is an auxiliary's negation, a *non-* premodifier, and the positive and negative
  sides of a *but not* contrast.
- `Determination` is "has a determiner" on a Noun Phrase and "needs one" on a Nominal or an
  adjective.
- `AdjunctPosition` holds positions (`Front`, `Central`, `End`, `Linking`) and adverb classes
  (`Phrasal`, `Multiplying`, `Bounding`).

Separately, a phrase cannot carry a structured value for its oblique or determiner, so the same
enumerations are declared again under other names: `ObliqueNumber`, `ObliqueKind`,
`ObliqueNounKind`, `ObliqueDetermination`, `ObliqueDistribution`, `QuantityNumber`,
`ConjunctNumber`, `FirstConjunctNumber`.

## Open questions

- Split each overloaded feature into one feature per concept, or keep them and state each
  meaning on the feature's comment?
- Should the DSL let a feature's value be another bundle (an oblique's features under one
  name), which would retire the `Oblique*` copies?

## Done when

- For each feature above, every Category that carries it is listed with what the value means
  there, and each equation that relies on two meanings sharing the feature is named.
- The features follow the owner's ruling, and no equation depends on an accidental sharing.
- The census shows identical Readings before and after.

## Not in scope

- `Case = Nominative` used as a position tag: `english-synesis-each-partitive`.
- `ComplementKind` on `Amount` in the comparative Productions: `english-comparative-productions`.

## Pointers

- `crates/english/grammar/english.fug`: the `feature` declarations named above and the
  `category` declarations that carry them.
- `crates/syntax/src/fug.rs`, `crates/syntax/src/grammar.rs`.
- Local evidence (gitignored): `docs/evidence/english-fug-hax-check/fable.md` (§3),
  `docs/evidence/english-fug-hax-check/gemini.md` (§3).
