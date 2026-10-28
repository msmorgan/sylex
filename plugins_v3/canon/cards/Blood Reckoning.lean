card := {
  name := "Blood Reckoning",
  cost := [3, black],
  types := [enchantment],
  text := [
    whenever (does (a isCreature) (attack (youOr (a [isPlaneswalker, youControl]))))
      (instruct (controllerOf (that creature)) (loseLife 1))
  ]
}
