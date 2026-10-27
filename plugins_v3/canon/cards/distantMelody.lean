card distantMelody : Spelled :=
  { name := "Distant Melody", cost := [generic 3, pip .blue], types := [.sorcery],
    text :=
      [ spell (sequentially
          [ choose (a (quality (.subtype .creature))),
            draw
              (forEach 1
                (and [isPermanent, youControl, ofChosen (.subtype .creature)])) ]) ] }
