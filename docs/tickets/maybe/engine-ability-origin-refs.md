---
needs: [runtime-prose-link]
---
# Occurrence-exact ability source refs

## Why

The TUI renders a derived ability's prose by looking its core value up in a value-keyed index
(today v1's `sylex_plugin_v1::provenance::ProvenanceIndex`; the v3 lowering will need its own).
The index renders correctly only while equal lowered values always render the same prose, which
holds on a canonical-form authored surface. This ticket stays parked until that stops being
enough: a UI needs per-source attribution (two identical grants from different sources are
indistinguishable in the index), a real text-changing effect lands in layer 3 [CR#613.1c], or an
index collision misrenders a card.

## Design

- Why an ability is present (the effect that granted it) and what renders it (the authored term)
  are separate facts and travel separately.
- Lowering emits the refs at each render-bearing subterm; they are never reconstructed afterward
  from the lowered structure.
- A ref is an opaque token outside core equality, hashing and serde, and no game decision reads
  it.
- A renderer uses the ref when present and the index as the fallback; a value with no authored
  source fails visibly and never invents prose.

## Done when

- One of the triggers in Why has arrived, and the claim re-checks that the index fails on the
  lowering then in use.
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
