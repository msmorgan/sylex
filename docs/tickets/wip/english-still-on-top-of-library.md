---
needs: []
---

# English: "if that card is still on top of your library"

## Why

Commander Spock ends its trigger with "If that card is still on top of your library, draw a card."
The census leaves the unit unread: the condition's *still* and its locative predicate *on top of
your library* have no reading together. The rest of the ability reads once its self-reference is
replaced.

## Done when

- The sentence reads with *still* scoping over the locative predicate, and Commander Spock is read
  in `cargo xtask english strata`.

## Pointers

- `cargo xtask english parse "If that card is still on top of your library, draw a card."`
- `docs/decisions/english.md`
