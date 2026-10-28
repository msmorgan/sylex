card := {
  name := "Burn at the Stake",
  cost := [2, red, red, red],
  types := [sorcery],
  text := [
    addedCost (tap (counted anyNumber [isCreature, youControl, isUntapped])) false,
    spell (instruct this_ (dealDamage (times 3 groupSize) (target isAnyTarget)))
  ]
}
