---
needs: []
---

# A printed coordination of abilities is one list at the grant word

## Why

`gain` and `abilityGrant` took one `ability: Ability`, so no card could write "gains hexproof and
indestructible" (*Plaza of Heroes*), and every canon card that prints a coordination of granted
abilities split it into several grants, most of them through an anaphor the card does not print:
*Efflorescence*'s and *Zidane, Tantalus Thief*'s "gains … and …", *Bloodfire Enforcers*'s,
*Embercleave*'s and Arlinn's emblem's "has … and …", *Arlinn Kord*'s "gains vigilance and haste",
and blitz's "it has haste and '…'" [CR#702.152a], which the body wrote as two static abilities
where the rule defines one. The words that lose or list several abilities already took a list:
`abilityLoss`, `creatureToken`, `getEmblem` and the `addedAbilities` primitive that the grant
words expand to.

## Design

- The grant words take `abilities: List(Ability)`, in printed order: `gain(subject, abilities,
  duration)`, `abilityGrant(subject, abilities)` and the copy exception `alsoHas(abilities)`
  ("except it has haste and dethrone"). A grant of one ability is a list of one. Each granted
  element is added as an ability of its own, in layer 6 [CR#113.2c,613.1f]; each one a copy
  exception lists becomes part of the copy's copiable values [CR#707.9b].
- The form is a list, not an `and` at `Ability`, for three reasons. A list is the one written
  form of a printed coordination of abilities wherever it stands: grant (`gain`, `abilityGrant`,
  `alsoHas`), loss (`abilityLoss`), token (`creatureToken`) and emblem (`getEmblem`) all take
  one, where an `and` would give every list-of-Ability position a second form
  (`[and([flying, vigilance])]` beside `[flying, vigilance]`) and one printed "with flying and
  vigilance" would recover as two terms. The rules have no ability made of several: each is an
  ability of its own [CR#113.2c], so an `and` would name nothing the rules know, in every Ability
  position (a card's text line, an ability word, a conferral). And a one-element list prints its
  element alone (`docs/decisions/spelling.md` §7), so a grant of one ability needs no second
  form.
- "Gains your choice of A or B" is a choice of one ability, not a coordination, and is not
  written here.
- `gainHaste` is retired: "gains haste" is `gain(subject, [haste], duration)`, one written form.

## Done when

- `gain`, `abilityGrant` and `alsoHas` take a list; every caller under `plugins/` is respelled,
  and the split grants above are each one grant over the printed list.
- Blitz's body is the three abilities [CR#702.152a] defines.
- *Plaza of Heroes* is authored.
- `docs/decisions/semantics.md` §4.7 states that abilities granted, lost or listed together are
  one list.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`plugins/builtin/templates/action/gain.syl`, `plugins/builtin/templates/ability/abilityGrant.syl`,
`plugins/builtin/primitives/static/alsoHas.syl`,
`plugins/builtin/templates/ability/abilityLoss.syl`,
`plugins/builtin/templates/noun/creatureToken.syl`.
