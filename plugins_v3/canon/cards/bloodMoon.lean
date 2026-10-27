card bloodMoon : Spelled :=
  { name := "Blood Moon", cost := [generic 2, pip .red], types := [.enchantment],
    text := [static ((qualityChange (allOf (and [isLand, non (hasSupertype .basic)])) .sets
          (bundle { characteristics := { subtypes := [mountain] } })))] }
