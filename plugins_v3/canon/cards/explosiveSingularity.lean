/-- Explosive Singularity -/
card := {
  name := "Explosive Singularity",
  cost := [8, red, red],
  types := [sorcery],
  text := [
    addedCost (tap (counted anyNumber [isCreature, youControl, isUntapped])) true,
    costShift this_ (less (times 1 groupSize)),
    spell (instruct this_ (dealDamage 10 (target isAnyTarget)))
  ]
}
