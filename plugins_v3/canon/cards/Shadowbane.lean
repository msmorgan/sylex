card := {
  name := "Shadowbane",
  cost := [1, white],
  types := [instant],
  text := [
    spell
      (establishThroughout
         (replace
            (would (aYourChoice isSource)
               (dealDamage anyAmount (andOr [you, allOf [isCreature, youControl]])))
            (instead
               (preventDamage allDamage
                  (also := doIf
                     (did any
                        (preventDamage allDamage
                           (damage := would (a [isSource, hasColor black]) (dealDamage anyAmount)))
                        thisWay)
                     (gainLife thatMuch))))
            (use := nextTimeOnly))
         thisTurn)
  ]
}
