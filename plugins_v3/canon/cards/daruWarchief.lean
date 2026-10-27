card daruWarchief : Spelled :=
  { name := "Daru Warchief", cost := [generic 2, pip .white, pip .white], types := [.creature],
    subtypes := [human, soldier],
    text :=
      [ static ((costShift (allOf (and [soldier, isSpell, did you (cast which) thisObject])) (less
        1))),
        static (getsPt (allOf (and [soldier, isCreature, youControl]))
          (up 1) (up 2)) ],
    power := stat 1, toughness := stat 1 }
