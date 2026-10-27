card bushiTenderfoot : Spelled := { body := .flip
  { characteristics :=
    { name := "Bushi Tenderfoot", cost := [pip .white], types := [.creature],
      subtypes := [human, soldier],
      text :=
        [ when
            (does (a (and [ isCreature,
                              did thisCreature (dealDamage anyAmount
                                  which) (within soFarThisTurn) ])) die)
            (flip thisCreature) ],
      power := stat 1, toughness := stat 1 } }
  { characteristics :=
    { name := "Kenzo the Hardhearted", supertypes := [.legendary], types := [.creature],
      subtypes := [human, samurai],
      text := [doubleStrike, bushido 2],
      power := stat 3, toughness := stat 4 } } }
