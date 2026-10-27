card primalSurge : Spelled :=
  { name := "Primal Surge", cost := [generic 8, pip .green, pip .green], types := [.sorcery],
    text :=
      [ spell (sequentially
          [ exile (topSlice 1),
            doIf (is it isPermanentCard)
              (may (putOntoBattlefield it) (repeat_ again))
                  ]) ] }
