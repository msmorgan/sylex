---
needs: [syl-argument-naming, syl-single-face-card-call]
specs: [voice-and-object-phrases]
---

# Declarations whose name or spelling says something else

## Why

Seven declarations are named or spelled for one use, or read poorly for what they build:

- `modifiesPrecedingAction` (`primitives/timing/`) is a `Duration` spelled "It can't be
  regenerated", though it is any rider that modifies the action before it [CR#608.2c].
- `thisObject` at `LookbackSpan` (`primitives/timing/`) is spelled "spells you cast".
- `splitOverPermanent` (`templates/pronoun/`) is spelled "any target".
- `under` (`primitives/zone/`), the move rider spelled "under <controller>'s control", reads
  as `under(you)`; it is renamed `underControlOf` (16 files call it).
- `actor` at `NounPhrase` (`primitives/pronoun/actor.syl`) is described as the performer ("whoever
  performs this action") and is read two ways. Six Templates read it as the thing their event
  happens to: `die`, `enter`, `leave`, `becomeStatus`, `phaseIn` and `phaseOut` (`die` is
  `move(actor, …)`), and so do five canon cards in their own terms (*Afflicted Deserter //
  Werewolf Ransacker*, *Bewitching Leechcraft*, *Darksteel Garrison*, *Tuktuk Scrapper*, *Verity
  Circle*). The keyword-action Definitions read it as whoever performs them (`mill` moves cards
  from `libraryOf(actor)`), as do `gainControl` and `actorControls`. The first reading gets its
  own word, `subject`, which reads what the verb phrase is said of as a participant of its event
  and prints no word; `actor` keeps the second (spec §2). The expansion tests
  (`crates/xtask/src/dsl_expand/tests.rs`) name the participant word by one constant,
  `SUBJECT_WORD`.
- `and` at `TriggerCondition` (`primitives/triggerCondition/and.syl`) is named for the printed
  coordinator, but it means that the ability triggers when any of its conditions is met
  [CR#603.1b,603.2]. The primitive is renamed `or`, for its meaning, and spelled with the "or"
  join. The printed "and" becomes a Template `and` over it (`templates/triggerCondition/`), and
  the Template `or` (`templates/triggerCondition/or.syl`) is removed, since the primitive is now
  what *Tenuous Truce*'s printed "or" spells. 6 canon cards call `and` there (*Autarch Mammoth*,
  *Bartered Cow*, *Balemurk Leech*, *Crack in Time*, *Up the Beanstalk*, *Shrine of Boundless
  Growth*) and 1 calls `or` (*Tenuous Truce*); nothing in `plugins/builtin/` calls either.
- `onlyIfSo` (`templates/static/onlyIfSo.syl`), spelled "<spec> if <condition>", prints no
  "so". It is renamed `onlyIf`. 6 calls in 6 files use it: `freerunning`, `prowl`, `spectacle`
  and `surge` (`definitions/keyword_ability/`), `mayDeclineUntap` (`templates/ability/`) and
  *Academy Journeymage*.

`syl-argument-naming` rewrites 4 arguments that hold these declarations' calls, and lands first.

This ticket lands after `syl-single-face-card-call`, which re-indents every single-faced canon card it edits.

## Done when

- Each is named for what it builds and spelled with words that mean it, or its one use moves into
  a Template that carries the printed wording; every caller in `plugins/` is updated.
- `subject` exists at `NounPhrase` with no `spelling`; the six Templates and the five canon
  cards write it, and `SUBJECT_WORD` names it. Every `actor` left at `NounPhrase` reads a
  performer; the landing lists each use of either word with the reading it took. The doc
  comments of the two words, and the sentences of `semantics.md` §2.1 and §2.3 that describe the
  `actor` word, say which each reads.
- The trigger-condition primitive is `or`, the printed "and" is the Template `and` over it, and
  no Template `or` exists at `TriggerCondition`; each of the 7 callers is checked against its
  card's Oracle text, so the 6 that print "and" call `and` and *Tenuous Truce* calls `or`.
- The `StaticSpec` Template is `onlyIf`, and no `onlyIfSo` is left in `plugins/`.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

- The `Subject` constructor `actor(np)` (`primitives/noun/actor.syl`): it is not renamed. Its
  calls are deleted with the sort `Subject` (`syl-passive-participle-and-trigger-events` to
  `syl-deontic-verb-phrase-operators`).
- A Definition's own performer, `$actor`: `syl-definition-actor-parameter`.

## Pointers

`plugins/builtin/primitives/timing/{modifiesPrecedingAction,thisObject}.syl`,
`plugins/builtin/primitives/period/thisObject.syl`,
`plugins/builtin/templates/pronoun/splitOverPermanent.syl`,
`plugins/builtin/primitives/pronoun/actor.syl`,
`plugins/builtin/templates/action/{die,enter,leave,becomeStatus,phaseIn,phaseOut}.syl`,
`crates/xtask/src/dsl_expand/tests.rs`, `docs/specs/voice-and-object-phrases.md` §2,
`plugins/builtin/primitives/triggerCondition/and.syl`,
`plugins/builtin/templates/triggerCondition/or.syl`,
`plugins/builtin/templates/static/onlyIfSo.syl`.
