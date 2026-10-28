/-- Profit // Loss -/
card := split
  {
    left := {
      name := "Profit",
      cost := [1, white],
      types := [instant],
      text := [spell (get (allOf [isCreature, youControl]) (up 1) (up 1) untilEndOfTurn), fuse]
    },
    right := {
      name := "Loss",
      cost := [2, black],
      types := [instant],
      text := [spell (get (allOf [isCreature, opponentsControl]) (down 1) (down 1) untilEndOfTurn), fuse]
    }
  }
