card := {
  name := "Investigator's Journal",
  cost := [2],
  types := [artifact],
  subtypes := [book, clue],
  text := [
    entersWithCounters thisArtifact
      (aggregateOver max isPlayer (countOf [isCreature, controlledBy they])) suspectCounter,
    activated (compound [[2], tapSymbol, (removeCounters (exactly 1) suspectCounter thisArtifact)])
      (draw 1),
    activated (compound [[2], sacrifice thisArtifact]) (draw 1)
  ]
}
