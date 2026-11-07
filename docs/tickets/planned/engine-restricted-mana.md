---
needs: [engine-special-actions]
---
# Engine: the spend restrictions `SpendOnly` cannot express

## Why

Mana a spell or ability produces can carry a restriction on how it is spent [CR#106.6]. The
engine has one shape for this: `ManaRider::SpendOnly(Predicate)` (`crates/core/src/mana.rs`), which
`unit_spendable_on` (`crates/engine/src/cast.rs`) checks against the spell or ability being paid
for. That covers Ancient Ziggurat's "only to cast a creature spell" and Mishra's Workshop's
artifact-spell rider. Three kinds of printed restriction do not fit it:

- **The cost, not the object.** Rosheen Meanderer ("only on costs that contain {X}"), Snowfall and
  Adarkar Unicorn ("only to pay cumulative upkeep costs"), Unblinking Observer ("to pay a disturb
  cost"), Cultivator Drone ("pay a cost that contains {C}") and Jegantha, the Wellspring ("can't be
  spent to pay generic mana costs") judge the cost or pip being paid. A predicate over the object
  being paid for cannot see it.
- **The producer.** Thieving Varmint ("spells you don't own") and Ice Cauldron ("the last card
  exiled with this artifact") read the mana's producer, and "of the chosen type" reads a choice
  its producing ability made. A unit's riders carry no producer, as `unit_spendable_on`'s comment
  notes.
- **Special actions.** Overgrown Zealot ("only to turn permanents face up") and Niko Defies
  Destiny ("to foretell cards") restrict mana to a special action's payment [CR#116.2b,116.2h].
  `PaymentSubject` (`crates/engine/src/payment.rs`) has no special-action subject.

## Done when

- A spend restriction can judge the cost or pip a unit pays, its producer and that producer's
  choices, and a special action's payment, alongside the object judgment `SpendOnly` makes today.
- `sylex_engine` tests pay restricted mana into a matching and a non-matching payment for each
  kind, with the real cards above.

## Not in scope

- Spend riders that grant an effect or trigger when the mana is spent (`GrantOnSpend`,
  `TriggerOnSpend`): those change what happens, not what may be paid.
- Offering and taking special actions themselves: `engine-special-actions`.

## Pointers

- `crates/core/src/mana.rs` (`ManaRider`)
- `crates/engine/src/cast.rs` (`unit_spendable_on`, `spendable_pool`)
- `crates/engine/src/payment.rs` (`PaymentSubject::spend_object`)
- `crates/engine/tests/mana_riders.rs`
- `plugins/builtin/primitives/mana/` (`spendOnly`, `cantSpendOn` and the `SpendPurpose`
  primitives)
