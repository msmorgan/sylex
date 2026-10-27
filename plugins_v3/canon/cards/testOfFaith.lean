card testOfFaith : Spelled :=
  { name := "Test of Faith", cost := [generic 1, pip .white], types := [.instant],
    text :=
      [ spell (establishThroughout
            (replace (would any (dealDamage anyAmount (target isCreature)))
              (instead
                (preventDamage (theNext 3)
                  (also := putCounters preventedThisWay p1p1Counter (that creature)))))
            thisTurn) ] }
