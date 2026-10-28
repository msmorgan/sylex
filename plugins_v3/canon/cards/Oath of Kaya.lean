card := {
  name := "Oath of Kaya",
  cost := [1, white, black],
  supertypes := [legendary],
  types := [enchantment],
  text := [
    when (does this_ enter) [instruct this_ (dealDamage 3 (target isAnyTarget)), gainLife 3],
    whenever
      (does anOpponent (attackWith (counted (orMore 1) isCreature) (a [isPlaneswalker, youControl])))
      [instruct this_ (dealDamage 2 (that player)), gainLife 2]
  ]
}
