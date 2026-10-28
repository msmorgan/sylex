card := {
  name := "Helm of Possession",
  cost := [4],
  types := [artifact],
  text := [
    mayDeclineUntap thisArtifact you,
    activated (compound [[2], tapSymbol, sacrifice (a isCreature)])
      (gainControl (target isCreature)
         (forAsLongAs [is thisArtifact youControl, is thisArtifact isTapped]))
  ]
}
