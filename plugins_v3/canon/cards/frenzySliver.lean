/-- Frenzy Sliver -/
card frenzySliver : Spelled :=
  { name := "Frenzy Sliver", cost := [generic 1, pip .black], types := [.creature],
    subtypes := [sliver],
    text := [static (abilityGrant (allOf (and [sliver, isCreature])) (frenzy 1))],
    power := stat 1, toughness := stat 1 }
