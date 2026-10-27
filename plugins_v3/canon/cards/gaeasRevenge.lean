/-- Gaea's Revenge -/
card gaeasRevenge : Spelled :=
  { name := "Gaea's Revenge", cost := [generic 5, pip .green, pip .green],
    types := [.creature], subtypes := [elemental],
    text :=
      [ static (cant any (counter thisSpell)),
        haste,
        static (cant (by_ (allOf (or [ and [isSpell, non (hasColor .green)],
                        abilityOf (allOf (and [isSource, non (hasColor .green)])) ])))
                            (target thisCreature)) ],
    power := stat 8, toughness := stat 5 }
