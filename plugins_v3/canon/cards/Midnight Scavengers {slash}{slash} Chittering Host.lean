/-- Midnight Scavengers // Chittering Host -/
card := transforming
  {
    front := {
      name := "Midnight Scavengers",
      cost := [4, black],
      types := [creature],
      subtypes := [human, rogue],
      text := [
        when (does thisCreature enter)
          (may
             (returnToHand (target [isCreature, in_ yourGraveyard, hasManaValue atMost 3])
                yourGraveyard))
      ],
      power := 3,
      toughness := 3
    },
    back := {
      name := "Chittering Host",
      types := [creature],
      subtypes := [eldrazi, horror],
      text := [
        haste,
        menace,
        when (does thisCreature enter)
          (establishFor (allOf [isCreature, youControl, isOther])
             [getsPt them (up 1) (up 0), abilityGrant them menace] untilEndOfTurn)
      ],
      power := 5,
      toughness := 6
    }
  }
