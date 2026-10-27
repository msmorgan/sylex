card lesserGargadon : Spelled :=
  { name := "Lesser Gargadon", cost := [generic 2, pip .red, pip .red],
    types := [.creature], subtypes := [beast],
    text :=
      [ whenever (does thisCreature attack) (or_ := [does thisCreature block])
          (sacrifice (a isLand)) ],
    power := stat 6, toughness := stat 4 }
