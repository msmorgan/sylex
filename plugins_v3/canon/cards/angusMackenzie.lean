/-- Angus Mackenzie -/
card angusMackenzie : Spelled :=
  { name := "Angus Mackenzie", cost := [pip .green, pip .white, pip .blue],
    supertypes := [.legendary], types := [.creature],
    subtypes := [human, cleric],
    text :=
      [ activatedOnlyDuring (compound [[pip .green, pip .white, pip .blue], tapSymbol])
          (preventAll any thisTurn (some .combat))
          (beforePart .combatDamage none) ],
    power := stat 2, toughness := stat 2 }
