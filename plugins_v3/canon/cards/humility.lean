card humility : Spelled :=
  { name := "Humility", cost := [generic 2, pip .white, pip .white], types := [.enchantment],
    text :=
      [ static (conjunction [ allAbilityLoss (allOf isCreature),
            getsBase them 1 1 ]) ] }
