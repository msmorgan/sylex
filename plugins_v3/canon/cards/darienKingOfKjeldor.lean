card darienKingOfKjeldor : Spelled :=
  { name := "Darien, King of Kjeldor", cost := [generic 4, pip .white, pip .white],
    supertypes := [.legendary], types := [.creature],
    subtypes := [human, soldier],
    text :=
      [ whenever (does any (dealDamage (to := you)))
          (may (create thatMuch (creatureToken 1 1 [.white] [soldier]))) ],
    power := stat 3, toughness := stat 3 }
