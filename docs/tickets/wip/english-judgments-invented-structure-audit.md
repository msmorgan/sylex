---
needs: []
---

# Judgment entries whose structure no card prints

## Why

A judgment entry may use invented words where that is what it tests (*Put a goddamn counter on
it.* shows that any word names a counter), but the structure it asserts must be one a card
prints. Entries have been added that pin a structure no card has: *Draw cards equal to that many
plus one.* is pinned at two Readings and marked `# from: Winding Constrictor`, which prints no
such sentence. Of the sentence-level entries with no `<card=…>`, 328 have text that occurs
verbatim in no Corpus Unit (233 positive, 95 negative).

## Done when

- Each entry whose text occurs in no Corpus Unit is classed: a real card's wording trimmed or
  with words swapped (kept, with its card named); an invented structure (rewritten onto the card
  that prints the structure, or retired with the reason); a `# from:` that names a card not
  printing the structure (corrected).
- `crates/english/judgments/README.md` states the rule: invented words are allowed where they
  are what the entry tests, invented structure is not.
- The landing reports the counts per class and each retirement with its reason, and
  `cargo test -p sylex_english` passes.

## Pointers

- `crates/english/judgments/`, `crates/english/judgments/README.md`.
- The texts of every unit: `units.text` in `data/derived/english-forests.sqlite`.
