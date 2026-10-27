/-- Oppressive Rays -/
card oppressiveRays : Spelled :=
  { name := "Oppressive Rays", cost := [pip .white], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (conjunction
                  [gatedBy [generic 3] (enchanted creature) attack,
                   gatedBy [generic 3] (enchanted creature) block]),
        static (costShift
          (allOf (and [ abilityHead .anyActivated,
                         abilityOf (enchanted creature) ]))
          (more 3)) ] }
