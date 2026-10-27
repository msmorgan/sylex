/-- Ghostly Prison -/
card ghostlyPrisonWhole : Spelled :=
  { name := "Ghostly Prison", cost := [generic 2, pip .white], types := [.enchantment],
    text := [static (gatedBy (scaledGenericMana
            (forEach 2
               (and [isCreature, controlledBy they, inCombat .attackerOf you]))) (actor
                                                                                           (allOf
                                                                                              isCreature)) (attack you))] }
