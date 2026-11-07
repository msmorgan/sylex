---
needs: [plugins-v3-spelling-conventions, lean-retirement]
---
# Entry Riders and Durations as surface adjunct nodes

## Why

Entry Riders are `List(MoveRider)` arguments in four places: `move`'s `riders`, `token`'s
`riders`, `withRiders` (a replaced entering) and event patterns (`not(rider)` exists only there).
An effect's Duration is a field (`establish(spec, duration)`, `gainDesignation(…, duration)`).
Oracle text attaches neither that way:

- Order varies: "tapped under your control" and "under your control tapped" both print.
- Coordination varies with no change in meaning: *Vivid Marsh* "enters tapped with two charge
  counters on it"; *Sphere of the Suns* "enters tapped and with three charge counters on it";
  *Anavolver* "with two +1/+1 counters on it and with flying"; *Phoenix Chick* "tapped and
  attacking with a +1/+1 counter on it".
- A fronted Duration scopes over a Coordination: *Wild Shape*'s "Until end of turn, target creature
  you control has …, becomes …, and gains …". A field on each verb cannot say that, and
  `plugins/canon/cards/Wild Shape.syl` repeats `untilEndOfTurn` per mode.

## Design

- Entry Riders and Durations are wrapping nodes around the Action they modify, not list or field
  arguments.
- One set of rider nodes serves moves, token creation, replaced enterings (`withRiders`) and event
  patterns that match an entering by its riders, so the static *Vivid Marsh* and an instruction
  "put it onto the battlefield tapped" share one "tapped". The same nodes ride moves that don't go
  to the battlefield ("exile it with three time counters on it", "exile it face down").
- There is no `and` node over riders: the coordinated wording ("enters tapped and with three
  charge counters", *Sphere of the Suns*) and the side-by-side one ("enters tapped with two charge
  counters", *Vivid Marsh*) are two Templates over the same nodes, as are a fronted and a trailing
  Duration. A coordinated variant is added only where a card prints one.
- A Duration node can wrap a whole Coordination of Verb Phrases (*Wild Shape*'s fronted "Until end
  of turn,").
- The placement of a moved pile ("in any order", "in a random order") stays a field of `move`, at
  its fixed position at the end; a move carries Entry Riders or that placement, as its destination
  allows, never both. `delay`'s window is not a printed effect Duration and stays.
- Two refusals: a rider on a move, creation, entering or pattern whose destination does not allow
  it; and a Duration on anything that establishes no Continuous Effect [CR#611.2a].
- Open question: Vintage-legal Oracle prints both "tapped under your control" (17 cards) and
  "under your control tapped" (5). Either riders nest in printed order (the term mirrors the text
  and read order is printed order), or there is one canonical nesting with Templates spelling both
  orders (one term per meaning).
- Open question: "Pile Placement", the name the field has had, is in no glossary; is it the term,
  and does it go in the game-model glossary?

## Done when

- The adjunct primitives exist, and `move`, `token`, `withRiders` and the event patterns take them
  instead of rider lists; `move` keeps its pile-placement field.
- No Action has a field holding a printed effect Duration.
- The declarations that took `riders` or `duration` (`putOntoBattlefield`, `returnToBattlefield`,
  `enter`, `create`, `gain`, `gainHaste`, `getsPt`, `become`, …) are bare declarations plus adjunct
  Templates, and the `TODO(spelling)` markers about Entry Riders and Durations are resolved under
  `plugins-v3-spelling-conventions`.
- Every canon card that used a `riders` list or a `duration` field is re-encoded, and canon has
  *Vivid Marsh*, *Sphere of the Suns*, *Phoenix Chick* and *Anavolver*; *Vivid Marsh* and *Sphere of
  the Suns* expand to an identical entering.
- The adjunct primitives' doc comments state the two refusals in Design.
- `semantics.md` (§2.3's Entry Rider, §4.7's `withRiders`) and the game-model glossary's **Entry
  Rider** and **Duration** entries describe the new attachment; **Entry Rider** covers the riders
  on moves that do not go to the battlefield, or a term beside it does.

## Not in scope

Enforcing the attachment refusals: no `.syl` checker exists until the Rust AST and the greenfield
discourse checker, which take the Design's refusals as requirements.

## Pointers

- `plugins/builtin/primitives/action/move.syl`, `plugins/builtin/primitives/action/token.syl`,
  `plugins/builtin/primitives/static/withRiders.syl`, `plugins/builtin/primitives/move/not.syl`,
  `plugins/builtin/primitives/zone/` (the `MoveRider` primitives)
- `plugins/builtin/primitives/action/establish.syl`, `plugins/builtin/primitives/action/gainDesignation.syl`
- `docs/contexts/game-model/CONTEXT.md` (Entry Rider, Duration)
