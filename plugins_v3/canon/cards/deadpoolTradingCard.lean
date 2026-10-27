/-- Deadpool, Trading Card -/
card deadpoolTradingCard : Spelled :=
  { name := "Deadpool, Trading Card", cost := [generic 2, pip .black, pip .red],
    supertypes := [.legendary], types := [.creature],
    subtypes := [mutant, mercenary, hero],
    text :=
      [ static
          (replace (would thisCreature enter)
            (alongside
              (may (exchangeTextBoxes thisCreature (a (and [isCreature, isOther])))))),
        at_ (beginningOfYour .upkeep) (loseLife 3),
        activated (compound [[generic 3], sacrifice thisCreature])
          (instruct (each isOtherPlayer) (draw 1)) ],
    power := stat 5, toughness := stat 3 }
