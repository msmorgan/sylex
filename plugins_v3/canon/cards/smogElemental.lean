card smogElemental : Spelled :=
  { name := "Smog Elemental", cost := [generic 4, pip .black, pip .black],
    types := [.creature], subtypes := [elemental],
    text :=
      [ flying,
        static (getsPt
          (allOf (and [ isCreature, hasKeyword flying,
                         opponentsControl ]))
          (down 1) (down 1)) ],
    power := stat 3, toughness := stat 3 }
