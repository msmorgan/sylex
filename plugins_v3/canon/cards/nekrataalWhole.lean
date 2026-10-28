card := {
  name := "Nekrataal",
  cost := [2, black, black],
  types := [creature],
  subtypes := [human, assassin],
  text := [
    firstStrike,
    when (does thisCreature enter)
      (doUnderProhibition (destroy (target [isCreature, non isArtifact, not (hasColor black)]))
         (regeneration (that creature)))
  ],
  power := 2,
  toughness := 1
}
