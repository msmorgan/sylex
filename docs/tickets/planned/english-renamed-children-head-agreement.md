---
needs: []
---
# Children renamed away from `head` where `unshared` would keep the head

## Why

Five Productions name their semantic head something other than `head` to keep it out of
implicit head agreement: `DistributiveCoordinationDetermined` and `JointCoordinationDetermined`
(`nominal`), `TransparentQuantity`, `TemporalQuantity` and `DeterminedTemporalQuantity`
(`quantity`). Their comments say the Production forgoes the head lexeme in Readings and the head
line, and the first cites CGEL Ch. 5 §18.4, p. 508, which supports the agreement facts but not
that mechanism. The engine has `unshared` ("features of the mother that it does not share with
its `head` child", `grammar.rs`), already used by `QuantityPartitiveOfEach` and
`DegreeModifiedAdjective`, which would keep the head and override only the features that
differ. `DistributiveCoordinationDetermined`'s comment says Countability and NounKind are set
there, but its equations copy both from `nominal`; only Number differs. Losing the head line may
also clear the final-preposition summary that `ExistentialBe`'s `unextended` reads, so that
gate may not apply to a displaced Subject built by these Productions.
`DegreeModifiedComparativeDeterminer` and `IndependentGenitive` also have no `head` child, with
no comment saying why.

## Open questions

- The done ticket `english-v4-reserved-child-names-contract` kept the renames on
  `DistributiveCoordinationDetermined`, `TransparentQuantity` and `TemporalQuantity`, and
  rejected an explicit agreement override. Revisit that and move these Productions to `head`
  with `unshared`, or keep the renames?
- Should a Nominal *and*-coordination of singulars carry `Number = Plural` at all, given that
  `ConjunctNumber` and several gates (`BarePlural`, `Unmodified`, the single participial
  postmodifiers) exist to read past it?

## Done when

- `unshared`'s implementation is read and it is confirmed that `unshared: [Number]` (and
  `NounKind` where the oblique supplies it) on a child named `head` gives the same Readings
  with the head lexeme kept, by making the change on one Production and comparing Readings.
- Whether a renamed head defeats `ExistentialBe`'s `unextended` is confirmed by parse.
- The Productions follow the owner's ruling, and every remaining Production without a `head`
  child says why in its comment; no comment says a feature is set where an equation copies it.
- The census shows no Reading lost on the supported corpus.

## Pointers

- `crates/english/grammar/english.fug`: the five Productions,
  `DegreeModifiedComparativeDeterminer`, `IndependentGenitive`, `ExistentialBe`,
  `feature ConjunctNumber`, `BarePlural`, `Unmodified`.
- `crates/syntax/src/grammar.rs` (`ProductionDecl`, `unshared`), `crates/syntax/src/step.rs`
  (head line).
- `docs/tickets/done/english-v4-reserved-child-names-contract.md`.
- Local evidence (gitignored): `docs/evidence/english-fug-hax-check/fable.md` (F7),
  `docs/evidence/english-fug-hax-check/gemini.md` (E1).
