---
needs: [cleanup-step6-spine]
specs: [voice-and-object-phrases]
---

# The sort `Action` is named `VerbPhrase`

## Why

The sort `Action` is a verb phrase without its subject, and the spec puts phrases at it that are
no actions: the continuous-effect phrases ("gets +1/+0"), the "enters" phrases and the modal
phrases ("can't be blocked") (spec §1). The English grammar's Category for the same thing is
`VerbPhrase` (`crates/english/grammar/english.fug`). The rename changes no term, and every other
ticket of the spec writes the new name.

## Done when

- No signature under `plugins/` names a sort `Action`, and neither does any table in
  `crates/xtask/src/dsl_check/schema.rs` (the `action` kind's sort and its `body` field,
  `EXPANSION_CONSTRUCTORS`, `RING_EXCEPTION`, and the record fields `DeonticMods.rider`,
  `DidClause.action`, `ManaSubstitute.action`, `Mode.action`, `RollRow.action` and
  `SbaRule.then`) or any test under `crates/xtask/`; each names `VerbPhrase`.
- A one-off script, kept in `docs/evidence/`, makes the rename and checks that every file under
  `plugins/` parses to the same tree apart from the sort's name; the landing reports the file and
  signature counts it compared.
- The glossary entry **Action** (`docs/contexts/game-model/CONTEXT.md`) says it is the verb
  phrase one verb's declaration builds and that its sort is `VerbPhrase`, and a new entry **Verb
  Phrase** names the sort. `docs/design/syl.md`, `semantics.md` and `spelling.md` write
  `VerbPhrase` wherever they name the sort, as in `syl.md` §1's
  `primitive activated(cost: Cost, action: Action)`.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

- The declaration kind word `action`, the `Deed` constructor `Action`
  (`Reserves::Only(&["Action"])` in `schema.rs`) and the folders `primitives/action/` and
  `templates/action/`: none of them is the sort.
- The sort `Clause` and which parameters take it: `syl-clause-and-imperative`.

## Pointers

`docs/specs/voice-and-object-phrases.md` §1, §10 (glossary); `crates/xtask/src/dsl_check/schema.rs`;
`crates/xtask/src/dsl_expand/tests.rs`; `plugins/builtin/`.
