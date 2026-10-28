card := {
  name := "Cold Storage",
  cost := [4],
  types := [artifact],
  text := [
    activated [3] (exile (target [isCreature, youControl])),
    activated (sacrifice thisArtifact)
      (returnToBattlefield (each [isCreature, cardIn (exiledWith thisArtifact)]) [(under you)])
  ]
}
