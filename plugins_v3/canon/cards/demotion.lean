card demotion : Spelled :=
  { name := "Demotion", cost := [pip .white], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (conjunction [ cant (enchanted creature) block,
            cant any (activate (allOf (and [abilityHead .anyActivated, abilityOf it]))) ]) ] }
