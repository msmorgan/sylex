card explore : Spelled :=
  { name := "Explore", cost := [generic 1, pip .green], types := [.sorcery],
    text :=
      [ spell (sequentially
          [ establishThroughout (can you (play (counted (additional (exactly 1)) isLand)))
              thisTurn,
            draw 1 ]) ] }
