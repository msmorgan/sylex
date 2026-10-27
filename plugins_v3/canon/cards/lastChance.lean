card lastChance : Spelled :=
  { name := "Last Chance", cost := [pip .red, pip .red], types := [.sorcery],
    text :=
      [ spell (sequentially
          [ extraTurn 1,
            delay (beginningOfThatTurns .endStep) loseGame ])
                ] }
