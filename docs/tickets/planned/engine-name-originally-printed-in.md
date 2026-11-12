---
needs: []
---

# Evaluate "with a name originally printed in" an expansion

## Why

*Golgothian Sylex*, *City in a Bottle* and *Apocalypse Chime* test whether an object's name was
originally printed in Antiquities, Arabian Nights or Homelands. The CR fixes each set of names as
a closed list [CR#206.3a..206.3c]. The test reads the object's current name, including a name it
has through a copy effect [CR#707.2]. An object with no name, such as a face-down permanent
[CR#708.2a], has no name to match. Core has no predicate for this and the engine has no list to
check it against.

## Design

- `sylex_catalogs` extracts the three name lists from the CR text of [CR#206.3a..206.3c], as it
  extracts the other CR catalogs, so no list is typed by hand. Extraction fails if a listed name is
  not in the card-name catalog.
- A `CharacteristicPredicate` variant carries the expansion. The engine evaluates it against the
  object's current names from the layer system: it matches when any of the object's names is on
  that expansion's list.

## Done when

- The extraction yields the three lists, and a test pins their lengths and a known member of each
  against the CR.
- The engine evaluates the predicate, with tests grounded in *Golgothian Sylex*. Its ability
  sacrifices a *Mishra's Factory* and a *Clone* copying one. It spares a token copy of *Mishra's
  Factory*, a face-down permanent, and a permanent whose name is on another expansion's list.
- `cargo test` and `cargo clippy --tests -- -D warnings` pass for `sylex_catalogs`, `sylex_core`
  and `sylex_engine`, and the cite gates pass.

## Not in scope

- The `.syl` word and the three cards: `syl-name-originally-printed-in`.

## Pointers

`crates/catalogs/src/{cr,kind,set}.rs`, `crates/core/src/filter.rs` (`CharacteristicPredicate`),
`crates/engine/src/layer.rs` (how `Named` reads names).
