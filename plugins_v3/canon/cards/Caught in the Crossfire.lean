/-- Caught in the Crossfire -/
card := {
  name := "Caught in the Crossfire",
  cost := [red, red],
  types := [instant],
  text := [
    spell
      (chooseSpree
         [
           ([1], instruct this_ (dealDamage 2 (each [isCreature, isOutlaw]))),
           ([1], instruct this_ (dealDamage 2 (each [isCreature, non isOutlaw])))
         ])
  ]
}
