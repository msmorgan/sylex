card orimsPrayer : Spelled :=
  { name := "Orim's Prayer", cost := [generic 1, pip .white, pip .white],
    types := [.enchantment],
    text :=
      [ whenever (does (counted (orMore 1) isCreature) (attack you))
          (gainLife (forEach 1 (and [isAttacking, isCreature]))) ] }
