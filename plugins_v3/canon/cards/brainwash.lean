card brainwash : Spelled :=
  { name := "Brainwash", cost := [pip .white], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (gatedBy [generic 3] (enchanted creature) attack) ] }
