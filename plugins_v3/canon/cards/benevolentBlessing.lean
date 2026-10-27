/-- Benevolent Blessing -/
card benevolentBlessing : Spelled :=
  { name := "Benevolent Blessing", cost := [generic 1, pip .white], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ flash,
        enchant isCreature,
        static (entersChoosing thisAura .color),
        static (retention
          (abilityGrant (enchanted creature) (protection
              (ofChosen .color)))
          (allOf (and [ or [aura, equipment],
                         youControl, attachedTo it ]))) ] }
