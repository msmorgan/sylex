---
needs: [plugins-v3-builtin-primitives-and-templates, lean-generated-canon-modules, lean-retirement]
---
# Daybound and reconfigure say all of what their CR text means

Standard constraints apply. Found by the 2026-10-25 triage of an external audit ("Atraxa"),
verified against `wmyykqnv`. Keyword Definitions carry their full bodies, so a body that drops
part of its "means" text is a fidelity gap. Both bodies below were probed in Lean on 2026-10-25
and check `[]`. Keyword Definitions are under `plugins/builtin/definitions/keyword_ability/`;
AST types are in Rust `semantics` (`lean/Semantics/Abilities.lean` today). Test names below are
the Lean proof-suite names (`lean/Semantics/Proofs/`), carried to Rust tests by
`semantics-rust-rewrite`. The `.syl` below follows the live files; match their syntax.

Two parts, each separately committable.

## Part 1: reconfigure [CR#702.151b]

[CR#702.151a]: Reconfigure [cost] means "[Cost]: Attach this permanent to another target
creature you control. Activate only as a sorcery" and "[Cost]: Unattach this permanent. Activate
only if this permanent is attached to a creature and only as a sorcery." [CR#702.151b]:
"Attaching an Equipment with reconfigure to another creature causes the Equipment to stop being
a creature until it becomes unattached from that creature." The Oracle reminder (*Lizard
Blades*, *Lion Sash*) prints "While attached, this isn't a creature." The body
(`reconfigure.syl`) writes only [CR#702.151a].

Write:
```
body = [
  activatedOnlyDuring(
    cost,
    attachTo(thisPermanent, target(and([isCreature, isOther, youControl]))),
    asSorcery),
  activated(
    cost,
    unattach(thisPermanent),
    asSorcery,
    guard = is(thisPermanent, attachedTo(a(isCreature)))),
  static(
    onlyWhile(
      characteristicChange(thisPermanent, [typeLine(loses, {types = [creature]})]),
      is(thisPermanent, attachedTo(a(isCreature)))))]
```
- The second ability's condition is respelled through `attachedTo`
  (`plugins/builtin/templates/predicate/attachedTo.syl`); it expands to today's raw
  `attachment(attachment, a(isCreature))`.
- The third static follows impending's type removal (`impending.syl`).
- Add [CR#702.151b] to the header comment.

Tests:
- Added: *Lizard Blades* (Vintage-legal) as `plugins/canon/cards/Lizard Blades.syl`:
  ```
  card {
    name = "Lizard Blades",
    cost = [1, red],
    types = [artifact, creature],
    subtypes = [equipment, lizard],
    text = [doubleStrike, static(abilityGrant(equipped(creature), doubleStrike)), reconfigure(mana([2]))],
    power = 1,
    toughness = 1
  }
  ```
  Confirm its Oracle text in `data/derived/cards.jsonl` first.
- Rewritten if its pattern no longer matches: `reconfigureUnattachesWithoutTheAttachDeed`
  (`Keyword.lean`) matches on the ability list; extend the pattern to the third element, keeping
  the claim about the second.
- Must stay green: `reconfigureAnchorsOnTheSource` (`ContrastAnchor.lean`),
  `keywordBodiesKeepTheirActionDeeds` (`Keyword.lean`).

## Part 2: daybound [CR#702.145b]

[CR#702.145b]: "Daybound" means "If it is night and this permanent is represented by a
double-faced card, it enters transformed," "As it becomes night, if this permanent is front face
up, transform it," and "This permanent can't transform except due to its daybound ability."
The first static (`daybound.syl`) omits "this permanent is represented by a double-faced card". A
copy of a daybound creature, or a creature granted daybound, is the case the clause is for
[CR#701.27a,701.27c].

No existing construct says it: `Predicate.currentFace`
(`plugins/builtin/primitives/predicate/currentFace.syl`) is a permanent's face;
`isTransformed` (`plugins/builtin/templates/predicate/isTransformed.syl`) is
`currentFace(back)`; `Card.transforming` and `Card.modalDfc` are the printed frame, not a
predicate; `TokenSpec.doubleFaced` is token creation.

Change:
1. Add a nullary Predicate beside `isCopyOfACard`, documented "The member is represented by a
   double-faced card [CR#712.1,702.145b]", and its primitive beside
   `plugins/builtin/primitives/predicate/isCopyOfACard.syl`. `.isCopyOfACard` has no checker
   arm (every Predicate match falls through a wildcard to kind `.object`); the `discourse` tests
   confirm whether the new atom needs one.
2. Name the constructor and primitive after the CR term *double-faced card* [CR#712.1]; the
   glossary does not redefine CR terms.
3. Daybound's first static becomes
   ```
   static(
     replace(
       would(actor(thisPermanent), enter),
       doIf(
         and([gameIs(night), is(thisPermanent, isDoubleFacedCard)]),
         withRiders([entersTransformed]))))
   ```
   The other two statics are unchanged.
4. Nightbound (`nightbound.syl`, "As it becomes day, if this permanent is back face up, transform
   it" [CR#702.145e]): write `is(thisPermanent, currentFace(back))` instead of `isTransformed`,
   which expands to the same and names the CR's "back face up".

Tests:
- Added: *Tavern Ruffian // Tavern Smasher* (Vintage-legal) as
  `plugins/canon/cards/Tavern Ruffian {slash}{slash} Tavern Smasher.syl`, following
  `plugins/canon/cards/Afflicted Deserter {slash}{slash} Werewolf Ransacker.syl`:
  ```
  card transforming(
    {
      front = {
        name = "Tavern Ruffian", cost = [3, red], types = [creature],
        subtypes = [human, warrior, werewolf], text = [daybound],
        power = 2, toughness = 5},
      back = {
        name = "Tavern Smasher", types = [creature], subtypes = [werewolf],
        text = [nightbound], power = 6, toughness = 5}})
  ```
  Confirm both faces in `data/derived/cards.jsonl` first.
- Added: `dayboundEntersTransformedOnlyAsADoubleFacedCard`, asserting that daybound's first
  replacement condition is `and([gameIs(night), is(thisPermanent, isDoubleFacedCard)])`, modelled
  on `enchantReplacesTheEntering` (`OffStack.lean`).
- Must stay green: `okDaybound`, `okNightbound` (`OffStack.lean`), and the nightbound pin in
  `Attachments.lean`.

## What must not change

The other statics of both keywords; the day/night game rules [CR#702.145c,702.145d,702.145f]
stay out of these bodies.

## Acceptance

`cargo test` and `cargo clippy --tests -- -D warnings` for `semantics` and `discourse`, the CR
citation gates (bless [CR#702.151b] and [CR#712.1] if newly cited).
