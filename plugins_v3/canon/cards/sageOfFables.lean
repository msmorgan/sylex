card sageOfFables : Spelled :=
  { name := "Sage of Fables", cost := [generic 2, pip .blue], types := [.creature],
    subtypes := [merfolk, wizard],
    text :=
      [ static (entersWithAdditionalCounters
          (each (and [isCreature, wizard, youControl,
                       isOther]))
          1 p1p1Counter),
        activated
          (compound [[generic 2],
            (removeCounters (exactly 1) p1p1Counter
                       (a (and [isCreature, youControl])))])
          (draw 1) ],
    power := stat 2, toughness := stat 2 }
