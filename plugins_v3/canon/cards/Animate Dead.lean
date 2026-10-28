/-- Animate Dead -/
card := {
  name := "Animate Dead",
  cost := [1, black],
  types := [enchantment],
  subtypes := [aura],
  text := [
    enchant [isCreature, isCard, in_ graveyard],
    when (does thisAura enter)
      [
        (establish
           (conjunction
              [
                abilityLoss it [written (enchant [isCreature, isCard, in_ graveyard])],
                abilityGrant it
                  (enchant [isCreature, did (by_ thisAura) (putOntoBattlefield which)])
              ]
              (subject := it))),
        returnToBattlefield (enchanted (ofType card creature)) [under you],
        attachTo thisAura it,
        (delay (does thisAura (leave battlefield))
           (instruct (controllerOf (that creature)) (sacrifice it)))
      ]
      (if_ := is it (in_ battlefield)),
    getsPt (enchanted creature) (down 1) (up 0)
  ]
}
