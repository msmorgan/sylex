/-- Sinister Strength -/
card sinisterStrength : Spelled :=
  { name := "Sinister Strength", cost := [generic 1, pip .black], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (conjunction [ getsPt (enchanted creature) (up 3) (up 1),
            qualityChange it .sets (colored (.some [.black])) ]) ] }
