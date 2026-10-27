/-- Flickering Ward -/
card flickeringWard : Spelled :=
  { name := "Flickering Ward", cost := [pip .white], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (entersChoosing thisAura .color),
        static (retention
          (abilityGrant (enchanted creature) (protection (ofChosen .color)))
          thisAura),
        activated [pip .white] (returnToHand thisAura) ] }
