---
needs: []
---
# Static abilities state the zones they function in

## Why

An ability of an instant or sorcery usually functions only on the stack, and an ability of any
other object usually only on the battlefield. [CR#113.6] lists the exceptions, and a static
ability's effect applies while its object is in the appropriate zone [CR#604.2,611.3b].
Activated and triggered abilities already carry this as data: `from: Option<Zone>` on
`ActivatedAbility` and `TriggeredAbility` (`crates/core/src/ability.rs`), with the battlefield as
the default. Static abilities carry nothing. `gather` in `crates/engine/src/layer.rs` decides for
them out of band: it skips every object that isn't on the battlefield, except an emblem in the
command zone [CR#114.4].

So no static ability functions anywhere else, including:

- characteristic-defining abilities, which function in all zones [CR#604.3,113.6a];
- an ability that states the zones it functions in, or doesn't [CR#113.6b,113.6c]. Grist, the
  Hunger Tide's "As long as Grist isn't on the battlefield, it's a 1/1 Insect creature in addition
  to its other types" functions everywhere except the battlefield. It is not a
  characteristic-defining ability, because it sets characteristics only while a condition holds
  [CR#604.3a];
- stack and casting-related statics [CR#113.6d,113.6e,113.6f,113.6g], entering-related statics
  [CR#113.6h,113.6i], and deck-construction statics [CR#113.6n];
- a max speed ability granting an ability that states its zones [CR#702.178b].

The proposal: every ability carries its function zones in the [CR#113.6] vocabulary. Battlefield
is the default, and one generic check decides whether an ability functions. Card text often
implies the zones ("as long as ~ isn't on the battlefield", "from your graveyard"), so `.syl` can
often infer them. This needs a design discussion with the owner before it is built:

- the vocabulary: one zone, as `from` is today, or a set or an exclusion that matches
  [CR#113.6b,113.6c];
- where the zones are set: authored or inferred in `.syl`, or attached when the engine lowers the
  card;
- whether `from` on activated and triggered abilities becomes the same property.

## Done when

- The owner has agreed a design, and it is written into `docs/specs/` or an ADR.
- `gather` no longer special-cases the battlefield and emblems: every static ability is gated by
  its declared zones.
- `sylex_engine` tests show Grist being a creature in the graveyard and library and not on the
  battlefield, and a characteristic-defining ability applying in a hand.

## Pointers

- `crates/engine/src/layer.rs` (`gather`), `crates/core/src/ability.rs` (`from`).
- `crates/engine/src/trigger.rs`: the triggered-ability zone check.
- `docs/decisions/semantics.md`.
