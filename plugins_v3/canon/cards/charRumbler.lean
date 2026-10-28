card charRumbler : Spelled :=
  { name := "Char-Rumbler", cost := [generic 2, pip .red, pip .red], types := [.creature],
    subtypes := [elemental],
    text :=
      [ doubleStrike,
        activated [pip .red]
          (get thisCreature (up 1) (up 0) untilEndOfTurn) ],
    power := some (lit (-1)), toughness := stat 3 }
