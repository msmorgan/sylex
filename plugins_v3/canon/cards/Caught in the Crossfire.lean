card := {
  name := "Caught in the Crossfire",
  cost := [red, red],
  types := [instant],
  text := [
    spell
      (chooseSpree
         [
           { cost := [1], action := instruct this_ (dealDamage 2 (each [isCreature, isOutlaw])) },
           { cost := [1], action := instruct this_ (dealDamage 2 (each [isCreature, non isOutlaw])) }
         ])
  ]
}
