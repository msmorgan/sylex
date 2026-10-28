/-- Ashes of the Fallen -/
card := {
  name := "Ashes of the Fallen",
  cost := [2],
  types := [artifact],
  text := [
    entersChoosing thisArtifact (subtype creature),
    qualityChange (each [isCreature, in_ yourGraveyard]) adds
      (chosenQuality (ofChosen (subtype creature)))
  ]
}
