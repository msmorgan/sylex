---
needs: [plugins-v3-builtin-primitives-and-templates, lean-generated-canon-modules, lean-retirement, cleanup-step6-surface]
---

# "When you lose control of <object>" has no event

## Why

`.syl` names a control change only from the gaining side (`gainControlOf`, constructor
`changeControl`: "gains control of … from you"). A player's losing control of a permanent is not
the same event: control returns to the owner when a "gain control" effect ends, and other effects
can take it. *Stolen Uniform*'s last sentence, a delayed triggered ability its resolution creates
[CR#603.7a], needs it: "When you lose control of that Equipment this turn, if it's attached to a
creature you control, unattach it." Other Vintage-legal cards printing "lose control of":
*Duplicity*, *Khârn the Betrayer*, *Krovikan Vampire*, *Ogre Geargrabber*, *Ray of Command*,
*Seraph*.

Written against the surface (`cleanup-step6-surface`).

## Done when

- Losing control is an event, either as a direction on the `changeControl` constructor or as a
  `loseControl(subject: NounPhrase)` primitive; the landing says which and why.
- It is admitted in event and lookback patterns alongside `gainControlOf`, and refused as an
  instruction.
- *Stolen Uniform* is in `plugins/canon/cards/` in full.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`plugins/builtin/primitives/action/gainControlOf.syl`,
`plugins/builtin/definitions/keyword_action/exchangeControl.syl`, `docs/decisions/semantics.md`
§4.6 (the event gate, pattern-only heads).
