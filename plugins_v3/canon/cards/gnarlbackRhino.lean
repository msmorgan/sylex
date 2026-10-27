/-- Gnarlback Rhino -/
card gnarlbackRhino : Spelled :=
  { name := "Gnarlback Rhino", cost := [generic 2, pip .green, pip .green], types := [.creature],
    subtypes := [rhino],
    text :=
      [ trample,
        whenever (does you (cast (a
          (and [isSpell, targeting thisCreature
          .someTarget]))))
          (draw 1) ],
    power := stat 4, toughness := stat 4 }
