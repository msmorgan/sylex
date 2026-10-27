/-- Indestructibility -/
card indestructibility : Spelled :=
  { name := "Indestructibility", cost := [generic 3, pip .white], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isPermanent,
        static (abilityGrant (enchanted permanent) indestructible) ] }
