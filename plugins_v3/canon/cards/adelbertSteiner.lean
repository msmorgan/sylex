card adelbertSteiner : Spelled :=
  { name := "Adelbert Steiner", cost := [generic 1, pip .white], supertypes := [.legendary],
    types := [.creature], subtypes := [human, knight],
    text :=
      [ lifelink,
        static (getsPt thisCreature
          (up (forEach 1 (and [equipment, youControl])))
          (up (forEach 1 (and [equipment, youControl])))) ],
    power := stat 2, toughness := stat 1 }
