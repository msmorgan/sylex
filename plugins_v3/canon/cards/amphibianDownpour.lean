card amphibianDownpour : Spelled :=
  { name := "Amphibian Downpour", cost := [generic 2, pip .blue], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ flash, storm,
        enchant isCreature,
        static (enchantedBecomesVanilla [.blue] frog 1 1) ] }
