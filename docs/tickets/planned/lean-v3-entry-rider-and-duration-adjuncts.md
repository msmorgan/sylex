---
needs: [plugins-v3-spelling-conventions, semantics-rust-rewrite]
---
# Entry Riders and Durations as surface adjunct nodes

Standard constraints apply. Lean first, in `lean/Semantics/`.

Today an Entry Rider is an entry in `Action.move`'s `riders` list, and an effect's Duration is a
field (`establish (spec) (duration : Option Duration)`, `gainDesignation … duration`). Oracle text
does not attach them that way:

- **Order varies.** "tapped under your control" appears on 45 cards, "under your control tapped"
  on 9.
- **Coordination varies with no change in meaning.** Vivid Marsh "enters tapped with two charge
  counters on it"; Sphere of the Suns and Noble's Purse "enter tapped and with three … counters
  on it"; 14 cards with "with … and with …" (Anavolver, Pouncing Wurm, Voidpouncer); Phoenix
  Chick "tapped and attacking with a +1/+1 counter on it".
- **A fronted Duration scopes over a Coordination.** Wild Shape: "Until end of turn, target
  creature you control has …, becomes …, and gains …", one Duration over three Verb Phrases. 261
  cards open a sentence with "Until end of turn,". A field on each verb cannot say that.

## Decisions already made

- **Adjunct nodes.** Entry Riders (Oracle English: Depictive Adjunct, or a PP Adjunct) and effect
  Durations (Duration Phrase) become Action nodes in their printed position. They wrap the Action
  they modify, or the Coordination they scope over. This follows semantics-v3 §0.1: constructions
  occupy their surface positions.
- **Nesting records order.** Riders wrap in printed order, so the checker steps them left to right
  and a rider's Anaphor ("on it") follows its Antecedent.
- **Wording differences live in Template choice, not nodes.** There is no `and` node over riders.
  A coordinated wording ("tapped and with …") and the side-by-side one ("tapped with …") are two
  Templates over the same nodes, so both expand to an identical AST. A fronted and a trailing
  Duration over the same scope are likewise two Templates.
- **Pile Placements** (`inOrder`, `randomOrder`) stay as they are. A move carries the Entry Riders
  or the Pile Placements its destination allows, never both (game-model glossary, Pile
  Placement).
- **The checker checks attachment by sort.** It refuses:
  - an Entry Rider on anything but a move to the battlefield or a Token's creation;
  - a Duration on anything that establishes no Continuous Effect [CR#611.2a].
- **No lowering work.** semantics_v3 is disconnected from other layers.

## Change

- **AST (Rust `semantics`):**
  - Add the adjunct nodes.
  - Remove `move`'s `riders` and every Action field that holds a printed effect Duration. Audit
    each `duration : Option Duration` on an Action: `delay`'s window is a different thing.
  - Remove the riders on Token creation, if any.
- **`discourse`:** the attachment refusals, and stepping in surface order.
- **Templates:**
  - Rewrite the Templates that took `riders` or `duration` (`putOntoBattlefield`,
    `returnToBattlefield`, `enter`, `create`, `gain`, `gainHaste`, `getsPt`, `becomes`, …) as bare
    Templates plus adjunct Templates.
  - Add a coordinated variant only where a card prints one.
  - Resolve the `TODO(spelling)` markers this ticket owns (Entry Riders and Durations), following
    `plugins-v3-spelling-conventions`.
- **Canon:**
  - Re-encode every card that used `riders` or a `duration` field.
  - Add witnesses for each shape above: Vivid Marsh, Sphere of the Suns, Phoenix Chick,
    Anavolver, and a fronted Duration over a Coordination.
- **ADR:** update semantics-v3 where `move`, `establish` and Entry Riders are described.

## Acceptance

- Each witness reads to its intended nodes. Vivid Marsh and Sphere of the Suns expand to an
  identical AST.
- Tests for the attachment refusals.
- The `semantics` and `discourse` tests pass. Report the canon cards whose encoding changed.
