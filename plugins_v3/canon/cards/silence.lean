card silence : Spelled :=
  { name := "Silence", cost := [pip .white], types := [.instant],
    text :=
      [ spell (establishThroughout (cant yourOpponents (cast any)) thisTurn) ] }
