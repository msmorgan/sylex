card := {
  name := "Crossroads Village",
  types := [land],
  subtypes := [town],
  text := [
    entersTapped thisLand,
    entersChoosing thisLand color,
    activated tapSymbol (addMana 1 ofTheChosenColor)
  ]
}
