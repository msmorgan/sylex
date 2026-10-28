card := {
  name := "Thriving Grove",
  types := [land],
  text := [
    entersTapped thisLand,
    entersChoosingFrom thisLand color (colorOtherThan green),
    activated tapSymbol (addMana 1 (ofTheChosenColor (some [of green])))
  ]
}
