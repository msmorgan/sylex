card := {
  name := "Temper",
  cost := [«variable», 1, white],
  types := [instant],
  text := [
    spell
      (establishThroughout
         (replace (would any (dealDamage anyAmount (target isCreature)))
            (instead
               (preventDamage (theNext (letter x))
                  (also := putCounters preventedThisWay p1p1Counter (that creature)))))
         thisTurn)
  ]
}
