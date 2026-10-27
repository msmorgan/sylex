card ferozsBan : Spelled :=
  { name := "Feroz's Ban", cost := [generic 6], types := [.artifact],
    text := [static ((costShift (allOf (and [isCreature, isSpell])) (more 2)))] }
