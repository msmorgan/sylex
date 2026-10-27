card windZendikon : Spelled :=
  { name := "Wind Zendikon", cost := [pip .blue], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isLand,
        static (qualityChange (enchanted land) .sets
          (bundle
            { characteristics :=
              { colors := [.blue], types := [.creature], subtypes := [elemental],
                text := [flying], power := stat 2, toughness := stat 2 } }
            (some .land))),
        when (does (enchanted land) die) (returnToHand (that card)) ] }
