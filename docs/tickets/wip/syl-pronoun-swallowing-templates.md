---
needs: [cleanup-step6-spine, syl-number-snapshot]
---

# Templates that fix an anaphor's number

## Why

Some Templates write an anaphor inside them, so the number of the pronoun the card prints is
fixed by which Template the author picked rather than written at the call site:

- `templates/action/{destroyCantRegenerate,destroyPluralCantRegenerate}.syl` fuse two printed
  sentences, "Destroy <subject>. It/They can't be regenerated.", and hardcode `pro(bare, one)` or
  `pro(bare, many)`. Their five callers are *Damnation*, *Do or Die*, *Snuff Out*, *Terror* and
  *Wrath of God*. *Nekrataal* already writes the two clauses itself: "That creature can't be
  regenerated" is `regeneration(that(creature))`.
- `templates/timing/onEachOfTheirTurns.syl` bakes in `pro(word(player), one)` and
  `templates/timing/duringTheirUntapSteps.syl` `pro(word(player), many)`. *Rites of Flourishing*
  calls the first; nothing calls the second.

## Design

- Each destroy call site writes the destroy and its "can't be regenerated" clause with the
  anaphor its card prints, and the two destroy Templates are deleted.
- `onEachOfTheirTurns` and `duringTheirUntapSteps` take a `whose: NounPhrase` argument, `they` or
  `themPlayers` at the call site, instead of a fixed number.
- The closed-class pronoun pairs stay: `it`/`them`, `that`/`those`, `itself`/`themselves`,
  `theOther`/`theRest`, `they`/`themPlayers` and `the`/`thePlural`.

## Done when

- `destroyCantRegenerate.syl` and `destroyPluralCantRegenerate.syl` are gone, and the five cards
  write both clauses with their printed anaphor.
- The two timing Templates take `whose`, and every caller passes it.
- The landing runs the `syl-number-snapshot` diff and explains every flagged line.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass;
  `cargo xtask cite check --list-noncompliant` is empty and `cargo xtask cite check` reports
  0 stale.

## Pointers

`plugins/builtin/templates/action/{destroyCantRegenerate,destroyPluralCantRegenerate,doUnderProhibition}.syl`,
`plugins/builtin/templates/timing/{onEachOfTheirTurns,duringTheirUntapSteps}.syl`,
`plugins/canon/cards/Nekrataal.syl`, `plugins/builtin/templates/pronoun/`.
