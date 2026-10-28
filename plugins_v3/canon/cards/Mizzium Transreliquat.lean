card := {
  name := "Mizzium Transreliquat",
  cost := [3],
  types := [artifact],
  text := [
    activated [3] (establishThroughout (copyChange thisArtifact (target isArtifact)) untilEndOfTurn),
    activated [1, blue, red]
      (establish (copyChange thisArtifact (target isArtifact) [exceptThisAbility]))
  ]
}
