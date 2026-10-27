card runedStalactite : Spelled :=
  { name := "Runed Stalactite", cost := [generic 1], types := [.artifact],
    subtypes := [equipment],
    text :=
      [ static (conjunction [ getsPt (equipped creature) (up 1) (up 1),
            qualityChange (equipped creature) .adds (everyTypeOf
                .creature) ]),
        equip [generic 2] ] }
