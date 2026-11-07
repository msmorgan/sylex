---
needs: []
---
# Constructed deck validation

## Why

Nothing checks that a deck is legal. A constructed deck has at least 60 cards and at most four of
any card with a particular English name other than basic lands [CR#100.2a]; a sideboard has at
most fifteen cards and the four-card limit counts deck and sideboard together [CR#100.4a]; the
format's banned and restricted lists limit cards further. The decklists today are
`crates/matchups` `DeckSpec` values (the historical lists) and the decklist reader in
`v1/crates/plugin/src/deck.rs`. Per-format legalities are in mtgjson (`cardLegalities` in
`data/mtgjson/AllPrintings.sqlite`, one column per format), but no current crate reads any column
but Vintage's.

Deck legality is worth little over a small card pool: this waits until the engine represents a
much larger slice of the in-scope cards.

## Done when

- A format model names the formats validated, and validation reads their banned and restricted
  status from mtgjson legalities as derived data, never a hand-kept list.
- A deck and sideboard are checked against [CR#100.2a,100.4a] and the format's lists, with each
  violation named in the error.
- The historical `DeckSpec` lists validate, and tests cover each violation kind with real cards.

## Pointers

- `crates/matchups/src/deck.rs`, `crates/matchups/src/wc99.rs`
- `crates/catalogs/src/cards.rs` (the Vintage legality read)
- `data/mtgjson/AllPrintings.sqlite` (`cardLegalities`)
