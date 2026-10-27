card bramblewoodParagon : Spelled :=
  { name := "Bramblewood Paragon", cost := [generic 1, pip .green], types := [.creature],
    subtypes := [elf, warrior],
    text :=
      [ static (entersWithAdditionalCounters
          (each (and [ isCreature, warrior, youControl,
                        isOther ]))
          1 p1p1Counter),
        static (abilityGrant
          (each (and [isCreature, youControl, hasCounter p1p1Counter]))
          trample) ],
    power := stat 2, toughness := stat 2 }
