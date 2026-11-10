---
needs: [cleanup-step6-surface]
---

# A Condition that tests whether an amount is even or odd

## Why

*Chaos Lord* prints "At the beginning of your upkeep, target opponent gains control of this
creature if the number of permanents is even" and "This creature can attack as though it had
haste unless it entered this turn". No word at `Condition` tests an amount's parity: `Parity`
(`even`, `odd`) feeds only `manaValueParity`, a `DeckTrait`, which has no caller. `compareAmt`
compares two amounts and cannot say "is even". So *Chaos Lord* is unwritable, and the `can`
Template has no `unless` parameter, since *Chaos Lord* is the only Vintage-legal card printing
"can … unless" (`docs/tickets/done/syl-cant-unless.md`).

Other Vintage-legal cards print an even/odd test on an amount:

- *Chaos Moon*: "count the number of permanents. If the number is odd, … If the number is even, …"
- *Kianne, Corrupted Memory*: "As long as Kianne's power is even, … As long as Kianne's power is
  odd, …"
- *Sab-Sunen, Luxa Embodied*: "can't attack or block unless it has an even number of counters on
  it", "Then if it has an odd number of counters on it, draw two cards."
- *Yennett, Cryptic Sovereign*: "You may cast it without paying its mana cost if its mana value is
  odd."

Some print parity as a filter on objects rather than a Condition: *Desecrate Reality* ("with an
even mana value", "with an odd mana value"), *Void Winnower* ("with even mana values"), and a
chosen parity ("choose odd or even", "of the chosen quality": *Ashling's Prerogative*,
*Extinction Event*, *Gollum, Riddle Master*, *Lavabrink Venturer*, *Mutinous Massacre*,
*Thanos, the Mad Titan*, *Twins of Discord*, *Zimone's Hypothesis*).

## Done when

- A primitive at `Condition` tests whether an amount is of a `Parity`, spelled in the form
  *Chaos Lord* prints ("<amount> is <parity>").
- *Chaos Lord* is in `plugins/canon/cards/`, its first ability's "if the number of permanents is
  even" written with that word.
- The `can` Template takes `unless: Option(Condition) = none` as `cant` and `must` do, read inside
  the permission as `cant`'s is (`docs/decisions/semantics.md` §4.7), and *Chaos Lord*'s "unless
  it entered this turn" is that `unless`.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check`, `cargo test -p xtask` and
  `cargo xtask cite check` pass.

## Not in scope

- Authoring the other parity cards listed above.

## Pointers

`plugins/builtin/primitives/parity/`, `plugins/builtin/primitives/condition/{compareAmt,manaValueParity}.syl`,
`plugins/builtin/templates/static/{can,cant,must}.syl`, `docs/tickets/done/syl-cant-unless.md`,
`docs/decisions/semantics.md` §4.7.
