card carrionGrub : Spelled :=
  { name := "Carrion Grub", cost := [generic 3, pip .black], types := [.creature],
    subtypes := [insect],
    text :=
      [ static (conjunction [ getsPt thisCreature (up (letter .x)) (up 0),
            letterDefinition .x
              (aggregate .max (.stat .power) (and [isCreature, in_ yourGraveyard])) ]),
        when (does thisCreature enter) (mill 4) ],
    power := stat 0, toughness := stat 5 }
