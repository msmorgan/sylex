card := {
  name := "Test of Faith",
  cost := [1, white],
  types := [instant],
  text := [
    spell
      (establishThroughout
         (replace (would any (dealDamage anyAmount (target isCreature)))
            (instead
               (preventDamage (theNext 3)
                  (also := putCounters preventedThisWay p1p1Counter (that creature)))))
         thisTurn)
  ]
}
