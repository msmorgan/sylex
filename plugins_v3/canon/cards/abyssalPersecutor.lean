card abyssalPersecutor : Spelled :=
  { name := "Abyssal Persecutor", cost := [generic 2, pip .black, pip .black],
    types := [.creature], subtypes := [demon],
    text :=
      [ flying, trample,
        static (cant you winGame),
        static (cant yourOpponents loseGame) ],
    power := stat 6, toughness := stat 6 }
