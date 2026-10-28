card := {
  name := "Carom",
  cost := [1, white],
  types := [instant],
  text := [
    spell
      [
        establishThroughout
          (replace (would any (dealDamage anyAmount (target isCreature)))
             (instead (redirectDamage (theNext 1) (target [isCreature, isOther]))))
          thisTurn,
        draw 1
      ]
  ]
}
