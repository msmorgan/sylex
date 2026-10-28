/-- Blood Spatter Analysis -/
card := {
  name := "Blood Spatter Analysis",
  cost := [black, red],
  types := [enchantment],
  text := [
    when (does thisEnchantment enter)
      (instruct thisEnchantment (dealDamage 3 (target [isCreature, anOpponentControls]))),
    whenever (does (counted (orMore 1) isCreature) die)
      [
        mill 1,
        putCounters 1 bloodstainCounter thisEnchantment,
        triggerReflexively
          (doOnlyIf (sacrifice thisEnchantment)
             (compareAmt (countersOn bloodstainCounter thisEnchantment) atLeast 5))
          (returnToHand (target [isCreature, isCard, in_ yourGraveyard]) yourGraveyard)
      ]
  ]
}
