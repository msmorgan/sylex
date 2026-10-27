card spitemare : Spelled :=
  { name := "Spitemare", cost := [generic 2, hybridPip .red .white, hybridPip .red .white],
    types := [.creature], subtypes := [elemental],
    text :=
      [ whenever (does any (dealDamage (to := thisCreature))) (instruct it (dealDamage thatMuch (target isAnyTarget))) ],
    power := stat 3, toughness := stat 3 }
