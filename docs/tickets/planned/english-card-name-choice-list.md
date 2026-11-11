---
needs: []
---
# Read the card names Garth One-Eye offers as a choice

## Why

Another card's name reads only where a position opts in to it, after *named* or *Partner with*
(`docs/decisions/lexicon.md` §6). Garth One-Eye lists other cards' names as the values of a
choice: *Choose a card name that hasn't been chosen from among Disenchant, Braingeyser, Terror,
Shivan Dragon, Regrowth, and Black Lotus.* The unit is deferred in `crates/english/deferred.txt`
under this ticket.

## Design

- A card name chosen *from among* a list is another position that opts in to another card's name,
  the same way the predicative complement of *named* does: each item is a card's name used as a
  name, a `NameComplement`, coordinated as after *named*.
- `docs/decisions/lexicon.md` §6 and the glossary's Another Card's Name entry list it beside *named*
  and *Partner with*. If the design needs special-casing of Garth One-Eye or its card names, the
  implementer stops and reports.

## Done when

- Garth One-Eye's unit reads, its deferral line is gone, and `cargo xtask english strata --check`
  passes.

## Pointers

- `crates/english/deferred.txt`
- `crates/english/grammar/english.fug` (`category NameComplement`, `coordination NameComplement`,
  `slot Name`)
