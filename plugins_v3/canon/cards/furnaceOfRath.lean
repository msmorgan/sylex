card furnaceOfRath : Spelled :=
  { name := "Furnace of Rath", cost := [generic 1, pip .red, pip .red, pip .red],
    types := [.enchantment],
    text :=
      [ static (replace (would (a isSource) (dealDamage anyAmount (a (or [isPermanent, isPlayer]))))
          (instead (instruct it (dealDamage (times 2 thatMuch) thatJoin)))) ] }
