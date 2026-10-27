/-- Energybending -/
card energybending : Spelled :=
  { name := "Energybending", cost := [generic 2], types := [.instant],
    subtypes := [lesson],
    text :=
      [ spell (sequentially
          [ establishThroughout (qualityChange (allOf (and [isLand, youControl])) .adds
                (everyTypeOf .basicLand)) untilEndOfTurn,
            draw 1 ]) ] }
