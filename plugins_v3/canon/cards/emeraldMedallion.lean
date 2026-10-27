card emeraldMedallion : Spelled :=
  { name := "Emerald Medallion", cost := [generic 2], types := [.artifact],
    text := [static ((costShift (allOf (and [hasColor .green, isSpell, did you (cast which)
      thisObject])) (less 1)))] }
