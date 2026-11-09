---
needs: []
---

# Bodiless keywords are read elsewhere or are primitives

## Why

Nineteen keyword abilities are declared with `body = []`. Some are read by another declaration,
as `reach` is by `flying` and `deathtouch` and `toxic` are by rule rows; first strike, double
strike, trample and banding are defined by the engine; and some are simply unfinished.
`docs/decisions/semantics.md` §3 says "a keyword ability is a Definition", with no exception.

## Design

- A keyword with an empty body is read by another declaration or is defined by the engine; an
  empty body that nothing reads is unfinished.
- An engine-defined keyword is declared as a primitive, not as an ability with an empty body.

## Done when

- First strike, double strike, trample and banding (and `bandsWithOther`) are primitives, with
  every caller in `plugins/` updated.
- `docs/decisions/semantics.md` §3 states the exception: an engine-defined keyword is a primitive.
- Every other keyword with an empty body is read by a named declaration; the landing lists each
  keyword with its reader.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

- Filling `craft`, `enlist` and `startYourEngines`: `syl-craft-materials`,
  `syl-enlist-optional-attack-cost` and `syl-speed-rules`.
- Moving the combat keywords' rules into `.syl` Definitions: `maybe/combat-keywords-into-syl`.

## Pointers

`plugins/builtin/definitions/keyword_ability/` (`firstStrike`, `doubleStrike`, `trample`,
`banding`, `bandsWithOther`, `reach`, `flying`, `deathtouch`, `toxic`),
`plugins/builtin/rules/sba/`, `docs/decisions/semantics.md` §3.
