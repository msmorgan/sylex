---
needs: [plugins-v3-data-language, lean-retirement]
---

# Remove the remaining sort-prefixed primitive names

## Why

Twelve primitives carry their sort as a name prefix: `actionSearch`, `actionShuffle`,
`actionVote`, `actionChooseModes`, `actionFlipCoins`, `actionPay`, `actionRemoveCounters`,
`actionShiftResult`, `nounPhraseSomeOf`, `amountAggregate`, `durationBetween` and `conditionDid`.
Each prefix works around a collision: the plain name is taken by a keyword action or a Template
at the same sort, and resolution by `(name, sort)` (`syl.md` §3) cannot tell them apart. A
prefix is not a word Oracle text uses, so it misnames the primitive.

## Done when

- None of the twelve names remains. Each collision is resolved at the primitive, Template or
  Definition boundary first: whichever declaration does not own the plain name is renamed to what
  it is or folded into the other, keeping structural primitives, keyword Definitions and worded
  Templates distinct.
- Every declaration and call in `builtin/` and `canon/` is updated, and a one-off script kept in
  `docs/evidence/` verifies that the `sylex_syl` trees before and after are equal modulo the
  explicit rename/fold map, reporting the file and call counts compared.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`plugins/builtin/primitives/{action,determiner,quantity,timing,condition}/`,
`docs/decisions/syl.md` §1 and §3.
