card := {
  name := "Prism Ring",
  cost := [1],
  types := [artifact],
  text := [
    entersChoosing thisArtifact color,
    whenever (does you (cast (a [isSpell, ofChosen color]))) (gainLife 1)
  ]
}
