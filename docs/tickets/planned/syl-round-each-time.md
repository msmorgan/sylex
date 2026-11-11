---
needs: [syl-spelling-conventions, lean-retirement, cleanup-step6-surface]
---

# "Round up each time." as a construct in its own position

## Why

Six Vintage-legal cards end a sentence of fractions with "Round up each time." or "Round
down each time.": *Peer into the Abyss*, *Fraying Omnipotence*, *Pox*, *Hydroid Krasis*, *Pox
Plague* and *Saw in Half*. `Saw in Half.syl` writes `half(up, …)` on each fraction, so spelling
cannot give back the shared sentence, and the only way out would be single-purpose Templates
like `basePtHalfRoundedUpOf`. The inline ", rounded up/down" is a different construction and
stays on its own fraction: *Banshee* and *Aspect of Wolf* round two fractions of one sentence in
different directions.

Written against the surface (`cleanup-step6-surface`).

## Done when

- An unrounded fraction amount exists for half and for a third (*Pox*), spelled "half <amount>"
  and "a third of <amount>".
- `roundEachTime(action, mode)` wraps the sentence it covers and is spelled as that sentence
  followed by "Round <mode> each time.", its parameters in that printed order (`syl.md` §5). It wraps the whole sentence, so a "then" chain of several
  Actions (*Pox*, *Fraying Omnipotence*) is covered; it is not a trailing step that looks back at
  the step before it.
- The checks are structural: every fraction inside the wrapped Action is unrounded and at least
  one exists; an unrounded fraction that no `roundEachTime` wraps is refused, because the spell or
  ability must say which way to round [CR#107.1a]. Each refusal has a test: an unrounded fraction
  nothing wraps, an "each time" with no fraction, a rounded fraction inside an "each time".
- *Saw in Half* is re-encoded with it, and *Pox* or *Fraying Omnipotence* is added as the witness
  for a "then" chain; a test shows every fraction of that chain is covered.
- `half` (rounded) is unchanged and *Banshee*-style sentences still use it.
- The `roundEachTime` declaration's doc comment records the construct.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`plugins/builtin/primitives/quantity/half.syl`,
`plugins/canon/cards/Saw in Half.syl`.
