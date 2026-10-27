/-- Tuktuk the Explorer -/
card tuktukTheExplorer : Spelled :=
  { name := "Tuktuk the Explorer", cost := [generic 2, pip .red], supertypes := [.legendary],
    types := [.creature], subtypes := [goblin],
    text :=
      [ haste,
        when (does thisCreature die)
          (create 1 (written
            { characteristics :=
              { name := "Tuktuk the Returned", supertypes := [.legendary],
                types := [.artifact, .creature],
                subtypes := [goblin, golem],
                power := stat 5, toughness := stat 5 } })) ],
    power := stat 1, toughness := stat 1 }
