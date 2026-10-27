card dreadSlaver : Spelled :=
  { name := "Dread Slaver", cost := [generic 3, pip .black, pip .black],
    types := [.creature], subtypes := [zombie, horror],
    text :=
      [ whenever
          (does (a (and [ isCreature,
                            did thisCreature (dealDamage anyAmount which) (within soFarThisTurn) ])) die)
          (sequentially
            [ returnToBattlefield it [(under you)],
              become (that creature)
                { characteristics := { colors := [.black], subtypes := [zombie] } } ]) ],
    power := stat 3, toughness := stat 5 }
