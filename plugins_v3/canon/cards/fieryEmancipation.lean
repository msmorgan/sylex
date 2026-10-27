card fieryEmancipation : Spelled :=
  { name := "Fiery Emancipation", cost := [generic 3, pip .red, pip .red, pip .red],
    types := [.enchantment],
    text :=
      [ static
          (replace
            (would (a (and [isSource, youControl]))
              (dealDamage anyAmount (a (or [isPermanent, isPlayer]))))
            (instead (instruct it (dealDamage (times 3 thatMuch) thatJoin)))) ] }
