---
needs: [syl-argument-naming, syl-single-face-card-call]
---

# Declarations whose name or spelling says something else

## Why

Six declarations are named or spelled for one use, or read poorly for what they build:

- `modifiesPrecedingAction` (`primitives/timing/`) is a `Duration` spelled "It can't be
  regenerated", though it is any rider that modifies the action before it [CR#608.2c].
- `thisObject` at `LookbackSpan` (`primitives/timing/`) is spelled "spells you cast".
- `splitOverPermanent` (`templates/pronoun/`) is spelled "any target".
- `under` (`primitives/zone/`), the move rider spelled "under <controller>'s control", reads
  as `under(you)`; it is renamed `underControlOf` (16 files call it).
- `actor(np)` (`primitives/noun/actor.syl`), the active `Subject` constructor, is named for the
  Actor, but its noun phrase is the clause's Subject, a Referent that pronouns and demonstratives
  read, and not the Actor (`semantics.md` §2.1, §4.2); an effect can be one
  (`doesnt(actor(thisEffect), remove(thisAura))`, *Black Ward*). It is renamed `subject(np)`. The
  noun-phrase word `actor` (`primitives/pronoun/actor.syl`), which reads the Actor, keeps its
  name.
- `and` at `TriggerCondition` (`primitives/triggerCondition/and.syl`) is named for the printed
  coordinator, but it means that the ability triggers when any of its conditions is met
  [CR#603.1b,603.2]. The primitive is renamed `or`, for its meaning, and spelled with the "or"
  join. The printed "and" becomes a Template `and` over it (`templates/triggerCondition/`), and
  the Template `or` (`templates/triggerCondition/or.syl`) is removed, since the primitive is now
  what *Tenuous Truce*'s printed "or" spells. 6 canon cards call `and` there (*Autarch Mammoth*,
  *Bartered Cow*, *Balemurk Leech*, *Crack in Time*, *Up the Beanstalk*, *Shrine of Boundless
  Growth*) and 1 calls `or` (*Tenuous Truce*); nothing in `plugins/builtin/` calls either.

`syl-argument-naming` rewrites 4 arguments that hold these declarations' calls, and lands first.

This ticket lands after `syl-single-face-card-call`, which re-indents every single-faced canon card it edits.

## Done when

- Each is named for what it builds and spelled with words that mean it, or its one use moves into
  a Template that carries the printed wording; every caller in `plugins/` is updated.
- The `Subject` constructor is `subject(np)`, and no `actor(` call is left in `plugins/`. The
  rename is corpus-wide (every `actor(` call, about 640 today), so a one-off script makes it and
  checks that every file parses to the same tree apart from the constructor's name; the landing
  reports the counts it compared. The glossary entry **Subject**
  (`docs/contexts/game-model/CONTEXT.md`), `syl.md` §2 and `semantics.md` §4.2, §4.6 and §4.7
  name `subject(…)`.
- The trigger-condition primitive is `or`, the printed "and" is the Template `and` over it, and
  no Template `or` exists at `TriggerCondition`; each of the 7 callers is checked against its
  card's Oracle text, so the 6 that print "and" call `and` and *Tenuous Truce* calls `or`.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`plugins/builtin/primitives/timing/{modifiesPrecedingAction,thisObject}.syl`,
`plugins/builtin/primitives/period/thisObject.syl`,
`plugins/builtin/templates/pronoun/splitOverPermanent.syl`, `plugins/builtin/primitives/noun/actor.syl`,
`plugins/builtin/primitives/static/doesnt.syl`,
`plugins/builtin/primitives/triggerCondition/and.syl`,
`plugins/builtin/templates/triggerCondition/or.syl`.
