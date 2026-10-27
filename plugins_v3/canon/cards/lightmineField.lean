card lightmineField : Spelled :=
  { name := "Lightmine Field", cost := [generic 2, pip .white, pip .white],
    types := [.enchantment],
    text :=
      [ whenever (does (counted (orMore 1) isCreature) attack)
          (instruct thisEnchantment
             (dealDamage (countOf (and [isCreature, isAttacking])) (eachOf (those creature)))) ] }
