---
needs: [lean-v3-proofs-and-cards, cleanup-step6-surface]
---

# Four card shapes `.syl` cannot yet write

## Why

Four Vintage-legal card shapes have no faithful `.syl` reading, each waiting on a
decision:

- *Alaundo the Seer*: the ability it grants ("It gains 'When the last time counter is removed
  from this card, …'") reads "it" and "this card" inside the grant, where nothing introduces an
  antecedent. Whether a granted ability's bearer is published into the granted ability's own
  reference scope is undecided.
- *Tahngarth, First Mate* needs the same granted-ability reading to be written whole.
- "All cards with the same name" (*Extirpate*, *Counterbore*, *Lobotomy* and others): `allOf` and
  `withTheSameName` exist, but no canon card writes the "same name as that card" search, so the
  reading is unchecked.
- The linked-exile law: `exiledWith` reads the cards its linked ability exiled [CR#607.2a], but
  nothing checks that the same card carries the exiling ability. Whether that is a law on `Card`
  (a check over the card's abilities) or an unchecked convention is undecided.

Written against the surface (`cleanup-step6-surface`).

## Open questions

- Whether a granted ability's bearer is published into the granted ability's own reference
  scope (*Alaundo the Seer*, *Tahngarth, First Mate*).
- Whether the linked-exile law is a law on `Card` (a check over the card's abilities) or an
  unchecked convention.

## Done when

- *Alaundo the Seer*, *Tahngarth, First Mate* and one "all cards with the same name" card are in
  `plugins/canon/cards/` with their printed text in full, or each
  refusal names the rule that forbids it.
- The linked-exile law is checked, or `docs/decisions/semantics.md` §4.8 says it is a convention.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

*Vedalken Squirrel-Whacker*'s "base power or base toughness" exchange: it is an
Un-set card, which is deferred.

## Pointers

`plugins/builtin/primitives/zone/exiledWith.syl`,
`plugins/builtin/templates/noun/withTheSameName.syl`,
`plugins/builtin/templates/determiner/allOf.syl`, `docs/decisions/semantics.md` §2.3, §2.4 and
§4.8.
