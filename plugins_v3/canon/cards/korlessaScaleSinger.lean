card korlessaScaleSinger : Spelled :=
  { name := "Korlessa, Scale Singer", cost := [pip .green, pip .blue],
    supertypes := [.legendary], types := [.creature],
    subtypes := [dragon, bard],
    text :=
      [ static (visibility .lookAt you topOfLibrary),
        static ((can you (cast (bare (and [isSpell, dragon])) (from_ := onTop)))) ],
    power := stat 1, toughness := stat 4 }
