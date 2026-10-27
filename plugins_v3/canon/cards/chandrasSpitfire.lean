card chandrasSpitfire : Spelled :=
  { name := "Chandra's Spitfire", cost := [generic 2, pip .red], types := [.creature],
    subtypes := [elemental],
    text :=
      [ flying,
        whenever (does any (dealDamage (to := anOpponent) (kind := some .noncombat)))
          (get thisCreature (up 3) (up 0) untilEndOfTurn) ],
    power := stat 1, toughness := stat 3 }
