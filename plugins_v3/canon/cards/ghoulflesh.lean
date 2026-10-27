/-- Ghoulflesh -/
card ghoulflesh : Spelled :=
  { name := "Ghoulflesh", cost := [pip .black], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (conjunction
                  [ getsPt (enchanted creature) (down 1) (down 1),
            qualityChange it .adds
              (bundle
                 { characteristics := { colors := [.black], subtypes := [zombie] } }) ]) ] }
