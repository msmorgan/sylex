card centaurOfAttention : Spelled :=
  { name := "Centaur of Attention", cost := [generic 3, pip .green, pip .green],
    types := [.creature], subtypes := [centaur, performer],
    text :=
      [ when (does thisCreature enter)
          (sequentially [rollDice 5 (.sides 6), storeResults thisCreature]),
        at_ (beginningOfYour .combat)
          (may (rerollStored anyNumber thisCreature)),
        static (conjunction [ getsPt thisCreature (up (letter .x)) (up (letter .x)),
            letterDefinition .x (greatestStoredMatch thisCreature) ]) ],
    power := stat 3, toughness := stat 3 }
