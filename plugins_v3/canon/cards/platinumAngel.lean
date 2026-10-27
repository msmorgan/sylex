card platinumAngel : Spelled :=
  { name := "Platinum Angel", cost := [generic 7], types := [.artifact, .creature],
    subtypes := [angel],
    text :=
      [ flying,
        static (cant you loseGame),
        static (cant yourOpponents winGame) ],
    power := stat 4, toughness := stat 4 }
