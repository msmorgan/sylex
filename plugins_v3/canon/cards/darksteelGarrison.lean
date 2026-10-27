/-- Darksteel Garrison -/
card darksteelGarrison : Spelled :=
  { name := "Darksteel Garrison", cost := [generic 2], types := [.artifact],
    subtypes := [fortification],
    text :=
      [ static (abilityGrant (fortified land) indestructible),
        whenever (does (fortified land) (setStatus tapped theActor))
          (get (target isCreature) (up 1) (up 1) untilEndOfTurn),
        fortify [generic 3] ] }
