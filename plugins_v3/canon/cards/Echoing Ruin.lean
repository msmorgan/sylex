card := {
  name := "Echoing Ruin",
  cost := [1, red],
  types := [sorcery],
  text := [
    spell
      (destroy
         (both (target isArtifact)
            (allOf [isArtifact, (shares cardName share (that artifact)), isOther])))
  ]
}
