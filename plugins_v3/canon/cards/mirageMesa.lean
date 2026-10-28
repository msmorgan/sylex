card := {
  name := "Mirage Mesa",
  types := [land],
  subtypes := [desert],
  text := [
    entersTapped thisLand,
    entersChoosing thisLand color,
    activated tapSymbol (addMana 1 ofTheChosenColor)
  ]
}
