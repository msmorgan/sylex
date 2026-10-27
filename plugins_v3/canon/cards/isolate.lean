card isolate : Spelled :=
  { name := "Isolate", cost := [pip .white], types := [.instant],
    text :=
      [ spell (exile (target (and [isPermanent, hasManaValue .eq 1]))) ] }
