---
needs: [syl-primitive-constructors, syl-this-head-template, syl-printed-read-order, syl-card-head-noun-sweep, syl-mode-frame-cataphora, syl-round-each-time, syl-tocast-noun-phrase, syl-empower-token-parameter, syl-token-definitions, syl-numeric-anaphors, syl-room-doors-as-designations, syl-concepts-modeled-twice, syl-each-plural-referent, syl-ability-words-once, syl-dead-weight-sweep, syl-one-declaration-per-keyword, syl-one-exile-declaration, syl-misnamed-declarations, syl-zone-words]
---

# Single-faced cards as an explicit call, and no `CardFace`

## Why

A card file written `card { name = …, … }` is read as `singleFaced(…)` around a `CardFace` whose
`characteristics` wrap the record's fields. Those are two implicit wraps, and `.syl` allows no
implicit wrap but `Option` (`docs/decisions/syl.md` §2 and §3). `cargo xtask dsl check` accepts
them only so the single-faced cards need no respelling. `CardFace` itself is a record whose one
field is `characteristics`, so it adds nothing over `Characteristics`.

Wrapping each card in `singleFaced({ … })` reindents every line of the 929 single-faced canon
cards, so every ticket that edits existing canon card lines lands first. The direct ones are
`syl-this-head-template` (374 cards use a `this<Head>` Template), `syl-printed-read-order`,
`syl-card-head-noun-sweep` (about 45 cards), `syl-zone-words` (53 cards use `hand` or
`graveyard`), `syl-misnamed-declarations` (36 cards), `syl-numeric-anaphors` (60 cards),
`syl-one-declaration-per-keyword` (5 cards), `syl-one-exile-declaration`, `syl-token-definitions`
(3 cards), `syl-tocast-noun-phrase` (3), `syl-empower-token-parameter` (*Countersculpt*),
`syl-mode-frame-cataphora` (2), `syl-round-each-time` (*Saw in Half*),
`syl-room-doors-as-designations` (3), `syl-concepts-modeled-twice`, `syl-each-plural-referent`,
`syl-ability-words-once` and `syl-dead-weight-sweep`; the rest reach it through them.

## Design

- A card face is its `Characteristics`: the card primitives take `Characteristics` where they take
  `CardFace` today, and `CardFace` is gone.
- A single-faced card is written as an explicit `singleFaced({ … })` call, and every face of a
  multi-faced card as its `Characteristics` record, with no `characteristics` wrap. Every card
  value stays the same.

## Done when

- `singleFaced`, `transforming`, `modalDfc`, `adventurer`, `split`, `flip`, `sharedLineSplit`,
  `leveler` and `prototype` (`plugins/builtin/primitives/card/`) take `Characteristics` in place
  of `CardFace`, and `CardFace` is not in `RECORDS` (`crates/xtask/src/dsl_check/schema.rs`).
- Every single-faced card in `plugins/canon/` is written as an explicit call, and every face of a
  multi-faced card is written without the `characteristics` wrap.
- `cargo xtask dsl check` no longer reads a record at `Card`, or a face record without
  `characteristics`, as the shorthand (`crates/xtask/src/dsl_check.rs`), and refuses each with a
  negative test.
- The respelled cards mean the same: a one-off script, kept in `docs/evidence/`, expands the old
  shorthand of every card it respells (the record wrapped in the single-faced call, its fields in
  `characteristics`) and checks that the `sylex_syl` tree equals the respelled card's with the
  `CardFace` layer removed, and the landing reports the card count it compared.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`crates/xtask/src/dsl_check.rs` (the single-face exception),
`crates/xtask/src/dsl_check/schema.rs` (`RECORDS`), `plugins/builtin/primitives/card/`,
`plugins/canon/cards/`, `docs/decisions/syl.md` §2 and §3.
