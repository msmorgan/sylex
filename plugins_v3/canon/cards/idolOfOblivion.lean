card := {
  name := "Idol of Oblivion",
  cost := [2],
  types := [artifact],
  text := [
    activatedOnlyIf tapSymbol (draw 1)
      (did you (createObject anyAmount (token (copyOf (a isToken)) [])) soFarThisTurn),
    activated (compound [[8], tapSymbol, sacrifice thisArtifact])
      (create 1 (creatureToken 10 10 [] [eldrazi]))
  ]
}
