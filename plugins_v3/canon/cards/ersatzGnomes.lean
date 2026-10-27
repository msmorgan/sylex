/-- Ersatz Gnomes -/
card ersatzGnomes : Spelled :=
  { name := "Ersatz Gnomes", cost := [generic 3], types := [.artifact, .creature],
    subtypes := [gnome],
    text :=
      [ activated tapSymbol
          (becomeColor (target isSpell) (.some [])),
        activated tapSymbol (becomeColor (target isPermanent) (.some []) untilEndOfTurn) ],
    power := stat 1, toughness := stat 1 }
