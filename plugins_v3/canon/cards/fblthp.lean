/-- Fblthp, the Lost -/
card fblthp : Spelled :=
  { name := "Fblthp, the Lost", cost := [generic 1, pip .blue],
    supertypes := [.legendary], types := [.creature], subtypes := [homunculus],
    text :=
      [ when (does this_ enter)
          (replace (draw 1)
            (doIf (or
                [ did it (enter (from_ := yourLibrary)) triggering,
                  is it (did any (cast which (from_ := yourLibrary)) thisObject) ])
              (instead (draw 2)))),
        when (does (a isSpell) (target this_)) (shuffleInto this_) ],
    power := stat 1, toughness := stat 1 }
