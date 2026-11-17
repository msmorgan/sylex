---
needs: []
---

# Same-head complement priority on `GappedPurpose`

## Why

Same-head complement priority (`docs/design/english.md` §3; `crates/syntax/src/step.rs`) misses two
unselected purpose *to*-infinitival paths on `verb:cost` (`TransitiveWithComplement(HollowToInf)`):

- `Extension::selects` compares `SlotKind`s by equality, so `Extension::Slot(SlotKind::HollowToInf)`
  does not select `Extension::Slot(SlotKind::ToInf)`. On 111 Corpus Units (*Arm-Mounted Anchor*'s
  `"This ability costs {2} less to activate …"`, *Agatha of the Vile Cauldron*, *Mavinda, Students'
  Advocate*'s `"it costs {8} more to cast this way"`), ungapped `adjunct ToInf` attaches `to
  activate` / `to cast this way` as an unselected `Adjunct` beside `cost`'s `HollowToInf`
  `Complement`.
- `GappedPurpose` (`VerbPhrase -> VerbPhrase adjunct:ToInf where adjunct.Gap = NP`) is written as a
  plain `Production` because `SchemaDecl::Adjunct` (`crates/syntax/src/compile.rs`) refuses `.Gap`
  equations, so inside relative clauses (*Strong Back*'s `"Equip abilities you activate that target
  enchanted creature cost {3} less to activate"`, *Pollywog Symbiote*) `GappedPurpose` carries
  `adjunct_slot = None` and also attaches `to activate` / `to cast` as an `Adjunct` of `cost`.

## Design

- Treat `Extension::Slot(SlotKind::HollowToInf)` as selecting `Extension::Slot(SlotKind::ToInf)` in
  `Extension::selects` (`crates/syntax/src/step.rs`).
- Allow `<function>.Gap` equations on non-delimited `SchemaDecl::Adjunct` in
  `crates/syntax/src/compile.rs` (raising the required Gap to the mother while closing `head`) and
  express `GappedPurpose` in `crates/english/grammar/english.fug` through `adjunct ToInf` (`host:
  VerbPhrase, side: After, host_kind: Predicate, cost: 2`) with `adjunct.Gap = NP`.

## Done when

- *Arm-Mounted Anchor*, *Agatha of the Vile Cauldron* and *Mavinda, Students' Advocate* read `to
  activate` / `to cast` only as `Complement` of `cost`, pinned by a judgment, while gapped purpose
  adjuncts on non-`ToInf`-selecting verbs (*"the first spell that mana from a Treasure was spent to
  cast"*) keep their `Adjunct` Readings.
- Before/after census shows zero lost faces on the supported corpus, and `cargo xtask english
  strata --check`, `cargo test` and `cargo clippy --tests -- -D warnings` pass for `sylex_syntax`
  and `sylex_english`.

## Pointers

- `crates/english/grammar/english.fug`: `GappedPurpose`, `adjunct ToInf`.
- `crates/syntax/src/compile.rs` (`SchemaDecl::Adjunct`, `adjunct_slot`),
  `crates/syntax/src/step.rs` (`Extension::Slot`).
- `crates/vocabulary/lexicon/english/verbs.ron` (`cost`).
