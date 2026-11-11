---
needs: [cleanup-step6-surface]
---

# Designations confer what their rules give them

## Why

Some designations carry an effect by rule, but their `.syl` declarations confer nothing
(`plugins/builtin/definitions/designation/`):

- `suspected`: a suspected permanent has menace and "This creature can't block" for as long as
  it's suspected [CR#701.60c].
- `goaded`: a goaded creature attacks each combat if able and attacks a player other than the
  controller of the permanent, spell, or ability that caused it to be goaded if able [CR#701.15b].
- `prepared`: as a permanent gains the designation, its controller creates a copy in exile with
  only its prepare spell's characteristics, and may cast that copy while the permanent stays
  prepared [CR#723.3c].

Each states its effect nowhere in `.syl`, so a card that suspects, goads or prepares a creature
reads as giving it a bare marker. `monarch` and `initiative` already confer theirs.

## Open questions

- How `goaded`'s conferral names the player its creature must attack other than: the controller
  of what caused this goading, one per goader [CR#701.15c].
- Whether `prepared`'s exile copy [CR#723.3c] is a conferral on the designation or a rule row
  under `plugins/builtin/rules/`.

## Done when

- `suspected`, `goaded` and `prepared` write the effect their rule states, as settled, with the
  rule cited.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass;
  `cargo xtask cite check --list-noncompliant` is empty and `cargo xtask cite check` reports
  0 stale, after `cargo xtask cite bless` for newly cited rules.

## Not in scope

- The preparation mechanic beyond the `prepared` designation's own conferral: preparation cards'
  inset prepare spells and their other rules [CR#723].
- The engine side of goad: evaluating the attack requirement (`engine-combat-requirements`) and
  storing one goaded instance per goader with its own duration (`designation-storage-axes`).

## Pointers

`plugins/builtin/definitions/designation/{suspected,goaded,prepared,monarch}.syl`,
`plugins/builtin/definitions/keyword_action/{suspect,goad}.syl`.
