card := {
  name := "Vampiric Embrace",
  cost := [2, black, black],
  types := [enchantment],
  subtypes := [aura],
  text := [
    enchant isCreature,
    getsPt (enchanted creature) (up 2) (up 2),
    abilityGrant (enchanted creature) flying,
    whenever
      (does
         (a
            [
              isCreature,
              did (actor (attachHost enchanted creature)) (dealDamage anyAmount which)
                (within := some (within soFarThisTurn none))
            ])
         die)
      (putCounters 1 p1p1Counter (that creature))
  ]
}
