/-- Soar -/
card soar : Spelled :=
  { name := "Soar", cost := [generic 1, pip .blue], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ asThoughFlashSacrificedAtCleanup,
        enchant isCreature,
        static (conjunction
          [ getsPt (enchanted creature) (up 0) (up 1),
            abilityGrant (enchanted creature) flying ]
          (subject := enchanted creature)) ] }
