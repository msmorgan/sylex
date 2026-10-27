card diminish : Spelled :=
  { name := "Diminish", cost := [pip .blue], types := [.instant],
    text :=
      [ spell
          (establishThroughout (getsBase (target isCreature) 1 1)
             untilEndOfTurn) ] }
