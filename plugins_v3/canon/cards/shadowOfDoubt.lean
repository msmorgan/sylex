card shadowOfDoubt : Spelled :=
  { name := "Shadow of Doubt", cost := [hybridPip .blue .black, hybridPip .blue .black],
    types := [.instant],
    text :=
      [ spell (sequentially
          [ establishThroughout (cant (bare isPlayer) (search libraries))
              thisTurn,
            draw 1 ]) ] }
