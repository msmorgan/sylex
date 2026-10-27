/-- Moonlit Meditation -/
card moonlitMeditation : Spelled :=
  { name := "Moonlit Meditation", cost := [generic 2, pip .blue], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant (and [or [isArtifact, isCreature], youControl]),
        static (replace
                  (nthOccurrence (.nth 1)
                     (would (by_ you)
                        (createObject anyAmount (token (copyOf (counted (orMore 1) isToken)) [])))
                     (some .turn))
                  (instead (may (create groupSize (copyOf (enchanted permanent)) [])))) ] }
