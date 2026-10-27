/-- Thousand Moons Infantry -/
card thousandMoonsInfantry : Spelled :=
  { name := "Thousand Moons Infantry", cost := [generic 2, pip .white], types := [.creature],
    subtypes := [human, soldier],
    text := [static (untapDuring thisCreature (each isOtherPlayer))],
    power := stat 2, toughness := stat 4 }
