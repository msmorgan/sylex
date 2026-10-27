card aboutFace : Spelled :=
  { name := "About Face", cost := [pip .red], types := [.instant],
    text :=
      [ spell (establishThroughout (ptSwitch (target isCreature)) untilEndOfTurn) ] }
