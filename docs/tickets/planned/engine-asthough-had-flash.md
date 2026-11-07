---
needs: [core-asthough-glaring-spotlight]
---
# "Cast spells as though they had flash"

## Why

Leyline of Anticipation and Vedalken Orrery ("You may cast spells as though they had flash.") and
Alchemist's Refuge ("{G}{U}, {T}: You may cast spells this turn as though they had flash.") grant
flash by counterfactual [CR#609.4]. The engine realizes only the negative counterfactual
(Glaring Spotlight's "as though they didn't have hexproof"); the positive one needs the engine to
know what a keyword the spell lacks would do, and it has no registry for that.

## Design

- `premise = Has(K)` adds K's own abilities to the candidate for that check only
  [CR#609.4,702.8a]; the keyword's own rows do the work.
- Only the Flash keyword writes the instant-speed cast row: a granter is never authored as an
  inline `May(Cast(window: InstantSpeed))`.
- K's abilities come from a keyword-definition registry on `GameState`, loaded from the plugin.

## Done when

- `GameState` has the keyword-definition registry, populated from the loaded plugin as the
  subtype and card-type registries are.
- The cast-timing check consults the overlays whose `then` is a `May(Cast { by, what })`
  selecting the spell, adds K's abilities to the candidate, and re-collects its cast rows.
- Tests: a sorcery-speed spell can't be cast at instant speed without a granter; with Vedalken
  Orrery (or Leyline of Anticipation) on its controller's battlefield it can; the opponent still
  can't; after Alchemist's Refuge's ability resolves its controller can for the rest of that turn
  only.

## Pointers

- `crates/core/src/deontic.rs` (`AsThough::Counterfactual`)
- `crates/engine/src/legal.rs` (`astough_target_rows`, `asthough_sees_through_target`,
  `masked_self_rows_forbid`, `may_cast_rows`), `crates/engine/src/cast.rs`
  (`castable_cost_ignoring_mana`), `crates/engine/src/derive.rs` (`composite_members`),
  `crates/engine/src/state.rs` (the subtype and card-type registries)
- `plugins/builtin/definitions/keyword_ability/flash.syl`,
  `plugins/builtin/templates/condition/asThoughHadFlash.syl`
