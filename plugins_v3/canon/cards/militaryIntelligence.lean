/-- Military Intelligence -/
card militaryIntelligence : Spelled :=
  { name := "Military Intelligence", cost := [generic 1, pip .blue], types := [.enchantment],
    text :=
      [ whenever (does you (attackWith (counted (orMore 2) isCreature))) (draw 1) ] }
