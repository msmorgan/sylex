/-- Melek, Izzet Paragon -/
card melekIzzetParagon : Spelled :=
  { name := "Melek, Izzet Paragon", cost := [generic 4, pip .blue, pip .red],
    supertypes := [.legendary], types := [.creature],
    subtypes := [weird, wizard],
    text :=
      [ static (visibility .reveal you topOfLibrary),
        static
          (can you (cast (bare (and [isSpell, isInstantOrSorcery])) (from_ := onTop))),
        whenever
          (does you (cast (a (and
            [isInstantOrSorcery, isSpell])) (from_ := yourLibrary)))
          (sequentially
            [ copyOnStack it,
              may (chooseNewTargets (that .copy)) ]) ],
    power := stat 2, toughness := stat 4 }
