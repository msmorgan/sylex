/-- Loaming Shaman -/
card loamingShaman : Spelled :=
  { name := "Loaming Shaman", cost := [generic 2, pip .green], types := [.creature],
    subtypes := [centaur, shaman],
    text :=
      [ when (does thisCreature enter)
          (instruct (target isPlayer) (shuffleInto
            (targets anyNumber (cardIn (graveyardOf they))))) ],
    power := stat 3, toughness := stat 2 }
