card temper : Spelled :=
  { name := "Temper", cost := [.variable, generic 1, pip .white], types := [.instant],
    text :=
      [ spell (establishThroughout
            (replace (would any (dealDamage anyAmount (target isCreature)))
              (instead
                (preventDamage (theNext (letter .x))
                  (also := putCounters preventedThisWay p1p1Counter (that creature)))))
            thisTurn) ] }
