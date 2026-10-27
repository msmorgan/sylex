/-- Harvest Hand // Scrounged Scythe -/
card harvestHand : Spelled := { body := .transforming
  { characteristics :=
    { name := "Harvest Hand", cost := [generic 3], types := [.artifact, .creature],
      subtypes := [scarecrow],
      text := [ when (does thisCreature die) (returnToBattlefieldTransformed it you) ],
      power := stat 2, toughness := stat 2 } }
  { characteristics :=
    { name := "Scrounged Scythe", types := [.artifact], subtypes := [equipment],
      text :=
        [ static (getsPt (equipped creature) (up 1) (up 1)),
          static
            (onlyWhile (abilityGrant (equipped creature) menace)
              (is (equipped creature)
                human)),
          equip [generic 2] ] } } }
