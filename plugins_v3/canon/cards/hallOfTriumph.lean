card := {
  name := "Hall of Triumph",
  cost := [3],
  supertypes := [legendary],
  types := [artifact],
  text := [
    entersChoosing thisArtifact color,
    getsPt (allOf [isCreature, youControl, ofChosen color]) (up 1) (up 1)
  ]
}
