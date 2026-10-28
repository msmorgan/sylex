/-- Garruk Relentless // Garruk, the Veil-Cursed -/
card := transforming
  {
    front := {
      name := "Garruk Relentless",
      cost := [3, green],
      supertypes := [legendary],
      types := [planeswalker],
      subtypes := [garruk],
      text := [
        (when
           (stateHolds (is thisPlaneswalker (compare atMost 2 (axes := [counter loyaltyCounter]))))
           (transform thisPlaneswalker)),
        activated (loyaltySymbol (up 0))
          [
            instruct thisPlaneswalker (dealDamage 3 (target isCreature)),
            dealDamageOwnPower (that creature) thisPlaneswalker
          ],
        activated (loyaltySymbol (up 0)) (create 1 (creatureToken 2 2 [green] [wolf]))
      ],
      loyalty := 3
    },
    back := {
      name := "Garruk, the Veil-Cursed",
      supertypes := [legendary],
      types := [planeswalker],
      subtypes := [garruk],
      text := [
        activated (loyaltySymbol (up 1))
          (create 1
             (written
                {
                  colors := [black],
                  types := [creature],
                  subtypes := [wolf],
                  text := [deathtouch],
                  power := 1,
                  toughness := 1
                })),
        activated (loyaltySymbol (down 1))
          (doIfDone (sacrifice (a isCreature))
             [search yourLibrary (a isCreature), reveal it, putInto it hand, shuffle]),
        activated (loyaltySymbol (down 3))
          [
            establishFor (allOf [isCreature, youControl])
              [abilityGrant them trample, getsPt them (up (letter x)) (up (letter x))]
              untilEndOfTurn,
            define x (countOf [isCreature, in_ yourGraveyard])
          ]
      ]
    }
  }
