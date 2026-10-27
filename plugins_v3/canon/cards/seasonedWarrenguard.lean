/-- Seasoned Warrenguard -/
card seasonedWarrenguard : Spelled :=
  { name := "Seasoned Warrenguard", cost := [pip .white], types := [.creature],
    subtypes := [rabbit, warrior],
    text :=
      [ whenever (does thisCreature attack)
          (while_ := whileTrue (exists_ (and [isToken, youControl])))
          (get thisCreature (up 2) (up 0) untilEndOfTurn) ],
    power := stat 1, toughness := stat 2 }
