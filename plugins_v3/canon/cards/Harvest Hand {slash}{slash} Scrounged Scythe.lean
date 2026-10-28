/-- Harvest Hand // Scrounged Scythe -/
card := transforming
  {
    front := {
      name := "Harvest Hand",
      cost := [3],
      types := [artifact, creature],
      subtypes := [scarecrow],
      text := [when (does thisCreature die) (returnToBattlefieldTransformed it you)],
      power := 2,
      toughness := 2
    },
    back := {
      name := "Scrounged Scythe",
      types := [artifact],
      subtypes := [equipment],
      text := [
        getsPt (equipped creature) (up 1) (up 1),
        onlyWhile (abilityGrant (equipped creature) menace) (is (equipped creature) human),
        equip [2]
      ]
    }
  }
