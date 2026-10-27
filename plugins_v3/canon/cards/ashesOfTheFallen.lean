/-- Ashes of the Fallen -/
card ashesOfTheFallen : Spelled :=
  { name := "Ashes of the Fallen", cost := [generic 2], types := [.artifact],
    text :=
      [ static (entersChoosing thisArtifact (.subtype .creature)),
        static (qualityChange (each (and [isCreature, in_ yourGraveyard])) .adds
          (chosenQuality (ofChosen (.subtype .creature)))) ] }
