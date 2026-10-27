card urzasIncubator : Spelled :=
  { name := "Urza's Incubator", cost := [generic 3], types := [.artifact],
    text :=
      [ static (entersChoosing thisArtifact (.subtype .creature)),
        static ((costShift (allOf (and [isCreature, isSpell, ofChosen (.subtype .creature)])) (less
          2))) ] }
