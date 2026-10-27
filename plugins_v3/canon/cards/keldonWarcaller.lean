card keldonWarcaller : Spelled :=
  { name := "Keldon Warcaller", cost := [generic 1, pip .red], types := [.creature],
    subtypes := [human, warrior],
    text :=
      [ whenever (does thisCreature attack)
          (putCounters 1 loreCounter
            (target (and [saga, youControl]))) ],
    power := stat 2, toughness := stat 2 }
