---
needs: []
---

# Engine: a replacement body's later part finds the object an earlier part moved

## Why

If an effect moves an object to a public zone, other parts of that effect can find that object
[CR#400.7j]. Ravenloft Adventurer reads "If a creature an opponent controls would die, instead
exile it and put a hit counter on it." Its body lowers to
`Sequentially([Move(Reg(2), Exile), PutCounters(Reg(2), HitCounter, 1)])`: both parts read the
`EventObject` parameter, and a parameter read never chases a move (`activation_product` and
`activation_objects` in `crates/engine/src/activation.rs` chase only instruction products). The
counter part reads the battlefield id the exile retired and puts no counter anywhere.

## Done when

- A part of a replacement body that reads the object an earlier part of the same body moved to a
  public zone acts on that object in its new zone.
- Test: Day of Judgment over two creatures an opponent controls, under Ravenloft Adventurer's
  replacement: each is exiled with a hit counter on it.

## Pointers

- `crates/engine/src/activation.rs` (`activation_product`, `activation_objects`)
- `crates/engine/src/replace_registry/body.rs` (`join_body`)
- `crates/engine/tests/replace_registry.rs` (`a_body_part_after_a_move_follows_the_event`)
