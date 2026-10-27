card amorphousAxe : Spelled :=
  { name := "Amorphous Axe", cost := [generic 2], types := [.artifact],
    subtypes := [equipment],
    text :=
      [ static (conjunction [ getsPt (equipped creature) (up 3) (up 0),
            qualityChange (equipped creature) .adds (everyTypeOf
                .creature) ]),
        equip [generic 3] ] }
