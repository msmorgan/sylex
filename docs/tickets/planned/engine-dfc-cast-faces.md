---
needs: [engine-transform]
---
# Engine: cast or play a chosen face of a double-faced card

## Why

Every cast path reads the front face: `cast.rs` takes characteristics from `derive::face`, which
is `Card::primary_face`, and `Action::CastSpell { object }` carries no face. So a double-faced card
can only ever be cast or played as its front face. The card model and the `.syl` corpus already
distinguish the two kinds (`DoubleFacedLayout::{Transforming, ModalDfc}`; `card modalDfc(…)` and
`card transforming(…)`), and `engine-transform` gave objects a `side`. Two populations need the
cast-time face:

- **Modal double-faced cards** (98 Vintage-legal). The player chooses which face to cast before
  putting it on the stack [CR#712.8b]. Only that face is evaluated for castability and only it is
  put on the stack [CR#712.8c]. Playing one as a land chooses a land face, and it enters with that
  face up [CR#712.9]. The Pathway lands are the witnesses: their back face is a land too.
- **Casting a nonmodal card "transformed" or "converted"**, for example the `transformedPaying`
  play payment ([CR#712.6b,702.146a], disturb) or a defeated Siege cast transformed (`engine-battles`). The spell is put on the
  stack back face up [CR#712.8a] with only the back face's characteristics, and its mana value
  comes from the front face's mana cost [CR#712.6b].

A resolving double-faced spell enters with the face that was up on the stack [CR#712.10].

## Done when

- `legal_actions` offers each castable face of a modal double-faced card and each playable land
  face, and the chosen face rides the action.
- The cast pipeline evaluates timing, legality, targets and cost from the chosen face, and the
  spell object carries that face's `side` on the stack and onto the battlefield.
- An effect that casts a card transformed puts it on the stack back face up, and its mana value
  reads the front face.
- Tests grounded in Branchloft Pathway // Boulderloft Pathway (playing either land face), a modal
  card whose faces are a spell and a land, and a disturb card cast transformed from the graveyard
  (checking its mana value).

## Not in scope

- Turning a permanent over on the battlefield: done in `engine-transform`.
- Copies of double-faced spells keeping their face: `engine-copy-permanent-spells`.

## Pointers

- `crates/engine/src/cast.rs`, `crates/engine/src/derive.rs` (`face`, `face_of`),
  `crates/engine/src/legal.rs` (`legal_actions`)
- `crates/card/src/card.rs` (`Card::DoubleFaced`, `DoubleFacedLayout`)
- `plugins/canon/cards/Branchloft Pathway {slash}{slash} Boulderloft Pathway.syl`
- `plugins/builtin/primitives/cost/transformedPaying.syl`
