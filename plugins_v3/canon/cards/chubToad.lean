/-- Chub Toad -/
card chubToad : Spelled :=
  { name := "Chub Toad", cost := [generic 2, pip .green], types := [.creature],
    subtypes := [frog],
    text :=
      [ whenever (does thisCreature block) (or_ := [does any (becomeBlocked thisCreature)])
          (get it (up 2) (up 2) untilEndOfTurn) ],
    power := stat 1, toughness := stat 1 }
