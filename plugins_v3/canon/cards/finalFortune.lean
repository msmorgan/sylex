card finalFortune : Spelled :=
  { name := "Final Fortune", cost := [pip .red, pip .red], types := [.instant],
    text :=
      [ spell (sequentially
          [ extraTurn 1,
            delay (beginningOfThatTurns .endStep) loseGame ])
                ] }
