/-- Riddles in the Dark -/
card riddlesInTheDark : Spelled :=
  { name := "Riddles in the Dark", cost := [generic 2, pip .blue], types := [.instant],
    text :=
      [ spell (sequentially
          [ lookAt (topSlice 4),
            separateIntoPiles them 2 [.faceDown, .faceUp],
            instruct anOpponent (choose onePile),
            putInto (that .pile) hand,
            putInto theOther graveyard ]) ] }
