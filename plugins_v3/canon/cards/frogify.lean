card frogify : Spelled :=
  { name := "Frogify", cost := [generic 1, pip .blue], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (enchantedBecomesVanilla [.blue] frog 1 1) ] }
