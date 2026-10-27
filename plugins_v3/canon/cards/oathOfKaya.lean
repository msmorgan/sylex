/-- Oath of Kaya -/
card oathOfKaya : Spelled :=
  { name := "Oath of Kaya", cost := [generic 1, pip .white, pip .black],
    supertypes := [.legendary], types := [.enchantment],
    text :=
      [ when (does this_ enter)
          (sequentially [instruct this_ (dealDamage 3 (target isAnyTarget)), gainLife 3]),
        whenever
          (does anOpponent
             (attackWith (counted (orMore 1) isCreature)
                (a (and [isPlaneswalker, youControl]))))
          (sequentially
             [instruct this_ (dealDamage 2 (that player)), gainLife
                2]) ] }
