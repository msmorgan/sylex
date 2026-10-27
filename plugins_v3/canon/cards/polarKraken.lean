card polarKraken : Spelled :=
  { name := "Polar Kraken", cost := [generic 8, pip .blue, pip .blue, pip .blue],
    types := [.creature], subtypes := [kraken],
    text :=
      [ trample,
        static (entersTapped thisCreature),
        cumulativeUpkeep (sacrifice (a isLand)) ],
    power := stat 11, toughness := stat 11 }
