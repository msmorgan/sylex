/-- Arcum's Weathervane -/
card arcumsWeathervane : Spelled :=
  { name := "Arcum's Weathervane", cost := [generic 2], types := [.artifact],
    text :=
      [ activated (compound [[generic 2], tapSymbol])
          (establish
            (qualityChange (target (and [isLand, hasSupertype .snow])) .loses
              (bundle { characteristics := { supertypes := [.snow] } }))),
        activated (compound [[generic 2], tapSymbol])
          (establish
            (qualityChange (target (and [isLand, hasSupertype .basic, not (hasSupertype
                .snow)])) .adds
              (bundle { characteristics := { supertypes := [.snow] } }))) ] }
