card repercussion : Spelled :=
  { name := "Repercussion", cost := [generic 1, pip .red, pip .red], types := [.enchantment],
    text :=
      [ whenever (does any (dealDamage (to := a isCreature)))
          (instruct thisEnchantment
             (dealDamage thatMuch (controllerOf (that creature)))) ] }
