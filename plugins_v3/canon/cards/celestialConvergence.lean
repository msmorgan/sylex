card celestialConvergence : Spelled :=
  { name := "Celestial Convergence", cost := [generic 2, pip .white, pip .white],
    types := [.enchantment],
    text :=
      [ static (entersWithCounters thisEnchantment 7 omenCounter),
        at_ (beginningOfYour .upkeep)
          (sequentially
            [ removeCounters (exactly 1) omenCounter thisEnchantment,
              doIf (compareAmt (countersOn omenCounter thisEnchantment) .atMost 0)
                (doIf (thereAre (counted (orMore 2)
                        (and [isPlayer, tiedFor .max (.playerStat .lifeTotal) isPlayer])))
                  drawGame
                  (instruct
                    (the (and [isPlayer, superlative .max (.playerStat .lifeTotal) isPlayer]))
                    winGame)) ]) ] }
