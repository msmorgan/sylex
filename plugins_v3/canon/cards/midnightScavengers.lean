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
  chitteringHostOnScavengers }
