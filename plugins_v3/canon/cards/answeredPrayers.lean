card answeredPrayers : Spelled :=
  { name := "Answered Prayers", cost := [generic 1, pip .white, pip .white],
    types := [.enchantment],
    text :=
      [ when (does (a (and [isCreature, youControl])) enter)
          (sequentially
            [ gainLife 1,
              doIf (not (is
                  thisEnchantment isCreature))
                (become thisEnchantment
                  { characteristics :=
                    { types := [.creature], subtypes := [angel],
                      text := [flying], power := stat 3, toughness := stat 3 } }
                  untilEndOfTurn) ]) ] }
