/-- Leonin Bola -/
card leoninBola : Spelled :=
  { name := "Leonin Bola", cost := [generic 1], types := [.artifact],
    subtypes := [equipment],
    text :=
      [ static (abilityGrant (equipped creature)
          (activated (compound [tapSymbol, unattach (theGrantor .permanent)])
            (tap (target isCreature)))),
        equip [generic 1] ] }
