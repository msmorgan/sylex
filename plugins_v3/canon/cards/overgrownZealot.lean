/-- Overgrown Zealot -/
card := {
  name := "Overgrown Zealot",
  cost := [1, green],
  types := [creature],
  subtypes := [elf, druid],
  text := [
    activated tapSymbol (addMana 1 (ofAnyColor sameColor)),
    activated tapSymbol (addMana 2 (ofAnyColor sameColor) [spendOnly [toTurnFaceUp isPermanent]])
  ],
  power := 0,
  toughness := 4
}
