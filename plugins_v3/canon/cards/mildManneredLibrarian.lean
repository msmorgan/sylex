card mildManneredLibrarian : Spelled :=
  { name := "Mild-Mannered Librarian", cost := [pip .green], types := [.creature],
    subtypes := [human],
    text :=
      [ activatedOnlyOnce [generic 3, pip .green]
          (sequentially
            [ establish
                (qualityChange thisCreature .sets
                  (bundle { characteristics := { subtypes := [werewolf] } })),
              putCounters 2 p1p1Counter it,
              draw 1 ])
          oncePerGame ],
    power := stat 1, toughness := stat 1 }
