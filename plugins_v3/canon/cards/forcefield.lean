/-- Forcefield -/
card forcefield : Spelled :=
  { name := "Forcefield", cost := [generic 3], types := [.artifact],
    text :=
      [ activated [generic 1]
          (establishThroughout
             (replace
               (would (aYourChoice (and [isCreature, isUnblocked]))
                 (dealDamage anyAmount you (some .combat)))
               (instead (preventDamage (allBut 1))) (use := .nextTimeOnly))
            thisTurn) ] }
