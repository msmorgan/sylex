/-- Ulamog's Crusher -/
card ulamogsCrusher : Spelled :=
  { name := "Ulamog's Crusher", cost := [generic 8], types := [.creature],
    subtypes := [eldrazi],
    text :=
      [ annihilator 2,
        static (must thisCreature attack) ],
    power := stat 8, toughness := stat 8 }
