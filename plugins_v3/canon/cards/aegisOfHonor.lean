card aegisOfHonor : Spelled :=
  { name := "Aegis of Honor", cost := [pip .white], types := [.enchantment],
    text :=
      [ activated [generic 1]
          (establishThroughout
            (replace (would (a isInstantOrSorcery) (dealDamage anyAmount you))
              (instead (redirectDamage allDamage (controllerOf it))) (use := .nextTimeOnly))
            thisTurn) ] }
