/-- Fact or Fiction -/
card factOrFiction : Spelled :=
  { name := "Fact or Fiction", cost := [generic 3, pip .blue], types := [.instant],
    text :=
      [ spell (sequentially
          [ reveal (topSlice 5),
            instruct anOpponent (separateIntoPiles them 2),
            putInto onePile hand,
            putInto theOther graveyard ]) ] }
