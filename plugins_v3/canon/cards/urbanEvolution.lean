card urbanEvolution : Spelled :=
  { name := "Urban Evolution", cost := [generic 3, pip .green, pip .blue],
    types := [.sorcery],
    text :=
      [ spell (sequentially
          [ draw 3,
            establishThroughout (can you
                                   (play
                                      (counted (additional (exactly 1))
                                         isLand))) thisTurn ]) ] }
