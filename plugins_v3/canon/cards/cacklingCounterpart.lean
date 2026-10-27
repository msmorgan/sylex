card cacklingCounterpart : Spelled :=
  { name := "Cackling Counterpart", cost := [generic 1, pip .blue, pip .blue],
    types := [.instant],
    text :=
      [ spell (create 1
          (copyOf (target (and [isCreature, youControl]))) []),
        flashback [generic 5, pip .blue, pip .blue] ] }
