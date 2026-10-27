card bindingAgony : Spelled :=
  { name := "Binding Agony", cost := [generic 1, pip .black], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        whenever (does any (dealDamage (to := enchanted creature)))
          (instruct thisAura
             (dealDamage thatMuch (controllerOf (that creature)))) ] }
