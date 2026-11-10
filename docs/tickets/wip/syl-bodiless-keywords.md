---
needs: [syl-single-face-card-call]
---

# Bodiless keywords are read elsewhere or are primitives

## Why

Nineteen keyword abilities are declared with `body = []`. Some are read by another declaration,
as `reach` is by `flying` and `deathtouch` and `toxic` are by rule rows; first strike, double
strike, trample and banding are implemented by the engine; the partner abilities (`partner`, its
four `partner…` variants, `chooseABackground` [CR#702.124k] and `doctorsCompanion` [CR#702.124m])
modify deck construction and function before the game begins [CR#702.124a], and nothing reads
them; and some are simply unfinished.
`docs/decisions/semantics.md` §3 says "a keyword ability is a Definition", with no exception.

Written against the start of step 6 (`syl-single-face-card-call`).

## Design

- A keyword with an empty body is read by another declaration, is a Primitive Keyword Ability, or
  modifies deck construction; any other empty body is unfinished.
- A Primitive Keyword Ability is declared as primitives, not as an ability with an empty body: one
  label primitive at `KeywordLabel` per keyword (`primitive firstStrike: KeywordLabel =
  FirstStrike`), carrying the keyword's `lexical`; one generic wrapper at `Ability`,
  `primitiveKeyword(keyword: KeywordLabel, quality: Option(Predicate) = none)`; and one Template
  per keyword at `Ability`, carrying its `spelling`, whose body is `primitiveKeyword(<label>)`.
  Callers name the keyword bare at both sorts, as before.
- Banding and "bands with other" have two labels, `banding` and `bandsWithOther`, because a
  `KeywordFamily` (fields `word` and `sort`) cannot select only the instances of one label that
  carry a quality, which *Shelkin Brownie* needs: its target loses all "bands with other"
  abilities and keeps banding. Both build the one wrapper; "bands with other <quality>" passes its
  quality as the wrapper's optional argument.
- A primitive at `KeywordLabel` builds only the constructor that capitalizes its own name and
  takes no parameters; every other `KeywordLabel` constructor stays reserved to `ability`
  declarations.
- The partner abilities other than "partner with [name]" keep their empty bodies: they modify
  deck construction and function before the game begins [CR#702.124a], and nothing models deck
  construction. "Partner with [name]" keeps its trigger [CR#702.124j].
- Toxic, and where its `amount` is read, is not changed here; `syl-unused-parameter-refusal`
  holds that question.

## Done when

- First strike, double strike, trample and banding (with "bands with other") are Primitive Keyword
  Abilities as the Design states, with every caller in `plugins/` resolving and printing as
  before.
- `cargo xtask dsl check` refuses a `KeywordLabel` primitive with parameters or with a
  constructor other than its own capitalized name, with a negative test.
- `docs/decisions/semantics.md` §3 states the exception: a Primitive Keyword Ability is a
  primitive, not a Definition.
- Each partner ability's doc comment states why its body is empty.
- Every other keyword with an empty body is read by a named declaration, modifies deck
  construction as the partner abilities do, or is unfinished with its ticket named in Not in
  scope; the landing lists each keyword with its reader or ticket.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

- Filling `craft`, `enlist` and `startYourEngines`: `syl-craft-materials`,
  `syl-enlist-optional-attack-cost` and `syl-speed-rules`.
- Moving the combat keywords' rules into `.syl` Definitions: `maybe/combat-keywords-into-syl`.
- A deck-construction rule for the partner abilities.

## Pointers

`plugins/builtin/definitions/keyword_ability/` (`firstStrike`, `doubleStrike`, `trample`,
`banding`, `bandsWithOther`, `reach`, `flying`, `deathtouch`, `toxic`, `partner*`),
`plugins/builtin/rules/sba/`, `crates/xtask/src/dsl_check/schema.rs`,
`docs/decisions/semantics.md` §3, `docs/decisions/syl.md` §3.
