card := transforming
  {
    front := {
      name := "Arlinn Kord",
      cost := [2, red, green],
      supertypes := [legendary],
      types := [planeswalker],
      subtypes := [arlinn],
      text := [
        activated (loyaltySymbol (up 1))
          (establishFor (targets (upTo 1) isCreature)
             [getsPt it (up 2) (up 2), abilityGrant it vigilance, abilityGrant it haste]
             untilEndOfTurn),
        activated (loyaltySymbol (up 0))
          [create 1 (creatureToken 2 2 [green] [wolf]), transform thisPlaneswalker]
      ],
      loyalty := 3
    },
    back := {
      name := "Arlinn, Embraced by the Moon",
      supertypes := [legendary],
      types := [planeswalker],
      subtypes := [arlinn],
      text := [
        activated (loyaltySymbol (up 1))
          (establishFor (allOf [isCreature, youControl])
             [getsPt them (up 1) (up 1), abilityGrant them trample] untilEndOfTurn),
        activated (loyaltySymbol (down 1))
          [instruct this_ (dealDamage 3 (target isAnyTarget)), transform thisPlaneswalker],
        activated (loyaltySymbol (down 6))
          (getEmblem
             [
               static
                 [
                   abilityGrant (allOf [isCreature, youControl]) haste,
                   abilityGrant them
                     (activated tapSymbol (dealDamageOwnPower thisCreature (target isAnyTarget)))
                 ]
             ])
      ]
    }
  }
