---
needs: [cleanup-step6-surface]
---

# Turn Points in the wording cards print

## Why

Three canon cards write a Turn Point whose spelling is not what they print:

- *Apathy* and *Dance of the Dead* print "At the beginning of the upkeep of enchanted creature's
  controller", the part before its possessor, but write `beginningOfPossessed(controllerOf(…),
  upkeep)`, spelled "the beginning of $possessor's $part".
- *Kitsune Mystic* prints "At the beginning of the end step" but writes
  `beginningOfEach(endStep)`, spelled "the beginning of each $part".

No Turn Point Template spells either wording, so neither card's text round-trips.

## Design

- Each wording is a Template over `beginningOf` with the meaning its nearest Template has: "the
  beginning of the $part of $possessor" as `beginningOfPossessed` does, "the beginning of the
  $part" as `beginningOfEach` does.

## Done when

- *Apathy*, *Dance of the Dead* and *Kitsune Mystic* call Templates that spell what they print.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`plugins/builtin/templates/turn_point/`, `plugins/canon/cards/Apathy.syl`,
`plugins/canon/cards/Dance of the Dead.syl`,
`plugins/canon/cards/Kitsune Mystic {slash}{slash} Autumn-Tail, Kitsune Sage.syl`.
