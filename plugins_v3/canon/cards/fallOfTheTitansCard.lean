/-- Fall of the Titans -/
card fallOfTheTitansCard : Spelled :=
  { name := "Fall of the Titans", cost := [.variable, .variable, pip .red], types := [.instant],
    text :=
      [ surge [.variable, pip .red],
        spell (instruct this_
          (dealDamage (letter .x) (eachOf (targets (upTo 2) isAnyTarget)))) ] }
