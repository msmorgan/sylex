card := {
  name := "Candles' Glow",
  cost := [1, white],
  types := [instant],
  subtypes := [arcane],
  text := [
    spell
      (establishThroughout
         (replace (would any (dealDamage anyAmount (target isAnyTarget)))
            (instead (preventDamage (theNext 3) (also := gainLife preventedThisWay))))
         thisTurn),
    Ability.splice (hasSubtype arcane) [1, white]
  ]
}
