card femerefEnchantress : Spelled :=
  { name := "Femeref Enchantress", cost := [pip .green, pip .white], types := [.creature],
    subtypes := [human, druid],
    text :=
      [ whenever (does any (putInto (a isEnchantment) graveyard (from_ := battlefield)))
          (draw 1) ],
    power := stat 1, toughness := stat 2 }
