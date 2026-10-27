card colossalGraveReaver : Spelled :=
  { name := "Colossal Grave-Reaver", cost := [generic 6, pip .black, pip .green],
    types := [.creature], subtypes := [dragon],
    text :=
      [ flying,
        whenever (does thisCreature enter) (or_ := [does thisCreature attack]) (mill 3),
        whenever
          (does any
             (putInto (counted (orMore 1) (and [isCard, isCreature])) yourGraveyard
                (from_ := yourLibrary)))
          (putOntoBattlefield (someOf (exactly 1) them)) ],
    power := stat 7, toughness := stat 6 }
