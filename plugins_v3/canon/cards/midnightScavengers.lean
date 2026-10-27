/-- Midnight Scavengers // Chittering Host -/
card midnightScavengers : Spelled := { body := .transforming
  { characteristics :=
    { name := "Midnight Scavengers", cost := [generic 4, pip .black], types := [.creature],
      subtypes := [human, rogue],
      text :=
        [ when (does thisCreature enter)
            (may
              (returnToHand (target (and [ isCreature, in_ yourGraveyard,
                                hasManaValue .atMost 3 ])) yourGraveyard)) ],
      power := stat 3, toughness := stat 3 } }
  { characteristics :=
    { name := "Chittering Host", types := [.creature], subtypes := [eldrazi, horror],
      text :=
        [ haste, menace,
          when (does thisCreature enter)
            (establishFor (allOf (and [isCreature, youControl, isOther]))
              [getsPt them (up 1) (up 0), abilityGrant them menace]
              untilEndOfTurn) ],
      power := stat 5, toughness := stat 6 } } }
