/-- Woeleecher -/
card woeleecherWhole : Spelled :=
  { name := "Woeleecher", cost := [generic 5, pip .white], types := [.creature],
    subtypes := [elemental], text := [(activated (compound [[pip .white], tapSymbol])
          (doIfDone
            (removeCounters (exactly 1) m1m1Counter (target isCreature))
            (gainLife 2)))],
    power := stat 3, toughness := stat 5 }
