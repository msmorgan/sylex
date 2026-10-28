card := {
  name := "Celestial Convergence",
  cost := [2, white, white],
  types := [enchantment],
  text := [
    entersWithCounters thisEnchantment 7 omenCounter,
    at_ (beginningOfYour upkeep)
      [
        removeCounters (exactly 1) omenCounter thisEnchantment,
        doIf (compareAmt (countersOn omenCounter thisEnchantment) atMost 0)
          (doIf
             (thereAre (counted (orMore 2) [isPlayer, tiedFor max (playerStat lifeTotal) isPlayer]))
             drawGame
             (instruct (the [isPlayer, superlative max (playerStat lifeTotal) isPlayer]) winGame))
      ]
  ]
}
