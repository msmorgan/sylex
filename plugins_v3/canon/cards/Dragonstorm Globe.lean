card := {
  name := "Dragonstorm Globe",
  cost := [3],
  types := [artifact],
  text := [
    entersWithAdditionalCounters (each [dragon, youControl]) 1 p1p1Counter,
    activated tapSymbol (addMana 1 (ofAnyColor sameColor))
  ]
}
