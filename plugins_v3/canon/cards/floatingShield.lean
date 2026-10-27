/-- Floating Shield -/
card floatingShield : Spelled :=
  { name := "Floating Shield", cost := [generic 2, pip .white], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (entersChoosing thisAura .color),
        static (retention
          (abilityGrant (enchanted creature) (protection
              (ofChosen .color)))
          thisAura),
        activated (sacrifice thisAura)
          (gain (target isCreature) (protection (ofChosen .color))
              untilEndOfTurn) ] }
