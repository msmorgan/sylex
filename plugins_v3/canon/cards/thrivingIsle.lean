card := {
  name := "Thriving Isle",
  types := [land],
  text := [
    entersTapped thisLand,
    entersChoosingFrom thisLand color (colorOtherThan blue),
    activated tapSymbol (addMana 1 (ofTheChosenColor (some [of blue])))
  ]
}
