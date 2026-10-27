/-- Green Ward -/
card greenWard : Spelled :=
  { name := "Green Ward", cost := [pip .white], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (retention
          (abilityGrant (enchanted creature) (protection (hasColor .green)))
          thisAura) ] }
