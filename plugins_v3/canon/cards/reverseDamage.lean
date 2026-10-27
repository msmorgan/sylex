card reverseDamage : Spelled :=
  { name := "Reverse Damage", cost := [generic 1, pip .white, pip .white], types := [.instant],
    text :=
      [ spell
          (establishThroughout
            (replace (would (aYourChoice isSource) (dealDamage anyAmount you))
              (instead (preventDamage allDamage (also := gainLife preventedThisWay)))
              (use := .nextTimeOnly))
            thisTurn) ] }
