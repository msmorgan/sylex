/-- Feral Contest -/
card feralContest : Spelled :=
  { name := "Feral Contest", cost := [generic 3, pip .green], types := [.sorcery],
    text :=
      [ spell (sequentially
          [ putCounters 1 p1p1Counter (target (and [isCreature, youControl])),
            establishThroughout (must (target (and [isCreature, isOther]))
                                   (block it)) thisTurn ]) ] }
