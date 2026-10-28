card := {
  name := "Chance Encounter",
  cost := [2, red, red],
  types := [enchantment],
  text := [
    whenever (does you (flipCoins anyAmount (some wins)))
      (putCounters 1 luckCounter thisEnchantment),
    at_ (beginningOfYour upkeep)
      (if_ := compareAmt (countersOn luckCounter thisEnchantment) atLeast 10) winGame
  ]
}
