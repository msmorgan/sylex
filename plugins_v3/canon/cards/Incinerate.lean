card := {
  name := "Incinerate",
  cost := [1, red],
  types := [instant],
  text := [
    spell
      [
        instruct this_ (dealDamage 3 (target isAnyTarget)),
        establishThroughout
          (cant any (regeneration (a [isCreature, was (dealDamage anyAmount which) thisWay])))
          thisTurn
      ]
  ]
}
