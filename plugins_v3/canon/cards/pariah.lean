card pariah : Spelled :=
  { name := "Pariah", cost := [generic 2, pip .white], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (replace (would any (dealDamage anyAmount you))
          (instead (redirectDamage allDamage (enchanted creature)))) ] }
