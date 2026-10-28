card := {
  name := "Mogg Salvage",
  cost := [2, red],
  types := [instant],
  text := [
    onlyWhile (altCost this_)
      [exists_ [isLand, island, anOpponentControls], exists_ [isLand, mountain, youControl]],
    spell (destroy (target isArtifact))
  ]
}
