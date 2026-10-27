card megatherium : Spelled :=
  { name := "Megatherium", cost := [generic 2, pip .green], types := [.creature],
    subtypes := [beast],
    text :=
      [ trample,
        when (does thisCreature enter)
          (doUnless (sacrifice thisCreature)
            (scaledGenericMana (forEach 1 (cardIn yourHand)))) ],
    power := stat 4, toughness := stat 4 }
