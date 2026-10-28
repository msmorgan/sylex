card := {
  name := "Stasis",
  cost := [1, blue],
  types := [enchantment],
  text := [
    partSkip (bare isPlayer) untapStep,
    at_ (beginningOfYour upkeep) (doUnless (sacrifice thisEnchantment) [blue])
  ]
}
