/-- Caught in the Crossfire -/
card caughtInTheCrossfire : Spelled :=
  { name := "Caught in the Crossfire", cost := [pip .red, pip .red], types := [.instant],
    text :=
      [ spell (chooseSpree
          [ ([generic 1],
             instruct this_ (dealDamage 2 (each (and [isCreature, isOutlaw])))),
            ([generic 1],
             instruct this_
                 (dealDamage 2 (each (and [isCreature, non isOutlaw])))) ]) ] }
