card counterspellCard : Spelled :=
  { name := "Counterspell", cost := [pip .blue, pip .blue], types := [.instant],
    text := [spell (counter (target isSpell))] }
