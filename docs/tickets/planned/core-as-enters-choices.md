---
needs: []
---

# "As this enters, choose …": stored entry choices and anchor words

## Why

"As this enters, choose a color/creature type/card name/number/opponent" is a replacement effect
[CR#614.1c], and the abilities that read "the chosen …" are linked to it [CR#607.2d]. `.syl`
writes these (`entersChoosing`, `entersChoosingFrom`, `entersChoosingPlayer`, read back by
`ofChosen`; Voice of All, Engineered Plague, Meddling Mage and about fifty other canon cards), but
the engine cannot run them: `apply_as_enters` in `replace.rs` folds only tap, attach, counters and
conditionals, and any other `also` shape hits its `todo!`. Core's `ChosenValueKind` covers only
color, card name and number.

Anchor words have no `.syl` form at all. "As this enchantment enters, choose Khans or Dragons"
followed by "• Khans — …" and "• Dragons — …" means the permanent has each bulleted ability only
as long as that word was chosen as it entered [CR#614.12c], and the bulleted abilities are linked
to the choice [CR#607.2m]. An entry choice chooses a quality (`QualitySort`), not one of two
printed words, and no condition reads which word was chosen.

## Done when

- The engine folds an as-enters choice into the entering object: the controller chooses as it
  enters, the value is stored on that object, and its linked abilities read it. Engine tests use
  real cards covering each chosen sort the canon cards use (at least a color, Voice of All; a
  creature type, Engineered Plague; a card name, Meddling Mage; an opponent, Black Vise).
- Core's chosen-value kinds cover every sort those cards choose.
- `.syl` can write an anchor-word choice: a primitive or template for the choice between printed
  words, and a way to give a bulleted ability only while its word is chosen. Citadel Siege and
  Outpost Siege are authored in `plugins/canon/cards/` with it.

## Not in scope

- Running anchor-word cards in the engine. The engine reads core, which `.syl` reaches only
  through the step-9 lowering.
- The other `also` shapes `apply_as_enters` cannot fold (`engine-as-enters-fold-breadth`).

## Pointers

- `crates/engine/src/replace.rs` (`apply_as_enters`), `crates/core/src/decision.rs`
  (`ChosenValueKind`), `crates/core/src/action.rs` (`Action::ChooseValue`)
- `plugins/builtin/templates/ability/entersChoosing.syl`, `entersChoosingFrom.syl`,
  `entersChoosingPlayer.syl`, `entryChoice.syl`; `plugins/builtin/primitives/predicate/ofChosen.syl`
- `plugins/canon/cards/Voice of All.syl`, `Engineered Plague.syl`, `Meddling Mage.syl`
- `docs/tickets/planned/engine-as-enters-fold-breadth.md`
