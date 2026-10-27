/-- Rustler Rampage -/
card rustlerRampage : Spelled :=
  { name := "Rustler Rampage", cost := [pip .white], types := [.instant],
    text :=
      [ spell (chooseSpree
          [ ([generic 1],
             untap
               (allOf (and [isCreature, controlledBy (target isPlayer)]))),
            ([generic 1],
             gain (target isCreature) doubleStrike untilEndOfTurn) ]) ] }
