card fyndhornDruid : Spelled :=
  { name := "Fyndhorn Druid", cost := [generic 2, pip .green], types := [.creature],
    subtypes := [elf, druid],
    text :=
      [ when (does thisCreature die) (if_ := some (was (becomeBlocked it) soFarThisTurn))
          (gainLife 4) ],
    power := stat 2, toughness := stat 2 }
