---
needs: []
---

# Preposition frame constraints are hardcoded in sylex_syntax for among

## Why

`docs/design/english.md` §1 keeps `sylex_syntax` ignorant of grammar features, values and
Categories outside the lexicon feature-bundle mapping table. `PrepositionFrame` carries two
booleans (`plural_complement`, used by `prep:among` alone, and `named_only`, used by `among`, `for`
and `into`). To compile
`plural_complement`, `sylex_syntax::compile::plural_complement` hardcodes the feature value
`"Plural"`, branches on specific `SlotKind` variants (`KeywordPhrase`, `MeasureNP`, `Name`,
`Unlike`, `NP`), splits each frame into `/coordinated` and `/number` twin Productions, and adds
`whole_coordination` and `only_head_coordination` to `Summary`, `State` and `Production`
(`rhs.len() == 1 && head == Some(0)`). Expressing preposition-level constraints this way grows a
new `PrepositionFrame` flag and compiler branch in `sylex_syntax` per preposition.

## Design

- Express preposition complement constraints through grammar-declared `slot` declarations and slot
  feature/coordination equations in `english.fug` (including `coordinated` / plural-complement
  requirements for `among` and cited-name slots), retiring `plural_complement`, `named_only` and
  hardcoded `"Plural"` from `sylex_syntax` (`PrepositionFrame` and `compile.rs`).

## Done when

- `sylex_syntax` names no grammar feature value string (including `"Plural"`), `PrepositionFrame`
  carries no `among`-specific boolean (`plural_complement`, `named_only`), and
  `whole_coordination` / `only_head_coordination` are removed unless retained as a general `.fug`
  capability independent of any single preposition.
- *Among*'s plural-complement requirement (CGEL, Ch. 7 §5.1(a), p. 636) and cited-name frame
  (*from among Disenchant, ..., and Black Lotus*) hold through the general mechanism, and
  `crates/english/judgments/plural_among_complement.txt` passes.
- The Census shows no verdict loss on the supported corpus, and every Corpus Unit whose Readings
  change is checked.
- `cargo xtask english strata --check`, `cargo test -p sylex_syntax -p sylex_vocabulary -p
  sylex_english` and `cargo clippy -p sylex_syntax -p sylex_vocabulary -p sylex_english --tests --
  -D warnings` pass.

## Pointers

- `docs/design/english.md` §1, §3.
- `crates/syntax/src/frames.rs`: `PrepositionFrame`.
- `crates/syntax/src/compile.rs`: `preposition_valence`, `plural_complement`,
  `unlike_preposition_frames`.
- `crates/syntax/src/step.rs`: `Summary::whole_coordination`, `State::only_head_coordination`.
- `crates/vocabulary/lexicon/english/prepositions.ron`: `among`, `for`, `into`.
