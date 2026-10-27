/-- Empty City Ruse -/
card emptyCityRuse : Spelled :=
  { name := "Empty City Ruse", cost := [pip .white], types := [.sorcery],
    text :=
      [ spell (establishThroughout (partSkip (target isOpponent) .combat) (duringNextTurnOf (that
              .player))) ] }
