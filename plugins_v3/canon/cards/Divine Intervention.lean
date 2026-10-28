card := {
  name := "Divine Intervention",
  cost := [6, white, white],
  types := [enchantment],
  text := [
    entersWithCounters thisEnchantment 2 interventionCounter,
    at_ (beginningOfYour upkeep) (removeCounters (exactly 1) interventionCounter thisEnchantment),
    when (does (by_ you) (removeCounters theLast interventionCounter thisEnchantment)) drawGame
  ]
}
