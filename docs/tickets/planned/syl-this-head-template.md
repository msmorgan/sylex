---
needs: [syl-printed-it-self-references, syl-argument-naming, syl-condition-first-static, syl-lookback-point-from-trigger, syl-single-face-card-call]
---

# One `this(head)` Template replaces the 21 `this<Head>` Templates

## Why

`plugins/builtin/templates/noun/` holds 21 `this<Head>` Templates (`thisCreature`, `thisAura`,
`thisSpell`, `thisPermanent`, …), each `headed(<head>, this)` for one head noun over the `this`
primitive (`plugins/builtin/primitives/pronoun/this_.syl`). Three have no caller (`thisClass`,
`thisEmblem`, `thisSpacecraft`), and a head with no Template can't be written at all. Callers also
mix `thisSpell`, `thisPermanent` and bare `this` for the same printed words.
It lands after `syl-printed-it-self-references`, which rewrites about 99 of the same call sites
to `it`, so those sites are rewritten once and the two tickets never edit them in parallel.

Its `this<Head>` sites also sit in calls three tickets rewrite first: `syl-argument-naming`
(83 arguments it names or unnames, in 79 files), `syl-condition-first-static` (the
condition-first statics, such as *Guardian Beast*'s `is(thisCreature, isUntapped)`) and
`syl-lookback-point-from-trigger` (3 `move` calls, such as `aura.syl`'s, that drop `at`).

This ticket lands after `syl-single-face-card-call`, which re-indents every single-faced canon card it edits.

## Open questions

- The Template's name and parameter: the `this` primitive already holds `(this, NounPhrase)`, so a
  Template named `this` at `NounPhrase` would collide (`syl.md` §3).

## Done when

- One Template taking the head noun replaces the 21 files, and every builtin and canon caller
  uses it with the head its text prints; where a caller writes `thisSpell`, `thisPermanent` or
  bare `this`, it now matches the printed words.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

Mentions where the card prints "it": `syl-printed-it-self-references`.

## Pointers

`plugins/builtin/templates/noun/this*.syl`, `plugins/builtin/primitives/pronoun/this_.syl`,
`docs/decisions/syl.md` §3.
