card pridemalkin : Spelled :=
  { name := "Pridemalkin", cost := [generic 2, pip .green], types := [.creature],
    subtypes := [cat],
    text :=
      [ when (does thisCreature enter)
          (putCounters 1 p1p1Counter (target (and [isCreature, youControl]))),
        static (abilityGrant
          (each (and [isCreature, youControl, hasCounter p1p1Counter]))
          trample) ],
    power := stat 2, toughness := stat 1 }
