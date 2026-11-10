---
needs: []
---

# The conditional static primitive carries no wording marking

## Why

`conditional(spec, condition, marking)` took a `CondMarking` (`ifSo`, `asLongAs`) that no design
document gives a meaning and no Rust code reads. A static's continuous effect applies at any
moment to whatever its text indicates either way [CR#611.3a], and the Templates `if`, `onlyIfSo`,
`asLongAs`, `onlyWhile`, `onlyUnless` and `partScope` already carry the printed word and the
clause order, so the marking was a second record of the Template choice. The primitive's name
also broke the adverb form of its siblings (`sequentially`, `simultaneously`, `optionally`).

## Done when

- The primitive is `conditionally(spec: StaticSpec, condition: Condition): StaticSpec =
  Conditionally`, and its comment says the printed word and order are the calling Template's.
- No `CondMarking` sort, `ifSo` or `asLongAs` marking word is left in `plugins/`; every Template
  body calls `conditionally` with two arguments.
- Two Templates expanding to the same tree (`if` and `onlyIfSo`; `asLongAs` and `onlyWhile`) pass
  `cargo xtask dsl check` with no refusal or warning.
- No card's wording changes.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

- Renaming the Templates `onlyIfSo` (`onlyIf` is the offered name) and `onlyWhile`; the
  `onlyWhile` name is part of `syl-conditional-wording-mismatches`.

## Pointers

`plugins/builtin/primitives/static/conditionally.syl`, `plugins/builtin/templates/static/`,
`plugins/builtin/templates/ability/{onlyUnless,partScope}.syl`.
