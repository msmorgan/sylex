card grandArbiter : Spelled :=
  { name := "Grand Arbiter Augustin IV", cost := [generic 2, pip .white, pip .blue],
    supertypes := [.legendary], types := [.creature],
    subtypes := [human, advisor],
    text :=
      [ static ((costShift (allOf (and [hasColor .white, isSpell, did you (cast which) thisObject]))
        (less 1))),
        static ((costShift (allOf (and [hasColor .blue, isSpell, did you (cast which) thisObject]))
          (less 1))),
        static ((costShift (allOf (and [isSpell, did yourOpponents (cast which) thisObject])) (more
          1))) ],
    power := stat 2, toughness := stat 3 }
