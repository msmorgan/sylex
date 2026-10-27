card bootsOfSpeed : Spelled :=
  { name := "Boots of Speed", cost := [pip .red], types := [.artifact],
    subtypes := [equipment],
    text :=
      [ static (conjunction [ getsPt (equipped creature) (up 1) (up 0),
            abilityGrant it haste ]),
        equip [generic 1] ] }
