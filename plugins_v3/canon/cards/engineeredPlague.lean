card engineeredPlague : Spelled :=
  { name := "Engineered Plague", cost := [generic 2, pip .black], types := [.enchantment],
    text :=
      [ static (entersChoosing thisEnchantment (.subtype .creature)),
        static (getsPt (allOf (and [isCreature, ofChosen (.subtype .creature)]))
          (down 1) (down 1)) ] }
