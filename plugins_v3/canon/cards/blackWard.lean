/-- Black Ward -/
card blackWard : Spelled :=
  { name := "Black Ward", cost := [pip .white], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (retention
          (abilityGrant (enchanted creature) (protection
              (hasColor .black)))
          thisAura) ] }
