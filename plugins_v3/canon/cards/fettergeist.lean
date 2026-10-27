card fettergeist : Spelled :=
  { name := "Fettergeist", cost := [generic 2, pip .blue], types := [.creature],
    subtypes := [spirit],
    text :=
      [ flying,
        at_ (beginningOfYour .upkeep)
          (doUnless (sacrifice thisCreature)
            (scaledGenericMana
              (forEach 1
                (and [isCreature, youControl, isOther])))) ],
    power := stat 3, toughness := stat 4 }
