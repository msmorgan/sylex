card etchingsOfTheChosen : Spelled :=
  { name := "Etchings of the Chosen", cost := [generic 1, pip .white, pip .black],
    types := [.enchantment],
    text :=
      [ static (entersChoosing thisEnchantment (.subtype .creature)),
        static (getsPt (allOf (and [isCreature, youControl, ofChosen (.subtype .creature)]))
          (up 1) (up 1)),
        activated
          (compound
            [ [generic 1],
              (sacrifice (a (and [isCreature, ofChosen (.subtype .creature)]))) ])
          (gain (target (and [isCreature, youControl])) indestructible untilEndOfTurn) ] }
