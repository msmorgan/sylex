/-- Concordant Crossroads -/
card concordantCrossroads : Spelled :=
  { name := "Concordant Crossroads", cost := [pip .green], supertypes := [.world],
    types := [.enchantment], text := [static (abilityGrant (allOf isCreature) haste)]
        }
