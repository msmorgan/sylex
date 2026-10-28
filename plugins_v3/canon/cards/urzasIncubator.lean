card := {
  name := "Urza's Incubator",
  cost := [3],
  types := [artifact],
  text := [
    entersChoosing thisArtifact (subtype creature),
    costShift (allOf [isCreature, isSpell, ofChosen (subtype creature)]) (less 2)
  ]
}
