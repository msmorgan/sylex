card voraciousBrood : Spelled :=
  { name := "Voracious Brood", cost := [generic 2, pip .green], types := [.creature],
    subtypes := [alien, insect],
    text :=
      [ static (entersWithCounters thisCreature
          (forEach 1 (and [isCreature, in_ yourGraveyard])) p1p1Counter),
        whenever (does any
                    (putInto (counted (orMore 1) (and [isCard, isCreature])) yourGraveyard
                       (from_ := some any)))
          (putCounters groupSize p1p1Counter thisCreature) ],
    power := stat 1, toughness := stat 1 }
