---
needs: []
---
# Require a modifier on bound keyword stems that spell a free keyword lemma

## Why

In `crates/vocabulary/src/lib.rs`, a keyword ability with `bound` (`landwalk`, `bound: (surface: "walk", stems: [LandType, CardType])`) emits `ParameterPart::ModifiedWords(stems, TypeModifier, Quality)`, which `sylex_syntax` compiles into both an unmodified `{id}/Stem` Production and a recursive `{id}/ModifiedStem` Production. Because `CardType` includes `"land"`, unmodified `stem:land` plus bound `"walk"` spells `"landwalk"` — rebuilding the free keyword `keyword:landwalk` and giving 8 Corpus Units (*Traveler's Cloak*'s *Enchanted creature has landwalk of the chosen type*, *Cairn Wanderer*, *Concerted Effort*, *Magnigoth Treefolk*, *Giant Slug*, *Illusionary Presence*, *Excavator*, *Death-Mask Duplicant*) a second Reading (`docs/design/lexicon.md` §5, `docs/design/english.md` §8).

## Design

- Extend `ParameterPart::ModifiedWords` in `sylex_syntax` (`crates/syntax/src/frames.rs` and `crates/syntax/src/compile.rs`) to distinguish stems that may stand bare from stems that require at least one premodifier: bare stems project to the stem category directly (`{id}/Stem`), modified-only stems project to the stem category only with an initial `modifier` (`{id}/ModifiedOnlyStem`), and `{id}/ModifiedStem` recurses over the stem category so stacked modifiers (*nonbasic snow landwalk*) still read.
- In `crates/vocabulary/src/lib.rs`, partition `bound.stems` by whether `format!("{stem}{}", bound.surface)` equals the ability's `lemma` or any of its `variants`' lemmas, placing matching stems (`stem:land` on `landwalk`) in the modified-only set and all other stems (`stem:Island`, `stem:artifact`, all `typecycling` stems) in the bare set.
- Add a `sylex_syntax` unit test and assert 1 Reading on *Traveler's Cloak* and `landwalk` in `crates/english/judgments/landwalk_bound_suffix.txt` while keeping *nonbasic landwalk* and *Islandwalk* at 1 Reading.

## Done when

- Unmodified `"landwalk"` (bare and with `of <Quality>`, as on *Traveler's Cloak* and *Cairn Wanderer*) reads only through `keyword:landwalk`, while modified `"nonbasic landwalk"` and subtype `"Islandwalk"` continue to read through `BoundKeyword`.
- `cargo xtask english strata --check` against a pre-change baseline shows 0 lost faces, 0 verdict regressions, and 0 Corpus Units with increased Readings.
- `cargo test` and `cargo clippy --tests -- -D warnings` pass for `sylex_syntax`, `sylex_vocabulary`, and `sylex_english`.

## Pointers

- `crates/vocabulary/src/lib.rs` (`bound` parameter generation, lines 903–925)
- `crates/vocabulary/catalogs/keyword_abilities.ron` (`landwalk`)
- `crates/syntax/src/frames.rs` (`ParameterPart::ModifiedWords`), `crates/syntax/src/compile.rs` (`ModifiedWords` compilation, lines 2940–2960)
- `crates/english/judgments/landwalk_bound_suffix.txt`, `crates/english/judgments/core_composition.txt`
- `docs/design/lexicon.md` (§5)
