card crumblingAshes : Spelled :=
  { name := "Crumbling Ashes", cost := [generic 1, pip .black], types := [.enchantment],
    text :=
      [ at_ (beginningOfYour .upkeep)
          (destroy (target (and [isCreature, hasCounter m1m1Counter]))) ] }
