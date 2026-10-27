/-- Foriysian Brigade -/
card foriysianBrigade : Spelled :=
  { name := "Foriysian Brigade", cost := [generic 3, pip .white], types := [.creature],
    subtypes := [human, soldier],
    text :=
        [static
           (can thisCreature (block (counted (additional (exactly 1)) isCreature))
              (timing := eachCombat))],
    power := stat 2, toughness := stat 4 }
