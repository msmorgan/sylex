card flaringPain : Spelled :=
  { name := "Flaring Pain", cost := [generic 1, pip .red], types := [.instant],
    text :=
      [ spell (establishThroughout
            (cant any (preventDamage allDamage (damage := would any (dealDamage anyAmount))))
            thisTurn),
        flashback [pip .red] ] }
