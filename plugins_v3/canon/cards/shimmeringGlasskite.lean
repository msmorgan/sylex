/-- Shimmering Glasskite -/
card shimmeringGlasskite : Spelled :=
  { name := "Shimmering Glasskite", cost := [generic 3, pip .blue], types := [.creature],
    subtypes := [spirit],
    text :=
      [ flying,
        whenever
          (nthOccurrence (.nth 1)
            (does (a (or [isSpell, abilityHead .anyOnStack])) (target thisCreature)) (some .turn))
          (counter (that .stack)) ],
    power := stat 2, toughness := stat 3 }
