card := {
  name := "Nykthos, Shrine to Nyx",
  supertypes := [legendary],
  types := [land],
  text := [
    activated tapSymbol (addMana 1 (manaOf [[colorless]])),
    activated (compound [[2], tapSymbol])
      [choose (a (quality color)), addMana (devotion you thatColor) ofTheChosenColor]
  ]
}
