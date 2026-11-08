---
needs: [runtime-prose-link]
---
# Occurrence-exact ability source refs

## Why

The TUI renders a derived ability's prose by looking its core value up in a value-keyed index,
v1's `sylex_plugin_v1::provenance::ProvenanceIndex`, which goes with `v1/` (`v1-retirement`).
The index renders correctly only while equal lowered values always render the same prose: two
identical grants from different sources are indistinguishable in it, and a text-changing effect in
layer 3 [CR#613.1c] or an index collision misrenders a card.

## Design

- Why an ability is present (the effect that granted it) and what renders it (the authored term)
  are separate facts and travel separately.
- Lowering emits the refs at each render-bearing subterm; they are never reconstructed afterward
  from the lowered structure.
- A ref is an opaque token outside core equality, hashing and serde, and no game decision reads
  it.
- Refs are the only route from a core value back to its prose: there is no value-keyed index
  fallback, which would be the reverse map from core that
  [architecture](../../decisions/architecture.md) "Two-way above, one-way below" rejects. A value
  with no authored source fails visibly and never invents prose.

## Done when

- The TUI renders ability prose from refs alone, and nothing reads `ProvenanceIndex`.
- Refs follow abilities through the layer pipeline (including the whole-pass fixpoint), stack
  bodies, conferral registries, and token and emblem creation, riding the source tracking the
  layer pipeline already keeps for timestamp and dependency ordering [CR#613.7,613.8].
- Tests render the right source for a printed ability, a lost one, a gained one, a granted static
  through the fixpoint, a type-conferred ability, a copied one, a token's and an emblem's, and a
  source-less synthetic value fails visibly.

## Pointers

- `crates/tui/src/game.rs` (`CardProvenance`, `ProvenanceRefs`), `crates/tui/src/ui/detail.rs`
- `crates/engine/src/layer.rs` (the layer-3 `SetText` slot; continuous-effect source tracking)
- `crates/engine/src/render.rs`
