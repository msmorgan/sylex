---
needs: [syl-tocast-noun-phrase]
---
# A card a cast permission names is "that spell" to the text after it

## Why

Two printed texts refer to the card a permission lets you cast as "that spell" / "those spells":

- *Rogue Class* level 3: "You may play cards exiled with this Class, and you may spend mana as
  though it were mana of any color to cast those spells."
- *Summon: Esper Valigarmanda* chapters II–IV: "You may cast an instant or sorcery card exiled
  with this Saga, and mana of any type can be spent to cast that spell."

Casting moves the card to the stack, where it is the spell [CR#601.2a], so reference resolution
has to resolve those anaphors to the permitted card. This is an exception to
`docs/decisions/semantics.md` §2.2 item 3, under which modal `can` closes its hypothetical
participants. Neither card is in the `.syl` corpus; the old fragments re-described the card
instead of writing the anaphor.

## Design

- When a `conjunction` or a `doIf` continuation steps `can(<who>, cast|play(X))`, X's referent is
  published to the later parts with the cast deed recorded, so "that spell" / "those spells" (and
  a subtype-of-spell head) resolves to it as after a performed cast. The permission's own
  determiner fixes plurality: `allOf` gives "those spells" (Rogue Class), `a` gives "that spell"
  (Summon: Esper Valigarmanda). A `can` outside a conjunction or continuation publishes nothing
  new.

## Done when

- Both texts are written in `.syl` with the printed anaphor ("those spells", "that spell") as the
  argument of the second part's `toCast` purpose, and reference resolution resolves each anaphor
  to the first part's referent with the right plurality.
- The second part alone (the spend-mana permission with "those spells" and no permission before
  it) is refused with an unresolved plural `spell` anaphor.
- Tests for a performed play or cast and for keyword cast permissions still pass.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

Letting `toCast` take a noun phrase (today it takes a `Predicate`): that is
`syl-tocast-noun-phrase`.

## Pointers

- `docs/decisions/semantics.md` §2 (reference resolution)
- `plugins/builtin/primitives/mana/toCast.syl`, `plugins/builtin/primitives/static/conjunction.syl`,
  `plugins/builtin/templates/static/can.syl`, `plugins/builtin/primitives/static/spendManaAs.syl`
