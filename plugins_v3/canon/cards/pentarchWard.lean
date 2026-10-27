/-- Pentarch Ward -/
card pentarchWard : Spelled :=
  { name := "Pentarch Ward", cost := [generic 2, pip .white], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (entersChoosing thisAura .color),
        when (does thisAura enter) (draw 1),
        static (retention
          (abilityGrant (enchanted creature) (protection
              (ofChosen .color)))
          thisAura) ] }
