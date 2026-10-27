card magusOfTheMoon : Spelled :=
  { name := "Magus of the Moon", cost := [generic 2, pip .red], types := [.creature],
    subtypes := [human, wizard],
    text := [static ((qualityChange (allOf (and [isLand, non (hasSupertype .basic)])) .sets
          (bundle { characteristics := { subtypes := [mountain] } })))], power := stat 2, toughness
            := stat 2 }
