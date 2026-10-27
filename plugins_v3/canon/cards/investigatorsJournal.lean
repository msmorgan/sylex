/-- Investigator's Journal -/
card investigatorsJournal : Spelled :=
  { name := "Investigator's Journal", cost := [generic 2], types := [.artifact],
    subtypes := [book, clue],
    text :=
      [ static (entersWithCounters thisArtifact (aggregateOver .max isPlayer (countOf (and
        [isCreature, controlledBy they]))) suspectCounter),
        activated
          (compound [[generic 2], tapSymbol,
            (removeCounters (exactly 1) suspectCounter thisArtifact)])
          (draw 1),
        activated (compound [[generic 2], sacrifice thisArtifact])
          (draw 1) ] }
