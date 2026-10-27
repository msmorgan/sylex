/-- Alabaster Leech -/
card alabasterLeech : Spelled :=
  { name := "Alabaster Leech", cost := [pip .white], types := [.creature],
    subtypes := [leech],
    text := [static ((costShift (allOf (and [isSpell, hasColor .white, did you (cast which)
      thisObject])) (moreBySymbols (symbols [pip .white]))))],
    power := stat 1, toughness := stat 3 }
