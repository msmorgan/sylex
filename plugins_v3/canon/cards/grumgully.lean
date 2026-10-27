card grumgully : Spelled :=
  { name := "Grumgully, the Generous", cost := [generic 1, pip .red, pip .green],
    supertypes := [.legendary], types := [.creature],
    subtypes := [goblin, shaman],
    text :=
      [ static (entersWithAdditionalCounters
          (each (and [ isCreature, youControl,
                        non human, isOther ]))
          1 p1p1Counter) ],
    power := stat 3, toughness := stat 3 }
