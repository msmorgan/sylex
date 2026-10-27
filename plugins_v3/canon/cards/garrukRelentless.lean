/-- Garruk Relentless // Garruk, the Veil-Cursed -/
card garrukRelentless : Spelled := { body := .transforming
  { characteristics :=
    { name := "Garruk Relentless", cost := [generic 3, pip .green],
      supertypes := [.legendary], types := [.planeswalker],
      subtypes := [garruk],
      text :=
        [ (when
              (stateHolds
                (is thisPlaneswalker (compare .atMost 2 (axes := [.counter loyaltyCounter]))))
              (transform thisPlaneswalker)),
          activated (loyaltySymbol (up 0)) (sequentially
                [ instruct thisPlaneswalker (dealDamage 3 (target isCreature)),
                  dealDamageOwnPower (that creature) thisPlaneswalker ]),
          activated (loyaltySymbol (up 0))
            (create 1 (creatureToken 2 2 [.green] [wolf])) ],
      loyalty := stat 3 } }
  { characteristics :=
    { name := "Garruk, the Veil-Cursed", supertypes := [.legendary], types := [.planeswalker],
      subtypes := [garruk],
      text :=
        [ activated (loyaltySymbol (up 1))
            (create 1 (written
              { characteristics :=
                { colors := [.black], types := [.creature], subtypes := [wolf],
                  text := [deathtouch], power := stat 1, toughness := stat 1 } })),
          activated (loyaltySymbol (down 1))
            (doIfDone (sacrifice (a isCreature))
              (sequentially
                [ search yourLibrary (a isCreature), reveal it, putInto it hand,
                  shuffle ])),
          activated (loyaltySymbol (down 3))
            (sequentially
              [ establishFor (allOf (and [isCreature, youControl]))
                  [abilityGrant them trample, getsPt them (up (letter .x)) (up (letter .x))]
                  untilEndOfTurn,
                define .x (countOf (and [isCreature, in_ yourGraveyard])) ]) ] } } }
