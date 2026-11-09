---
needs: [plugins-v3-data-language, lean-retirement]
---

# Square-bracketed text in `.syl`, so cleave works

## Why

Cleave's second ability changes a spell's text by removing everything in square brackets
[CR#702.148a,702.148b]. `cleave.syl` writes it as `textChange(thisSpell, removeBracketed)`, but a
card's `.syl` term has no way to mark which of its parts are bracketed, so the text change
removes nothing and none of the 12 Vintage-legal cleave cards can be written. The English side
already reads a cleave line twice, as its Uncleaved and Cleaved Text, with the bracketed spans
kept beside it (`crates/english/src/cleave.rs`).

## Open questions

- How a bracketed span is represented in a card's `.syl` term.

## Done when

- `.syl` represents a bracketed span as settled, such that `removeBracketed` can read it from the
  term and spelling can print the brackets back, and at least two cleave cards are in
  `plugins/canon/cards/`.
- That `removeBracketed` removes exactly the bracketed spans of a cleave card's term is not
  checkable until the Rust expansion layer exists; the landing shows it for one card.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`plugins/builtin/definitions/keyword_ability/cleave.syl`,
`plugins/builtin/primitives/static/removeBracketed.syl`, `crates/english/src/cleave.rs`,
`docs/oracle-style-guide.md` §3 (square brackets).
