/-- Market Gnome -/
card marketGnome : Spelled :=
  { name := "Market Gnome", cost := [pip .white], types := [.artifact, .creature],
    subtypes := [gnome],
    text :=
      [ when (does thisCreature die) (sequentially [gainLife 1, draw 1]),
        when (does any (exile thisCreature))
          (while_ := whileDoing (does you (activate (a (abilityHead (.keyword craft))))))
          (sequentially [gainLife 1, draw 1]) ],
    power := stat 0, toughness := stat 3 }
