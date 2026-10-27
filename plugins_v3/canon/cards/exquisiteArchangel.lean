card exquisiteArchangel : Spelled :=
  { name := "Exquisite Archangel", cost := [generic 5, pip .white, pip .white],
    types := [.creature], subtypes := [angel],
    text :=
      [ flying,
        static (replace (would you loseGame) (instead (sequentially
            [ exile thisCreature,
              setLife (statOf (.playerStat .startingLifeTotal) you) ]))) ],
    power := stat 5, toughness := stat 5 }
