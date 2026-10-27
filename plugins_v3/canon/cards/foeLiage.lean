card foeLiage : Spelled :=
  { name := "Foe-liage", cost := [generic 3, pip .green], types := [.creature],
    subtypes := [plant, mutant],
    text :=
      [ whenever (does (a isLand) enter) (during := duringPart .turn you)
          (putCounters 1 p1p1Counter thisCreature) ],
    power := stat 3, toughness := stat 3 }
