/-- Curator of Destinies -/
card curatorOfDestinies : Spelled :=
  { name := "Curator of Destinies", cost := [generic 4, pip .blue, pip .blue],
    types := [.creature], subtypes := [sphinx],
    text :=
      [ static (cant any (counter thisSpell)),
        flying,
        when (does thisCreature enter)
          (sequentially
            [ lookAt (topSlice 5),
              separateIntoPiles them 2 [.faceDown, .faceUp],
              instruct anOpponent (choose onePile),
              putInto (that .pile) hand,
              putInto theOther graveyard ]) ],
    power := stat 5, toughness := stat 5 }
