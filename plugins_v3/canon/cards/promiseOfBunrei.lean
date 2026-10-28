/-- Promise of Bunrei -/
card := {
  name := "Promise of Bunrei",
  cost := [2, white],
  types := [enchantment],
  text := [
    when (does (a [isCreature, youControl]) die)
      (doIfDone (sacrifice thisEnchantment) (create 4 (creatureToken 1 1 [] [spirit])))
  ]
}
