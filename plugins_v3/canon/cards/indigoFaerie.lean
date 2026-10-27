/-- Indigo Faerie -/
card indigoFaerie : Spelled :=
  { name := "Indigo Faerie", cost := [generic 1, pip .blue], types := [.creature],
    subtypes := [faerie, wizard],
    text :=
      [ flying,
        activated [pip .blue]
          (establishThroughout
             (qualityChange (target isPermanent) .adds
                (colored (.some [.blue]))) untilEndOfTurn) ],
    power := stat 1, toughness := stat 1 }
