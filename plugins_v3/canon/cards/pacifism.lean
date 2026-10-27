/-- Pacifism -/
card pacifism : Spelled :=
  { name := "Pacifism", cost := [generic 1, pip .white], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (conjunction
                  [cant (enchanted creature) attack, cant
                     (enchanted creature) block]) ] }
