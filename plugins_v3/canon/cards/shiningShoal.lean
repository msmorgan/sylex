card := {
  name := "Shining Shoal",
  cost := [«variable», white, white],
  types := [instant],
  subtypes := [arcane],
  text := [
    altCost this_ (exile (a [hasColor white, hasManaValue eq (letter x), in_ yourHand])),
    spell
      (establishThroughout
         (replace
            (would (aYourChoice isSource)
               (dealDamage anyAmount (andOr [you, allOf [isCreature, youControl]])))
            (instead (redirectDamage (theNext (letter x)) (target isAnyTarget))))
         thisTurn)
  ]
}
