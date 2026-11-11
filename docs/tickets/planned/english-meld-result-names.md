---
needs: []
---
# Read the meld result's name in *meld them into [name]*

## Why

Another card's name reads only where a position opts in to it, after *named* or *Partner with*
(`docs/decisions/lexicon.md` §6), and a meld card's own text names its meld result outside those
positions: *exile them, then meld them into Brisela, Voice of Nightmares* (Gisela, the Broken
Blade). The seven units that print it are deferred in `crates/english/deferred.txt` under this
ticket.

## Design

- The meld result's name is in scope as the face's own name only on the card of its meld pair
  that prints the meld ability [CR#701.42a]: Gisela, the Broken Blade; Graf Rats; Hanweir
  Battlements; Mishra, Claimed by Gix; Titania, Voice of Gaea; Urza, Lord Protector; Vanille,
  Cheerful l'Cie. Its partner never mentions the result and does not have it in scope.
- The pairing comes from the card data: mtgjson names each meld part with its result,
  `Gisela, the Broken Blade // Brisela, Voice of Nightmares` (layout `meld`, side `a`).
- The name reads through the face's scope like any own name, with no construct, gate or list of
  its own. If the design needs special-casing of meld cards beyond their scope, the implementer
  stops and reports.

## Done when

- The deferred meld units read, their deferral lines are gone, and `cargo xtask english strata
  --check` passes.

## Pointers

- `crates/english/deferred.txt`, `crates/english/src/corpus.rs` (`Face::scope`)
- `crates/english/grammar/english.fug` (`category Name`)
