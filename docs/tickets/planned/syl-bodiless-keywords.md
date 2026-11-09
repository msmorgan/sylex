---
needs: []
---

# Bodiless keywords are read elsewhere or are primitives

## Why

Nineteen keyword abilities are declared with `body = []`. Some are read by another declaration,
as `reach` is by `flying` and `deathtouch` and `toxic` are by rule rows; first strike, double
strike, trample and banding are defined by the engine; the partner abilities (`partner`, its four
`partner…` variants, `chooseABackground` [CR#702.124k] and `doctorsCompanion` [CR#702.124m])
modify deck construction and function before the game begins [CR#702.124a], and nothing reads
them; and some are simply unfinished.
`docs/decisions/semantics.md` §3 says "a keyword ability is a Definition", with no exception.

## Design

- A keyword with an empty body is read by another declaration or is defined by the engine; an
  empty body that nothing reads is unfinished.
- An engine-defined keyword is declared as a primitive, not as an ability with an empty body.

## Open questions

- How an engine-defined keyword keeps its name at `KeywordLabel`. Canon names these keywords
  there: `theKeyword(firstStrike)`, `theKeyword(doubleStrike)` and `theKeyword(trample)` (*Cairn
  Wanderer*, *Odric, Lunarch Marshal*), `anyIn({word = bandsWithOther})` (*Tolaria*, *Shelkin
  Brownie*). Only an `ability` declaration registers a name at `KeywordLabel`, and every
  `KeywordLabel` constructor is reserved (`Reserves::All`,
  `crates/xtask/src/dsl_check/schema.rs`), so a plain primitive is refused there without a schema
  change.
- What the partner abilities become: read by a deck-construction rule, primitives, or empty
  abilities kept as they are.
- `toxic`'s `amount` is read by a rule row (`toxicPoison.syl`), not by toxic's own empty body.
  Does `syl-unused-parameter-refusal` count a rule-row read as reading the parameter, or does this
  ticket's design change where `amount` is read?

## Done when

- First strike, double strike, trample and banding (and `bandsWithOther`) are primitives, with
  every caller in `plugins/` updated.
- `docs/decisions/semantics.md` §3 states the exception: an engine-defined keyword is a primitive.
- The partner abilities are declared as settled.
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
