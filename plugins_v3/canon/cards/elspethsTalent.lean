card elspethsTalent : Spelled :=
  { name := "Elspeth's Talent", cost := [generic 2, pip .white, pip .white],
    types := [.enchantment], subtypes := [aura],
    text :=
      [ enchant isPlaneswalker,
        static elspethsTalentGrant,
        whenever (does you (activate loyaltyAbilityOfEnchanted))
          (establishFor (allOf (and [isCreature, youControl]))
            [getsPt them (up 2) (up 2), abilityGrant them vigilance]
            untilEndOfTurn) ] }
