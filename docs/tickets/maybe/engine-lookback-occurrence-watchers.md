---
needs: [engine-lki-robustness]
---
# Look-back triggers see every object that left in the same occurrence

## Why

`scan_triggers` (`crates/engine/src/trigger.rs`) walks an `Occurrence::Batch` and calls
`scan_event` once per fact. `scan_event` rebuilds the watcher set for each fact from live zones
plus a single `Watcher::Leaving` snapshot for that fact's own leaver. An object that left the
battlefield in the same batch is by then a live watcher in its new zone, and the zone gate drops
its battlefield trigger. So when Blood Artist and two other creatures die to one destroy-all, Blood
Artist triggers once instead of three times.

Look-back triggers use "the existence of those abilities and the appearance of objects immediately
prior to the event" [CR#603.10]. In the [CR#603.10a] example, the artifact's ability "triggers
twice, even though the artifact goes to its owner's graveyard at the same time as the creatures",
and Blood Artist's ruling says it triggers for each creature that dies with it. Fixing this
changes how watchers are assembled, so claiming it opens a design discussion with the owner.

## Done when

- The owner has settled the watcher shape: per-fact co-leaver snapshots, or one pre-event watcher
  image per occurrence used for every look-back class (which also keeps an object that entered in
  the same batch from seeing a look-back event).
- The owner has settled how a trigger's look-back class is decided: from its `EventFilter` shape,
  or by a declared flag.
- The co-leaver snapshots match only look-back triggers [CR#603.10a..603.10f], and the change
  keeps `batch_fired` (a "one or more" trigger fires once per occurrence, [CR#603.2c]) and the
  `Blocked` dedup in the same loop correct.
- Tests:
  - Blood Artist and two other creatures die to one destroy-all: three triggers.
  - The [CR#603.10a] example: a "whenever a creature dies, you gain 1 life" artifact destroyed with
    two creatures triggers twice.
  - A co-leaver's snapshot does not fire a non-look-back trigger, such as "whenever a creature
    enters", for another member of the batch.

## Pointers

- `crates/engine/src/trigger.rs` (`scan_triggers`, `scan_event`, `Watcher`)
- `docs/tickets/critical/engine-lki-robustness.md`: co-leaver snapshots need derived
  characteristics and abilities for the match to be right.
- `docs/tickets/planned/engine-combat-block-status.md`: changes the `Blocked` dedup.
