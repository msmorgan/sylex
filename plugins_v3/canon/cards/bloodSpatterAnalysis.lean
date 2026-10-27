/-- Blood Spatter Analysis -/
card bloodSpatterAnalysis : Spelled :=
  { name := "Blood Spatter Analysis", cost := [pip .black, pip .red], types := [.enchantment],
    text :=
      [ when (does thisEnchantment enter)
          (instruct thisEnchantment
             (dealDamage 3 (target (and [isCreature, anOpponentControls])))),
        whenever (does (counted (orMore 1) isCreature) die)
          (sequentially
            [ mill 1,
              putCounters 1 bloodstainCounter thisEnchantment,
              triggerReflexively
                (doOnlyIf (sacrifice thisEnchantment)
                  (compareAmt (countersOn bloodstainCounter thisEnchantment) .atLeast 5))
                (returnToHand (target (and [isCreature, isCard, in_ yourGraveyard]))
                  yourGraveyard) ]) ] }
