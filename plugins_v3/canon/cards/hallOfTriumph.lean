card hallOfTriumph : Spelled :=
  { name := "Hall of Triumph", cost := [generic 3], supertypes := [.legendary],
    types := [.artifact],
    text :=
      [ static (entersChoosing thisArtifact .color),
        static (getsPt (allOf (and [isCreature, youControl, ofChosen .color]))
          (up 1) (up 1)) ] }
