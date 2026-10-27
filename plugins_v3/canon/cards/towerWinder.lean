/-- Tower Winder -/
card towerWinder : Spelled :=
  { name := "Tower Winder", cost := [generic 1, pip .green], types := [.creature],
    subtypes := [snake],
    text :=
      [ reach, deathtouch,
        when (does thisCreature enter)
          (sequentially
            [ searchLibraryOrGraveyard (a (named "Command Tower")),
              reveal it, putInto it hand,
              doIf (did you (search yourLibrary) thisWay) shuffle ]) ],
    power := stat 1, toughness := stat 1 }
