card := {
  name := "Divine Deflection",
  cost := [«variable», white],
  types := [instant],
  text := [
    spell
      (establishThroughout
         (replace (would any (dealDamage anyAmount (andOr [you, allOf [isPermanent, youControl]])))
            (instead
               (preventDamage (theNext (letter x))
                  (also := instruct this_ (dealDamage thatMuch (target isAnyTarget))))))
         thisTurn)
  ]
}
