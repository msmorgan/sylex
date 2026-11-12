---
needs: [syntax-lexeme-and-coordinator-ids]
---

# Bind-time feature, child-role and normal-form resolution in `sylex_syntax`

## Why

`sylex_syntax` re-derives closed grammar invariants through runtime string formatting, raw
role-name comparisons and `Ident` parsing instead of resolving them at bind time.
`CompiledGrammar::leaf_facts` formats closed lexical enums (`Number`, `Person`, `Tense`, `Case`,
`WordForm`, `Countability`) with `Debug` and scans `self.values` by string—silently dropping a leaf
via `?` if a projected value name is missing—while `compile` round-trips `Gap`, `SlotKind`,
`FrameFamily` and `SlotFill` through `Debug` strings and rewrites equation paths by string prefix.
Outside grammar binding, four modules (`step`, `alternatives`, `reading` and `tree_key`) match
`Production::names` against raw string literals while `Production` scatters per-child-position
rules across sparse `(child_index, ...)` side vectors and `CompiledGrammar::head_line_child`
allocates a `Vec` per call. Meanwhile `Parse::high_attachment` allocates a
`BTreeMap<Ident, Ident>` per node from `Parse::node_features`, and `normal_form` splits `Ident`s on
`'#'`, formats `"Coordination:{base}"` and longer-frame names, checks `"+by"`, and re-interns
strings under the global `dpsi` lock.

## Design

- `CompiledGrammar` pre-resolves every projected lexical enum variant and lexeme `Countability` to
  its `(feature_id, u16)` value at bind time (returning a bind error if a declared projected
  feature omits one), so `leaf_facts` populates `LeafFacts::features` by direct lookup without
  formatting, heap allocation or fallible lookup.
- `compile` maps `Gap`, `SlotKind`, `FrameFamily` and `SlotFill` directly from their Rust enums
  and rewrites initial-conjunct equation paths structurally, removing `gap_named`, `Debug`
  formatting of closed enums and `replacen("initial.", "first.", 1)`.
- Child grammatical functions are resolved at bind time to a closed child-role enum rather than
  matched as `&str` at parse, forest or ranking time.
- `Production` consolidates per-child-position admission gates and pre-resolved Category-feature
  slot offsets (`node_value`) onto a per-`dot` child-slot slice—checked in `Production::accept`
  before copying and propagating multi-child `State`—instead of scanning sparse `(child_index,
  ...)` side vectors.
- `Production` precomputes static structural and Ranking Key facts at bind time (`head_line_child`,
  `is_frame_production`, `post_adjunct_host`, `hosts_comma_set_dependent`, `fronts`,
  `ends_in_head_line`, `set_off_before_head`, `trailing_adjunct`, anchor and attachment-role
  summaries).
- `CompiledGrammar` resolves `high_attachment` declarations at bind time to `(category, slot,
  FeatureValue)` triples checked directly against `Summary::values` without building a per-node
  feature map.
- `CompiledGrammar` precomputes each Category's base Category and flat-coordination `Ident` and
  each frame Production's longer-`PP` Production link and passive-`by` flag at bind time, so
  `normal_form` performs no runtime string splitting, formatting or `Ident::new` interning.

## Done when

- On the full pool, `cargo xtask english forests` produces table-by-table identical content to
  before the change, and `cargo xtask english strata --check` writes an identical `strata.json`
  report.
- A missing value on a declared projected feature is a bind error (covered by a unit test), and
  `leaf_facts` resolves projected features without formatting or silently dropping a leaf.
- No code in `sylex_syntax` formats Rust `Debug` names to look up grammar values, matches raw
  child-role string literals outside grammar binding, builds a `BTreeMap<Ident, Ident>` in
  `high_attachment`, or parses `'#'`, `"+by"` or `"Coordination:"` out of `Ident`s in
  `normal_form`; and `Production::accept` checks the current slot's local admission gates before
  copying and propagating `State`.
- `cargo test` and `cargo clippy --tests -- -D warnings` pass for `sylex_syntax` and
  `sylex_english`.

## Not in scope

- Dense `Lexeme` ids and typed Linked Coordination coordinator ids
  (`syntax-lexeme-and-coordinator-ids`).
- Single-copy chart deduplication, boundary-indexed chart tables and shared forest traversals
  (`syntax-chart-and-forest-dedup`).
- Changing the grammar (`english.fug`), admitted Readings or Reading Normal Form.

## Pointers

- `crates/syntax/src/compile.rs` (`CompiledGrammar`, `gap_named`, `head_line_child`,
  `is_frame_production`), `crates/syntax/src/step.rs` (`Production`, `Production::node_value`,
  `CompiledGrammar::leaf_facts`, `projected_values`).
- `crates/syntax/src/chart.rs` (`Parse::node_features`), `crates/syntax/src/alternatives.rs`,
  `crates/syntax/src/reading.rs` (`high_attachment`, `attachment_key`, `anchors`),
  `crates/syntax/src/tree_key.rs`.
- `crates/syntax/src/normal_form.rs`.
