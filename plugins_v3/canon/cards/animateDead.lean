/-- Animate Dead -/
card animateDead : Spelled :=
  { name := "Animate Dead", cost := [generic 1, pip .black], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant (and [isCreature, isCard, in_ graveyard]),
        when (does thisAura enter)
          (sequentially
            [ (establish (conjunction
                  [ abilityLoss it [written (enchant (and [isCreature, isCard, in_ graveyard]))],
                    abilityGrant it (enchant ((and [isCreature, did (by_ thisAura)
                      (putOntoBattlefield which)]))) ]
                  (subject := it))),
              returnToBattlefield (enchanted (.ofType .card .creature)) [under you],
              attachTo thisAura it,
              (delay (does thisAura (leave battlefield)) (instruct (controllerOf (that creature))
                (sacrifice it))) ])
          (if_ := is it (in_ battlefield)),
        static (getsPt (enchanted creature) (down 1) (up 0)) ] }
