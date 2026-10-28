card := {
  name := "Abolish",
  cost := [1, white, white],
  types := [instant],
  text := [
    altCost this_ (discard (a [plains, in_ yourHand])),
    spell (destroy (target (or [isArtifact, isEnchantment])))
  ]
}
