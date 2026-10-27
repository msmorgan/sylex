card summerBloom : Spelled :=
  { name := "Summer Bloom", cost := [generic 1, pip .green], types := [.sorcery],
    text := [spell
               (establishThroughout
                  (can you (play (counted (additional (upTo 3)) isLand))) thisTurn)] }
