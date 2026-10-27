card opt : Spelled :=
  { name := "Opt", cost := [pip .blue], types := [.instant],
    text := [spell (sequentially [scry 1, draw 1])] }
