/-- Sphinx of the Second Sun -/
card sphinxOfTheSecondSun : Spelled :=
  { name := "Sphinx of the Second Sun", cost := [generic 6, pip .blue, pip .blue],
    types := [.creature], subtypes := [sphinx],
    text :=
      [ flying,
        at_ (beginningOfYour .postcombatMain)
          (additionalPhase .beginningPhase (some .postcombatMain) 1) ],
    power := stat 6, toughness := stat 6 }
