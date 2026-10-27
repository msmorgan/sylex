/-- Edgewalker -/
card edgewalker : Spelled :=
  { name := "Edgewalker", cost := [generic 1, pip .white, pip .black], types := [.creature],
    subtypes := [human, cleric],
    text :=
      [ static (spellsCost (and [cleric, isSpell, did you (cast which) thisObject])
          (lessBySymbols (symbols [pip .white, pip .black]) (coloredOnly := true))) ],
    power := stat 2, toughness := stat 2 }
