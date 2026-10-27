card bloodshedFever : Spelled :=
  { name := "Bloodshed Fever", cost := [pip .red], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (must (enchanted creature) attack) ] }
