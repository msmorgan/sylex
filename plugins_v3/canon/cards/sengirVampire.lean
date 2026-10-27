card sengirVampire : Spelled :=
  { name := "Sengir Vampire", cost := [generic 3, pip .black, pip .black], types := [.creature],
    subtypes := [vampire],
    text :=
      [ flying,
        whenever
          (does (a (and [ isCreature,
                            did thisCreature (dealDamage anyAmount which) (within soFarThisTurn) ])) die)
          (putCounters 1 p1p1Counter thisCreature) ],
    power := stat 4, toughness := stat 4 }
