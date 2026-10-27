card gandalfWhiteRider : Spelled :=
  { name := "Gandalf, White Rider", cost := [generic 3, pip .white],
    supertypes := [.legendary], types := [.creature],
    subtypes := [avatar, wizard],
    text :=
      [ vigilance,
        whenever (does you (cast (a isSpell)))
          (sequentially
            [ get (each (and [isCreature, youControl])) (up 1) (up 0) untilEndOfTurn,
              scry 1 ]),
        when (does thisCreature die) (may (putInto it (fromTop 5 library))) ],
    power := stat 3, toughness := stat 3 }
