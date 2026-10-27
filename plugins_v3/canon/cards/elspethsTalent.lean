card elspethsTalent : Spelled :=
  { name := "Elspeth's Talent", cost := [generic 2, pip .white, pip .white],
    types := [.enchantment], subtypes := [aura],
    text :=
      [ enchant isPlaneswalker,
        static (abilityGrant (enchanted planeswalker)
              (activated (loyaltySymbol (up 1))
                (create 3 (creatureToken 1 1 [.white] [soldier])))),
        whenever (does you (activate (a (and [abilityHead .loyalty, abilityOf (enchanted
          planeswalker)]))))
          (establishFor (allOf (and [isCreature, youControl]))
            [getsPt them (up 2) (up 2), abilityGrant them vigilance]
            untilEndOfTurn) ] }
