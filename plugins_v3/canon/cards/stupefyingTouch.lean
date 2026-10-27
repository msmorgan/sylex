card stupefyingTouch : Spelled :=
  { name := "Stupefying Touch", cost := [generic 1, pip .blue], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        when (does thisAura enter) (draw 1),
        static (cant any (activate (allOf (and [ abilityHead .anyActivated,
                         abilityOf (enchanted creature) ])))) ] }
