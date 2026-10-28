card := {
  name := "Crash",
  cost := [2, red],
  types := [instant],
  text := [altCost this_ (sacrifice (a [isLand, mountain])), spell (destroy (target isArtifact))]
}
