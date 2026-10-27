card moggManiac : Spelled :=
  { name := "Mogg Maniac", cost := [generic 1, pip .red], types := [.creature],
    subtypes := [goblin],
    text :=
      [ whenever (does any (dealDamage (to := thisCreature)))
          (instruct it
             (dealDamage thatMuch (target (or [isPlaneswalker, isOpponent])))) ],
    power := stat 1, toughness := stat 1 }
