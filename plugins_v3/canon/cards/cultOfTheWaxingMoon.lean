/-- Cult of the Waxing Moon -/
card cultOfTheWaxingMoon : Spelled :=
  { name := "Cult of the Waxing Moon", cost := [generic 4, pip .green],
    types := [.creature], subtypes := [human, shaman],
    text :=
      [ whenever
          (does any (transform (a (and [isPermanent, youControl]))
            (into := some (and [isCreature, not (hasSubtype human)]))))
          (create 1 (creatureToken 2 2 [.green] [wolf])) ],
    power := stat 5, toughness := stat 4 }
