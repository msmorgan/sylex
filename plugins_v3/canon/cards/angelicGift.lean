card angelicGift : Spelled :=
  { name := "Angelic Gift", cost := [generic 1, pip .white], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        when (does thisAura enter) (draw 1),
        static (abilityGrant (enchanted creature) flying) ] }
