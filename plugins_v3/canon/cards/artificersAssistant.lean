/-- Artificer's Assistant -/
card artificersAssistant : Spelled :=
  { name := "Artificer's Assistant", cost := [pip .blue], types := [.creature],
    subtypes := [bird],
    text :=
      [ flying,
        whenever (does you (cast (a
          (and [isHistoric, isSpell])))) (scry 1)
            ],
    power := stat 1, toughness := stat 1 }
