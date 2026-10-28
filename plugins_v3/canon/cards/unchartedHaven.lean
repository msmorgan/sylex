card := {
  name := "Uncharted Haven",
  types := [land],
  text := [
    entersTapped thisLand,
    entersChoosing thisLand color,
    activated tapSymbol (addMana 1 ofTheChosenColor)
  ]
}
