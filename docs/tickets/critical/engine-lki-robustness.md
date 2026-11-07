---
needs: []
---

# Engine: `LkiSnapshot` captures the object's derived characteristics and controller, and every snapshot read uses them

## Why

`LkiSnapshot` (`crates/engine/src/lki.rs`) stores the card spine, the base controller, tap state,
damage, attachment host, counters and the zone left. Its doc still says characteristics derive
from the card definition because no continuous effects exist; layers 2 and 4–7 now exist.
Every characteristic read of a departed object therefore reads the printed face:

- `snapshot_has_type` and `snapshot_stat` (`crates/engine/src/trigger.rs`) and the other
  `filter_matches_snapshot_with_activation` leaves read `snapshot_face`. A land animated into a
  creature that dies fails "whenever a creature dies"; a creature that lost a type still
  matches it.
- `last_known_stat` (`crates/engine/src/resolve/count.rs`) is printed power and toughness plus
  +1/+1 and -1/-1 counters. Fling ignores an anthem or Giant Growth on the sacrificed creature,
  though its ruling says to "use the sacrificed creature's power as it last existed on the
  battlefield" [CR#608.2h].
- A stolen creature's snapshot names its owner as controller.

Look-back triggers use "the appearance of objects immediately prior to the event" [CR#603.10].

## Design

- Every ticket that adds a captured fact to the snapshot uses one shared capture shape; whether
  that shape is an enumerated struct or a per-characteristic map is the implementer's call.

## Done when

- `LkiSnapshot::capture` records the object as it last existed in the zone it left
  [CR#608.2h], from `state.layers()`: card types, subtypes, supertypes, colors, name, mana value,
  power, toughness and abilities (keywords included), plus its controller and which face is up.
  Every snapshot reader reads them; no snapshot read uses the printed face. The stale doc
  paragraph is gone.
- `snapshot_stat_reads_printed_stats` (`trigger.rs`) is rewritten to assert a derived read (a
  pumped creature's snapshot reports its pumped power).
- Tests: a creature pumped by an anthem is sacrificed to Fling, and Fling deals the pumped
  power; an animated land dies and "whenever a creature dies" triggers; a creature stolen by a
  layer-2 effect dies and its snapshot's controller is the thief.

## Pointers

- `crates/engine/src/lki.rs`
- `crates/engine/src/trigger.rs` (`snapshot_face` and its readers)
- `crates/engine/src/resolve/count.rs` (`last_known_stat`)
- `engine-snapshot-predicate-breadth`, `engine-snapshot-object-classification` and
  `engine-snapshot-combat-state` add captured facts on the same shape.
