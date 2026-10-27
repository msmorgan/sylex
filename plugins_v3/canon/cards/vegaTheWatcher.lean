card vegaTheWatcher : Spelled :=
  { name := "Vega, the Watcher", cost := [generic 1, pip .white, pip .blue],
    supertypes := [.legendary], types := [.creature],
    subtypes := [bird, spirit],
    text :=
      [ flying,
        whenever (does you (cast (a
          (and [isSpell, not
          (did any (cast which (from_ := yourHand)) thisObject)]))))
          (draw 1) ],
    power := stat 2, toughness := stat 2 }
