---
needs: [plugins-v3-builtin-primitives-and-templates, lean-generated-canon-modules, lean-retirement]
---
# "Regenerate target creature" publishes its target to the following text

Standard constraints apply. Found by the 2026-10-25 triage of an external audit ("Atraxa"),
verified against the tree at `wmyykqnv`. Test names below are the Lean proof-suite names
(`lean/Semantics/Proofs/`), carried to Rust tests by `semantics-rust-rewrite`.

## The defect

*Debt of Loyalty* (Vintage-legal) reads "Regenerate target creature. You gain control of that
creature if it regenerates this way." It cannot be written. A probe on 2026-10-25 of the
spelling below returned `[anaphor …, anaphor …, thisWayUngrounded, anaphor (word creature) …]`.

`regenerate` (`plugins_v3/builtin/definitions/keyword_action/regenerate.syl`) is a shield: it
establishes a single-use replacement of the subject's destruction [CR#701.19a]. The subject is
mentioned only inside the replacement's `would` pattern, which the checker steps as a
hypothetical box, so its target never reaches the following text. The pin
`regenerationPatternDoesNotLeakIntoFollowingText` (`BindingIdentity.lean`) asserts that
`regenerateBy (target isCreature)` publishes no target. That pin asserts the defect.

A target is chosen as the spell is cast [CR#601.2c,115.1], not when the shield applies, and
"that creature" names it. The ADR already admits this for conditions: "A referent a condition
introduces … is a hypothetical and never a candidate, though targets the action branch announces
are ([CR#601.2c])" (`docs/decisions/semantics-v3.md` §0.1 item 2, step 2).

## Change

1. Requirement on `discourse`: when it steps an `establish` whose spec is a `replace` (the shield
   form), a mention with the `target` determiner inside the replacement's `would` pattern is
   announced in the box enclosing the `establish`, with its `.target` stamp, so later text reads
   it by D′ ("it") and AC>R1 ("that creature"). Non-target mentions in the pattern stay
   hypothetical. Add no AST node, field or variant ([CR#701.19a] is unchanged; §0.1 item 5(c)).
2. Add *Debt of Loyalty* to canon as `plugins_v3/canon/cards/Debt of Loyalty.syl`, its text
   following *Matopi Golem*'s reflexive "When it regenerates this way"
   (`plugins_v3/canon/cards/Matopi Golem.syl`, tested by
   `regenerationEventDescribesTheApplicationRatherThanTheShield`, `ActionFamilies.lean`):
   ```
   triggerThisWay(
     regenerate(target(isCreature)),
     does(any, regeneration(it)),
     gainControl(that(creature)))
   ```
   If this is not the form for a postposed "if it regenerates this way" on a spell, use the form of
   the "If it regenerates this way, draw a card" fixture (`Turn.lean`) and say which.

## What must not change

- `regenerate` stays a shield with `.ownScope none`; only `regeneration` carries the deed
  (owner ruling 2026-10-25 in `lean-v3-deed-duplicates`; [CR#701.19c]).
- `regenerateTargetsOnce` (`BindingIdentity.lean`): the application still keeps the one targeted
  identity.
- A non-target subject ("Regenerate this creature") publishes nothing new: the conjunct
  `countOnes .object shield = 0` of
  `regenerationEventDescribesTheApplicationRatherThanTheShield` stays.
- Conditions' hypothetical referents stay hypothetical.

## Tests (`discourse`)

- Rewritten: `regenerationPatternDoesNotLeakIntoFollowingText` → `regeneratedTargetIsReadBack`,
  asserting that `regenerateBy (target isCreature)` announces exactly one target, because the old
  claim is the defect.
- Added: *Debt of Loyalty* checks clean, with "that creature" resolved to the target. Every
  fixture must be a Vintage-legal card (owner, 2026-10-25), so no invented "Regenerate target
  creature. Tap it." test is added.
- Must stay green: `regenerateTargetsOnce`, `okRegenerationApplication` (`BindingIdentity.lean`),
  `regenerationEventDescribesTheApplicationRatherThanTheShield`, *Matopi Golem*,
  `modifyingProhibitionReadsOnlyThePrecedingActionsObjects` (`Deontic.lean`).

## Acceptance

`cargo test` and `cargo clippy --tests -- -D warnings` for `semantics` and `discourse`, the CR
citation gates.
