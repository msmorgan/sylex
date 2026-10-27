card patricianGeist : Spelled :=
  { name := "Patrician Geist", cost := [generic 2, pip .blue], types := [.creature],
    subtypes := [spirit, knight],
    text :=
      [ flying,
        static (getsPt (allOf (and [spirit, youControl, isOther]))
          (up 1) (up 1)),
        static ((costShift (allOf (and [isSpell,
                      did you (cast which (from_ := yourGraveyard)) thisObject])) (less 1))) ],
    power := stat 2, toughness := stat 2 }
