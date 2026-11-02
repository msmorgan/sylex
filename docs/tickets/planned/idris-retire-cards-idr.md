---
needs: [semantics-v3-dsl-and-plugins, plugins-v3-builtin-primitives-and-templates]
---
**[port-required: semantics-v3] Retire `idris/src/Cards.idr` by migrating its
71-card coverage inventory into `plugins_v3/canon/cards/` (`.syl`).** Updated 2026-11-02: not into
`lean/`, whose checker is retired after Idris; a card the `.syl` model cannot express yet is listed in its
mechanic's open ticket instead.
Preserves the 38-canonizable / 22-blocked card breakdown (including the 13
unbuilt-mechanic cards — Time Walk, Mindslaver, Vodalian Illusionist, Pine
Walker, Iona, Steely Resolve, Cavern of Souls, Meddling Mage, Citadel Siege,
Outpost Siege, Mind Bend, Furnace of Rath, Doubling Season — and **Donate**'s
mixed-kind multi-target + rest-of-game layer-2 control-shift [CR#611.2a,613.1b,611.2c]
notes) for `semantics-v3`.

**Historical scope (2026-07-19):** Retire `idris/src/Cards.idr` by migrating its
coverage into a RON acceptance corpus. Card correctness is now gated by Lean
(`cargo xtask lean check`); the hand-authored Idris card list is a
typecheck-only leaf superseded by that path.

## Why it's safe, and the one catch

`Cards.idr` is a **leaf**: nothing imports it, no Rust/test/xtask reads it, the
re-emit gate never pulls it in (`idris_check` probes import `Core` only), and CI
never compiles Idris. Retiring is mechanically just: delete the file + drop
`Cards` from the `idris/mtg.ipkg` modules list (+ two stale `--` comments at
`Spec.idr:684`, `Semantics.idr:2315`, and `VERIFY.md` prose).

The catch: those 71 hand-authored cards are the **only** thing exercising
certain `Semantics.idr` constructor arms — because the arm's real card isn't in
canon yet. `Cards.idr` reached those arms by writing Idris *directly*, bypassing
the Rust→Idris bridge. So a straight delete drops model-arm coverage that the
re-emit path cannot replace until each card is authored on the Rust side. The
fix is to migrate, not just delete.

## Card inventory (71 modeled, 11 already canon)

Of the 60 not-yet-canon Idris-modeled cards:

- **38 CANONIZABLE-NOW** — grammar arm exists AND emit is clean. Authoring them
  Rust-side recovers their Semantics.idr-arm coverage through the enforced bridge.
- **22 BLOCKED** — the Rust side cannot reach the arm today (so their coverage
  is *already* unreachable via re-emit regardless of this ticket):
  - **13 lack a Rust card-grammar arm** — Time Walk (extra-turn verb), Mindslaver
    (control-a-*player*), Vodalian Illusionist (phase-out verb), Pine Walker
    (morph/turn-face-up), Iona / Steely Resolve / Cavern of Souls / Meddling Mage
    / Citadel Siege / Outpost Siege (as-enters chosen-value / mode), Mind Bend
    (word-class text-change), Furnace of Rath + Doubling Season (`ReplaceAmount` /
    event-amount read). Each is its mechanic's open ticket
    (`engine-turn-modification`, `engine-control-another-player`,
    `engine-phasing`, `engine-face-down`, `core-as-enters-choices`,
    `engine-restricted-mana`).
  - **9 have grammar but `idris_emit` GAPS** — Wear//Tear, Brazen Borrower,
    Delver (`Card::TwoFaced` unmapped, `idris_emit.rs:3608` — one arm unblocks
    all three, the cheapest win), Banishing Light + Drudge Skeletons/regen
    (`Action::CreateReplacement`, `:1759`), Approach of the Second Sun
    (`Count::EventCount`, `:1125`), Notion Thief (`EventFilter::Nth`, `:3039`),
    Oblivion Ring (`Reference::Linked`, `:572`), History of Benalia
    (subtype-confer `Property::TurnBased`, `:335`).

## Do (this ticket)

Updated 2026-11-02 (owner): the earlier plan routed these cards through the v1 bridge (a RON
acceptance corpus checked by `idris-check`, emit-bridge arms, renderer arms). The bridge and
`idris-check` are deleted with Idris in `idris-retirement`, so that route is gone.

1. **Author the canonizable cards as `.syl`** under `plugins_v3/canon/cards/`, each passing the
   current `.syl` checks. The inventory above was measured against the v1 bridge; re-sort it by
   what the `.syl` model expresses today.
2. **A card the `.syl` model cannot express yet** is named, with its intent, in its mechanic's open
   ticket (the list above names them), so nothing lives only in `Cards.idr`.
3. **Delete `idris/src/Cards.idr`**, drop `Cards` from `idris/mtg.ipkg`, and fix the two stale
   comments and `VERIFY.md` (all of which `idris-retirement` then deletes).

## Donate — the sole true orphan

**Donate** (`{2}{U}` Sorcery — "Target player gains control of target permanent
you control.") is the one not-yet-canon card whose design intent had **no ticket
anywhere**; it lives only at `Cards.idr:389` (it is
CANONIZABLE): author it in step 1. Two intents it is the worked example for:

- **Mixed-kind multi-target** — slot 0 a player (`APlayer`), slot 1 an object
  ("permanent you control"); the two sorted anaphors disambiguate by noun, no
  labels.
- **Rest-of-game continuous control shift** — continuous `Modify(Target 1,
  GainControl(Target 0))` with no stated duration lasts until end of game
  [CR#611.2a]; control-changing applies in layer 2 [CR#613.1b]; the affected
  object is fixed at resolution [CR#611.2c]. Routes through
  `Modification::SetController` (`idris_emit.rs:2149`), which already
  round-trips. (The *one-shot* `Action::GainControl` at `action.rs:236` is a
  separate `idris_emit` gap at `:1738` — a decision for `idris-mirror-enum-gaps`,
  not needed by Donate.)

Standard constraints apply.
