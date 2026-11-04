---
needs: [plugins-v3-spelling-conventions, lean-retirement]
---
# Entry Riders and Durations as surface adjunct nodes

Standard constraints apply. Done in `.syl` and Rust (`semantics`, `discourse`) after
`semantics-rust-rewrite`; the Lean names below are the current AST, which that epic ports.

Today Entry Riders are entries in a `MoveRider` list in four places:
- `Action.move`'s `riders`;
- a Token's creation;
- a replaced entering, `Replacement.withRiders`. *Vivid Marsh* and *Sphere of the Suns* are
  statics of this kind, "This land enters tapped …";
- an event pattern that matches an entering by its riders (`MoveRider.not` exists only there).

An effect's Duration is a field (`establish (spec) (duration : Option Duration)`,
`gainDesignation … duration`). Oracle text does not attach either of them that way:

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
  Durations (Duration Phrase) become nodes in their printed position. They wrap the Action they
  modify, or the Coordination they scope over. This follows semantics.md §1: constructions
  occupy their surface positions.
  - The same rider nodes serve all four places above. A replaced entering's riders wrap the
    entering the replacement describes, and a pattern's riders wrap the entering it matches. So
    the static *Vivid Marsh* and an instruction "put it onto the battlefield tapped" share one
    representation of "tapped".
- **Nesting records order.** Riders wrap in printed order, so the checker steps them left to right
  and a rider's Anaphor ("on it") follows its Antecedent.
- **Wording differences live in Template choice, not nodes.** There is no `and` node over riders.
  A coordinated wording ("tapped and with …") and the side-by-side one ("tapped with …") are two
  Templates over the same nodes, so both expand to an identical AST. A fronted and a trailing
  Duration over the same scope are likewise two Templates.
- **Pile Placements stay a field on `move`.** `inOrder` ("in any order", "in a random order")
  keeps its fixed position at the end of the move, so it stays a field: `riders` is split, not
  removed. A move carries the Entry Riders or the Pile Placements its destination allows, never
  both (game-model glossary, Pile Placement).
- **Riders on other moves are nodes too.** `withCounters` and `faceDown` also ride moves that do
  not go to the battlefield ("exile it with three time counters on it", "exile it face down").
  They are printed adjuncts like the rest, so they take the same nodes, and the attachment check
  allows them on those moves. The game-model glossary's Entry Rider says it rides "a move to the
  battlefield, a Token's creation, or a static ability, and nothing else". Amend it, or add a term
  beside it, with CR citations, via `domain-modeling`.
- **The checker checks attachment by sort.** It refuses:
  - a rider on a move, creation, entering or pattern whose destination does not allow it;
  - a Duration on anything that establishes no Continuous Effect [CR#611.2a].
- **No lowering work.** semantics_v3 is disconnected from other layers.

## Change

- **AST (Rust `semantics`):**
  - Add the adjunct nodes.
  - Split `move`'s `riders`: Pile Placements stay in a field of their own, and every other rider
    becomes a node.
  - Move the rider lists of Token creation, `Replacement.withRiders` and event patterns to the
    same nodes.
  - Remove every Action field that holds a printed effect Duration. Audit each
    `duration : Option Duration` on an Action: `delay`'s window is a different thing.
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
- **ADR:** update `semantics.md` where `move`, `establish`, `Replacement.withRiders` and Entry
  Riders are described.
- **Glossary:** amend the game-model entries Entry Rider, Pile Placement and Duration for their new
  attachment, and the Entry Rider scope above, via `domain-modeling`.

## Acceptance

- Each witness reads to its intended nodes. Vivid Marsh and Sphere of the Suns expand to an
  identical AST.
- Tests for the attachment refusals.
- The `semantics` and `discourse` tests pass. Report the canon cards whose encoding changed.
