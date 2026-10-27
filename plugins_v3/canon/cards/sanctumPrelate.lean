/-- Sanctum Prelate -/
card sanctumPrelate : Spelled :=
  { name := "Sanctum Prelate", cost := [generic 1, pip .white, pip .white],
    types := [.creature], subtypes := [human, cleric],
    text :=
      [ static (entersChoosing thisCreature .number),
        static
            (cant any
               (cast (bare (and [isSpell, non isCreature, hasManaValue .eq chosenNumber])))) ],
    power := stat 2, toughness := stat 2 }
