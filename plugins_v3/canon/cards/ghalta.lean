card ghalta : Spelled :=
  { name := "Ghalta, Primal Hunger", cost := [generic 10, pip .green, pip .green],
    supertypes := [.legendary], types := [.creature],
    subtypes := [elder, dinosaur],
    text :=
      [ static (conjunction [ costShift this_ (less (letter .x)),
            letterDefinition .x (aggregate .sum (.stat .power) (and [isCreature, youControl])) ]),
        trample ],
    power := stat 12, toughness := stat 12 }
