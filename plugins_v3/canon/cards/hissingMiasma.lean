card hissingMiasma : Spelled :=
  { name := "Hissing Miasma", cost := [generic 1, pip .black, pip .black],
    types := [.enchantment],
    text :=
      [ whenever (does (a isCreature) (attack you))
          (instruct (controllerOf it) (loseLife 1)) ] }
