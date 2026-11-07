---
needs: []
---
# Layer 4 keeps only subtypes that match one of the object's types

## Why

An object can't gain a subtype that doesn't correspond to one of its types [CR#205.3d]. Removing
a card type also removes the subtypes correlated only with it [CR#205.1a]. The layer-4 arms in
`crates/engine/src/layer.rs` enforce neither:

- `Modification::Subtypes` adds or sets any subtype, whatever the object's types;
- `Modification::CardTypes(CollectionOp::Remove(_))` and `Set` change only `card_types` and leave
  every subtype in place;
- `resolve_subtype` turns a subtype name missing from the registry into a name-only `Subtype`
  with no types. That is a silent fallback; every subtype is registered with its types.

`plugins/builtin/primitives/static/typeLine.syl` states [CR#205.3d], but nothing enforces it.

## Done when

- In the layer-4 subtype arm, `Add` and `Set` keep a subtype only if one of its correlated types
  (`Subtype::types` [CR#205.3c]) is among the object's types at that point. The arm cites
  [CR#205.3d].
- The check gates applying, never the effect's existence: an effect that adds a subtype lasts as
  long as it says [CR#611.2a], and each time layers are applied it adds its subtype only if the
  object then has a corresponding type.
- The check reads the object's types as earlier layer-4 effects and this effect's own type change
  have left them [CR#613.1d,613.7]: "becomes an artifact creature Golem" keeps Golem.
- When layer 4 removes or replaces card types, it removes the subtypes correlated only with the
  removed types, citing [CR#205.1a].
- `resolve_subtype` fails loudly on an unregistered name.
- `sylex_engine` tests:
  - Amass Orcs 1, then amass Goblins 1 on the same token: a 0/0 black Orc Goblin Army creature
    token with two +1/+1 counters [CR#701.47a]. Enchanted with Imprisoned in the Moon, it is a
    colorless land with no subtypes [CR#205.1a]. With the Aura destroyed, it is a 2/2 black Orc
    Goblin Army creature token again, because the amass Goblins effect never ended;
  - a creature subtype added to a noncreature is not gained, and is gained once the object becomes
    a creature while the effect lasts (the reverse timestamp order);
  - a subtype added together with its type in one effect is kept;
  - `Set` drops subtypes that match none of the object's types;
  - a subtype matching any one of several types is kept.

## Pointers

- `crates/engine/src/layer.rs` (`apply_static`'s layer-4 arms, `resolve_subtype`).
- `crates/core/src/type.rs` (`Subtype::types`).
