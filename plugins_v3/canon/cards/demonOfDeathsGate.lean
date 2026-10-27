card demonOfDeathsGate : Spelled :=
  { name := "Demon of Death's Gate", cost := [generic 6, pip .black, pip .black, pip .black],
    types := [.creature], subtypes := [demon],
    text :=
      [ static (altCost this_
          (compound
            [ payLife 6,
              (sacrifice (counted (exactly 3) (and [isCreature, hasColor .black]))) ])),
        flying, trample ],
    power := stat 9, toughness := stat 9 }
