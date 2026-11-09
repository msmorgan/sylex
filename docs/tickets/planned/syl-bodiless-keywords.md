---
needs: []
---

# Bodiless keywords are read elsewhere or are primitives

## Why

A keyword ability declared with `body = []` must either be read by another declaration, as
`reach` is by `flying` and `deathtouch` and `toxic` are by rule rows, or be defined by the engine.
An engine-defined keyword is declared as a primitive, not as an ability with an empty body: first
strike, double strike, trample and banding today are empty abilities. An empty body that nothing
reads is unfinished. `docs/decisions/semantics.md` §3's "a keyword ability is a Definition" does
not yet say this.

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
