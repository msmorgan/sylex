card := {
  name := "Healing Grace",
  cost := [white],
  types := [instant],
  text := [
    spell
      [
        establishThroughout
          (replace (would (by_ (aYourChoice isSource)) (dealDamage anyAmount (target isAnyTarget)))
             (instead (preventDamage (theNext 3))))
          thisTurn,
        gainLife 3
      ]
  ]
}
