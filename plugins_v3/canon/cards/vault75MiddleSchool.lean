card vault75MiddleSchool : Spelled :=
  { name := "Vault 75: Middle School", cost := [generic 2, pip .white, pip .white],
    types := [.enchantment], subtypes := [saga],
    text :=
      [ when (chapterMark [1])
          (exile (allOf (and [isCreature, hasPower .atLeast 4]))),
        when (chapterMark [2, 3])
          (putCounters 1 p1p1Counter (each (and [isCreature, youControl]))) ] }
