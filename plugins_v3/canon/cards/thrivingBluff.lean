card := {
  name := "Thriving Bluff",
  types := [land],
  text := [
    entersTapped thisLand,
    entersChoosingFrom thisLand color (colorOtherThan red),
    activated tapSymbol (addMana 1 (ofTheChosenColor (some [of red])))
  ]
}
