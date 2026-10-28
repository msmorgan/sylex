card := {
  name := "Endless Atlas",
  cost := [2],
  types := [artifact],
  text := [
    activatedOnlyIf (compound [[2], tapSymbol]) (draw 1)
      (thereAre (withTheSameName (counted (orMore 3) [isLand, youControl])))
  ]
}
