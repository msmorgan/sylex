/-- Pins winning a coin flip, which differs from merely obtaining heads or tails
[CR#705.2], and the luck-counter threshold on its upkeep trigger. -/
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
