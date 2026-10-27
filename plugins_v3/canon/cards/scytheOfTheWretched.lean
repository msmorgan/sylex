/-- Scythe of the Wretched. -/
card scytheOfTheWretched : Spelled :=
  { name := "Scythe of the Wretched", cost := [generic 2], types := [.artifact],
    subtypes := [equipment],
    text :=
      [ static (getsPt
          (equipped creature)
          (up 2) (up 2)),
        whenever (does ((a (and [isCreature,
              predicateDid { subj := actor (attachHost .equipped creature),
                             action := dealDamage anyAmount which,
                             within := some (within soFarThisTurn none) }]))) die)
          (sequentially
            [ returnToBattlefield (that card)
                [(under you)],
              attachTo thisEquipment (that creature) ]),
        equip [generic 4] ] }
