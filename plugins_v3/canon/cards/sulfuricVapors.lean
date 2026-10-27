card sulfuricVapors : Spelled :=
  { name := "Sulfuric Vapors", cost := [generic 3, pip .red], types := [.enchantment],
    text :=
      [ static
          (replace
            (would (a (and [isSpell, hasColor .red]))
              (dealDamage anyAmount (a (or [isPermanent, isPlayer]))))
            (instead (instruct it (dealDamage (plus thatMuch 1) thatJoin)))) ] }
