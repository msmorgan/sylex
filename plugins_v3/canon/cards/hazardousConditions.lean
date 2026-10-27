/-- Hazardous Conditions -/
card hazardousConditions : Spelled :=
  { name := "Hazardous Conditions", cost := [generic 2, pip .black, pip .green],
    types := [.sorcery],
    text :=
      [ spell (get (allOf (and [isCreature, not hasCounter]))
          (down 2) (down 2) untilEndOfTurn) ] }
