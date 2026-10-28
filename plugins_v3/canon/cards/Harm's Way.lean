card := {
  name := "Harm's Way",
  cost := [white],
  types := [instant],
  text := [
    spell
      (establishThroughout
         (replace
            (would (aYourChoice isSource)
               (dealDamage anyAmount (andOr [you, allOf [isPermanent, youControl]])))
            (instead (redirectDamage (theNext 2) (target isAnyTarget))))
         thisTurn)
  ]
}
