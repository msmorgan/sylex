---
needs: []
---

# `shortenedName` is accepted only on a face whose Card Scope holds a Shortened Name

## Why

Two places decide whether a face has a Shortened Name [CR#201.5c], and they disagree.

- English reads a face's text in its Card Scope (`sylex_vocabulary::face_scope`): the face has a
  Shortened Name when it is a legendary permanent, its name has a shortened spelling, and no
  other vocabulary in scope on the card spells it (`sylex_vocabulary::shortened_name`).
- `cargo xtask dsl check` accepts the `shortenedName` Template on a face by
  `sylex_vocabulary::short_name` alone (`crates/xtask/src/dsl_check/self_name.rs`), which has no
  vocabulary filter.

So a `.syl` card may say `shortenedName` on *Sliver Queen*, whose `##` spelling realizes as
*Sliver*, while English never reads *Sliver* there as the card's own name:
`cargo xtask english parse --as-card "Sliver Queen" --legendary "Sliver can't attack."` has no
Reading.

## Done when

- `dsl check` refuses `shortenedName` on a face whose Card Scope holds no Shortened form, with
  the refusal it gives today for a face without a shortening, and accepts it on every face whose
  Card Scope holds one.
- One function answers the question for both callers; `short_name` is not a second definition.
- A `dsl_check` test covers *Sliver Queen* (refused) and a legendary face with a Shortened Name
  (accepted); `cargo xtask dsl check` passes on `plugins/`.

## Pointers

- `crates/xtask/src/dsl_check/self_name.rs` (`has_shortened_name`),
  `crates/xtask/src/dsl_check/refusal.rs` (`ShortenedNameWithoutShortening`).
- `crates/vocabulary/src/names.rs` (`face_scope`, `stated_scope`, `shortened_name`),
  `crates/vocabulary/src/shortening.rs` (`short_name`, `legendary_permanent`).
- `docs/design/lexicon.md` §6; `docs/design/spelling.md` (own-name words).
