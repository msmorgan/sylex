card turnTheTables : Spelled :=
  { name := "Turn the Tables", cost := [generic 3, pip .white, pip .white], types := [.instant],
    text :=
      [ spell (establishThroughout
            (replace (would any (dealDamage anyAmount you (some .combat)))
              (instead (redirectDamage allDamage (target (and [isCreature, isAttacking])))))
            thisTurn) ] }
