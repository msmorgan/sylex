/-- Borrowed Malevolence -/
card borrowedMalevolence : Spelled :=
  { name := "Borrowed Malevolence", cost := [pip .black], types := [.instant],
    text :=
      [ escalate [generic 2],
        spell (chooseModes (oneThrough 2)
          [ get (target isCreature) (up 1) (up 1) untilEndOfTurn,
            get (target isCreature) (down 1) (down 1) untilEndOfTurn ]) ] }
