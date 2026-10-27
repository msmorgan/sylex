/-- Wall of Shards -/
card wallOfShards : Spelled :=
  { name := "Wall of Shards", cost := [generic 1, pip .white], supertypes := [.snow],
    types := [.creature], subtypes := [wall],
    text :=
      [ defender, flying,
        cumulativeUpkeep (instruct anOpponent (gainLife 1)) ],
    power := stat 1, toughness := stat 8 }
