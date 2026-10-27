/-- Fist of Suns -/
card fistOfSuns : Spelled :=
  { name := "Fist of Suns", cost := [generic 3], types := [.artifact],
    text :=
      [ static (altCost (allOf (and [isSpell, did you (cast which) thisObject]))
          [pip .white, pip .blue, pip .black, pip .red, pip .green]) ] }
