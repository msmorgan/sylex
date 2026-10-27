/-- Grave Servitude -/
card graveServitude : Spelled :=
  { name := "Grave Servitude", cost := [generic 1, pip .black], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ asThoughFlashSacrificedAtCleanup,
        enchant isCreature,
        static (conjunction
          [ getsPt (enchanted creature) (up 3) (down 1),
            qualityChange it .sets (colored (.some [.black])) ]
          (subject := enchanted creature)) ] }
