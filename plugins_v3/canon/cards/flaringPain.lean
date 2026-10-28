card := {
  name := "Flaring Pain",
  cost := [1, red],
  types := [instant],
  text := [
    spell
      (establishThroughout
         (cant any (preventDamage allDamage (damage := would any (dealDamage anyAmount)))) thisTurn),
    flashback [red]
  ]
}
