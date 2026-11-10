---
needs: [syl-single-face-card-call]
---

# "Add {G}" without an unlabeled list of lists

## Why

Mana a card adds by its symbols is written `addMana(amount, manaOf(…))`, where `manaOf` takes a
`List(List(ColorOrColorless))` whose two list levels have meanings nothing names: the outer list is
"or", each inner list a sequence of symbols. Every "Add {G}" is `addMana(1,
manaOf([[colored(green)]]))`, its `1` printed nowhere; *Adarkar Unicorn*'s "Add {U} or {C}{U}" is
`manaOf([[colored(blue)], [colorless, colored(blue)]])`, and *Coal Stoker*'s "add {R}{R}{R}" a
sequence of three under `1`, while *The Flux*'s "Add six {R}" is `6` over a sequence of one. A
reader can't tell from the call which list is the choice and which the sequence, the elements are
colors rather than the mana symbols the card prints, and the amount multiplies whichever sequence is
chosen. `manaOf` is also spelled `"Add <…>"`, though it is an argument of `addMana` and the verb
belongs to the action; the other `ProducedMana` primitives (`ofAnyColor`, `ofTheChosenColor`, …)
describe only the mana.

## Design

- `addMana(produced: ProducedMana, riders: List(ManaRider) = [])` carries no amount of its own:
  the amount is part of the produced mana, so "Add {G}" stores no unprinted `1`. It spells
  "add <produced>", its riders printed as sentences after it. `manaOf` is deleted.
- `symbols(symbols: List(ManaSymbol))` at `ProducedMana` is mana written as a sequence of
  symbols, wrapped even when the sequence is one symbol: `symbols([pip(blue), pip(blue)])`,
  `symbols([colorlessPip])`. Its elements are ordinary mana symbols; one that stands for no one
  mana of a type ([CR#106.1b,107.4a,107.4c]) is for lowering to refuse, not its sort. A bare
  mana symbol never stands for produced mana, so each of a filterland's sequences stays visible.
- `or(alternatives: List(ProducedMana))` is a choice between productions, in printed order:
  *Mystic Monastery*'s "Add {U}, {R}, or {W}", *Cascade Bluffs*'s "Add {U}{U}, {U}{R}, or
  {R}{R}", *Thriving Heath*'s "Add {W} or one mana of the chosen color".
  `ofTheChosenColor` has no `alternative`; `or` says it.
- `counted(amount: Amount, produced: ProducedMana)` is a printed count of produced mana: *The
  Flux*'s "six {R}", "X {G}", "two mana of any one color". The produced mana is produced `amount`
  times, each mana on its own, so every choice inside it is made once per mana: *Culling Ritual*'s
  "Add {B} or {G} for each permanent destroyed this way" is `counted(forEach(1, …),
  or([symbols([pip(black)]), symbols([pip(green)])]))`, a color per mana, while *Brigid, Doun's
  Mind*'s "Add X {G} or X {W}" is `or([counted(letter(x), symbols([pip(green)])), counted(letter(x),
  symbols([pip(white)]))])`, one choice for all. Only an inner term that says the mana share a color
  (`ofAnyColor(sameColor)`) makes one choice for all the counted mana. A written sequence and a
  count stay different terms: *Soulbright Flamekin*'s eight {R} in a row is one `symbols`, *The
  Flux*'s "six {R}" a `counted`.
- "For each" is the general `forEach` amount inside `counted`, as on every other verb: *Gaea's
  Cradle*'s "{G} for each creature you control" is `counted(forEach(1, and([isCreature,
  youControl])), symbols([pip(green)]))`. "An amount of {G} equal to …" is `counted` over the
  read amount. Both print their amount elsewhere than "six {R}" does, and the general Tail
  construction places it; until spellings can, `counted`'s spelling marks them
  `TODO(spelling)`, and no mana-only Template stands for either.
- The `ProducedMana` words that printed "one mana of …" (`ofAnyColor`, `ofAnyColorAmong`,
  `ofAnyColorOf`, `ofTheChosenColor`, `ofTypeProducedBy`, `ofTypeProducibleBy`) spell the mana
  without its count, and a card printing "one" writes `counted(1, …)`. `matchingManaCostOf` and
  `ofTheLastNotedType` print no count and take none. `ofColorProducibleBy`, added for *Fellwar
  Stone*'s "any color" (it had been written with `ofTypeProducibleBy`'s "any type"), is spelled
  without the count from the start.

## Done when

- Symbol-written mana production is written in the Design's shape: no position whose meaning
  comes from list nesting alone, and "Add" spelled by `addMana`; every caller in `plugins/` is
  updated.
- The rewritten cards mean the same mana: the landing lists each card's old and new call.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`plugins/builtin/primitives/action/addMana.syl`, `plugins/builtin/primitives/producedMana/`
(the `ProducedMana` primitives), `plugins/builtin/templates/amount/forEach.syl`,
`plugins/canon/cards/{Adarkar Unicorn,Coal Stoker,The Flux,Gaea's Cradle,Thriving Heath}.syl`.
