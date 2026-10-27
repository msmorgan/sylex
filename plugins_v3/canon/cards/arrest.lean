/-- Arrest -/
card arrest : Spelled :=
  { name := "Arrest", cost := [generic 2, pip .white], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (conjunction
          [ cant (enchanted creature) attack,
            cant (enchanted creature) block,
            cant any (activate (allOf (and [abilityHead .anyActivated, abilityOf it]))) ]) ] }
