card reflectDamage : Spelled :=
  { name := "Reflect Damage", cost := [generic 3, pip .red, pip .white], types := [.instant],
    text :=
      [ spell
          (establishThroughout
            (replace (would (aYourChoice isSource) (dealDamage anyAmount))
              (instead (redirectDamage allDamage (controllerOf thatSource)))
              (use := .nextTimeOnly))
            thisTurn) ] }
