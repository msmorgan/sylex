card := {
  name := "Arcum's Weathervane",
  cost := [2],
  types := [artifact],
  text := [
    activated (compound [[2], tapSymbol])
      (establish
         (qualityChange (target [isLand, hasSupertype snow]) loses (bundle { supertypes := [snow] }))),
    activated (compound [[2], tapSymbol])
      (establish
         (qualityChange (target [isLand, hasSupertype basic, not (hasSupertype snow)]) adds
            (bundle { supertypes := [snow] })))
  ]
}
