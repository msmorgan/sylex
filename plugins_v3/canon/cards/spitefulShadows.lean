card spitefulShadows : Spelled :=
  { name := "Spiteful Shadows", cost := [generic 1, pip .black], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        whenever (does any (dealDamage (to := enchanted creature)))
          (instruct it (dealDamage thatMuch (controllerOf it))) ] }
