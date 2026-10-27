card thornOfAmethyst : Spelled :=
  { name := "Thorn of Amethyst", cost := [generic 2], types := [.artifact],
    text := [static ((costShift (allOf (and [non isCreature, isSpell])) (more 1)))] }
