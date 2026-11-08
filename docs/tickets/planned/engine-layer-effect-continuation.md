---
needs: []
---
# A continuous effect that has started applying keeps applying in later layers even if its source ability is removed during the layer pass, and the layer fixpoint cannot oscillate

## Why

An effect that starts to apply in one layer continues to apply to the same objects in every
other applicable layer, "even if the ability generating the effect is removed during this
process" [CR#613.6]. `layers()` instead re-gathers every static effect from each object's
derived ability list on each fixpoint iteration, so an effect whose source ability a layer-6
effect removes (Humility) drops out of every layer on the next iteration. Two effects that
remove each other's source abilities (Humility and Opalescence: Opalescence makes Humility a
creature, Humility then strips its own ability) alternate between iterations: the loop stops
only when two successive signatures match, so it runs to its cap of 16, the result depends on
parity, and the `debug_assert!(converged, …)` trips. Humility is a live card in the
`v1/plugins/canon` corpus the engine loads, so the departure is reachable today.

## Done when

- An effect that has applied in a layer continues in each later applicable layer of the same
  pass even if its source ability was removed [CR#613.6]; only newly granted statics enter
  through the re-gather.
- `depends_on` detects existence dependency: applying one effect would remove the ability that
  generates another [CR#613.8a]. Effects in a dependency loop apply in timestamp order
  [CR#613.8b].
- The fixpoint always converges; the comment in `layers()` claiming the effect set "grows
  monotonically" is corrected.
- Tests, against real cards:
  - Humility with Opalescence (and Worship) in both timestamp orders gives the results of the
    2009-10-01 Humility/Opalescence ruling.
  - Humility with two Opalescences in each timestamp order gives the results of the 2006-02-01
    ruling.
  - A creature whose own static applies in a layer before 6 (Dryad of the Ilysian Grove's
    layer-4 land-type effect) keeps that effect under Humility.

## Pointers

- `crates/engine/src/layer.rs`: `gather`, `depends_on`, `effect_signature`, `layers()`
  (`MAX_FIXPOINT_ITERS`, the convergence loop and assert).
- `crates/engine/tests/layers.rs`: the existing Humility tests.
- `v1/plugins/canon/cards/Humility.ron` (live); Opalescence and Dryad of the Ilysian Grove are
  `.ron.todo` in `v1/plugins/wizards/cards/`, and as `.syl` only Dryad exists
  (`plugins/canon/cards/`).
- Rulings: `cardRulings` in `data/mtgjson/AllPrintings.sqlite` for Humility and Opalescence.
