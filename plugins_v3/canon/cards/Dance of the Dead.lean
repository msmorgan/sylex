/-- Dance of the Dead -/
card := {
  name := "Dance of the Dead",
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
        putOntoBattlefield (enchanted (ofType card creature))
          (riders := [entersAs tapped, under you]),
        attachTo thisAura it,
        (delay (does thisAura (leave battlefield))
           (instruct (controllerOf (that creature)) (sacrifice it)))
      ]
      (if_ := is it (in_ battlefield)),
    conjunction [getsPt (enchanted creature) (up 1) (up 1), doesntUntap it (controllerOf it)]
      (subject := enchanted creature),
    at_ (beginningOfPossessed upkeep (controllerOf (enchanted creature)))
      (instruct (that player) (may (pay [1, black] once) (untap (that creature))))
  ]
}
