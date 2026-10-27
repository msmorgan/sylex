/-- Profit // Loss -/
card profitLoss : Spelled := { body := .split
  { characteristics :=
    { name := "Profit", cost := [generic 1, pip .white], types := [.instant],
      text :=
        [ spell (get (allOf (and [isCreature, youControl])) (up 1) (up 1) untilEndOfTurn),
          fuse ] } }
  { characteristics :=
    { name := "Loss", cost := [generic 2, pip .black], types := [.instant],
      text :=
        [ spell (get (allOf (and [isCreature, opponentsControl])) (down 1) (down 1)
              untilEndOfTurn),
          fuse ] } } }
