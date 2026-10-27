card kenrithsTransformation : Spelled :=
  { name := "Kenrith's Transformation", cost := [generic 1, pip .green],
    types := [.enchantment], subtypes := [aura],
    text :=
      [ enchant isCreature,
        when (does thisAura enter) (draw 1),
        static (enchantedBecomesVanilla [.green] elk 3 3) ] }
