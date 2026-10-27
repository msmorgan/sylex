/-- Propaganda -/
card propaganda : Spelled :=
  { name := "Propaganda", cost := [generic 2, pip .blue], types := [.enchantment],
    text := [static (gatedBy (scaledGenericMana
            (forEach 2
               (and [isCreature, controlledBy they, inCombat .attackerOf you]))) (actor
                                                                                           (allOf
                                                                                              isCreature)) (attack you))] }
