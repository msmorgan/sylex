---
needs: [cleanup-step6-surface]
specs: [kind-sorts-and-position-unions]
---

# A word for "with a name originally printed in" an expansion

## Why

Three Vintage-legal cards test an object's name against the expansion that first printed it:

- *Golgothian Sylex*: "Each nontoken permanent with a name originally printed in the Antiquities
  expansion is sacrificed by its controller."
- *City in a Bottle*: "Whenever one or more other nontoken permanents with a name originally
  printed in the Arabian Nights expansion are on the battlefield, their controllers sacrifice
  them." and "Players can't cast spells or play lands with a name originally printed in the
  Arabian Nights expansion."
- *Apocalypse Chime*: "Destroy all nontoken permanents with a name originally printed in the
  Homelands expansion."

The test reads the object's name, not the card's own printing [CR#206.3]: the CR lists the names
each expansion covers [CR#206.3a..206.3c], and a reprint with another expansion symbol still
matches. A copy of *Mishra's Factory* matches through its copied name [CR#707.2], and an object
with no name, such as a face-down permanent [CR#708.2a], never matches. No word
says this, so the three cards are unwritable. A card-level printing record would give the wrong
answer for copies.

## Design

- An `Expansion` sort with one primitive per expansion the CR lists in [CR#206.3a..206.3c]:
  `arabianNights`, `antiquities`, `homelands`.
- A primitive at `PrepositionPhrase`, `nameOriginallyPrintedIn(expansion: Expansion)`, spelled
  "with a name originally printed in the <expansion> expansion". It applies to any object with a
  name, so it covers the spells and lands in *City in a Bottle*'s second ability as well as
  permanents.

## Done when

- The `Expansion` sort and its three primitives exist under `plugins/builtin/primitives/`, and
  the predicate exists under `plugins/builtin/primitives/predicate/`.
- *Golgothian Sylex*, *City in a Bottle* and *Apocalypse Chime* are in `plugins/canon/cards/`, each
  test written with that predicate, so every expansion word has a caller.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check`, `cargo test -p xtask` and
  `cargo xtask cite check` pass.

## Not in scope

- The name lists and the engine's evaluation of the predicate: `engine-name-originally-printed-in`.
- Cards that test an expansion symbol or a printing rather than a name (*World-Bottling Kit*,
  *Sheoldred's Terror*), none of them Vintage-legal.

## Pointers

`plugins/builtin/primitives/predicate/named.syl`, `plugins/builtin/primitives/parity/` (a small
sort), `docs/tickets/planned/catalogs-into-plugins.md` (its Open bullet on expansion names).
