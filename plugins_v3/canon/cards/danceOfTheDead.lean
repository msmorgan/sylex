/-- Dance of the Dead -/
card danceOfTheDead : Spelled :=
  { name := "Dance of the Dead", cost := [generic 1, pip .black], types := [.enchantment],
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
              putOntoBattlefield (enchanted (.ofType .card .creature))
                (riders := [entersAs .tapped, under you]),
              attachTo thisAura it,
              (delay (does thisAura (leave battlefield)) (instruct (controllerOf (that creature))
                (sacrifice it))) ])
          (if_ := is it (in_ battlefield)),
        static (conjunction
          [ getsPt (enchanted creature) (up 1) (up 1),
            doesntUntap it (controllerOf it) ]
          (subject := enchanted creature)),
        at_ (beginningOfPossessed .upkeep (controllerOf (enchanted creature)))
          (instruct (that player) (may (pay [generic 1, pip .black] .once)
            (untap (that creature)))) ] }
