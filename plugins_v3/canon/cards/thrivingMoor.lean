card := {
  name := "Thriving Moor",
  types := [land],
  text := [
    entersTapped thisLand,
    entersChoosingFrom thisLand color (colorOtherThan black),
    activated tapSymbol (addMana 1 (ofTheChosenColor (some [of black])))
  ]
}
