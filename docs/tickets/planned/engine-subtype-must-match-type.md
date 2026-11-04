---
needs: []
---
# Layer 4 keeps only subtypes that match one of the object's types

"An object can't gain a subtype that doesn't correspond to one of that object's types"
[CR#205.3d], and "if an object's card type is removed, the subtypes correlated with that card
type will remain if they are also the subtypes of a card type the object currently has; otherwise,
they are also removed for the entire time the object's card type is removed" [CR#205.1a]. The
engine's layer-4 subtype change (`Modification::Subtypes` in `crates/engine/src/layer.rs`,
around line 1895) adds or sets any subtype whatever the object's types. `idris-retirement`
carried the [CR#205.3d] citation to `primitives/static/typeLine.syl`, which states the rule without
anything enforcing it.

## Settled

- The check is a condition on applying, never on the effect's existence. An effect that adds a
  subtype lasts as long as it says, or until the end of the game when it states no duration
  [CR#611.2a]; no rule ends it for doing nothing at the moment. Whenever layers are applied, it
  adds its subtype only if the object has a corresponding type at that point, and does nothing
  otherwise.
- Correspondence is read from the `Subtype` being added (`Subtype::types` [CR#205.3c]): a subtype
  is addable to an object that has one of its types. There is no unregistered-subtype case.
- Effects apply in layer 4 in timestamp order [CR#613.1d,613.7], so the check reads the object's
  types as earlier layer-4 effects, and this effect's own type change, have left them: "becomes an
  artifact creature Golem" keeps Golem.

## Witness (owner, 2026-11-02; rules check against the CR)

Amass Orcs 1, then amass Goblins 1 on the same token: a 0/0 Orc Goblin Army creature token with
two +1/+1 counters ([CR#701.47a]: "it becomes a [subtype] in addition to its other types").
Enchant it with *Imprisoned in the Moon* ("Enchanted permanent is a colorless land with '{T}: Add
{C}' and loses all other card types and abilities"): while enchanted it is a colorless land with no
subtypes; Orc, Goblin and Army are removed with the creature type [CR#205.1a]. Destroy the Aura: it
is a 2/2 black Orc Goblin Army creature token again, because amass's Goblin effect never ended.

## Change

- In the layer-4 subtype arm, `Add` and `Set` keep a subtype only if one of its correlated types is
  among the object's types at that point; cite [CR#205.3d] there.
- When layer 4 removes a card type, remove the subtypes correlated only with it [CR#205.1a] (check
  whether the engine already does; cite it where it does).
- Tests in `sylex_engine`:
  - the witness above, through all three states;
  - a creature subtype added to a noncreature is not gained, and is gained once the object becomes
    a creature while the effect lasts (the reverse timestamp order);
  - a subtype added together with its type in one effect is kept;
  - `Set` drops subtypes that match none of the object's types;
  - a subtype matching any one of several types is kept.
