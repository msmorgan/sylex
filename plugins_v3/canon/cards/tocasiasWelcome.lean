card tocasiasWelcome : Spelled :=
  { name := "Tocasia's Welcome", cost := [generic 2, pip .white], types := [.enchantment],
    text :=
      [ whenever
          (does (counted (orMore 1) (and [isCreature, youControl,
                                               hasManaValue .atMost 3])) enter)
          (draw 1) (limit := oncePerTurn) ] }
