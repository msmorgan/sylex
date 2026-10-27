card gratuitousViolence : Spelled :=
  { name := "Gratuitous Violence", cost := [generic 2, pip .red, pip .red, pip .red],
    types := [.enchantment],
    text :=
      [ static
          (replace
            (would (a (and [isCreature, youControl]))
              (dealDamage anyAmount (a (or [isPermanent, isPlayer]))))
            (instead (instruct it (dealDamage (times 2 thatMuch))))) ] }
