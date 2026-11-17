---
needs: []
---

# Same-head complement priority on `GappedPurpose`

## Why

`GappedPurpose` (`VerbPhrase -> VerbPhrase adjunct:ToInf where adjunct.Gap = NP`) is written as a
plain `Production` because `SchemaDecl::Adjunct` (`crates/syntax/src/compile.rs`) refuses `.Gap`
equations. Only `SchemaDecl::Adjunct` populates `Production::adjunct_slot` and `host_check`, so
same-head complement priority (`docs/design/english.md` §3; `crates/syntax/src/step.rs`) never
checks `GappedPurpose` against `Extension::Slot(SlotKind::ToInf)`. On 111 Corpus Units
(*Arm-Mounted Anchor*'s `"This ability costs {2} less to activate …"`, *Agatha of the Vile
Cauldron*, *Mavinda, Students' Advocate*'s `"costs {8} more to cast"`), `to activate` / `to cast`
attaches to `cost` twice: once as `verb:cost`'s selected hollow `ToInf` `complement` and once as an
unselected `GappedPurpose` `adjunct`.

## Design

- Allow `adjunct.Gap` equations on `SchemaDecl::Adjunct` in `crates/syntax/src/compile.rs` and
  express `GappedPurpose` in `crates/english/grammar/english.fug` through `adjunct ToInf` (`host:
  VerbPhrase, side: After, host_kind: Predicate`) with `adjunct.Gap = NP`, so `adjunct_slot =
  Some(SlotKind::ToInf)` suppresses the unselected purpose adjunct on any head whose frames select
  `ToInf`.

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
