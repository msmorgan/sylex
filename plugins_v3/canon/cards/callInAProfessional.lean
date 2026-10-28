card := {
  name := "Call In a Professional",
  cost := [2, red],
  types := [instant],
  text := [
    spell
      [
        establishThroughout (cant (bare isPlayer) (gainLife anyAmount)) thisTurn,
        establishThroughout
          (cant any (preventDamage allDamage (damage := would any (dealDamage anyAmount)))) thisTurn,
        instruct this_ (dealDamage 3 (target isAnyTarget))
      ]
  ]
}
