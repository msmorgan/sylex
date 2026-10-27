card eidolonOfObstruction : Spelled :=
  { name := "Eidolon of Obstruction", cost := [generic 1, pip .white],
    types := [.enchantment, .creature], subtypes := [spirit],
    text :=
      [ firstStrike,
        static (costShift
          (allOf (and [ abilityHead .loyalty,
                         abilityOf (allOf (and [isPlaneswalker, opponentsControl])) ]))
          (more 1)) ],
    power := stat 2, toughness := stat 1 }
