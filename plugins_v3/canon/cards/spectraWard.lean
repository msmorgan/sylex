/-- Spectra Ward -/
card spectraWard : Spelled :=
  { name := "Spectra Ward", cost := [generic 3, pip .white, pip .white], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (conjunction
          [ getsPt (enchanted creature) (up 2) (up 2),
            retention (abilityGrant (enchanted creature) (protection (eachKind .color)))
              (allOf aura) ]
          (subject := enchanted creature)) ] }
