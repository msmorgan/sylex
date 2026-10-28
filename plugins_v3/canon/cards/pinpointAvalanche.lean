card := {
  name := "Pinpoint Avalanche",
  cost := [3, red, red],
  types := [instant],
  text := [
    spell
      [
        instruct this_ (dealDamage 4 (target isCreature)),
        establish (cant any (preventDamage allDamage)) modifiesPrecedingAction
      ]
  ]
}
