card := {
  name := "Thriving Heath",
  types := [land],
  text := [
    entersTapped thisLand,
    entersChoosingFrom thisLand color (colorOtherThan white),
    activated tapSymbol (addMana 1 (ofTheChosenColor (some [of white])))
  ]
}
