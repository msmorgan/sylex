/-- Transguild Courier -/
card transguildCourier : Spelled :=
  { name := "Transguild Courier", cost := [generic 4], types := [.artifact, .creature],
    subtypes := [golem],
    text := [static (qualityDefinition thisCreature .sets (colored .every))],
    power := stat 3, toughness := stat 3 }
