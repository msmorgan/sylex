card defang : Spelled :=
  { name := "Defang", cost := [generic 1, pip .white], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (replace (would (by_ (enchanted creature)) (dealDamage anyAmount))
          (instead (preventDamage allDamage))) ] }
