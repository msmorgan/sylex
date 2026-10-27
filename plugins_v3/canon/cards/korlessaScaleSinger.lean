card korlessaScaleSinger : Spelled :=
  { name := "Korlessa, Scale Singer", cost := [pip .green, pip .blue],
    supertypes := [.legendary], types := [.creature],
    subtypes := [dragon, bard],
    text :=
      [ static (visibility .lookAt you topOfLibrary),
        static (castFromTop (and [isSpell, dragon])) ],
    power := stat 1, toughness := stat 4 }
