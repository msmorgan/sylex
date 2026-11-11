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

- The meld result's name is in scope as an own name on both cards of its meld pair
  [CR#712.4a]: on the card that prints the meld ability (Gisela, the Broken Blade; Graf Rats;
  Hanweir Battlements; Mishra, Claimed by Gix; Titania, Voice of Gaea; Urza, Lord Protector;
  Vanille, Cheerful l'Cie) and on its partner, which never names it.
- The pairing comes from the card data, which records each meld part's result for both parts
  alike: Scryfall's `all_parts` entry with component `meld_result`, as mtgjson's
  `Gisela, the Broken Blade // Brisela, Voice of Nightmares` (layout `meld`, side `a`) does.
- The name reads through the face's scope (`corpus::own_names`) like any own name, with no
  construct, gate or list of its own. A judgment's `<card=NAME>` takes the same own names.

## Done when

- The deferred meld units read, their deferral lines are gone, and `cargo xtask english strata
  --check` passes.
- Positive judgments cover the meld units, a negative judgment shows the result's name does not
  read as own on an unrelated card, and the Mishra comma-scope judgment holds on the full text.

## Pointers

- `crates/english/deferred.txt`, `crates/english/src/corpus.rs` (`Face::scope`)
- `crates/english/grammar/english.fug` (`category Name`)
