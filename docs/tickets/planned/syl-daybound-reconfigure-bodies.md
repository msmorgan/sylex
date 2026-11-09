---
needs: [plugins-v3-builtin-primitives-and-templates, lean-generated-canon-modules, lean-retirement]
---
# Daybound and reconfigure say all of what their CR text means

## Why

A keyword Definition carries its full body, so a body that drops part of its "means" text is a
fidelity gap. `reconfigure.syl` writes only [CR#702.151a], not that an attached reconfigure
Equipment stops being a creature [CR#702.151b]. `daybound.syl`'s first static omits "this
permanent is represented by a double-faced card" [CR#702.145b], and `.syl` has no predicate that
says it. The two parts are separately committable.

## Design

- Reconfigure's "stops being a creature" [CR#702.151b] is not a static in the body: reconfigure
  represents two activated abilities [CR#702.151a], and Lizard Blades' ruling says the
  not-a-creature effect persists if the Equipment loses its abilities. It is a rule row conferred
  by the reconfigure ability, because [CR#702.151b] applies however the Equipment became
  attached, and an Equipment that's a creature can equip only if it has reconfigure
  [CR#301.5c].
- Daybound's "represented by a double-faced card" is a nullary Predicate primitive,
  `isDoubleFacedCard` [CR#712.1,702.145b], beside `isCopyOfACard`. It stays card-only: a token
  copy of a double-faced card is a double-faced token, not a card [CR#707.8a].

## Done when

- Reconfigure's body is its two activated abilities [CR#702.151a], its unattach guard respelled
  `is(thisPermanent, attachedTo(a(isCreature)))`, and the [CR#702.151b] rule row exists,
  conferred by the reconfigure ability.
- `isDoubleFacedCard` exists, documented "The member is represented by a double-faced card
  [CR#712.1,702.145b]". Daybound's first static is a `replace` of its entering that, if it is
  night and the permanent `isDoubleFacedCard`, enters it transformed (`withRiders`).
- Nightbound's static ("if this permanent is back face up" [CR#702.145e]) writes
  `is(thisPermanent, currentFace(back))` instead of `isTransformed`.
- The other statics of both keywords are unchanged, and the day/night game rules stay out of
  these bodies.
- `plugins/canon/cards/Lizard Blades.syl` ({1}{R} Artifact Creature — Equipment Lizard 1/1:
  double strike, "Equipped creature has double strike.", reconfigure {2}) and
  `plugins/canon/cards/Tavern Ruffian {slash}{slash} Tavern Smasher.syl` ({3}{R} Human Warrior
  Werewolf 2/5 with daybound // Werewolf 6/5 with nightbound, in the form of
  `Afflicted Deserter {slash}{slash} Werewolf Ransacker.syl`) are in canon.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

- `plugins/builtin/definitions/keyword_ability/reconfigure.syl`, `daybound.syl`, `nightbound.syl`,
  `impending.syl`
- `plugins/builtin/templates/predicate/attachedTo.syl`, `plugins/builtin/templates/predicate/isTransformed.syl`
- `plugins/builtin/primitives/predicate/isCopyOfACard.syl`, `plugins/builtin/primitives/predicate/currentFace.syl`
